# CLAUDE.md

See **[AGENTS.md](AGENTS.md)** — that file holds the canonical agent
instructions for this repository, and the topical guides it links to under
[`AGENTS/`](AGENTS/) hold the detail. Keeping the guidance in one place
avoids the copies drifting apart.

Start here if you have no prior context:

| Read | For |
| ---- | --- |
| [AGENTS.md](AGENTS.md) | the project snapshot, the documentation map, and the six rules that bind every change |
| [spec/01-purpose-and-scope/index.md](spec/01-purpose-and-scope/index.md) | the whole contract in one table (§1.4), and which goal wins when two conflict (§1.5) |
| [spec/index.md](spec/index.md) | the section map, plus the roadmap (§14) and open tasks (§15) |
| [AGENTS/safety.md](AGENTS/safety.md) | **before** writing any code that touches behaviour, and before telling a user what the output is safe for |
| [AGENTS/spec-driven-development.md](AGENTS/spec-driven-development.md) | how a change flows: spec first, then tests, then code |

The rules themselves are specified in [`spec/`](spec/) (single source of
truth), and `cargo test` enforces that the rule index, the
[§11.1](spec/11-testing-strategy/index.md) coverage table, and the
[§5.1](spec/05-built-in-policies/index.md) policy table agree with the
code.

[`index.md`](index.md) is the user-facing README, and [`docs/`](docs/)
holds the tutorials and reference pages.

Before finishing any change, run the four checks:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
```
