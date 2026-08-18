# CLAUDE.md

See **[AGENTS.md](AGENTS.md)** — that file holds the canonical agent
instructions for this repository. Keeping the guidance in one place avoids
the copies drifting apart.

Start here:

| Read | For |
| ---- | --- |
| [AGENTS.md](AGENTS.md) | the snapshot, the layout, the conventions, and the one rule that matters most |
| [spec/index.md](spec/index.md) | this site's scope and content model — §1.1 subproject vs neighbour, §3.3 what a subproject page must cover |
| [index.md](index.md) | the human-oriented overview |

**The one rule that matters most:** this site is *derived, not normative*.
Every crate repository holds the source of truth for its own behaviour in
its `spec/` directory — `R` rules for `er7`, `D` for `er7-redact`, `S` for
`serde-er7`. Never state a behaviour here that the owning crate's spec does
not state, and never hand-write a tool's output: capture it from a real
run.

Before finishing any change:

```sh
pnpm check    # svelte-check: types and accessibility
pnpm build    # prerender; fails on a dead internal link
```
