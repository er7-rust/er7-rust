[index](../index.md) → §2 Wire shapes

# §2 Wire shapes

This is the normative table: what each wrapper type must serialize as, and
must accept when deserializing. A change to any shape here is a breaking
change (see [§8](../08-versioning-and-compatibility/index.md), rule S10).

## 2.1 The table

| Type | Shape | Example |
|------|-------|---------|
| `Message` | object, fields `"separators"`, `"segments"` | `{"separators": {...}, "segments": [...]}` |
| `Segment` | object, fields `"name"`, `"fields"` | `{"name": "PID", "fields": [...]}` |
| `Field` | array of `Repetition` | `[["555-1111"], ["555-2222"]]` |
| `Repetition` | array of `Component` | `[["SMITH"], ["JOHN"]]` |
| `Component` | array of subcomponent strings | `["ACME", "1.2.3", "ISO"]` |
| `Subcomponent` | a bare string, `raw` | `"SMITH"` |
| `Separators` | object, six named `char`/`Option<char>` fields | `{"field": "\|", "component": "^", "repetition": "~", "escape": "\\", "subcomponent": "&", "truncation": null}` |
| `Terminator` | one of the strings `"Cr"`, `"Lf"`, `"CrLf"` | `"Lf"` |

## 2.2 Rules

- **S3**: `Subcomponent` serializes `raw`, the text exactly as the sender
  wrote it. It never serializes [`er7::Subcomponent::value`]'s decoded
  form. This is what makes the round-trip guarantee in
  [§4](../04-round-trip-guarantee/index.md) possible: decoding is lossy (an
  escaped formatting instruction such as `\.br\` has no plain-text form to
  decode back to), so any wire format built on the decoded value could not,
  in general, be turned back into the original ER7 text.

- **S4**: `Field`, `Repetition`, and `Component` serialize as bare arrays,
  not as single-field objects (`{"repetitions": [...]}`). Each of these
  types holds exactly one thing — a list of the level below — so an object
  wrapper would add a key that carries no information, at every one of the
  (potentially many) nodes at that level in a real message.

- **S5**: `Message` and `Segment` serialize as objects, not arrays, because
  each carries two different kinds of information (delimiters and
  segments; a name and fields) that an array position alone would not
  distinguish without an implicit, undocumented convention for which index
  means what.

- **S6**: Each `char` field of `Separators` — `field`, `component`,
  `repetition`, `escape`, `subcomponent`, and the `Option<char>`
  `truncation` — serializes through `Serializer::serialize_char`
  individually, not packed into a combined string such as `"^~\\&"`. This
  keeps every format's own `char` handling in play (some binary formats
  give `char` a fixed-width representation a packed string would not get),
  and it means a caller reading the JSON can identify each delimiter by
  name without parsing a positional string.

- **S7**: `Terminator` serializes as its Rust variant identifier — `"Cr"`,
  `"Lf"`, or `"CrLf"` — via `serialize_str`, not `serialize_unit_variant`.
  See `src/terminator.rs` for the trade-off this makes (a value that reads
  the same in every format, at the cost of the compact index a binary
  format's `serialize_unit_variant` could otherwise use).

## 2.3 Deserializing: what is required and what is optional

- `Message` requires both `"separators"` and `"segments"`; missing either
  is a `missing_field` error (S9).
- `Segment` requires both `"name"` and `"fields"`.
- `Separators` requires `"field"`, `"component"`, `"repetition"`,
  `"escape"`, and `"subcomponent"` — the five delimiters every ER7 message
  has (`er7` spec §3.2 makes the same five mandatory when reading a
  header). `"truncation"` is the one optional key: a message that omits it
  gets `None`, matching how `er7::Separators::from_header` treats a
  message that supplies fewer than five encoding characters.
- Every object ignores keys it does not recognize (S8), so a `Message`
  produced by a newer version of this crate that has grown an additional
  field can still be read by an older one, forward-compatibly, as long as
  no currently-required key changes shape.

## 2.4 Why not derive

`#[derive(Serialize, Deserialize)]` on newtype wrappers around `er7`'s own
struct fields would produce a different, more verbose shape: `Field` would
serialize as `{"repetitions": [...]}}`, `Component` as
`{"subcomponents": [...]}`, and so on, one extra object layer at every
level. That shape is not wrong, but it reads far worse in JSON than the
bare-array form in §2.1, and it is why every implementation in this crate
is hand-written against the `Serializer`/`Deserializer` traits directly.
