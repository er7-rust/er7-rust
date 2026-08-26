# serde-er7

Serde support for [`er7`](https://github.com/er7-rust/er7-rust/tree/main/er7), the
pipe-hat encoding that carries HL7® v2 messages between healthcare systems —
so a parsed message can flow through JSON, YAML, or any other Serde data
format, and come back out unchanged.

```rust
let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
            PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";

let message = serde_er7::Message::parse(text)?;

let json = serde_json::to_string(&message)?;
let back: serde_er7::Message = serde_json::from_str(&json)?;

assert_eq!(back.to_er7(), text);
```

## Contents

- [Why this crate exists](#why-this-crate-exists)
- [Install](#install)
- [The shape each level serializes as](#the-shape-each-level-serializes-as)
- [What it does](#what-it-does)
- [What it deliberately does not do](#what-it-deliberately-does-not-do)
- [Documentation](#documentation)
- [Development](#development)
- [License](#license)

## Why this crate exists

`er7` parses, queries, edits, and writes ER7 text with zero dependencies of
its own — a deliberate choice for code that sits at the bottom of a stack of
HL7 crates. That means `er7::Message` has no Serde support built in, and
adding it there would cost every user of `er7` a dependency they may not
want.

This crate is the bridge instead: two dependencies, `serde` and `er7`, and
nothing else. Every `Serialize`/`Deserialize` impl is written by hand
against the low-level trait methods — the same pattern
[serde's own documentation](https://docs.rs/serde/latest/serde/) walks
through for a manual implementation — because the shapes below (a bare
array for a field's repetitions, a bare string for a leaf) are not what
`#[derive(Serialize)]` would produce on its own.

## Install

```sh
cargo add serde-er7
```

## The shape each level serializes as

| Level | Wrapper | Serializes as |
|-------|---------|----------------|
| Message | `Message` | object: `{"separators": ..., "segments": [...]}` |
| Segment | `Segment` | object: `{"name": "PID", "fields": [...]}` |
| Field | `Field` | array of repetitions |
| Repetition | `Repetition` | array of components |
| Component | `Component` | array of subcomponent strings |
| Subcomponent | `Subcomponent` | a bare string, `raw` (escape sequences intact, not decoded) |
| Separators | `Separators` | object of six named fields, chars as one-character strings |
| Terminator | `Terminator` | one of the strings `"Cr"`, `"Lf"`, `"CrLf"` |

So `PID-5.1` (`SMITH`) is the bare JSON string `"SMITH"`, `PID-5`
(`SMITH^JOHN`) is `["SMITH", "JOHN"]`, and a repeating field such as
`555-1111~555-2222` is `[["555-1111"], ["555-2222"]]` — one array level per
level of the tree, all the way up to the two objects at the top:
[`Segment`](https://docs.rs/serde-er7/latest/serde_er7/struct.Segment.html)
and
[`Message`](https://docs.rs/serde-er7/latest/serde_er7/struct.Message.html)
itself.

## What it does

- **Every level of the tree**: `Message`, `Segment`, `Field`, `Repetition`,
  `Component`, `Subcomponent`, plus `Separators` and `Terminator` — a
  Serde-enabled wrapper for every public type `er7` exposes.
- **Format-agnostic**: nothing in this crate mentions JSON, YAML, or any
  other format by name. `serde_json` appears only as a dev-dependency, to
  test and demonstrate against.
- **Round trip, the same guarantee `er7` makes**: every subcomponent
  serializes as its `raw` text, escape sequences included, not decoded — so
  `Message::parse(text)?` through any Serde format and back out through
  `.to_er7()` reproduces the same bytes `er7::parse(text)?.to_er7()` would.
- **Absent, empty, and null stay distinct**: a field that was never sent, a
  field sent as `||`, and a field holding the explicit `""` null serialize
  and deserialize as three different values, never collapsed into one.
- **Ergonomic wrappers, not just trait impls**: every wrapper implements
  `Deref`/`DerefMut` to its `er7` type, plus `From` conversions both ways,
  so `message.query(...)`, `message.segments`, and the rest of `er7`'s API
  work directly on a `Message` without unwrapping it first.

## What it deliberately does not do

This crate adds exactly one thing to `er7`: Serde support for its existing
value tree. Like `er7` itself, it is an encoding bridge, not a dictionary —
it does not know which fields a segment should have, what data type each
carries, or what any code table means. It does not validate, and it does
not pick a wire format for you: that choice — `serde_json`, `serde_yaml`,
anything else — is the caller's, every time.

For the HL7 v2.5 dictionary layer, see the sibling crates
[`hl7-2-5-to-xml-using-rust`](https://github.com/hl7-rust/hl7-2-5-to-xml-using-rust)
and
[`hl7-2-5-to-json-using-rust`](https://github.com/hl7-rust/hl7-2-5-to-json-using-rust).

## Documentation

| Where | What |
| ----- | ---- |
| [`docs/usage/`](docs/usage/index.md) | tutorial: parsing, JSON in both directions, and the tree shapes |
| [`docs/api/`](docs/api/index.md) | the complete public API surface |
| [`docs/faq/`](docs/faq/index.md) | frequently asked questions |
| [`examples/`](examples/README.md) | runnable programs, one concept each |
| [`spec/`](spec/index.md) | the normative specification — source of truth for behaviour |
| [`AGENTS.md`](AGENTS.md) | conventions and required checks for anyone, human or agent, changing this code |
| [`AGENTS/`](AGENTS/index.md) | the topical guides: architecture, conventions, testing, safety, workflows, release, spec-driven development |

Rendered API docs are at <https://docs.rs/serde-er7/>, or locally with
`cargo doc --no-deps --open`. A tutorial-style version of the same material
is at <https://er7-rust.github.io/serde-er7/>.

## The crate family

| Crate | Adds |
| ----- | ---- |
| [`er7`](https://crates.io/crates/er7) | the encoding itself: parse, query, edit, and write ER7, with zero dependencies |
| **`serde-er7`** | this crate: Serde support for every type in that tree |
| [`er7-redact`](https://crates.io/crates/er7-redact) | redaction: remove patient detail without changing the shape of the message |
| [`hl7-2-5-to-xml`](https://crates.io/crates/hl7-2-5-to-xml) / [`hl7-2-5-to-json`](https://crates.io/crates/hl7-2-5-to-json) | the HL7 v2.5 dictionary: data types, message structures, and a renderer |

The boundary between them is `er7`'s own spec §1.3.1, and they are
presented together at <https://er7-rust.github.io/ecosystem/>.

## Development

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lint-clean
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
cargo run --example round_trip_via_json   # try an example
```

Behavioural changes start in [`spec/`](spec/index.md), not in the code —
see [`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

## Contributing, conduct, and security

The workspace-level documents at the repository root cover this crate,
deliberately — one copy cannot drift:
[CONTRIBUTING.md](https://github.com/er7-rust/er7-rust/blob/main/CONTRIBUTING.md)
(including how to report a problem without pasting patient data),
[CODE_OF_CONDUCT.md](https://github.com/er7-rust/er7-rust/blob/main/CODE_OF_CONDUCT.md),
and
[SECURITY.md](https://github.com/er7-rust/er7-rust/blob/main/SECURITY.md).

## License

Multi-licensed, so a downstream project can pick whichever fits: MIT,
Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only. See
[LICENSE.md](LICENSE.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
