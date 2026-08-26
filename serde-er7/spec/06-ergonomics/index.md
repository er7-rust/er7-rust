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

`Component::serialize` (and every other multi-child level) clones each
child to construct a temporary wrapper it can pass to
`SerializeSeq::serialize_element`, rather than maintaining a second,
lifetime-parameterized family of borrowing wrapper types that could avoid
the clone. This is a deliberate simplicity trade-off: an HL7® message is
kilobytes, not gigabytes, so the clone cost is not worth doubling the
crate's type surface to avoid. If profiling ever shows this mattering for
a real workload, the fix is additive — a borrowing family living alongside
the existing owning one — not a rewrite of what already exists.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
