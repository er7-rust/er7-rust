[AGENTS.md](../AGENTS.md) → architecture

# Architecture

## Repo layout

```
src/
├── lib.rs           crate docs, module wiring, re-exports (incl. `pub use er7;`)
├── message.rs        Message      — the entry point most callers use
├── segment.rs         Segment
├── field.rs            Field
├── repetition.rs         Repetition
├── component.rs            Component
└── subcomponent.rs            Subcomponent   — the leaf
    separators.rs      Separators  (sibling of Message, not nested under it)
    terminator.rs       Terminator (standalone; a render option, not tree data)
```

The indentation above mirrors the tree nesting: `Message` contains
`Segment`s, which contain `Field`s, which contain `Repetition`s, which
contain `Component`s, which contain `Subcomponent`s. `Separators` and
`Terminator` sit outside that nesting — `Separators` is one field of
`Message`, and `Terminator` is not part of the tree at all.

## The wrapper-type pattern

Every module follows the same shape:

```rust
pub struct X(pub er7::X);              // 1. the wrapper
impl From<er7::X> for X { .. }         // 2. conversions both ways
impl From<X> for er7::X { .. }
impl Deref for X { .. }                // 3. ergonomic access to er7's own API
impl Serialize for X { .. }            // 4. hand-written, per spec §2
impl<'de> Deserialize<'de> for X { .. }
```

See [`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md) for
what each `Serialize`/`Deserialize` pair must produce and accept, and
[`spec/06-ergonomics/index.md`](../spec/06-ergonomics/index.md) for why the
pattern looks like this (in particular §6.3 on the orphan rule, which is
*why* there is a wrapper at all).

## Two shapes of implementation

- **Struct-shaped** (`Message`, `Segment`, `Separators`): `serialize_struct`
  / a `Visitor` with `visit_map`, following the `Point { x, y }` pattern
  from [serde's manual-implementation
  guide](https://docs.rs/serde/latest/serde/) directly.
- **Sequence-shaped** (`Field`, `Repetition`, `Component`): `serialize_seq`
  / a `Visitor` with `visit_seq`, collecting into the inner `Vec` field.
- **Scalar-shaped** (`Subcomponent`: a string; `Terminator`: an enum
  written as a string): `serialize_str` / `visit_str`.

No module mixes shapes — each wrapper is exactly one of the three, which is
what makes
[`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md)'s table a
complete description of the crate's behaviour rather than an approximation.

## Dependency direction

```
serde-er7  →  serde   (trait vocabulary)
serde-er7  →  er7     (the value tree being wrapped)
```

Nothing in `er7` depends on `serde-er7` — the dependency is one-directional
by design (spec §1.2). `er7` has no knowledge this crate exists.

## Where a new type would go

If `er7` grows a new public type, the pattern above is the template: a new
`src/<name>.rs` module, a wrapper struct, conversions, `Deref`, and a
hand-written `Serialize`/`Deserialize` pair whose shape gets documented in
[`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md) *before*
the code is written (spec-driven, per [`AGENTS.md`](../AGENTS.md) rule 5).
