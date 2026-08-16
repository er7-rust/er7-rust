# CLAUDE.md

See **[AGENTS.md](AGENTS.md)** — that file holds the canonical agent
instructions for this repository, and the topical guides it links to under
[`AGENTS/`](AGENTS/) hold the detail. Keeping the guidance in one place
avoids the copies drifting apart.

Start here if you have no prior context:

| Read | For |
| ---- | --- |
| [AGENTS.md](AGENTS.md) | the project snapshot, the documentation map, and the rules that bind every change |
| [spec/01-purpose-and-scope.md](spec/01-purpose-and-scope.md) | what this crate is and is not |
| [spec/02-wire-shapes.md](spec/02-wire-shapes.md) | the normative shape every type serializes as — read this before touching a `Serialize`/`Deserialize` impl |
| [spec/index.md](spec/index.md) | the section map, the rule index, and the roadmap (§9) |
| [AGENTS/architecture.md](AGENTS/architecture.md) | the wrapper-type pattern every module follows |

The rules themselves are specified in [`spec/`](spec/) (single source of
truth). [`index.md`](index.md) is the user-facing README, and
[`docs/`](docs/) holds the tutorial and reference pages.

Before finishing any change, run the four checks:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
```
