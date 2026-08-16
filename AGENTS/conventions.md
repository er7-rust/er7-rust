[AGENTS.md](../AGENTS.md) → conventions

# Conventions

How code in this crate is written. Match the existing style; do not invent
new ones. This crate follows `er7`'s own
[`AGENTS/conventions.md`](https://github.com/er7-rust/er7-rust/blob/main/AGENTS/conventions.md)
wherever it applies unmodified; this file only states what differs or adds
to it.

## Formatting

- 4-space indentation, no tabs. `cargo fmt` before finishing.
- One blank line between top-level items.
- Raw strings (`r"..."`) for anything containing a backslash. **Caution**:
  a raw string does not interpret `\r` as a carriage return — it stays two
  literal characters. Where a real segment terminator is needed inside a
  string literal, use a normal string with `\r` (or `\\&` for a literal
  backslash next to it), the same way `er7`'s own doctests do. This bit
  three separate doctests/tests during this crate's own development; watch
  for it in review.

## Naming

- Every wrapper type is named identically to the `er7` type it wraps
  (`Message` wraps `er7::Message`, and so on) — never a `Serde`-prefixed or
  `-Wrapper`-suffixed name. The type's own module and doc comment make the
  relationship clear.
- Private `Visitor` types are named `<Type>Visitor` (`MessageVisitor`,
  `SegmentVisitor`, ...) and are not exported.

## Types and data shape

- **One wrapper type per `er7` level, no more, no fewer.** Do not
  introduce an intermediate type that does not correspond to something in
  `er7`'s own tree.
- **Public fields, always.** Every wrapper's inner value is `pub` (`pub
  struct Message(pub er7::Message)`), matching `er7`'s own "keep public
  fields public" rule and letting a caller who does not need `From`/`Deref`
  construct or destructure a wrapper directly.
- **`Deref`/`DerefMut`/`From` both ways, always** — see
  [`spec/06-ergonomics.md`](../spec/06-ergonomics.md), rule S11. Do not add
  a wrapper type without them.
- **Clone during `Serialize`, not `Deserialize`.** Every multi-child
  `Serialize` impl clones its children to build owned wrapper values; every
  `Deserialize` impl builds owned values directly with no clone needed.
  This asymmetry is intentional — see
  [`spec/06-ergonomics.md`](../spec/06-ergonomics.md) §6.4 — do not "fix"
  it by adding `Cow`-based borrowing without updating that section first.

## Doc comments

Every public item carries a rustdoc comment, in this shape:

1. **One-sentence summary** on the first line.
2. Blank line.
3. **Why**, where it is not obvious from the name — this is where a
   trade-off, an invariant, or a spec cross-reference goes.
4. **`Example:`** section with a runnable doctest, for anything a caller
   calls directly.
5. **Cross-references** to the matching `spec/` section, written as prose
   with a Markdown link (`[§2](../spec/02-wire-shapes.md)` reads correctly
   in rendered Markdown; rustdoc renders it as ordinary text, which is
   fine — rustdoc cannot link out of the crate to a non-doc file anyway),
   plus intra-doc links to sibling items and to the wrapped `er7` type.

`src/lib.rs` carries `#![warn(missing_docs)]`, and
`cargo rustdoc --lib -- -W missing-docs` must be clean — including doc
comments on struct fields, not only on the types.

## Doctests specifically

Because this crate's whole purpose is Serde interop, most doctests use
`serde_json` (a dev-dependency) to demonstrate a real serialize/deserialize
round trip rather than only constructing values in memory. Prefer:

```rust
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // ...
/// # Ok(())
/// # }
/// ```
```

over `# fn main() -> Result<(), er7::Error>` for any doctest that calls
`serde_json`, since its errors are a different type than `er7::Error`.

## Inline comments

- Default to none. Well-named identifiers and the doc comments above
  already say *what*.
- Add a `//` comment only when the **why** is non-obvious, and prefer
  citing the spec: `// See spec/04-round-trip-guarantee.md §4.2.` A reader
  can then find the reasoning rather than re-deriving it.

## Lints

- `cargo clippy --all-targets -- -D warnings` must be clean. `--all-targets`
  covers `examples/` and `tests/` too.
- Do not silence clippy with `#[allow(…)]` without a comment on the same
  item explaining why.

## Tests

Test conventions live in [`testing.md`](testing.md).
