[index](../index.md) → §4 The round-trip guarantee

# §4 The round-trip guarantee

## 4.1 The guarantee

For any ER7 text `t` that `er7::parse` accepts, and any Serde data format
`F` capable of representing arbitrary UTF-8 strings, arbitrary-precision
nesting, and `Option`:

```text
Message::parse(t)?  --serialize with F-->  bytes
bytes  --deserialize with F-->  Message  --.to_er7()-->  t'
```

`t'` equals whatever `er7::parse(t)?.to_er7()` would already produce on its
own — canonical input round-trips byte for byte; non-canonical terminators
and blank lines are normalized once, at the `Message::parse` call, exactly
as they would be by `er7::parse` alone. The Serde round trip in the middle
introduces no further change. This is `er7`'s own round-trip guarantee (its
spec §7.2, rule R16), carried through a Serde format rather than weakened
by one.

## 4.2 What makes it possible

Rule S3 ([§2.2](../02-wire-shapes/index.md)): every subcomponent serializes
its `raw` field, not the escape-decoded `value()`. Decoding is lossy on
purpose — `er7`'s spec §6.2 keeps formatting escapes such as `\.br\` and
`\H\` undecoded because they say something a plain string cannot carry — so
a wire format built on the decoded form could not, in general, encode back
to the original bytes. Serializing `raw` sidesteps the problem entirely:
nothing is decoded, so nothing needs to be re-encoded.

## 4.3 What does not round-trip, and why that is out of scope

`er7::Message::to_text()` and the `to_text` methods on every tree level
(see `er7`'s spec §7, its own documented exception) decode leaves for
display and are explicitly **not** re-parseable — a decoded `\F\` becomes a
literal field separator, which a parser would misread as structure. This
crate has no `to_text`-based wire shape and no plan to add one: a
"deserialize the decoded form back into a `Message`" operation cannot
satisfy §4.1 in general, and offering it anyway would invite exactly the
silent corruption `er7`'s own documentation warns against.

## 4.4 The distinction that must survive alongside the bytes

`er7`'s spec R10/R11 require absent, empty, and the explicit null (`""`)
to stay three different answers, not one, because collapsing them is a
patient-safety bug (a withdrawn value read back as merely blank). Because
this crate changes no data, only its container, that distinction survives
automatically: an absent field has no repetitions and serializes as `[]`
(S4); an empty field has repetitions holding empty subcomponents; a null
field's one subcomponent serializes as the four-character string `"\"\""`
(the ER7 null, written literally, since it is never decoded per §4.2).
`tests/integration.rs`'s
`keeps_absent_empty_and_null_distinct_through_json` test pins this down
directly.

## 4.5 Testing this guarantee

Every fixture in `tests/integration.rs` follows the same shape: parse,
serialize, deserialize, assert the ER7 text is unchanged. New fixtures
added for a bug fix or a new example message should follow the same
pattern rather than asserting on the JSON's own shape, which is already
covered by [§2](../02-wire-shapes/index.md) and the per-level unit tests.
