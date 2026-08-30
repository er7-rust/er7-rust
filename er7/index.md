# er7

**[website](https://er7-rust.github.io/)**
•
**[documentation](https://docs.rs/er7/)**
•
**[source](https://github.com/er7-rust/er7-rust)**
•
**[crate](https://crates.io/crates/er7)**
•
**[email](mailto:joel@joelparkerhenderson.com)**

Parse, query, edit, and write HL7® v2 messages in the **ER7** pipe-hat
encoding — as a Rust library and a command-line tool, with zero
dependencies.

```
MSH|^~\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5
PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F
```

ER7 is compact, ubiquitous, and unforgiving: a value's meaning comes
entirely from its position, so one misplaced `|` silently shifts everything
after it. This crate makes that structure explicit and keeps it intact.

## Contents

- [Install](#install)
- [Command line](#command-line)
- [Library](#library)
- [What it does](#what-it-does)
- [What it deliberately does not do](#what-it-deliberately-does-not-do)
- [Documentation](#documentation)
- [Development](#development)
- [License](#license)

## Install

```sh
cargo add er7
```

Or for the command-line tool:

```sh
cargo install er7
```

## Command line

```sh
# Show every value with the HL7 path that names it
er7 samples/oru_r01.er7
```

```
MSH-1       |
MSH-2       ^~\&
MSH-3       LAB
MSH-9.1     ORU
MSH-9.2     R01
MSH-9.3     ORU_R01
MSH-10      MSG00042
MSH-11      P
MSH-12      2.5
PID-3.1     444333222
PID-3.4.1   ACME
PID-3.4.2   1.2.840.114398.1.100
PID-5.1     EVERYWOMAN
PID-5.2     EVE
PID-13[1]   555-555-1111
PID-13[2]   555-555-2222
OBX[1]-3.2  Cholesterol
OBX[1]-5    187
OBX[2]-3.2  Triglycerides
```

Every label in that outline is a valid query, so you can read a path off the
output and paste it straight back in:

```sh
# Pull out specific values
er7 --query PID-5.1 --query OBX-5 samples/oru_r01.er7

# Show text exactly as sent, without decoding escape sequences
er7 --raw --query OBX-5 samples/oru_r01.er7

# Rewrite as canonical ER7, with line feeds so a terminal can show it
er7 --normalize --terminator lf samples/oru_r01.er7

# Read a batch file, take its second message
cat samples/batch.er7 | er7 --message 2
```

`er7 --help` lists the rest. The full contract is
[spec §12](spec/12-command-line-interface/index.md).

## Library

```rust
let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01|MSG00042|P|2.5\r\
            PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";

let message = er7::parse(text)?;

assert_eq!(message.control_id().as_deref(), Some("MSG00042"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));

// What went in comes back out, byte for byte.
assert_eq!(message.to_er7(), text);
```

`query` returns the first match and `query_all` returns every one, so
`OBX-5` across a result with three observations gives three values.

Editing goes through `set`, which encodes delimiters on the way in, so a
value can never break the structure that holds it:

```rust
let separators = message.separators;
message
    .segment_at_mut("PID", 1).unwrap()
    .field_mut(5).unwrap()
    .repetition_mut(1).unwrap()
    .component_mut(1).unwrap()
    .subcomponent_mut(1).unwrap()
    .set("O'BRIEN & SONS", &separators);

assert!(message.to_er7().contains(r"O'BRIEN \T\ SONS"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("O'BRIEN & SONS"));
```

There is a runnable program for each of these in
[`examples/`](examples/README.md), and a step-by-step walk-through in
[`docs/usage/`](docs/usage/index.md).

## What it does

- **Full hierarchy**: message, segment, field, repetition (`~`), component
  (`^`), subcomponent (`&`).
- **Delimiters from the message**: MSH-1 and MSH-2 are read, never assumed,
  including the truncation character HL7 v2.7 added. A message that uses
  `#*!?@` parses as happily as one that uses `|^~\&`.
- **Round trip, byte for byte**: text is stored exactly as it arrived and
  decoded only when you ask for a value, so a message survives a trip
  through this crate unchanged — unusual delimiters, unknown segments,
  empty positions, escape sequences and all.
- **Escape sequences**: the whole vocabulary — `\F\ \S\ \T\ \R\ \E\`,
  `\Xdd..\`, `\H\`, `\N\`, `\Zdd..\`, `\Cxxyy\`, `\Mxxyyzz\`, `\.br\` — is
  tokenized and classified. The ones that stand for characters decode; the
  ones that describe presentation are preserved as written. See
  [`docs/escapes/`](docs/escapes/index.md).
- **HL7 paths**: `PID-5.1`, `OBX[2]-5`, `PID-13[2].1`, in either the
  `PID-5.1` or `PID.5.1` spelling. See
  [`docs/paths/`](docs/paths/index.md).
- **Absent, empty, and null** are three different answers, not one — the
  explicit `""` means *clear this value*, and losing that distinction
  corrupts patient records.
- **Batch files**: `FHS`/`BHS`/`BTS`/`FTS` envelopes are recognized and
  messages come out one at a time, as borrowed slices of the input
  (`split_messages`) or streamed from a `BufRead` without holding the
  whole file in memory (`read_messages`).
- **Nothing fails except a missing header**: unknown segments, local `Z`
  segments, ragged field counts, and stray positions are data, not errors.

## What it deliberately does not do

This crate is an **encoding, not a dictionary**. It does not know which
fields a segment has, what data type each carries, which message structures
exist, or what any code table means — all of that is version-specific and
belongs in a layer above. It performs no validation, and no transport
(MLLP framing is a separate concern).

The one exception is a handful of `MSH` accessors — `message_code`,
`trigger_event`, `message_structure`, `control_id`, `version` — because
routing a message requires reading them and those positions have never
moved in any HL7 v2 release. The reasoning, and what was declined, is in
[spec §10](spec/10-msh-conveniences/index.md).

### The crate family

`er7` is the bottom of a stack; each layer above it is a separate crate, so
a caller pays only for what they use.

| Crate | Adds |
| ----- | ---- |
| [`er7-redact`](https://crates.io/crates/er7-redact) | redaction: remove patient detail without changing the shape of the message |
| [`serde-er7`](https://crates.io/crates/serde-er7) | Serde support, so a message tree can travel as JSON, YAML, or any other format |
| [`hl7-2-from-er7-into-xml`](https://crates.io/crates/hl7-2-from-er7-into-xml) / [`hl7-2-from-er7-into-json`](https://crates.io/crates/hl7-2-from-er7-into-json) | the HL7 v2.5 dictionary: data types, message structures, and a renderer |

All four are presented together at
<https://er7-rust.github.io/ecosystem/>, and the boundary between them is
[spec §1.3.1](spec/01-purpose-and-scope/index.md).

## Documentation

| Where | What |
| ----- | ---- |
| [`docs/usage/`](docs/usage/index.md) | tutorial: from a string to values, edits, and back |
| [`docs/paths/`](docs/paths/index.md) | HL7 path notation, in full |
| [`docs/escapes/`](docs/escapes/index.md) | escape sequences, with worked examples |
| [`docs/api/`](docs/api/index.md) | the complete public API surface |
| [`docs/faq/`](docs/faq/index.md) | frequently asked questions |
| [`examples/`](examples/README.md) | runnable programs, one concept each |
| [`spec/`](spec/index.md) | the normative specification — source of truth for behaviour |
| [`spec/02-er7-encoding/index.md`](spec/02-er7-encoding/index.md) | the ER7 format itself, independent of this crate |
| [`AGENTS.md`](AGENTS.md) | conventions and required checks for anyone, human or agent, changing this code |
| [`samples/`](samples/) | example messages: a lab result, an admission update with a `Z` segment, a batch file |

Rendered API docs are at <https://docs.rs/er7/>, or locally with
`cargo doc --no-deps --open`. The same material, presented for the web, is
at <https://er7-rust.github.io/>.

## Development

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lint-clean
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
cargo run -- samples/oru_r01.er7          # try the CLI
cargo run --example parse_a_message       # try an example
```

Behavioural changes start in [`spec/`](spec/index.md), not in the code —
see
[`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

## Contributing, conduct, and security

The workspace-level documents at the repository root cover this crate,
deliberately — one copy cannot drift:
[CONTRIBUTING.md](https://github.com/er7-rust/er7-rust/blob/main/CONTRIBUTING.md)
(including how to report a problem without pasting patient data),
[CODE_OF_CONDUCT.md](https://github.com/er7-rust/er7-rust/blob/main/CODE_OF_CONDUCT.md),
and
[SECURITY.md](https://github.com/er7-rust/er7-rust/blob/main/SECURITY.md).
This crate's own [CONTRIBUTING.md](CONTRIBUTING.md) is a pointer at the
root one.

## License

Multi-licensed, so a downstream project can pick whichever fits: MIT,
Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only. See
[LICENSE.md](LICENSE.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
