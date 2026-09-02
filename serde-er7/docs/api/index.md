[← docs](../../index.md#documentation)

# API reference

The rendered version of this — with full signatures, doctests, and
intra-doc links — is at <https://docs.rs/serde-er7/>, or locally with
`cargo doc --no-deps --open`. This page is the short version: every public
item, one line each, organized the way most callers will reach for them.

## Entry point

| Item | What |
| ---- | ---- |
| `Message` | Wraps `er7::Message`. `Message::parse(text)` parses ER7 directly into a Serde-enabled value. Serializes as `{"separators": ..., "segments": [...]}`. |
| `Message::parse(text: &str) -> Result<Message, er7::Error>` | Parse ER7 text; forwards `er7::parse`'s own error. |

## The tree, top to bottom

| Item | Wraps | Wire shape |
| ---- | ----- | ---------- |
| `Segment` | `er7::Segment` | `{"name": "...", "fields": [...]}` |
| `Field` | `er7::Field` | array of `Repetition` |
| `Repetition` | `er7::Repetition` | array of `Component` |
| `Component` | `er7::Component` | array of subcomponent strings |
| `Subcomponent` | `er7::Subcomponent` | a bare string (`raw`, not decoded) |

## Delimiters and rendering

| Item | Wraps | Wire shape |
| ---- | ----- | ---------- |
| `Separators` | `er7::Separators` | object of six named fields; `char`s as one-character strings |
| `Terminator` | `er7::Terminator` | one of the strings `"Cr"`, `"Lf"`, `"CrLf"` |

## Conventions every wrapper follows

All eight types above additionally provide, uniformly:

- `Deref`/`DerefMut` to the wrapped `er7` type — call any `er7` method
  (`.query(...)`, `.to_er7()`, `.is_null()`, ...) directly on the wrapper.
- `From<er7::X> for X` and `From<X> for er7::X` — `.into()` either
  direction.
- `Debug`, `Clone`, `PartialEq`, `Eq` (and `Copy`/`Default` where the
  wrapped `er7` type itself supports them).
- Field `.0` is `pub`, so a wrapper can also be constructed or destructured
  directly: `Message(er7_message)`, `let Message(inner) = wrapped;`.

See [`spec/02-wire-shapes/index.md`](../../spec/02-wire-shapes/index.md)
for the complete, normative version of the wire-shape column above, and
[`spec/06-ergonomics/index.md`](../../spec/06-ergonomics/index.md) for why
the conventions above look the way they do.

## Strict deserialization

| Item | What |
| ---- | ---- |
| `Strict<T>` | Wraps `T` (one of `Message`, `Segment`, `Separators`). `Deserialize` rejects an unrecognized key instead of ignoring it — for `Strict<Message>`, at every level: top-level keys, a nested segment's keys, and the separators object's keys. `T::deserialize` alone stays tolerant, unconditionally; this is a separate, opt-in type, not a flag on the existing one. `Deref`/`DerefMut`/`From` both ways and a delegating `Serialize`, same as every other wrapper. |

Reach for it when validating a hand-written JSON fixture (like the one in
[§2](../usage/index.md#2-back-the-other-way-json-to-er7) of the usage
guide): a typo on a required key becomes an error naming the typo, instead
of a generic "missing field" that does not; a typo on the one optional key
this crate has (`separators.truncation`) becomes an error at all, instead
of silently defaulting. See
[`spec/11-strict-mode/index.md`](../../spec/11-strict-mode/index.md) for
the full rationale.

## The `er7` re-export

`serde_er7::er7` re-exports the whole `er7` crate, so `er7::Message`,
`er7::Error`, `er7::Path`, and everything else `er7` exposes are reachable
without a separate dependency on `er7` in your own `Cargo.toml`.

## What is not here

No format-specific function (`to_json_string`, `from_yaml_str`, ...) is
part of this API — see
[`spec/03-dependencies-and-format-agnosticism/index.md`](../../spec/03-dependencies-and-format-agnosticism/index.md)
for why. Call your chosen format's own function
(`serde_json::to_string(&message)`, and so on) directly.
