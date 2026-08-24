# serde-er7 specification

This is the normative specification for `serde-er7` — the source of truth
for behaviour. Where the rustdoc, README, or any other document disagrees
with this one, this one is right; where this document and a test disagree,
that is a bug in whichever one is wrong, and the fix updates both together.

This crate is a companion to
[`er7`](https://github.com/er7-rust/er7-rust/tree/main/er7) and its own
spec (`spec/index.md` in that repository, hereafter "the `er7` spec"). This
document does not restate anything the `er7` spec already settles — parsing
rules, escape sequences, the meaning of absent/empty/null — it only
specifies the one thing this crate adds: Serde support for the tree `er7`
already defines.

## Sections

| § | Title | Covers |
|---|-------|--------|
| [1](01-purpose-and-scope/index.md) | Purpose and scope | What this crate is, the rule index, which goal wins when two conflict |
| [2](02-wire-shapes/index.md) | Wire shapes | The exact `Serialize`/`Deserialize` shape for every level of the tree |
| [3](03-dependencies-and-format-agnosticism/index.md) | Dependencies and format-agnosticism | Why exactly two dependencies, and why no format is named in the library code |
| [4](04-round-trip-guarantee/index.md) | The round-trip guarantee | What survives a Serde round trip, and the one thing that deliberately does not |
| [5](05-error-handling/index.md) | Error handling | How malformed input is reported, and by what mechanism |
| [6](06-ergonomics/index.md) | Ergonomics: Deref and From | The non-normative conveniences layered over the wrapper types |
| [7](07-testing-strategy/index.md) | Testing strategy | Unit, doc, and integration tests, and what each layer is responsible for catching |
| [8](08-versioning-and-compatibility/index.md) | Versioning and compatibility | SemVer commitments, the wire-shape table as a compatibility surface, the N-3 Rust MSRV |
| [9](09-roadmap-and-open-questions/index.md) | Roadmap and open questions | What is deliberately deferred, and why |
| [10](10-glossary/index.md) | Glossary | Terms this document uses that are specific to this crate |

## Rule index

Every normative rule below carries an ID (`S1`, `S2`, ...) so it can be
cited from code, tests, and commit messages without restating it. The `S`
prefix distinguishes these from the `er7` spec's own `R`-numbered rules
when the two are discussed together.

| ID | One-line statement | Section |
|----|---------------------|---------|
| S1 | Exactly two runtime dependencies: `serde` and `er7` | [§3](03-dependencies-and-format-agnosticism/index.md) |
| S2 | No format-specific crate is a runtime dependency | [§3](03-dependencies-and-format-agnosticism/index.md) |
| S3 | A subcomponent serializes as its `raw` text, never `value`-decoded | [§2](02-wire-shapes/index.md), [§4](04-round-trip-guarantee/index.md) |
| S4 | Field, Repetition, and Component serialize as bare arrays, not objects | [§2](02-wire-shapes/index.md) |
| S5 | Message and Segment serialize as objects with named fields | [§2](02-wire-shapes/index.md) |
| S6 | A `char` delimiter serializes via `serialize_char`, not as part of a larger string | [§2](02-wire-shapes/index.md) |
| S7 | `Terminator` serializes as its Rust variant name, as a string | [§2](02-wire-shapes/index.md) |
| S8 | Deserializing an object ignores unknown fields rather than rejecting them | [§5](05-error-handling/index.md) |
| S9 | A missing required field is a `missing_field` error naming that field | [§5](05-error-handling/index.md) |
| S10 | The wire shape in [§2](02-wire-shapes/index.md) is part of this crate's public API and its SemVer contract | [§8](08-versioning-and-compatibility/index.md) |
| S11 | Every wrapper type implements `Deref`, `DerefMut`, and `From` both ways | [§6](06-ergonomics/index.md) |
| S12 | Every public item carries a doc comment; `cargo rustdoc --lib -- -W missing-docs` stays clean | [§7](07-testing-strategy/index.md) |

## Which goal wins when two conflict

In order:

1. **Correctness against the `er7` spec.** If a wire shape would make a
   round trip lose information `er7` itself preserves — the absent/empty/
   null distinction above all — that shape is wrong, no matter how much
   more convenient the alternative reads in JSON.
2. **Format-agnosticism.** A choice that only reads well in one format
   (relying on a JSON-specific feature, say) is wrong even if it never
   causes a round-trip failure, because it privileges one Serde
   implementation over the rest.
3. **Readability of the wire shape.** Once the first two are satisfied,
   prefer the shape a human would choose reading the JSON cold — a bare
   string for a leaf, a bare array for a plain sequence — over one that
   mirrors Rust's own struct layout more literally.
4. **Everything else** — code brevity, symmetry between levels, and so on.

## Required checks

Before finishing any change:

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lint-clean
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

All four are clean on `main` and must stay that way.
