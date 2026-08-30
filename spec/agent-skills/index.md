[er7-rust](../../index.md) → [spec](../index.md) → agent skills

# §11 Agent skills

This workspace publishes packaged
[Claude Code Skills](https://code.claude.com/docs/en/skills) as
repository-top-level folders, one per audience, rather than one skill
trying to serve both:

- **`er7-skill`** — for **using** `er7`, `er7-redact`, and `serde-er7` as
  a dependency, in any project. ER7 concepts and terminology, which crate
  a task needs, the round-trip and absent/empty/null rules that are easy
  to get wrong, and worked recipes.
- **`er7-rust-maintainer-skill`** — for **changing** this repository
  itself. An agent packaging of this workspace's own `AGENTS.md` files
  (root and each crate's), indexing them rather than duplicating them:
  spec-driven development, the four checks, the safety rules, the
  trademark check.

## Why two, and why top-level folders rather than `.claude/skills/`

Splitting by audience follows the same reasoning the workspace already
applies everywhere else it has two readers with different needs — a
crate's `docs/` for a user versus its `AGENTS.md`/`spec/` for an agent
changing it (`AGENTS/spec-driven-development.md`'s "three artefacts,
three jobs" table, restated at the workspace level in each crate's own
`AGENTS.md`). One skill trying to be both would either teach a
maintainer's discipline to someone who only wants to call `er7::parse`,
or bury a dependency's user in this repository's own release process.

Each lives at the repository root (`er7-skill/SKILL.md`,
`er7-rust-maintainer-skill/SKILL.md`), not under `.claude/skills/`,
because this repository is not the place either skill runs *from* — it
is the place they are published *for*. A downstream project copies the
folder it wants into its own `.claude/skills/`; the maintainer skill also
serves a downstream fork or another project's tooling that wants this
workspace's conventions as a loadable skill. The root workspace
`AGENTS.md` and root `index.md` both name the two folders and link them,
per this workspace's existing convention that a non-crate top-level
directory gets a line in both.

## Keeping them from drifting

- **`er7-skill` must never state a rule the owning crate's `spec/`
  contradicts.** Its five numbered rules (round trip, absent/empty/null,
  `Subcomponent::set`, nothing below the header fails, no real patient
  data) are restatements of rules already in `er7`'s and `er7-redact`'s
  own rule indices, not a second source of truth for them — if one of
  those crates' rules changes, this skill's restatement changes in the
  same commit.
- **`er7-rust-maintainer-skill` must never duplicate an `AGENTS.md`
  file's content**, only point at it. It is deliberately a thin index —
  see its own opening paragraph, which says so — so a change to the
  spec-driven-development workflow, the four checks, or a safety rule
  needs no matching edit here, only in the `AGENTS.md` it points at.
- Both are surfaced from `AGENTS.md`, `index.md`, and the site's
  `/agent-skill/` route; adding, renaming, or removing either skill
  updates all three in the same change, the same discipline every other
  top-level document in this workspace already follows.

Implemented 2026-08-30: both folders created and committed separately
(`er7-skill`, then `er7-rust-maintainer-skill`), each verified against
`bin/check-trademarks` before committing. This section, and its Contents
row here, were written after the fact — the two skills existed and were
committed before this spec section did, which is itself the divergence
this section closes: behaviour that reaches a published, repository-top-
level artefact belongs in the spec regardless of how or when the work was
scheduled, the same reasoning `llms-json-and-llms-txt`
([§10](../llms-json-and-llms-txt/index.md)) states for the same reason.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
