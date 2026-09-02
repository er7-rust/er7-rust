[index](../index.md) → §11 Strict mode

# §11 Strict mode: an opt-in `deny_unknown_fields`

## 11.1 What this section resolves

[§9.2](../09-roadmap-and-open-questions/index.md) asked whether a future
version should offer an opt-in validating mode, so a caller writing a JSON
fixture by hand (`examples/build_message_from_json.rs`) gets a specific,
actionable error when a key is mistyped. This section is that future
version, and the rule it adds.

Concretely, two distinct things improve. A typo on a *required* key
(`"feilds"` for `"fields"`) already fails without this section — S9's
`missing_field("fields")` — but that message does not say *why* the key is
missing, and does not mention `"feilds"` at all; a caller has to notice the
typo by eye. `Strict<T>` reports an "unknown field" error naming `feilds`
itself and listing `name`/`fields` as what it could have meant, instead. A
typo on the one *optional* key this crate has (`Separators`'
`"truncation"`) is the case S8's tolerance genuinely hides: misspelled, it
is silently treated as absent and defaults to `None`, with no error of any
kind, whether or not `None` is what the fixture's author meant.

## 11.2 Rule S13: `Strict<T>` rejects unknown fields; `T` alone still does not

`Strict<T>` is a wrapper type, `pub struct Strict<T>(pub T)`, implementing
`Deserialize` for `T` in `{Message, Segment, Separators}` — the three
object-shaped types in [§2](../02-wire-shapes/index.md)'s table. Where the
ordinary `T::deserialize` ignores a key it does not recognize (S8),
`Strict::<T>::deserialize` reports it with
`serde::de::Error::unknown_field`, naming the key and the field names it
could have been.

`Field`, `Repetition`, `Component`, and `Subcomponent` serialize as bare
arrays or strings, not objects (S4), so "unknown field" has no meaning at
those levels — there is no `Strict<Field>` and no `Strict<Component>`, and
adding one later would need its own argument, the same way S1's exception
clause works. `Terminator` is already effectively strict: an unrecognized
variant string is already a deserialize error (`rejects_an_unknown_variant`
in [§7.1](../07-testing-strategy/index.md)), so there is no `Strict<Terminator>`
either — nothing about it changes between the two modes.

**Strictness nests.** `Strict<Message>` rejects an unknown key not only on
the message object itself, but inside every segment and inside the
separators object it contains — a typo three levels down is caught the
same as one at the top. This is why `Strict<Message>` is not implemented as
a thin flag checked only in `MessageVisitor`: internally it deserializes
`"segments"` and `"separators"` through `serde::de::DeserializeSeed`
implementations that carry the strict flag down to `SegmentVisitor` and
`SeparatorsVisitor` directly, rather than through the ordinary `Segment`/
`Separators` `Deserialize` impls (which are always tolerant, per S8, and
have no way to be told otherwise from outside). `Strict<Segment>` and
`Strict<Separators>` exist in their own right too — for a caller who only
ever hand-writes segment- or separators-level fixtures — and use exactly
the same strict visitors `Strict<Message>` reaches internally, so the
behaviour cannot drift between the nested and the standalone case.

## 11.3 This does not change what `T::deserialize` accepts

**S8 is unchanged.** `serde_json::from_str::<Message>(json)` (or `Segment`,
or `Separators`) behaves exactly as before this section — tolerant,
ignoring unknown keys — because `Strict<T>` is a distinct, additive type
with its own `Deserialize` impl, not a flag on the existing one. A crate
version that adds §11 is not a breaking change to S8 or to any type's wire
shape ([§2](../02-wire-shapes/index.md) is unchanged: `Strict<T>` accepts
and requires exactly what `T` does, plus the one additional rejection).
This is the "clear default (tolerant, matching `er7`'s own R6)" [§9.2](../09-roadmap-and-open-questions/index.md)
asked for: the default is still tolerant, and strictness is something a
caller opts into by naming `Strict<T>` at the call site, not something a
producer can silently impose on every consumer.

## 11.4 Ergonomics

`Strict<T>` follows the same convention [§6](../06-ergonomics/index.md)
(rule S11) states for every other wrapper type in this crate: `Deref`,
`DerefMut`, and `From` both ways, plus a `Serialize` impl (delegating to
`T`'s own) so a `Strict<Message>` can be written out and compared the same
way a plain `Message` can, even though strictness itself is a
deserialize-only concept.

## 11.5 Why not a global flag

An earlier design considered a thread-local or other ambient flag, set
before calling the ordinary `T::deserialize` and read from inside every
`visit_map`. It was rejected: a caller who forgets to unset it (or whose
code panics between setting and unsetting) leaves every *later*,
unrelated `T::deserialize` call in the process silently strict, which is
exactly the kind of action-at-a-distance bug this crate's own safety
posture (`AGENTS/safety.md`) argues against introducing. `Strict<T>` is
requested at the one call site that wants it and nowhere else, which is
what an ordinary Rust type parameter is for.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
