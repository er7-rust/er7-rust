# er7-redact

**[website](https://er7-rust.github.io/er7-redact/)**
•
**[documentation](https://docs.rs/er7-redact/)**
•
**[source](https://github.com/er7-rust/er7-rust/tree/main/er7-redact)**
•
**[crate](https://crates.io/crates/er7-redact)**
•
**[email](mailto:joel@joelparkerhenderson.com)**

Remove patient detail from HL7® v2 messages in the **ER7** pipe-hat
encoding — as a Rust library and a command-line tool — **without breaking
the message**.

```
PID|1||PATID1234^5^M11^ADT1^MR^MCM||JONES^WILLIAM^A^III||19610615|M||C|1200 N ELM STREET^^GREENSBORO^NC^27401-1020
```

becomes

```
PID|1||11a9d74f8a6a54a7^5^M11^ADT1^MR^MCM||REDACTED^REDACTED^REDACTED^REDACTED||1961|M||C|^^^^
```

Same segments, same fields, same components, same delimiters. Every path
that resolved to a value still resolves to one, so the interface engine,
the test harness, and the message viewer downstream all behave the way they
did on the original.

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
cargo add er7-redact
```

Or for the command-line tool:

```sh
cargo install er7-redact
```

## Command line

```sh
# Redact with the built-in policy
er7-redact samples/adt_a08.er7
```

```
MSH|^~\&|ADT1|MCM|LABADT|MCM|20260815140000||ADT^A08^ADT_A01|MSG00001|P|2.5
EVN|A08|20260815140000
PID|1||11a9d74f8a6a54a7^5^M11^ADT1^MR^MCM||REDACTED^REDACTED^REDACTED^REDACTED||1961|M||C|^^^^
NK1|1|REDACTED^REDACTED^REDACTED|SPO^Spouse^HL70063|^^^^|
PV1|1|I|2000^2012^01||||REDACTED^REDACTED^REDACTED^REDACTED|||SUR||||ADM|A0||||63d6f85fb1af0958
AL1|1|DA|1605^ACETAMINOPHEN^L|MO|HEADACHE
```

```sh
# Say what would change, and change nothing
er7-redact --report samples/adt_a08.er7
```

```
PID[1]-3[1].1.1   pseudonym
PID[1]-5[1].1.1   replace REDACTED
PID[1]-5[1].2.1   replace REDACTED
PID[1]-7[1].1.1   first 4
PID[1]-11[1].1.1  clear
```

The report carries paths and actions and **no values**, so it can go
straight into a ticket.

```sh
er7-redact --show-policy > de-identify.policy   # the built-in list, as a file to edit
er7-redact -p de-identify.policy message.er7    # apply it
er7-redact -r "NTE-3 clear" message.er7         # or one rule, inline

# The two postures, as flags
er7-redact --all-but-the-header message.er7     # reject every value but keep MSH
er7-redact --accept-all -p strict.policy m.er7  # run its rules, not its reject
```

## Library

```rust
use er7_redact::{Policy, Redactor};

let mut message = er7::parse(text)?;
let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);

println!("{}", message.to_er7());
println!("{report}");
```

A policy is an ordered list of rules — an HL7 path and an action:

```rust
use er7_redact::{Action, Policy};

let policy = Policy::accept_all()
    .with("PID-3.1", Action::Pseudonym)?   // stable stand-in, so messages still join
    .with("PID-5", Action::redacted())?    // REDACTED^REDACTED^REDACTED
    .with("PID-7", Action::First(4))?      // 19610615 → 1961
    .with("PID-11", Action::Clear)?        // ^^^^
    .with("PID-19", Action::Null)?;        // "" — tell the receiver to clear its copy
```

…or the same thing as a file, which is what a team reviews in a pull
request:

```
PID-3.1  pseudonym
PID-5    replace REDACTED
PID-7    first 4       # the birth year is enough for most tests
PID-11   clear
PID-19   null
```

Invert it — reject every value by default, and name what to accept — when
the message is unfamiliar:

```rust
let policy = Policy::all_but_the_header()
    .with("OBX-3", Action::Keep)?
    .with("OBX-5", Action::Keep)?;
```

The same thing as a file ends with the two lines that say what the policy
does by default:

```
OBX-3  keep
OBX-5  keep

reject        replace REDACTED
unrecognised  refuse
```

## What it does

| | |
| --- | --- |
| **Preserves the shape** | Leaf text is rewritten; no segment, field, repetition, component, or subcomponent is added or removed. `Null` is the one documented exception. |
| **Keeps absent, empty, and null apart** | An empty field stays empty — writing `REDACTED` into it would invent a value. An explicit `""` stays null — overwriting it would turn "clear this" into a value. |
| **Never creates a position** | A rule for a field the message does not carry does nothing, rather than padding the segment out to reach it. |
| **Cannot corrupt the message** | Replacement text goes in escaped, so a `\|` in a placeholder can never split a field. |
| **Eight built-in actions, plus a caller-supplied ninth** | `keep`, `clear`, `null`, `replace`, `mask`, `first`, `last`, `pseudonym` — or `Action::custom` for a real MAC, a lookup table, or a date shift. |
| **Catches a known value wherever it repeats** | A value found at a named position is redacted everywhere else it appears too — case-insensitively, whole-word — on by default, so a name in an `NTE-3` comment does not survive just because no rule named that position. |
| **Finds what a policy is missing** | `--uncovered` (or `Redactor::uncovered`) lists every leaf a rule never named, so you can check a policy against a real message before trusting it. |
| **Stable pseudonyms** | The same identifier maps the same way in every message redacted with the same key, so a redacted export is still joinable. |
| **Reports what it did** | One row per position changed, fully qualified, with no values in it. |
| **One dependency** | [`er7`](https://crates.io/crates/er7), which has none of its own. |

## What it deliberately does not do

This is a **positional editor, not a compliance tool**.

- It cannot tell you whether the result is de-identified. That is a
  judgement about a whole data set, its recipients, and what else they
  hold — made by a person who is accountable for it.
- It does not know which positions *your* senders use. Run
  `er7 message.er7` and read what is actually in there.
- It does not find an identifier written into free text **unless that
  exact value already turned up at a named position** — the known-values
  sweep catches a repeat, not a first mention. A name that appears in an
  `NTE-3` comment and nowhere else still survives every positional policy;
  name that position explicitly, or reject by default.
- `pseudonym` is **not** cryptographic. It is a keyed hash that preserves
  equality on purpose, and anyone with the key can invert it. Use it inside
  your own trust boundary; for data leaving it, `clear` or `replace`.
- There is no way back: no mapping table, no key escrow, no undo.

A message this crate has redacted is a message with less in it, which is
progress, and is not the same thing as a safe one.

## Documentation

| Where | What |
| ----- | ---- |
| [`spec/`](spec/index.md) | the normative specification — one file per section, rules `D1`–`D18` |
| [`docs/usage/`](docs/usage/index.md) | the walk-through |
| [`docs/policies/`](docs/policies/index.md) | the policy format, the actions, the built-in tables |
| [`docs/api/`](docs/api/index.md) | every public item |
| [`docs/faq/`](docs/faq/index.md) | the questions the rest raise |
| [`examples/`](examples/README.md) | runnable programs that assert their own results |
| [`AGENTS.md`](AGENTS.md) | how to change this code |

## Development

```sh
cargo test                                  # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings   # lint
cargo fmt --check                           # format
cargo rustdoc --lib -- -W missing-docs      # every public item documented
```

All four are clean on `main` and must stay that way. Behavioural changes
start in [`spec/`](spec/index.md) — see
[`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

Every message in this repository is **synthetic**, and must stay that way:
a repository about redaction is exactly where somebody would be tempted to
commit a real one. See [`AGENTS/safety.md`](AGENTS/safety.md).

## See also

- [`er7`](https://github.com/er7-rust/er7-rust/tree/main/er7) — parse,
  query, edit, and write ER7, with zero dependencies. The layer underneath
  this one.
- [`serde-er7`](https://github.com/er7-rust/er7-rust/tree/main/serde-er7) —
  Serde support for the same value tree.
- [`hl7-2-from-er7-into-xml`](https://crates.io/crates/hl7-2-from-er7-into-xml) and
  [`hl7-2-from-er7-into-json`](https://crates.io/crates/hl7-2-from-er7-into-json) — the HL7
  v2.5 dictionary layer.

The whole family, and the boundary between the layers, is at
<https://er7-rust.github.io/ecosystem/>; this crate's own tutorial is at
<https://er7-rust.github.io/er7-redact/>.

## Contributing, conduct, and security

The workspace-level documents at the repository root cover this crate,
deliberately — one copy cannot drift:
[CONTRIBUTING.md](https://github.com/er7-rust/er7-rust/blob/main/CONTRIBUTING.md)
(including how to report a problem without pasting patient data),
[CODE_OF_CONDUCT.md](https://github.com/er7-rust/er7-rust/blob/main/CODE_OF_CONDUCT.md),
and
[SECURITY.md](https://github.com/er7-rust/er7-rust/blob/main/SECURITY.md).

## License

MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only — see
[LICENSE.md](LICENSE.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
