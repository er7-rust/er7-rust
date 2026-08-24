[AGENTS.md](../AGENTS.md) → spec-driven development

# Spec-driven development

This crate uses **spec-driven development**: every behavioural change
starts in the [`spec/`](../spec/) directory, then propagates outward to
tests, code, and docs. The spec is the source of truth.

Start at [`spec/index.md`](../spec/index.md), which holds the table of
contents and the rule index. Section numbers (`§N.x`) are stable, and that
is how code comments, tests, and commit messages cite the spec.

## What "spec-driven" means here

1. **The `spec/` files are canonical.** When the spec and the code
   disagree, the spec is right and the code is a bug — or the spec is right
   and needs updating *before* the code changes.
2. **No silent behaviour changes.** A change to observable behaviour that
   does not touch the matching `spec/` section is incomplete. For this
   crate "observable behaviour" includes **every wire shape**
   ([§2](../spec/02-wire-shapes/index.md)): the JSON somebody stored last
   year is the compatibility surface, not the Rust signature that produced
   it.
3. **Tests express the spec.** [§7.1](../spec/07-testing-strategy/index.md)
   maps every rule to the test that enforces it, and `cargo test` checks
   that mapping is complete.
4. **Docs follow the spec.** `index.md`, `docs/**`, and `examples/**` are
   *derived*; they explain and illustrate, they do not define.

## Three artefacts, three jobs

| Artefact | Answers | Example |
| -------- | ------- | ------- |
| `spec/**` | what the crate **does** | "a subcomponent serializes as its raw text" |
| `AGENTS/**` | how the code is **written and changed** | "write the impl by hand, against the low-level trait methods" |
| `docs/**`, `examples/**`, `index.md` | how a caller **uses** it | "here is how to get a message into a document store" |

## Rules have stable IDs

**Rules** `S<n>` are the numbered guarantees, indexed in
[`spec/index.md`](../spec/index.md#rule-index). The `S` prefix keeps them
apart from `er7`'s `R` rules and `er7-redact`'s `D` rules when several are
discussed together. **IDs are never reused.**

Adding a rule means: a row in the index, a clause in the owning section, a
test, and a row in the [§7.1](../spec/07-testing-strategy/index.md)
coverage table — all in the same change. The coverage table is checked by
`every_rule_has_a_coverage_row`, so a missing row fails the build rather
than review.

## When you must touch the spec

| Change | Section |
| ------ | ------- |
| any type's serialized shape | [§2](../spec/02-wire-shapes/index.md) — and read §8 first |
| what is a dependency, or which formats are named | [§3](../spec/03-dependencies-and-format-agnosticism/index.md) |
| what survives a round trip | [§4](../spec/04-round-trip-guarantee/index.md) |
| how a malformed input is reported | [§5](../spec/05-error-handling/index.md) |
| `Deref`, `From`, or any other convenience | [§6](../spec/06-ergonomics/index.md) |
| what is tested, and where | [§7](../spec/07-testing-strategy/index.md) |
| the SemVer commitments | [§8](../spec/08-versioning-and-compatibility/index.md) |

## The change loop

1. Edit the matching `spec/` file.
2. Update the rule index if a rule changed.
3. Write the failing test.
4. Change the code.
5. Update the [§7.1](../spec/07-testing-strategy/index.md) coverage table.
6. Update the derived docs.
7. Run the four checks ([workflows](workflows.md)).
8. Commit, naming the section that changed.

## Planning lives in the spec

There is no separate `plan.md` or `tasks.md`:
[`spec/09-roadmap-and-open-questions/index.md`](../spec/09-roadmap-and-open-questions/index.md)
holds both the roadmap and the open questions, including the decisions that
were considered and **declined**. A recorded "no, and here is why" saves
the next reader from re-litigating it.
