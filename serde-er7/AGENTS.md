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
| Crate        | `serde-er7`                                                                  |
| Purpose      | Serde `Serialize`/`Deserialize` for every `er7` type, so an ER7 message can flow through any Serde data format. |
| Layer        | A bridge crate on top of `er7`'s encoding layer — no dictionary, no format, no validation of its own. |
| Language     | Rust (edition 2024, MSRV 1.96)                                              |
| License      | MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only            |
| Runtime deps | exactly two: `serde` and `er7` (spec S1) — no format crate (spec S2)        |
| Workspace    | one of three members in this repository — see [`../AGENTS.md`](../AGENTS.md) |
| Repository   | https://github.com/er7-rust/er7-rust (shared with `er7`, `er7-redact`)      |
| Crate        | https://crates.io/crates/serde-er7                                          |
| Docs         | https://docs.rs/serde-er7/                                                  |
| Website      | https://er7-rust.github.io/serde-er7/                                       |
| Sibling      | `er7` is a **path** dependency (`{ path = "../er7", version = "0" }`); `tests/` reads that crate's `samples/` from `../er7` via `include_str!` |
| Maintainer   | Joel Parker Henderson — joel@joelparkerhenderson.com                        |

## How this repo is documented

```
index.md                   ← README (user-facing introduction; README.md links here)
spec/                      ← living spec-driven-development specification,
                             one file per section (start at spec/index.md),
                             including the rule index and roadmap (§9)
AGENTS.md                  ← this file (agent entry point)
AGENTS/
├── architecture.md        ← repo layout, wrapper-type pattern, module map
├── conventions.md         ← coding style and doc-comment shape
├── testing.md             ← unit tests, doctests, integration tests, the four checks
├── safety.md              ← what changes when the data is clinical
├── workflows.md           ← cargo commands, daily flow, pitfalls
├── release.md             ← versioning and publish steps
└── spec-driven-development.md  ← how the spec/ files drive changes
docs/
├── api/index.md           ← full public API reference
├── usage/index.md         ← tutorial-style walk-through
└── faq/index.md           ← frequently asked questions
examples/                  ← runnable `cargo run --example <name>` programs
tests/                     ← black-box integration tests, incl. against er7's own samples/
```

There is **no separate** `plan.md` or `tasks.md` — both live as a spec
section
([`spec/09-roadmap-and-open-questions/index.md`](spec/09-roadmap-and-open-questions/index.md)).

## Rules that bind every change

Each maps to an `S`-numbered rule in
[`spec/index.md`](spec/index.md#rule-index).

1. **Exactly two runtime dependencies: `serde` and `er7`.** No format crate
   (`serde_json`, `serde_yaml`, ...) as a runtime dependency — only as
   `[dev-dependencies]`, for tests, doctests, and examples (S1, S2). See
   [`spec/03-dependencies-and-format-agnosticism/index.md`](spec/03-dependencies-and-format-agnosticism/index.md).
2. **Do not change a wire shape without updating
   [`spec/02-wire-shapes/index.md`](spec/02-wire-shapes/index.md) in the
   same change.** The JSON (or any-format) shape each type produces is part
   of this crate's compatibility surface, not an implementation detail
   (S10). See
   [`spec/08-versioning-and-compatibility/index.md`](spec/08-versioning-and-compatibility/index.md).
3. **Do not break the round trip.** `Message::parse(text)?` through any
   Serde format and back out through `.to_er7()` must reproduce what
   `er7::parse(text)?.to_er7()` alone would. This is why every
   `Subcomponent` serializes `raw`, never the escape-decoded `value()`
   (S3). See
   [`spec/04-round-trip-guarantee/index.md`](spec/04-round-trip-guarantee/index.md).
4. **Keep absent, empty, and null distinct** — the same patient-safety
   constraint `er7` itself enforces (its R10/R11), carried through
   unchanged because this crate changes no data, only its container. See
   [`spec/04-round-trip-guarantee/index.md`](spec/04-round-trip-guarantee/index.md)
   §4.4.
5. **Update the spec first.** Behavioural changes — even small ones — start
   by editing the matching file under `spec/`, then the code, then the
   tests. See
   [`spec/09-roadmap-and-open-questions/index.md`](spec/09-roadmap-and-open-questions/index.md)
   §9.3.

## Quick orientation for a brand-new agent

1. Read this file (you are here).
2. Read
   [`spec/01-purpose-and-scope/index.md`](spec/01-purpose-and-scope/index.md)
   and [`spec/02-wire-shapes/index.md`](spec/02-wire-shapes/index.md) —
   what this crate is, and the exact shape every type must produce.
3. Skim [`spec/index.md`](spec/index.md) for the section map and the rule
   index.
4. Skim [`AGENTS/architecture.md`](AGENTS/architecture.md) for the layout.
5. Run `cargo test` to confirm a green baseline before changing anything.

## Common commands

```sh
cargo build                                # Build
cargo test                                 # Unit + integration + doc tests
cargo test -- --nocapture                  # Show println!() output
cargo doc --no-deps --open                 # Build and open rustdoc
cargo run --example round_trip_via_json    # Run an example
cargo clippy --all-targets -- -D warnings  # Lint
cargo fmt                                  # Format
cargo rustdoc --lib -- -W missing-docs     # Confirm every public item is documented
```

The last four are the **four checks**; all four are clean on `main` and
must stay that way. A fuller walk-through lives in
[`AGENTS/testing.md`](AGENTS/testing.md) and
[`AGENTS/release.md`](AGENTS/release.md).
