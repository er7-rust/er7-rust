# er7-rust

A Cargo workspace holding three Rust crates for working with HL7® v2
messages in the **ER7** pipe-hat encoding — each independently versioned
and published, sharing this repository and one workspace `Cargo.toml`.

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7. See [`TRADEMARKS.md`](TRADEMARKS.md).

| Crate | What it does |
| ----- | ------------- |
| [`er7`](er7/) | Parse, query, edit, and write ER7 messages, with zero dependencies |
| [`er7-redact`](er7-redact/) | Redact patient detail from an ER7 message without changing its shape |
| [`serde-er7`](serde-er7/) | Serialize and deserialize ER7 message trees with Serde |

`er7-redact` and `serde-er7` depend on `er7` via a path dependency, so a
change to `er7` in this workspace is picked up by its siblings immediately,
without publishing.

Each crate has its own README, specification, examples, and tests — start
in that crate's own directory for anything crate-specific. This root only
holds what the three genuinely share: see
[`spec/01-family-policy/index.md`](spec/01-family-policy/index.md) for the
shared dependency, testing, and safety policy, and [`AGENTS.md`](AGENTS.md)
for agent guidance on the workspace as a whole.

The whole family, and the boundary between the layers, is documented at
<https://er7-rust.github.io/ecosystem/>.

Two packaged [Claude Code Skills](https://code.claude.com/docs/en/skills)
ship alongside the crates, for the two different audiences that read this
repository: [`er7-skill/`](er7-skill/) teaches an AI coding agent how to
*use* these crates correctly in some other project — ER7 concepts and
terminology, which crate to reach for, the round-trip and
absent/empty/null rules that are easy to get wrong, and worked recipes for
parsing, editing, batch input, and redaction. [`er7-rust-maintainer-skill/`](er7-rust-maintainer-skill/)
teaches an agent how to *change* this repository itself — spec-driven
development, the four checks, and the safety rules, packaged from this
repository's own `AGENTS.md` files rather than duplicating them. Drop
either into a project's own `.claude/skills/` to use it there.

## Install

```sh
cargo install er7          # the command-line tool
cargo add er7              # the library
```

[`INSTALL.md`](INSTALL.md) covers all three crates, the two binaries, the
requirements, and building from source.

## Building

```sh
cargo build --workspace
cargo test --workspace
```

The four checks that define "done" for any change are in
[§1.2 of the family policy](spec/01-family-policy/index.md).

## Project documents

The questions an evaluation asks, answered in one place each:

| Document | Answers |
| -------- | ------- |
| [`INSTALL.md`](INSTALL.md) | How do I install and run this? |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | How do I help — with time, with code, or with money? |
| [`RFC.md`](RFC.md) | What does this project want to learn, and what feedback would change it? |
| [`SECURITY.md`](SECURITY.md) | How do I report a vulnerability, and what is in scope? |
| [`GOVERNANCE.md`](GOVERNANCE.md) | Who decides what happens to this project, and how? |
| [`NEWS.md`](NEWS.md) | What is new, where do updates appear, and who do I contact about press? |
| [`CHANGELOG.md`](CHANGELOG.md) | What changed, change by change, across three independently versioned crates? |
| [`COMPARISONS.md`](COMPARISONS.md) | How does this compare to interface engines, HAPI, the other Rust crates — and when should I choose one of those instead? |
| [`BENCHMARKS.md`](BENCHMARKS.md) | How fast is it, measured how, on what? |
| [`MAINTAINERS.md`](MAINTAINERS.md) | Who is behind this, and what happens if they are unavailable? |
| [`AI_STATEMENT.md`](AI_STATEMENT.md) | How is this software built, and what oversight is on it? |
| [`LICENSE.md`](LICENSE.md) | Under which of the five licenses may I use it? |
| [`CITATION.cff`](CITATION.cff) | How do I cite it? |
| [`TRADEMARKS.md`](TRADEMARKS.md) | Whose trademarks are these, and what does this project claim about them? |
| [`spec/`](spec/index.md) | What policy do the three crates share, and why? |

**On AI oversight, specifically:** the maintainer alone decides what a
crate does and what a released version claims — an agent scopes nothing
on its own. As of 2026-09-02, an agent working in this repository (this
project uses Claude Code) may, once the maintainer has scoped and named a
release, judge whether it meets this project's own stated readiness
criteria and run `cargo publish` for it, without a further per-release
checkpoint. See [`GOVERNANCE.md`'s Release authority](GOVERNANCE.md#release-authority)
for the rule and [`AI_STATEMENT.md`](AI_STATEMENT.md#6-human-oversight)
for the full disclosure.

## License

This workspace is multi-licensed: **MIT OR Apache-2.0 OR BSD-3-Clause OR
GPL-2.0-only OR GPL-3.0-only**, at your option. See
[`LICENSE.md`](LICENSE.md). Each published crate also carries its own
`LICENSE.md` with the same five licenses, because a crate published to
crates.io must carry its license with it.

Copyright © Joel Parker Henderson <joel@joelparkerhenderson.com>
