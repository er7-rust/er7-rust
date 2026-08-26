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

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
