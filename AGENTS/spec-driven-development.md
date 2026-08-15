[AGENTS.md](../AGENTS.md) → spec-driven development

# Spec-driven development

This crate uses **spec-driven development**: every behavioural change starts
in the [`spec/`](../spec/) directory, then propagates outward to tests,
code, and docs. The spec is the source of truth.

The spec is split **one file per section** — start at
[`spec/index.md`](../spec/index.md), which holds the table of contents.
Section numbers (`§N.x`) are stable: `§6.2` always means section 6
(`spec/06-escape-sequences.md`), subsection 2, and that is how code
comments, tests, and commit messages cite the spec.

## What "spec-driven" means here

1. **The `spec/` files are canonical.** When the spec and the code
   disagree, the spec is right and the code is a bug — or the spec is right
   and needs updating *before* the code changes.
2. **No silent behaviour changes.** A change to observable behaviour that
   does not touch the matching `spec/` section file is incomplete.
3. **Tests express the spec.** A unit test or doc-test is the executable
   form of a spec clause. New clauses get new tests, and
   [`spec/13-testing-strategy.md`](../spec/13-testing-strategy.md) §13.1
   maps every rule to the test that enforces it.
4. **Docs follow the spec.** `index.md`, `docs/**`, `examples/**`, and the
   rustdoc examples are *derived*; they explain and illustrate the spec,
   they do not define it.

## Three artefacts, three jobs

Confusing these is the most common mistake in this repository.

| Artefact | Answers | Example |
| -------- | ------- | ------- |
| `spec/**` | what the crate **does** | "unescape resolves only the sequences that stand for characters" |
| `AGENTS/**` | how the code is **written and changed** | "use `checked_sub(1)?` so index 0 yields `None`" |
| `docs/**`, `examples/**`, `index.md` | how a caller **uses** it | "here is how to pull a patient name out of an ADT" |

If you are about to document a coding convention in `spec/`, or a
behavioural guarantee in `AGENTS/`, stop and move it.

## Rules and tasks have stable IDs

- **Rules** `R<n>` are the numbered behavioural guarantees, indexed in
  [`spec/01-purpose-and-scope.md`](../spec/01-purpose-and-scope.md) §1.4.
  Prose, tests, and commit messages cite them. **IDs are never reused**,
  even after a rule is withdrawn.
- **Tasks** `T<n>` are units of pending work, in
  [`spec/17-open-tasks.md`](../spec/17-open-tasks.md). Also never reused.

Adding a rule means: an entry in §1.4, a clause in the owning section, a
test, and a row in the §13.1 coverage table — all in the same change.

## When you must touch the spec

Any change to:

| Change | Section |
| ------ | ------- |
| how delimiters are read or validated | [§3](../spec/03-delimiters.md) |
| what parsing accepts, or how it splits | [§4](../spec/04-parsing.md) |
| the tree's shape, or absent/empty/null | [§5](../spec/05-value-tree.md) |
| which escape sequences decode, or how | [§6](../spec/06-escape-sequences.md) |
| what is written out, or the round trip | [§7](../spec/07-writing.md) |
| path notation, or what a query returns | [§8](../spec/08-paths-and-queries.md) |
| batch splitting | [§9](../spec/09-batch-input.md) |
| the MSH accessors | [§10](../spec/10-msh-conveniences.md) |
| the `Error` enum | [§11](../spec/11-error-handling.md) |
| any CLI option, output format, or exit code | [§12](../spec/12-command-line-interface.md) |
| the public API surface, in any breaking way | [§14](../spec/14-compatibility-and-versioning.md) |
| `Cargo.toml` `[dependencies]`, features, or targets | [§15](../spec/15-dependencies-and-build.md) |

If your change touches any of these and does not edit the matching spec
file, stop and update the spec first.

## The change loop

For a non-trivial behavioural change:

1. **Edit the matching `spec/` file** to describe the target behaviour.
   Plain prose; include a worked example where the behaviour is subtle.
2. **Update the rule index** (§1.4) if you added or changed a rule.
3. **Write or update tests** that encode the new clauses. They should fail
   against the current implementation.
4. **Edit the code** until the new tests pass and the old ones still do.
5. **Update the coverage table** (§13.1).
6. **Update derived docs** — `index.md`, `docs/**`, `examples/**` — so they
   read consistently with the new spec text.
7. **Run the four checks** ([`workflows.md`](workflows.md)).
8. **Commit** with a message naming the spec section that changed, and the
   task ID if it closes one.

For a non-behavioural change — refactor, formatting, comment clean-up — you
do not need to touch the spec; it covers behaviour, not code shape.

## When spec and code disagree

If you discover a divergence — even an old one — record it in
[`spec/18-open-questions-and-divergences.md`](../spec/18-open-questions-and-divergences.md)
*before* deciding what to do about it. The fix may turn out to be a code
change, a spec change, or both, but in every case the divergence needs to be
visible to future readers.

Section 18 is also where **decisions that were considered and declined**
live. A recorded "no, and here is why" saves the next reader from
re-litigating it.

## Boundary: what does not belong in the spec

- Internal code structure — [`architecture.md`](architecture.md).
- Coding conventions — [`conventions.md`](conventions.md).
- Test craft — [`testing.md`](testing.md). (Test *obligations* do belong in
  the spec, at §13.1.)
- Release mechanics — [`release.md`](release.md).
- Day-to-day commands — [`workflows.md`](workflows.md).

The `spec/` files are for the **observable behaviour** of the crate's
public API and CLI. Everything else lives in these topical guides.

## Planning and tasks live in the spec

This project does not maintain a separate `plan.md` or `tasks.md`. Instead:

- The **roadmap** is [`spec/16-roadmap.md`](../spec/16-roadmap.md), in
  priority order, with a rationale per item.
- The **backlog** is [`spec/17-open-tasks.md`](../spec/17-open-tasks.md),
  with stable `T<n>` IDs and a "done when" clause each.
- **Open questions and known divergences** are
  [`spec/18-open-questions-and-divergences.md`](../spec/18-open-questions-and-divergences.md).

When you finish a task, delete it from §17 in the same change that ships the
work — the commit history is the archive. When you take one on, link the
commit message to its `T<n>`.

## Why this discipline

- Clinical software deserves a written rationale, not just a diff.
- An agent or a human joining the project should be able to read the spec
  and know what the crate *should* do, without reverse-engineering it from
  the tests.
- A reviewer can compare a behavioural diff to a spec diff and immediately
  see what is and is not in scope.
