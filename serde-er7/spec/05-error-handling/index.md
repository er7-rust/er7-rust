[index](../index.md) → §5 Error handling

# §5 Error handling

## 5.1 Two different kinds of failure

This crate sits between two error domains that must not be conflated:

- **ER7-level failure** — the input text has no usable header. This is
  `er7::Error`, entirely `er7`'s own concern (its spec §11), and
  `Message::parse` simply forwards it unchanged.
- **Serde-level failure** — the *Serde* input (JSON, YAML, ...) does not
  match the shape in [§2](../02-wire-shapes/index.md): a required key is
  missing, a string was expected where an array appeared, and so on. This
  is reported through the format's own error type (`D::Error`/`S::Error`),
  via the standard `serde::de::Error`/`serde::ser::Error` constructors.

A message can be well-formed ER7 and fail at the Serde layer (malformed
JSON handed to `from_str`), or vice versa is not applicable in the other
direction — Serde deserialization builds a `Message` directly, without
re-parsing ER7 text, so an `er7::Error` never surfaces from a
`Deserialize` call.

## 5.2 Rule S8: unknown fields are ignored, not rejected

Every `visit_map` implementation in this crate matches known keys and
routes everything else to `serde::de::IgnoredAny`, exactly the pattern
[serde's own manual-implementation
guide](https://docs.rs/serde/latest/serde/) shows for `Point`. This is a
direct extension of `er7`'s own tolerance principle (its spec §11.3, rule
R6: "nothing below the header may fail") into the Serde layer — a producer
that has added a field this crate does not yet know about should not break
a consumer that only needs the fields it already understands.

This is a deliberate asymmetry with `#[serde(deny_unknown_fields)]`-style
strictness some Serde types choose: `T::deserialize` always behaves as if
that attribute is absent, for every wrapper type. A change that made
`Message::deserialize` (or any other type's) reject unknown fields *by
default* would be a breaking change to this rule and would need a spec
update here first — that has not happened. What has been added is an
opt-in alternative *type*, not a flag on this one:
[§11](../11-strict-mode/index.md)'s `Strict<T>` gives a caller who wants
`deny_unknown_fields`-style checking a way to ask for it explicitly, at the
one call site that wants it, without this rule's own default changing for
anyone who does not.

## 5.3 Rule S9: a missing required field names itself

`missing_field("separators")`, `missing_field("name")`, and so on — every
required key uses `serde::de::Error::missing_field`, which every Serde
format renders with the field's own name in the message. A caller
debugging a hand-written JSON fixture (see
`examples/build_message_from_json.rs`) gets "missing field `segments`," not
a generic "invalid input."

## 5.4 A duplicate key is also an error

`serde::de::Error::duplicate_field` fires if the same key appears twice in
one object — a case only some formats can even produce (JSON permits
duplicate keys at the token level; this crate's `visit_map` loop rejects
the second occurrence rather than silently taking the last one). This
matches the general Serde convention for hand-written struct visitors and
avoids a caller's typo (`"separators"` twice, `"segments"` never) silently
producing a `Message` with defaulted, wrong data.

## 5.5 No panics

No implementation in this crate panics on malformed *input* of either kind.
A logic error in this crate's own code (an index out of bounds while
converting `Vec<Segment>` — none currently exist) would still be a bug, but
handling a caller's malformed data is always a `Result`, never an
`unwrap`/`expect`/`panic!`. `#![warn(missing_docs)]` and `clippy`'s default
lints, both required by the checks in
[the index](../index.md#required-checks), are the mechanical backstop for
this.
