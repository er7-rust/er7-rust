[er7](../../index.md) → [docs](../) → faq

# Frequently asked questions

## What is ER7?

The original text encoding for HL7® v2 messages: pipe-delimited, positional,
compact. Also called *pipe-hat*, after `|` and `^`. It is defined in
chapter 2 of every HL7 v2 release and is still what nearly every production
healthcare interface speaks. The format itself is described in
[spec §2](../../spec/02-er7-encoding/index.md).

## Does this crate validate messages?

No, and that is deliberate. It does not check cardinality, required fields,
lengths, data types, or code-table membership.

A *partial* validator is worse than none, because it implies the messages it
passes are correct. Real validation needs the HL7 dictionary for the
message's version, which is exactly what this crate does not carry.

## Why doesn't it know what `PID-5` means?

Because `PID-5` means different things in different HL7 versions, and
carrying the dictionaries for v2.1 through v2.9 would mean shipping one
version's tables to everyone or all of them to everyone.

Keeping the encoding separate lets a dictionary crate choose its own
version, and lets a user who only needs to route or audit messages pay for
nothing. The reasoning in full is
[spec §18.1](../../spec/18-open-questions-and-divergences/index.md).

The layers above this one ship as their own crates —
[`er7-redact`](https://crates.io/crates/er7-redact) for redaction,
[`serde-er7`](https://crates.io/crates/serde-er7) for Serde support,
[`hl7-2-5-to-xml`](https://crates.io/crates/hl7-2-5-to-xml) and
[`hl7-2-5-to-json`](https://crates.io/crates/hl7-2-5-to-json) for the HL7
v2.5 dictionary. The full list is
[spec §1.3.1](../../spec/01-purpose-and-scope/index.md).

## Then why does it read MSH-9 and MSH-10?

Five accessors — `message_code`, `trigger_event`, `message_structure`,
`control_id`, `version` — are the single documented exception, on two
grounds that both have to hold: every tool needs them to route or log a
message, and those positions have not moved in any v2 release, so reading
them requires no version knowledge.

MSH-3 through MSH-7 fail the first test; deriving MSH-9.3 from MSH-9.1 and
MSH-9.2 fails the second. See
[spec §10](../../spec/10-msh-conveniences/index.md).

## Will it change my message?

No. Canonical input comes back byte for byte
([spec §7.2](../../spec/07-writing/index.md)). Text is stored exactly as it
arrived and decoded only when you ask for a value.

Parsing normalizes exactly two things, and nothing else: blank lines are
dropped, and segment terminators become whatever `RenderOptions` chose. If
your input already has no blank lines and consistent terminators, the output
is identical.

## Why does `to_er7()` not end with a terminator?

Because a trailing terminator surprises callers that compare or concatenate
the result, and the transport — MLLP, or a file — already marks where the
message ends.

Set `RenderOptions { trailing_terminator: true, .. }` for strict wire
output. The CLI's `--normalize` does exactly that.

## Why does a message print as one line?

ER7 terminates segments with a carriage return, so a terminal draws them on
top of each other. Use `--terminator lf` on the CLI, or
`Terminator::Lf` in `RenderOptions`, when you want to read it.

## What is the difference between `to_er7` and `to_text`?

`to_er7` gives you what a receiver would read — escape sequences intact.
`to_text` decodes the leaf text but keeps the structural delimiters, so
`SMITH^JOHN` stays `SMITH^JOHN` and `a\T\b` becomes `a&b`.

Use `to_er7` for anything that will be sent, stored, or parsed again; use
`to_text` for display, logging, and database writes. `to_text` output is
**not** re-parseable: a decoded `\F\` becomes a literal `|`.

## What is the difference between an empty field and `""`?

Everything, clinically.

| On the wire | Means | A receiver must |
| ----------- | ----- | --------------- |
| the field is absent | no information | leave the stored value alone |
| `\|\|` | present, no value | leave the stored value alone |
| `\|""\|` | explicit null | **clear** the stored value |

Ask `is_null()` when it matters. A query decodes a null to the empty
string, since that is the value being conveyed, so the query alone cannot
tell you. See [spec §5.3](../../spec/05-value-tree/index.md).

## `query` returned fewer results than I expected

A position the message does not carry contributes **nothing** — no entry in
the vector, not an empty string. So `query_all("OBX-5")` on three `OBX`
segments returns two values if only two carried a fifth field.

That is usually what you want, and occasionally a surprise. Iterate
`segments_named("OBX")` if you need one entry per segment regardless.

## Why is `PID-13` one value but `PID-13.1` two?

A path that stops at the field returns the whole field, repetition
separators included — `555-1111~555-2222`. A path that goes deeper splits
into one answer per repetition.

`PID-13` as a whole field is a meaningful thing to ask for, and joining its
repetitions back with `~` is the only honest way to return it as one
string. See [docs/paths](../paths/index.md).

## Does it handle messages that don't use `|^~\&`?

Yes. The delimiter set is read from MSH-1 and MSH-2, never assumed, and
every function that needs delimiters takes them. `MSH#*!?@#…` parses,
queries, and round-trips exactly like a conventional message.

The v2.7 truncation character (MSH-2 position 5, conventionally `#`) is
read too, as `Option<char>`.

## Why doesn't `\.br\` get decoded?

Because there is nothing honest to decode it *to*. Display commands,
highlighting, character-set switches, and local `\Z..\` sequences say
something about presentation or encoding that a plain `String` cannot carry
— dropping them would lose information, and guessing would invent it.

They are preserved exactly as written, and `er7::escape::escapes` gives you
the classified token stream if you want to render them yourself. There is a
worked example in [docs/escapes](../escapes/index.md).

## Can I build a message from scratch?

Yes, two ways. If you can already write the content as ER7 — the usual
case, since an ACK is mostly values the inbound message already carries —
`er7::parse_with` turns that text into segments; parsing is the builder.
For a value that is not text yet, every field of every type is `pub`, so
struct literals work directly. There is no separate builder API, and
[§5.5](../../spec/05-value-tree/index.md) says why not.
`examples/build_a_message.rs` works a full ACK both ways.

## Does it do MLLP?

No. MLLP framing — the 0x0B / 0x1C 0x0D bytes around a message on a TCP
connection — is transport, and this crate handles the bytes between the
frames. Strip the framing before parsing.

## Does it have any dependencies?

None, and that is a guarantee enforced by a test
([spec §15.1](../../spec/15-dependencies-and-build/index.md)). No runtime
dependencies, no dev-dependencies, no features. Healthcare integration code
gets audited, and this crate is meant to sit at the bottom of a stack.

## What Rust version does it need?

Rust 1.95 or later. The workspace policy is **N-3** — the minimum supported
version is whatever stable Rust is today, minus three releases — so the
floor moves forward roughly twice a year rather than being frozen
([spec §14.4](../../spec/14-compatibility-and-versioning/index.md)).
Edition 2024 sets a hard floor of 1.85 underneath that, which no longer
binds. There is no `no_std` support — the crate uses `String` and `Vec`
throughout.

## Is it fast?

Parsing and writing are each a single pass, O(n). `unescape` and `escape`
return `Cow::Borrowed` when there is nothing to do, and `split_messages`
returns borrowed slices rather than copies.

There are no benchmarks; whether they would earn their keep is tracked as
[T3](../../spec/17-open-tasks/index.md). If you have a workload where this
matters, please open an issue with the numbers.

## How do I report a bug or ask for a feature?

Open a GitHub issue at
<https://github.com/er7-rust/er7-rust>. Please include a
**synthetic** message that reproduces the problem — never a real one, even
redacted.

If the answer turns out to be "this is deliberate", it will be recorded in
[spec §18](../../spec/18-open-questions-and-divergences/index.md) so the
next person finds the reasoning rather than re-asking.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
