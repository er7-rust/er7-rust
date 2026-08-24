# `er7` crate specification

**Status:** living document. Updated alongside every behavioural change.
**Applies to:** `er7` 0.1.1. **Audience:** maintainers, AI agents,
downstream integrators reading the crate's contract. **Companion docs:**
[`AGENTS.md`](../AGENTS.md) for agent guidance, [`index.md`](../index.md)
for the user-facing README, [`docs/api/index.md`](../docs/api/index.md) for
the rendered API surface, and the workspace root's
[`spec/01-family-policy/index.md`](../../spec/01-family-policy/index.md)
for the dependency, testing, and safety policy this crate shares with its
siblings `er7-redact` and `serde-er7`.

This directory is the **canonical specification** — one file per section,
indexed below — that drives spec-driven development (see
[`AGENTS/spec-driven-development.md`](../AGENTS/spec-driven-development.md)).
When the spec and the code disagree, the spec is the source of truth and
the code is a bug — or the spec needs updating *before* the code changes.

The discipline:

1. **Behaviour is described here first.** A change to observable behaviour
   that does not touch the matching section file is incomplete.
2. **Every behavioural rule is testable.** Rules carry stable `R<n>` IDs,
   indexed in [§1.4](01-purpose-and-scope/index.md), and each one names the
   test that enforces it in the §13.1 coverage table.
3. **Plans and tasks live here too.** §16 holds the **roadmap** in priority
   order; §17 holds the **backlog** with stable `T<n>` IDs; §18 holds
   **open questions and known divergences**. There is no separate
   `plan.md` or `tasks.md`.

---

## Table of contents

| § | Section | File |
| - | ------- | ---- |
| 1 | Purpose and scope, and the rule index (R1–R25) | [01-purpose-and-scope/index.md](01-purpose-and-scope/index.md) |
| 2 | The ER7 encoding (domain background) | [02-er7-encoding/index.md](02-er7-encoding/index.md) |
| 3 | Delimiters | [03-delimiters/index.md](03-delimiters/index.md) |
| 4 | Parsing | [04-parsing/index.md](04-parsing/index.md) |
| 5 | The value tree | [05-value-tree/index.md](05-value-tree/index.md) |
| 6 | Escape sequences | [06-escape-sequences/index.md](06-escape-sequences/index.md) |
| 7 | Writing and round trip | [07-writing/index.md](07-writing/index.md) |
| 8 | Paths and queries | [08-paths-and-queries/index.md](08-paths-and-queries/index.md) |
| 9 | Batch and multi-message input | [09-batch-input/index.md](09-batch-input/index.md) |
| 10 | MSH conveniences | [10-msh-conveniences/index.md](10-msh-conveniences/index.md) |
| 11 | Error handling | [11-error-handling/index.md](11-error-handling/index.md) |
| 12 | Command-line interface | [12-command-line-interface/index.md](12-command-line-interface/index.md) |
| 13 | Testing strategy | [13-testing-strategy/index.md](13-testing-strategy/index.md) |
| 14 | Compatibility and versioning | [14-compatibility-and-versioning/index.md](14-compatibility-and-versioning/index.md) |
| 15 | Dependencies and build | [15-dependencies-and-build/index.md](15-dependencies-and-build/index.md) |
| 16 | Roadmap | [16-roadmap/index.md](16-roadmap/index.md) |
| 17 | Open tasks (backlog) | [17-open-tasks/index.md](17-open-tasks/index.md) |
| 18 | Open questions and known divergences | [18-open-questions-and-divergences/index.md](18-open-questions-and-divergences/index.md) |
| 19 | Glossary | [19-glossary/index.md](19-glossary/index.md) |

Section numbers are stable: prose, code comments, tests, and commit
messages cite `§N.x`, and the behavioural rule index (R1–R25) lives in
[01-purpose-and-scope/index.md](01-purpose-and-scope/index.md) §1.4.

## Where each rule is implemented

| Section | Module |
| ------- | ------ |
| §3 Delimiters | `src/separators.rs` |
| §4 Parsing, §9 Batch input | `src/parse.rs` |
| §5 Value tree, §8 Queries, §10 MSH | `src/message.rs` |
| §6 Escape sequences | `src/escape.rs` |
| §7 Writing | `src/render.rs` |
| §8 Path notation | `src/path.rs` |
| §11 Error handling | `src/lib.rs` |
| §12 Command line | `src/main.rs` |
