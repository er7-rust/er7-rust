# CLAUDE.md

See **[AGENTS.md](AGENTS.md)** — that file holds the canonical agent
instructions for this repository. Keeping the guidance in one place avoids
the copies drifting apart.

Start here:

| Read | For |
| ---- | --- |
| [AGENTS.md](AGENTS.md) | the snapshot, the layout, the conventions, and the one rule that matters most |
| [spec/index.md](spec/index.md) | this site's scope and content model |
| [index.md](index.md) | the human-oriented overview |

**The one rule that matters most:** this site is *derived, not normative*.
The `er7` crate repository holds the source of truth for behaviour in its
`spec/` directory. Never state a behaviour here that the crate's spec does
not state.

Before finishing any change:

```sh
pnpm check    # svelte-check: types and accessibility
pnpm build    # prerender; fails on a dead internal link
```
