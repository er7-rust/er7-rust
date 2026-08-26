[AGENTS.md](../AGENTS.md) → conventions

# Conventions

How code in this crate is written. Match the existing style; do not invent
new ones.

## Formatting

- 4-space indentation, no tabs. `cargo fmt` before finishing.
- One blank line between top-level items.
- Raw strings (`r"..."`) for anything containing a backslash, which in this
  crate means most escape-sequence literals. `r"a\F\b"` reads; `"a\\F\\b"`
  does not.

## Naming

- Types use the standard's vocabulary: `Separators`, `Segment`, `Field`,
  `Repetition`, `Component`, `Subcomponent`. Where this crate and HL7® have
  different words for the same thing, HL7 wins — see
  [§18.3](../spec/18-open-questions-and-divergences/index.md) for the one
  case where that cost something.
- Accessors are nouns (`field`, `component`, `separators`); predicates are
  `is_*`; conversions are `to_*`.
- Use UK spelling in user-facing prose (`behaviour`, `organisation`) and US
  spelling in code identifiers (Rust convention). The existing codebase
  follows this split — preserve it.

## Types and data shape

- **1-based accessors return `Option`, never panic.** Use `checked_sub(1)?`
  so that index `0` yields `None` rather than wrapping. HL7 numbering starts
  at 1; a `0` is a caller's off-by-one and must not read as element 1.
- **Return `Cow<'_, str>` when the common case needs no allocation** —
  `unescape` and `escape` both do, and both check cheaply before
  allocating.
- **Keep public fields public.** Struct-literal construction is a
  documented part of the API ([§5.1](../spec/05-value-tree/index.md)).
  Adding a field is therefore a breaking change; think before you do.
- **Do not memoize.** No caches, no lazily computed fields. Public mutable
  fields and a cache cannot both be safe, and the fields win.

## Doc comments

Every public item carries a rustdoc comment, in this shape:

1. **One-sentence summary** on the first line.
2. Blank line.
3. **Why**, in a sentence or two, where it is not obvious from the name.
   This is where a constraint, an invariant, or a rejected alternative goes.
4. **`Example:`** section with a runnable doc-test, for anything a caller
   calls directly.
5. **Cross-references** to the spec section and rule — written as prose,
   `(spec §6.2, rule R13)`, since rustdoc cannot link out of the crate —
   and intra-doc links to sibling items.

Skeleton:

```rust
/// Decode the escape sequences that stand for characters, leaving every
/// other sequence exactly as written.
///
/// Display formatting, character-set switches, and local extensions say
/// something a plain string cannot carry, so dropping them would lose more
/// than keeping them (spec section 6.2, rule R13).
///
/// Example:
///
/// ```
/// use er7::{Separators, escape::unescape};
/// let separators = Separators::default();
/// assert_eq!(unescape(r"Smith \T\ Jones", &separators), "Smith & Jones");
/// assert_eq!(unescape(r"line\.br\next", &separators), r"line\.br\next");
/// ```
///
/// See also [`escape`] for the inverse.
pub fn unescape<'a>(text: &'a str, separators: &Separators) -> Cow<'a, str> {
```

`src/lib.rs` carries `#![warn(missing_docs)]`, and
`cargo rustdoc --lib -- -W missing-docs` must be clean — including doc
comments on **enum variants and struct fields**, not only on the types.

## Inline comments

- Default to none. Well-named identifiers and the doc comments above
  already say *what*.
- Add a `//` comment only when the **why** is non-obvious: a hidden
  constraint, a subtle invariant, a workaround, a rejected alternative.
- Prefer citing the spec: `// Header fields 1 and 2 are the delimiters
  themselves (spec §4.4.2).` A reader can then find the reasoning rather
  than re-deriving it.
- Never narrate the *what*. `// increment the index` is noise.

## The three properties to protect

A change that breaks one of these is wrong even if every test still passes.
They are the same three as
[`AGENTS.md`](../AGENTS.md), restated as coding rules.

### 1. Round trip (R16)

- Store text **exactly as received**. Do not trim, normalize, case-fold, or
  decode at parse time.
- Any new field on the tree must be reconstructible into the same bytes.
- When you add a parse rule, add the matching write rule in the same change.

### 2. Distinction (R10, R11)

- Never write `if value.is_empty()` where `""` and `|""|` need different
  treatment. Ask `is_null()` first.
- Never make `is_empty` true for a null node, or vice versa.

### 3. Tolerance (R6)

- Below the header, return data, not `Result`. `parse_with` returns
  `Message`, not `Result<Message, Error>`, and that is the model.
- Do not add an `Error` variant to describe something the crate can
  recover from ([§11.3](../spec/11-error-handling/index.md)).

## Lints

`cargo clippy --all-targets -- -D warnings` must be clean. `--all-targets`
covers `examples/` and `tests/` too.

`Cargo.toml` turns on clippy's **pedantic** group, and the four checks run
clippy with `-D warnings`, so pedantic findings fail the build
([`spec/15-dependencies-and-build/index.md`](../spec/15-dependencies-and-build/index.md)
§15.7). In practice that means three habits:

- **`#[must_use]` on every pure accessor and constructor.** A discarded
  `to_er7()` or `query()` is a bug the caller cannot see.
- **`# Errors` on every public `fn` returning `Result`**, naming the
  variants and — as importantly — what is *not* an error. `query` returning
  `Ok(None)` for a position the message lacks is the whole of R20, and the
  doc says so.
- **`# Panics` only where a panic is reachable.** If it is not, restructure
  the code until clippy agrees rather than documenting a panic that cannot
  happen.

Where a lint is wrong for one line, `#[allow(..., reason = "...")]` next to
that line — never a crate-level hole in the group.

## Tests

Test conventions live in [`testing.md`](testing.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
