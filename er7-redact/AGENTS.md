# AGENTS.md

Guidance for AI coding agents (Claude Code, Codex, Copilot, Cursor, Aider,
etc.) working in this repository.

This file is the **entry point**. It is intentionally short. Drill into the
topical guides under [`AGENTS/`](AGENTS/) for the full picture, and read
[`spec/index.md`](spec/index.md) for the canonical specification that
drives changes.

## Project snapshot

| Field        | Value                                                                       |
| ------------ | --------------------------------------------------------------------------- |
| Crate        | `er7-redact`                                                                |
| Purpose      | Remove patient detail from HL7 v2 ER7 messages without breaking the message. |
| Layer        | A positional editor over `er7` — no validation, no transport, no undo.      |
| Language     | Rust (edition 2024, MSRV 1.85)                                              |
| License      | MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only           |
| Runtime deps | exactly one: [`er7`](https://crates.io/crates/er7) (D16)                     |
| Workspace    | one of three members in this repository — see [`../AGENTS.md`](../AGENTS.md) |
| Repository   | https://github.com/er7-rust/er7-rust (shared with `er7`, `serde-er7`)       |
| Crate        | https://crates.io/crates/er7-redact                                         |
| Docs         | https://docs.rs/er7-redact/                                                 |
| Website      | https://er7-rust.github.io/er7-redact/ — source in the sibling repo `er7-rust.github.io` |
| Maintainer   | Joel Parker Henderson — joel@joelparkerhenderson.com                        |

## How this repo is documented

```
index.md                   ← README (user-facing introduction; README.md links here)
spec/                      ← living specification, one file per section
                             (start at spec/index.md), including the rule
                             index (§1.4), roadmap (§14), and open tasks (§15)
AGENTS.md                  ← this file (agent entry point)
AGENTS/
├── architecture.md        ← repo layout, modules, the redaction pass
├── conventions.md         ← coding style and doc-comment shape
├── testing.md             ← unit tests, doctests, the four checks
├── safety.md              ← patient-safety and privacy constraints
├── workflows.md           ← common cargo commands, daily flow
├── release.md             ← versioning and publish steps
└── spec-driven-development.md  ← how the spec/ files drive changes
docs/
├── usage/index.md         ← tutorial-style walk-through
├── policies/index.md      ← the policy format and the built-in tables
├── api/index.md           ← full public API reference
└── faq/index.md           ← frequently asked questions
examples/                  ← runnable `cargo run --example <name>` programs
help/releasing/            ← release checklist (mirrors AGENTS/release.md)
samples/                   ← example ER7 messages and a policy file
```

There is **no separate** `plan.md` or `tasks.md` — both live as spec
sections ([`spec/14-roadmap.md`](spec/14-roadmap.md),
[`spec/15-open-tasks.md`](spec/15-open-tasks.md)).

## Six rules that bind every change

The load-bearing constraints. Each maps to a numbered rule in
[`spec/01-purpose-and-scope.md`](spec/01-purpose-and-scope.md) §1.4 and is
expanded in the matching topical guide.

1. **Do not move the shape.** Redaction rewrites leaf text and nothing else
   (D1). A redaction that shifts a field is worse than none: the message
   looks fine and says something else. `Action::Null` is the one documented
   exception. See [`AGENTS/conventions.md`](AGENTS/conventions.md).
2. **Do not invent.** No position is created (D2), no empty leaf is filled
   (D3), no explicit null is overwritten (D4). Writing `REDACTED` into an
   empty field announces that something was there.
3. **When in doubt, redact.** Privacy is priority 1
   ([§1.5](spec/01-purpose-and-scope.md)): a value that should have gone
   and did not cannot be undone.
4. **Do not claim more than the crate knows** (D14). This is a positional
   editor, not a compliance tool; `pseudonym` is not secure (D12). Never
   call an output "de-identified". See [`AGENTS/safety.md`](AGENTS/safety.md).
5. **Never leak in a report** (D13) — paths and actions, no values — and
   **never commit real patient data**, redacted or otherwise.
6. **Update the spec first.** Behavioural changes start by editing the
   matching file under `spec/`, then the tests, then the code. See
   [`AGENTS/spec-driven-development.md`](AGENTS/spec-driven-development.md).

## Quick orientation for a brand-new agent

1. Read this file (you are here).
2. Read [`spec/01-purpose-and-scope.md`](spec/01-purpose-and-scope.md) —
   the rule index in §1.4 is the whole contract in one table, and §1.5 says
   which goal wins when two conflict.
3. Skim [`spec/index.md`](spec/index.md) for the section map, and §14/§15
   for what work is in flight.
4. Skim [`AGENTS/architecture.md`](AGENTS/architecture.md) for the layout.
5. For any task touching behaviour, open
   [`AGENTS/safety.md`](AGENTS/safety.md) **before** writing code.
6. Run `cargo test` to confirm a green baseline before changing anything.

The sibling workspace member [`../er7`](../er7/) supplies the encoding
layer, and its spec (`../er7/spec/index.md`) settles everything about
parsing, escape sequences, and the absent/empty/null distinction. Do not
reimplement any of it here.

## Common commands

```sh
cargo build                                 # Build
cargo test                                  # Unit + integration + doc tests
cargo run -- samples/adt_a08.er7            # Redact a sample
cargo run -- --report samples/adt_a08.er7   # Show what would change
cargo run -- --show-policy                  # The built-in policy, as a file
cargo run --example redact_a_message        # Run an example
cargo clippy --all-targets -- -D warnings   # Lint
cargo fmt                                   # Format
cargo rustdoc --lib -- -W missing-docs      # Confirm every public item is documented
```

The last four are the **four checks**; all four are clean on `main` and
must stay that way. A fuller walk-through lives in
[`AGENTS/workflows.md`](AGENTS/workflows.md) and
[`AGENTS/release.md`](AGENTS/release.md).
