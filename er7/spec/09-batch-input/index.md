[`er7` specification](../index.md) — section 9 of 19. Section numbers
(§9.x) are stable and cited from code, tests, and commit messages.

# 9. Batch and multi-message input

Implemented in `src/parse.rs`. Background in
[§2.6](../02-er7-encoding/index.md).

## 9.1 `split_messages` [R21]

```rust
pub fn split_messages(text: &str) -> Vec<&str>
```

Cuts input holding several messages, or a whole HL7® batch file, into the
individual messages.

The returned slices **borrow from the input** and keep its original segment
terminators, so each one can be handed straight to
[`parse`](../04-parsing/index.md) with no copy and no change of meaning.
This is why the return type is `Vec<&str>` rather than `Vec<String>`.

**[R21]** The rules:

| Line | Effect |
| ---- | ------ |
| segment name `MSH` | begins a new message |
| segment name `FHS`, `BHS`, `BTS`, `FTS` | dropped; ends the message in progress and begins nothing |
| the first surviving line, whatever it is | begins a message |
| anything else | continues the message in progress |

A leading byte-order mark is removed first, and blank lines are skipped, as
in [§4.1](../04-parsing/index.md).

## 9.2 Why envelope segments are dropped

`FHS`, `BHS`, `BTS`, and `FTS` describe the **file**, not any message in
it: who sent the batch, how many messages it holds, when it was written. A
caller who wants the messages does not want them, and a caller who does
want them can read the raw text.

The name is matched **exactly** — the leading run of letters and digits
([§4.2](../04-parsing/index.md)) — so a local segment such as `BTSX` is not
mistaken for a batch trailer. The three-letter prefix alone would not be
safe.

## 9.3 Why a headerless first line still starts a message

If the first surviving line is not an `MSH`, it still opens a message,
which `parse` then rejects with `Error::MissingHeader`
([§11](../11-error-handling/index.md)).

The alternative — silently dropping everything before the first `MSH` —
would turn a malformed file into a quietly shorter list of messages. A
caller counting messages, or reconciling against a `BTS` count, would get
a wrong answer with no signal. Reporting it is better.

## 9.4 What this is not

`split_messages` does **not** unframe MLLP
([§2.7](../02-er7-encoding/index.md)). Input is expected to be text; a
caller reading from a socket strips the 0x0B / 0x1C 0x0D framing bytes
first. Transport is out of scope (R24).

It also does not validate the batch: a `BTS` message count that disagrees
with the number of `MSH` segments is not checked, because checking is out
of scope (R24) and because the count would be reported through an error
type this crate does not want to grow
([§11](../11-error-handling/index.md)).

The same two points hold for `read_messages` ([§9.5](#95-streaming-input-read_messages)) as well: neither reads
from a socket nor validates a batch count.

## 9.5 Streaming input: `read_messages` [R27]

```rust
pub fn read_messages<R: BufRead>(reader: R) -> MessageReader<R>
```

`split_messages` needs the whole input in one `&str` before it can cut
anything. A production batch file can reach hundreds of megabytes, and a
caller who only wants to process each message in turn should not have to
hold the whole file in memory to do it ([T4](../17-open-tasks/index.md),
before it closed).

`read_messages` reads from anything that implements `std::io::BufRead` —
a file, a socket, standard input — and returns `MessageReader<R>`, which
implements `Iterator<Item = std::io::Result<String>>`. Each item is one
message's ER7 text. Memory use is bounded by the message currently being
assembled, not by the size of the input.

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::io::Cursor;

let batch = "FHS|^~\\&|SENDER\rMSH|^~\\&|A\rMSA|AA|1\rMSH|^~\\&|B\rMSA|AA|2\rFTS|2";
for source in er7::read_messages(Cursor::new(batch)) {
    let message = er7::parse(&source?)?;
    println!("{:?}", message.control_id());
}
# Ok(())
# }
```

**[R27]** Same batch rules as `split_messages` (R21): a line named `MSH`
starts a new message; `FHS`, `BHS`, `BTS`, `FTS` end the message in
progress and start none; the first surviving line starts a message
whatever it is ([§9.3](#93-why-a-headerless-first-line-still-starts-a-message)); a leading byte-order mark is stripped once, at the very
start of the stream. Blank lines are dropped, exactly as in
[§4.1](../04-parsing/index.md).

An `Err` item ends the iteration for good: the next call to `next()`
returns `None` rather than trying to resume, because an I/O failure or a
line that is not valid UTF-8 leaves no reliable place to pick back up. A
message that was only partly read when the error happened is discarded,
not returned.

### 9.5.1 Why the returned text is owned, not borrowed

`split_messages` borrows: it returns `&str` slices of the caller's own
buffer, which costs nothing beyond a `Vec` of spans — possible only because
the whole input already sits in one contiguous `&str`.

A `BufRead` offers no such thing. Its internal buffer is refilled and
overwritten as bytes are consumed, so nothing borrowed from it survives
past the next read. `read_messages` rejoins each message's segments into an
owned `String`, with `\r` (`Terminator::Cr`, [§7](../07-writing/index.md)'s
own default) between them — never the original terminator bytes, since
nothing here keeps the original text around to copy them from. Parsing the
result gives back the same tree `split_messages` plus `parse` would have
produced; only the terminator character between segments can differ, never
a segment, a field, or a decoded value.

This closes the open sub-question T4 was scheduled with: the streaming
form yields `String`, not a zero-copy slice, and `split_messages` remains
the borrowing, whole-input-in-memory form for a caller who already has one.
Choosing between them is choosing between memory and a copy — not a choice
this crate can make for every caller, which is why both exist.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
