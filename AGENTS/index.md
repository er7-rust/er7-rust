[serde-er7](../index.md) → AGENTS

# Agent guides

Topical guides for anyone — human or AI agent — changing this code. Start at
[`AGENTS.md`](../AGENTS.md), which is the entry point; these are the detail.

| Guide | What it covers |
| ----- | -------------- |
| [architecture](architecture.md) | repo layout, the wrapper-type pattern, the module map |
| [conventions](conventions.md) | coding style, the doc-comment shape, hand-written impls |
| [testing](testing.md) | where a test goes, how to name it, the four checks |
| [safety](safety.md) | what changes when the data is clinical — **read before writing behaviour** |
| [workflows](workflows.md) | cargo commands, the daily flow, common pitfalls |
| [release](release.md) | versioning and publish steps |
| [spec-driven-development](spec-driven-development.md) | how the `spec/` files drive changes |

The behaviour these guides help you change is specified in
[`spec/`](../spec/index.md), which is the single source of truth.
