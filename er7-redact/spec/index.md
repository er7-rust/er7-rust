# `er7-redact` crate specification

**Status:** living document. Updated alongside every behavioural change.
**Applies to:** `er7-redact` 0.2.0.
**Audience:** maintainers, AI agents, and downstream integrators reading
the crate's contract.
**Companion docs:** [`AGENTS.md`](../AGENTS.md) for agent guidance,
[`index.md`](../index.md) for the user-facing README,
[`docs/api/index.md`](../docs/api/index.md) for the rendered API surface.

This directory is the **canonical specification** — one file per section,
indexed below — that drives spec-driven development. When the spec and the
code disagree, the spec is the source of truth and the code is a bug — or
the spec needs updating *before* the code changes.

This crate is a companion to [`er7`](../../er7/), a sibling member of the
same workspace, and its own spec (`../../er7/spec/index.md`, hereafter
"the `er7` spec"). This document does not restate anything the `er7` spec
already settles — how text is parsed, what an escape sequence means, how
absent, empty, and null differ. It specifies only the one thing this crate
adds:
**removing patient detail from a message without breaking it**.

The discipline:

1. **Behaviour is described here first.** A change to observable behaviour
   that does not touch the matching section file is incomplete.
2. **Every behavioural rule is testable.** Rules carry stable `D<n>` IDs,
   indexed below, and [§11](11-testing-strategy.md) names the test that
   enforces each one.
3. **Plans and tasks live here too.** [§14](14-roadmap.md) holds the
   roadmap, [§15](15-open-tasks.md) the backlog with stable `T<n>` IDs, and
   [§16](16-open-questions-and-declined-decisions.md) the open questions
   and the decisions that were considered and declined. There is no
   separate `plan.md` or `tasks.md`.

---

## Table of contents

| § | Section | File |
| - | ------- | ---- |
| 1 | Purpose and scope, and the rule index (D1–D18) | [01-purpose-and-scope.md](01-purpose-and-scope.md) |
| 2 | The redaction model | [02-redaction-model.md](02-redaction-model.md) |
| 3 | Actions | [03-actions.md](03-actions.md) |
| 4 | What redaction preserves | [04-what-redaction-preserves.md](04-what-redaction-preserves.md) |
| 5 | Built-in policies | [05-built-in-policies.md](05-built-in-policies.md) |
| 6 | The policy file format | [06-policy-file-format.md](06-policy-file-format.md) |
| 7 | Pseudonyms | [07-pseudonyms.md](07-pseudonyms.md) |
| 8 | The report | [08-report.md](08-report.md) |
| 9 | Error handling | [09-error-handling.md](09-error-handling.md) |
| 10 | Command-line interface | [10-command-line-interface.md](10-command-line-interface.md) |
| 11 | Testing strategy | [11-testing-strategy.md](11-testing-strategy.md) |
| 12 | Dependencies and build | [12-dependencies-and-build.md](12-dependencies-and-build.md) |
| 13 | Compatibility and versioning | [13-compatibility-and-versioning.md](13-compatibility-and-versioning.md) |
| 14 | Roadmap | [14-roadmap.md](14-roadmap.md) |
| 15 | Open tasks (backlog) | [15-open-tasks.md](15-open-tasks.md) |
| 16 | Open questions and declined decisions | [16-open-questions-and-declined-decisions.md](16-open-questions-and-declined-decisions.md) |
| 17 | Glossary | [17-glossary.md](17-glossary.md) |

Section numbers are stable: prose, code comments, tests, and commit
messages cite `§N.x`, and the behavioural rule index (D1–D18) lives in
[01-purpose-and-scope.md](01-purpose-and-scope.md) §1.4. The `D` prefix
("de-identification") distinguishes these rules from the `er7` spec's
`R`-numbered rules and `serde-er7`'s `S`-numbered ones when several are
discussed together.

## Where each section is implemented

| Section | Module |
| ------- | ------ |
| §2 The redaction model, §4 What redaction preserves | `src/redact.rs` |
| §3 Actions | `src/action.rs` |
| §5 Built-in policies, §6 Policy file format | `src/policy.rs` |
| §7 Pseudonyms | `src/pseudonym.rs` |
| §8 The report | `src/redact.rs` |
| §9 Error handling | `src/lib.rs` |
| §10 Command line | `src/main.rs` |

## Required checks

Before finishing any change:

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lint-clean, examples included
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

All four are clean on `main` and must stay that way.
