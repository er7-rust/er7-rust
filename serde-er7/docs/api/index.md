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

See [`spec/02-wire-shapes.md`](../../spec/02-wire-shapes.md) for the
complete, normative version of the wire-shape column above, and
[`spec/06-ergonomics.md`](../../spec/06-ergonomics.md) for why the
conventions above look the way they do.

## The `er7` re-export

`serde_er7::er7` re-exports the whole `er7` crate, so `er7::Message`,
`er7::Error`, `er7::Path`, and everything else `er7` exposes are reachable
without a separate dependency on `er7` in your own `Cargo.toml`.

## What is not here

No format-specific function (`to_json_string`, `from_yaml_str`, ...) is
part of this API — see
[`spec/03-dependencies-and-format-agnosticism.md`](../../spec/03-dependencies-and-format-agnosticism.md)
for why. Call your chosen format's own function
(`serde_json::to_string(&message)`, and so on) directly.
