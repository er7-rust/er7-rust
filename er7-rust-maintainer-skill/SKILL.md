---
name: er7-rust-maintainer-skill
description: Technical implementation skill for working ON the er7-rust repository itself (the er7, er7-redact, and serde-er7 crates, and the er7-rust.github.io site) — not for using the crates as a dependency. Use when implementing a task from a crate's spec, fixing a bug in this repository, changing behavior in er7/src, er7-redact/src, or serde-er7/src, editing a spec/ file, preparing a release, or reviewing a pull request against this repository's own conventions.
---

# er7-rust-maintainer-skill — change this repository correctly

This skill is for **changing** the `er7-rust` workspace itself. If the task
is instead using `er7`, `er7-redact`, or `serde-er7` as a dependency in
some other project, that is `er7-skill`, not this one.

**This file is a Skill-shaped index, not a duplicate of the repository's
own guidance.** The workspace already documents itself exhaustively in
`AGENTS.md` and `spec/`, at the workspace root and inside each crate; this
skill's job is to get an agent to the right one of those files fast,
plus name the handful of rules that are easy to violate by skipping
straight to the code.

## Read this first, in order

1. **The workspace root** [`AGENTS.md`](../AGENTS.md) — the three-crate
   layout, the non-crate directories (the website, this skill folder, and
   `er7-skill`), and where workspace-level policy lives
   ([`spec/01-family-policy/index.md`](../spec/01-family-policy/index.md)).
2. **The crate you are actually touching** — `er7/AGENTS.md`,
   `er7-redact/AGENTS.md`, or `serde-er7/AGENTS.md`. Each is a self-
   contained entry point: project snapshot, documentation map, the rules
   that bind every change in that crate, and links to its own topical
   guides (`AGENTS/architecture.md`, `AGENTS/safety.md`,
   `AGENTS/spec-driven-development.md`, and siblings). **Do not guess a
   crate's conventions from another crate** — read that crate's own file.
3. **If the change is to the website**, `er7-rust.github.io/AGENTS.md`.

## The one discipline that binds every change

**Spec-driven development.** Behavior lives in a crate's `spec/`
directory before it is implemented:

1. Edit the matching `spec/` section to describe the target behavior.
2. Update that crate's rule index if you added or changed a rule
   (`R<n>`, never reused).
3. Write or update tests that encode the new clauses.
4. Edit the code until the new tests pass and the old ones still do.
5. Update the coverage table that maps every rule to a test.
6. Update derived docs (`index.md`, `docs/**`, `examples/**`, rustdoc
   examples) so they read consistently with the new spec text.
7. Run the four checks (below).
8. Commit, naming the spec section and, if it closes one, the task ID
   (`T<n>`, also never reused — delete the task from `spec/**/open-tasks/`
   in the same change that ships it).

A behavioral change that does not touch the matching `spec/` file is
incomplete, full stop — this applies even to a change that looks small
from the diff.

## The four checks

Run these — workspace-wide, or scoped to the crate you touched with
`-p <crate>` — before treating any change as finished:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
for c in er7 er7-redact serde-er7; do
    cargo rustdoc --lib -p "$c" -- -W missing-docs
done
```

## Rules that are easy to violate by skipping straight to the code

- **Never put real patient data anywhere in this repository** — not in a
  test, a sample, an example, a comment, or a commit message. All test
  data is synthetic (`SMITH^JOHN`, `MSG00042`, `444333222`). If a user
  pastes a real message to reproduce a bug, rebuild it as a synthetic
  message with the same shape.
- **Never collapse absent, empty, and explicit null.** `is_empty()` and
  `is_null()` must never both be true for the same node; an accessor
  returns `None` for absent, never a default.
- **Never corrupt a message on the way back out.** No trimming,
  normalizing, case-folding, or "fixing" at parse or render time —
  `parse(text).to_er7() == text` for canonical input, always.
- **Do not add a dictionary, a validator, or a transport** to `er7` or
  `er7-redact`. Both crates are deliberately *not* those things; a task
  that seems to need one belongs in a layer above.
- **Dependencies are an audit surface.** Do not add one without the user
  asking for it — check each crate's own dependency count before
  proposing a change that would raise it.
- **If you mention `HL7`, `FHIR`, or `CDA` anywhere**, run
  `bin/check-trademarks` before finishing: the first use of a mark in
  prose on a page needs `®` immediately after it, and the page needs the
  disclaimer verbatim. This applies to every Markdown file, website
  route, Rust doc comment, and crate `description` — including a new file
  you just added.
- **`tasks.md` and `plan.md`** hold workspace-level professionalization
  work only; per-crate roadmaps and backlogs live in each crate's own
  `spec/**/roadmap/` and `spec/**/open-tasks/` — do not add a crate-
  specific task to the workspace files or vice versa.

## Release process

Each crate's own `AGENTS/release.md` (or `help/releasing/`) has the
checklist: version bump rules (a `0.x` minor bump is the one allowed to
break), `CHANGELOG.md`'s per-crate-dated entry format, and what
`cargo publish` requires. Cutting a release is a deliberate decision, not
a mechanical consequence of a task closing — say so rather than
publishing as a side effect of finishing a task. That decision — what
ships, and what version it gets — stays the maintainer's alone,
unconditionally. What an agent *may* now be directed to do, once the
maintainer has made that decision: judge whether the scoped release meets
this project's own stated readiness criteria (the four checks, spec/code/
test agreement, correct SemVer classification, a clean
`cargo package --list`), and, combined with the publish authority
[`GOVERNANCE.md`](../GOVERNANCE.md)'s "Release authority" section
describes, run `cargo publish` for it. Deciding that a crate should
release *at all* is not part of that delegation.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
