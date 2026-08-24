[AGENTS.md](../AGENTS.md) → spec-driven development

# Spec-driven development

This crate uses **spec-driven development**: every behavioural change
starts in the [`spec/`](../spec/) directory, then propagates outward to
tests, code, and docs. The spec is the source of truth.

The spec is split **one file per section** — start at
[`spec/index.md`](../spec/index.md), which holds the table of contents.
Section numbers (`§N.x`) are stable, and that is how code comments, tests,
and commit messages cite the spec.

## What "spec-driven" means here

1. **The `spec/` files are canonical.** When the spec and the code
   disagree, the spec is right and the code is a bug — or the spec is right
   and needs updating *before* the code changes.
2. **No silent behaviour changes.** A change to observable behaviour that
   does not touch the matching `spec/` section is incomplete.
3. **Tests express the spec.**
   [§11.1](../spec/11-testing-strategy/index.md) maps every rule to the
   test that enforces it.
4. **Docs follow the spec.** `index.md`, `docs/**`, and `examples/**` are
   *derived*; they explain and illustrate, they do not define.

## Three artefacts, three jobs

| Artefact | Answers | Example |
| -------- | ------- | ------- |
| `spec/**` | what the crate **does** | "an empty leaf is left empty" |
| `AGENTS/**` | how the code is **written and changed** | "read with `value`, write with `set`" |
| `docs/**`, `examples/**`, `index.md` | how a caller **uses** it | "here is how to pin the built-in policy" |

## Rules and tasks have stable IDs

- **Rules** `D<n>` are the numbered behavioural guarantees, indexed in
  [spec §1.4](../spec/01-purpose-and-scope/index.md). **IDs are never
  reused.**
- **Tasks** `T<n>` are units of pending work, in
  [spec §15](../spec/15-open-tasks/index.md). Also never reused.

Adding a rule means: an entry in §1.4, a clause in the owning section, a
test, and a row in the §11.1 coverage table — all in the same change.

## When you must touch the spec

| Change | Section |
| ------ | ------- |
| what a rule selects, or the order rules apply in | [§2](../spec/02-redaction-model/index.md) |
| what an action does | [§3](../spec/03-actions/index.md) |
| what redaction preserves | [§4](../spec/04-what-redaction-preserves/index.md) |
| the built-in policies | [§5](../spec/05-built-in-policies/index.md) |
| the policy file format | [§6](../spec/06-policy-file-format/index.md) |
| the pseudonym function | [§7](../spec/07-pseudonyms/index.md) — and read §13.2 first |
| what a report holds | [§8](../spec/08-report/index.md) |
| the `Error` enum | [§9](../spec/09-error-handling/index.md) |
| any CLI option, output format, or exit code | [§10](../spec/10-command-line-interface/index.md) |
| `Cargo.toml` `[dependencies]` | [§12](../spec/12-dependencies-and-build/index.md) |

## The change loop

1. Edit the matching `spec/` file.
2. Update the rule index (§1.4) if a rule changed.
3. Write the failing test.
4. Change the code.
5. Update the coverage table (§11.1).
6. Update the derived docs.
7. Run the four checks.
8. Commit, naming the section and any `T<n>`.

## When spec and code disagree

Record it in
[spec §16](../spec/16-open-questions-and-declined-decisions/index.md)
*before* deciding what to do about it. §16 is also where decisions that
were considered and **declined** live — a recorded "no, and here is why"
saves the next reader from re-litigating it.

## Planning and tasks live in the spec

There is no separate `plan.md` or `tasks.md`:
[§14](../spec/14-roadmap/index.md) is the roadmap,
[§15](../spec/15-open-tasks/index.md) the backlog with `T<n>` IDs, and
[§16](../spec/16-open-questions-and-declined-decisions/index.md) the open
questions. Finish a task and delete it from §15 in the same change — the
commit history is the archive.
