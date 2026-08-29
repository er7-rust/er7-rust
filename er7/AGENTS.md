# AGENTS.md

Guidance for AI coding agents (Claude Code, Codex, Copilot, Cursor, Aider,
etc.) working in this repository.

This file is the **entry point**. It is intentionally short. Drill into the
topical guides under [`AGENTS/`](AGENTS/) for the full picture, and read
[`spec/index.md`](spec/index.md) for the canonical specification that
drives changes.

## Project snapshot

| Field        | Value                                                                        |
| ------------ | ---------------------------------------------------------------------------- |
| Crate        | `er7`                                                                        |
| Purpose      | Parse, query, edit, and write HL7® v2 messages in the ER7 pipe-hat encoding.  |
| Layer        | Encoding only — no dictionary, no validation, no transport.                  |
| Language     | Rust (edition 2024, MSRV 1.96)                                               |
| License      | MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only            |
| Runtime deps | **none**, and that is a guarantee (R25)                                      |
| Workspace    | one of three members in this repository — see [`../AGENTS.md`](../AGENTS.md) |
| Repository   | https://github.com/er7-rust/er7-rust (shared with `er7-redact`, `serde-er7`) |
| Crate        | https://crates.io/crates/er7                                                 |
| Docs         | https://docs.rs/er7/                                                         |
| Website      | https://er7-rust.github.io/ — source in [`../er7-rust.github.io/`](../er7-rust.github.io/), a directory of this repository rather than a Cargo member |
| Maintainer   | Joel Parker Henderson — joel@joelparkerhenderson.com                         |

## How this crate is documented

This crate lives at `er7/` inside the `er7-rust` workspace, alongside its
siblings `er7-redact/` and `serde-er7/` — the tree below is `er7`'s own
layout, not the whole repository's. The documentation is layered so each
reader can stop at the depth they need:

```
index.md                   ← README (user-facing introduction; README.md links here)
spec/                      ← living spec-driven-development specification,
                             one file per section (start at spec/index.md),
                             including the rule index (§1.4), roadmap (§16),
                             and open tasks (§17)
AGENTS.md                  ← this file (agent entry point)
AGENTS/
├── architecture.md        ← repo layout, modules, data model, public API
├── conventions.md         ← coding style and doc-comment shape
├── testing.md             ← unit tests, doctests, the four checks
├── safety.md              ← patient-safety constraints and scope discipline
├── workflows.md           ← common cargo commands, daily flow
├── release.md             ← versioning and publish steps
└── spec-driven-development.md  ← how the spec/ files drive changes
docs/
├── api/index.md           ← full public API reference
├── usage/index.md         ← tutorial-style walk-through
├── escapes/index.md       ← escape sequences, with worked examples
├── paths/index.md         ← HL7 path notation reference
└── faq/index.md           ← frequently asked questions
examples/                  ← runnable `cargo run --example <name>` programs
help/releasing/            ← release checklist (mirrors AGENTS/release.md)
samples/                   ← example ER7 messages used by docs and tests
```

There is **no separate** `plan.md` or `tasks.md` — both live as spec
sections ([`spec/16-roadmap/index.md`](spec/16-roadmap/index.md),
[`spec/17-open-tasks/index.md`](spec/17-open-tasks/index.md)). If a
planning artefact needs a home, add a section there.

## Five rules that bind every change

These are the load-bearing constraints. Each is expanded in the matching
topical guide, and each maps to a numbered rule in
[`spec/01-purpose-and-scope/index.md`](spec/01-purpose-and-scope/index.md)
§1.4.

1. **Do not break the round trip.** `parse(text).to_er7()` reproduces
   canonical input byte for byte (R16). This is why leaf text is stored raw
   and decoded on demand, and why nothing but blank lines is trimmed. Never
   normalize a value at parse time. See
   [`AGENTS/conventions.md`](AGENTS/conventions.md).
2. **Keep absent, empty, and null distinct** (R10, R11). The explicit `""`
   means *clear this value*; collapsing it into "empty" corrupts patient
   records. See [`AGENTS/safety.md`](AGENTS/safety.md).
3. **Nothing below the header may fail** (R6). Unknown segments, ragged
   fields, odd delimiters, undecodable escapes — all data, never errors.
   Do not turn a fallback into a failure.
4. **Do not add a dictionary, a validator, or a transport** (R24), and do
   not add a dependency (R25). Both are the crate's reason to exist. See
   [`AGENTS/safety.md`](AGENTS/safety.md).
5. **Update the spec first.** Behavioural changes — even small ones — start
   by editing the matching file under `spec/`, then the code, then the
   tests. See
   [`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

## Quick orientation for a brand-new agent

If you have just been spawned with no prior context, do this in order:

1. Read this file (you are here).
2. Read
   [`spec/01-purpose-and-scope/index.md`](spec/01-purpose-and-scope/index.md)
   — the rule index in §1.4 is the whole contract in one table, and §1.5
   tells you which goal wins when two conflict.
3. Skim [`spec/index.md`](spec/index.md) for the section map, and §16/§17
   for what work is currently in flight.
4. Skim [`AGENTS/architecture.md`](AGENTS/architecture.md) for the layout.
5. For any task touching behaviour, open
   [`AGENTS/safety.md`](AGENTS/safety.md) **before** writing code.
6. Run `cargo test` to confirm a green baseline before changing anything.

## Common commands

```sh
cargo build                              # Build
cargo test                               # Unit + integration + doc tests
cargo test -- --nocapture                # Show println!() output
cargo doc --no-deps --open               # Build and open rustdoc
cargo run -- samples/oru_r01.er7         # Run the CLI on a sample
cargo run --example parse_a_message      # Run an example
cargo clippy --all-targets -- -D warnings  # Lint
cargo fmt                                # Format
cargo rustdoc --lib -- -W missing-docs   # Confirm every public item is documented
```

The last four are the **four checks**; all four are clean on `main` and must
stay that way. A fuller walk-through lives in
[`AGENTS/workflows.md`](AGENTS/workflows.md) and
[`AGENTS/release.md`](AGENTS/release.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
