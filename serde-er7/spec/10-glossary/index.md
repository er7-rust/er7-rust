[index](../index.md) → §10 Glossary

# §10 Glossary

Terms specific to this document; for ER7/HL7® terms themselves (segment,
field, repetition, component, subcomponent, delimiter, escape sequence),
see the `er7` spec's own glossary (its spec §19).

**Wire shape**
: The JSON-shaped (or any-Serde-format-shaped) representation a type's
  `Serialize` implementation produces and its `Deserialize` implementation
  accepts. Specified per type in [§2](../02-wire-shapes/index.md).

**Format-agnostic**
: This crate's implementations are written against `serde::Serializer`/
  `Deserializer`, the trait-level abstraction, rather than against any one
  format's concrete API — so the same code works unmodified with
  `serde_json`, `serde_yaml`, `bincode`, or any other Serde-compatible
  crate. See [§3](../03-dependencies-and-format-agnosticism/index.md).

**Round trip**
: Parse ER7 text, serialize with some Serde format, deserialize back,
  render to ER7 text again — and get the same bytes out that a plain
  `er7::parse(...).to_er7()` would already produce, with no additional
  loss introduced by the Serde format in the middle. Specified in
  [§4](../04-round-trip-guarantee/index.md).

**Wrapper (or wrapper type)**
: A newtype such as `pub struct Message(pub er7::Message)`, local to this
  crate, that exists so that a foreign trait (`Serialize`) can be
  implemented on a foreign type (`er7::Message`) without violating Rust's
  orphan rule. See [§6.3](../06-ergonomics/index.md).

**Owning vs. borrowing wrapper**
: An owning wrapper holds its `er7` value by value (`pub er7::Component`);
  a borrowing wrapper would hold a reference (`&'a er7::Component`). This
  crate uses only owning wrappers, at the cost of a clone when serializing
  nested children. See [§6.4](../06-ergonomics/index.md).

**S-numbered rule**
: A normative rule in this specification, numbered `S1`, `S2`, ... to
  distinguish it from an `er7` spec `R`-numbered rule when both are
  discussed together. See the [rule index](../index.md#rule-index).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
