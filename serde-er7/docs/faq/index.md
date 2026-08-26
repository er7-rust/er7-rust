[← docs](../../index.md#documentation)

# FAQ

## Why isn't there a `serde_er7::to_string`/`from_str`?

Because that would mean picking a format, and the entire point of building
against `serde::Serializer`/`Deserializer` rather than writing an
ER7-to-JSON converter directly is that the format stays the caller's
choice. Call your format's own function: `serde_json::to_string(&message)`,
`serde_yaml::to_string(&message)`, whatever you already use. See
[`spec/03-dependencies-and-format-agnosticism/index.md`](../../spec/03-dependencies-and-format-agnosticism/index.md).

## Why does `Subcomponent` serialize the raw text instead of the decoded value?

So the round trip holds: `Message::parse(text)?` through JSON (or any other
format) and back out through `.to_er7()` reproduces `text` (modulo the same
normalization `er7::parse` alone would already do). Decoding is lossy — a
formatting escape like `\.br\` has no plain-text form to decode back to —
so a wire format built on decoded text could not, in general, be turned
back into valid ER7. If you want the decoded, human-readable value, read it
from the message after deserializing: `message.query("OBX-3.2")?`, the same
call you'd make against plain `er7`. See
[`spec/04-round-trip-guarantee/index.md`](../../spec/04-round-trip-guarantee/index.md).

## Why are `Field`/`Repetition`/`Component` arrays instead of objects?

Each of those types holds exactly one thing — a list of the level below —
so `{"repetitions": [...]}}` would add a key that carries no information at
every node of that shape in a real message, of which there can be many.
`Message` and `Segment` are objects because each carries two different
kinds of information that an array position alone would not distinguish.
See [`spec/02-wire-shapes/index.md`](../../spec/02-wire-shapes/index.md)
§2.2.

## Can I serialize just one `Segment`, without a whole `Message`?

Yes — every wrapper type serializes and deserializes on its own; nothing
requires going through `Message`. See §5 of
[`docs/usage/index.md`](../usage/index.md#5-working-with-one-piece-of-a-message)
and `examples/inspect_a_segment_as_json.rs`.

## My hand-written JSON fails to deserialize with a `missing field` or `invalid type` error — what's wrong?

Almost always a nesting-depth mismatch: a scalar field like `PID-1` needs
**three** array levels around its one subcomponent (`[[["1"]]]` — one
repetition, containing one component, containing one subcomponent), not one
or two. See
[`docs/usage/index.md` §3](../usage/index.md#3-the-shape-worked-through-by-hand)
for the nesting rule, and consider building the fixture by deserializing a
message you already have and inspecting its JSON (as in
`examples/inspect_a_segment_as_json.rs`) rather than writing the nesting by
hand from scratch.

## Does this crate validate anything — required fields, code tables, data types?

No. Like `er7` itself, this crate does not know what any segment or field
means. The only "validation" it does is at the Serde layer: a JSON object
missing `"name"` or `"segments"` is a deserialize error, because those keys
are structurally required to build the wrapper type at all — not because
this crate knows anything about what a well-formed `PID` segment contains.
For the HL7® v2.5 dictionary layer, see `hl7-2-5-to-xml-using-rust` and
`hl7-2-5-to-json-using-rust`.

## Why does deserializing ignore fields it doesn't recognize?

Forward compatibility, following `er7`'s own tolerance principle: a
producer that has added a field this crate does not yet model should not
break a consumer that only needs the fields it already understands. See
[`spec/05-error-handling/index.md`](../../spec/05-error-handling/index.md)
§5.2.

## Why does `er7` itself not just depend on `serde` directly?

`er7` has zero dependencies by design — it is meant to sit at the bottom of
a stack of HL7 crates in a domain where dependencies are audited. Adding
`serde` there would impose it on every consumer of `er7`, including ones
that never touch Serde. This crate exists so that choice stays opt-in. See
[`spec/01-purpose-and-scope/index.md`](../../spec/01-purpose-and-scope/index.md)
§1.2.

## Is this crate 1.0 yet? Can the wire shape change?

Not yet — see
[`spec/08-versioning-and-compatibility/index.md`](../../spec/08-versioning-and-compatibility/index.md)
for exactly what counts as a breaking change here (it includes the wire
shape itself, not only the Rust API) and how that interacts with the
pre-1.0 SemVer convention.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
