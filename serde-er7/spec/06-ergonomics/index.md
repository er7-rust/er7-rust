[index](../index.md) → §6 Ergonomics: Deref and From

# §6 Ergonomics: Deref and From

## 6.1 Rule S11

Every wrapper type in this crate (`Message`, `Segment`, `Field`,
`Repetition`, `Component`, `Subcomponent`, `Separators`, `Terminator`)
implements:

- `Deref<Target = er7::X>` and, where the wrapped value is naturally
  mutable, `DerefMut`, so `er7::X`'s own methods and fields are reachable
  directly — `message.query(...)`, `message.segments`, `component.is_null()`
  — without unwrapping `.0` first.
- `From<er7::X> for X` and `From<X> for er7::X`, so converting either
  direction is `.into()` at the call site, not a manual `.0` access or a
  constructor call.

## 6.2 Why these are not part of the wire contract

Unlike the shapes in [§2](../02-wire-shapes/index.md),
`Deref`/`DerefMut`/`From` are Rust-side ergonomics with no wire
representation — removing or adding one does not change what bytes a
`Serialize` call produces or a `Deserialize` call accepts. They are still
part of this crate's public API, and removing one is still a breaking
change under normal SemVer rules
([§8](../08-versioning-and-compatibility/index.md)), but they are tracked
separately from S10 because a wire-shape change and an ergonomics change
call for different migration advice to downstream users.

## 6.3 Why the orphan rule forces the wrapper pattern

`er7::Message` is defined in the `er7` crate; `serde::Serialize` is defined
in the `serde` crate. Neither is local to `serde-er7`, so Rust's orphan
rule forbids `impl Serialize for er7::Message` here directly. The wrapper
newtype (`pub struct Message(pub er7::Message)`) is what makes the `impl`
legal: `Message` is local to this crate, so implementing a foreign trait
(`Serialize`) on it is allowed. Every wrapper in this crate exists for this
reason, not merely as a style preference — see `src/message.rs` for the
one place this is spelled out at the type most callers reach for first.

## 6.4 Why one owning wrapper per level, not a borrowing family

The public API has one owning wrapper per tree level and no second,
lifetime-parameterized family of borrowing types. Doubling the type
surface is not worth it.

**Serialization does not clone, though.** Every public `Serialize` impl
delegates to a private borrowing serializer (`src/borrow.rs`) that writes
the §2 shape straight from the `er7` tree. Before 2026-11-26 each level
cloned its children to wrap them. Measured on a synthetic message with
100,000 repetitions, that doubled peak memory (77 → 154 MiB) and made five
JSON serializations take 298 ms. Borrowing brings that to 81 MiB and
13 ms, with byte-identical output ([`BENCHMARKS.md`](../../../BENCHMARKS.md)).
The fix needed no new public type, which is why the measurement came
first.

## 6.5 `Strict<T>` follows the same convention

[§11](../11-strict-mode/index.md)'s `Strict<T>` is not one of the eight
per-level wrappers listed in [§6.1](#61-rule-s11), but it still carries
`Deref`, `DerefMut`, and `From` both ways, plus a delegating `Serialize`,
for the same reason: a caller reaching for `Strict<Message>` should not
lose any of the ergonomics a plain `Message` already has.

The public-API conformance test `every_wrapper_converts_and_mutates_through`
exercises both conversion directions and mutable dereferencing for all eight
per-level wrappers, plus each concrete reverse conversion supported by
`Strict<T>`. This keeps S11 from being inferred from two representative
types while the remaining implementations go unexecuted.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
