[er7](../../index.md) → [docs](../) → api

# API surface

The complete public API of the `er7` crate, in one page. Rendered rustdoc
with full signatures is at <https://docs.rs/er7/>, or locally with
`cargo doc --no-deps --open`.

Behaviour is specified in [`spec/`](../../spec/index.md); this page is a map,
not a contract.

## Entry points

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `er7::parse` | `fn(&str) -> Result<Message, Error>` | needs an `MSH`/`FHS`/`BHS` header ([§4.2](../../spec/04-parsing.md)) |
| `er7::parse_with` | `fn(&str, Separators) -> Message` | for a headerless fragment; cannot fail |
| `er7::split_messages` | `fn(&str) -> Vec<&str>` | batch or concatenated input; slices borrow ([§9](../../spec/09-batch-input.md)) |

`er7::parse` names both a module and a function. That is legal — they live
in different namespaces — so `er7::parse(text)` calls the function and
`er7::parse::split_messages` resolves the module path.

## The value tree

Six types, one per level, all fields `pub`
([§5.1](../../spec/05-value-tree.md)). All derive `Debug`, `Clone`,
`PartialEq`, `Eq`; the four below `Segment` also derive `Default`.

```
Message { separators: Separators, segments: Vec<Segment> }
Segment { name: String, fields: Vec<Field> }
Field { repetitions: Vec<Repetition> }
Repetition { components: Vec<Component> }
Component { subcomponents: Vec<Subcomponent> }
Subcomponent { raw: String }
```

### `Message`

| Method | Returns | Purpose |
| ------ | ------- | ------- |
| `segment(&str)` | `Option<&Segment>` | the first segment with that name |
| `segment_at(&str, usize)` | `Option<&Segment>` | the 1-based Nth of that name |
| `segment_at_mut(&str, usize)` | `Option<&mut Segment>` | the same, mutable |
| `segments_named(&str)` | `impl Iterator<Item = &Segment>` | every segment with that name |
| `header()` | `Option<&Segment>` | the first segment, which declared the delimiters |
| `query(&str)` | `Result<Option<String>, Error>` | first match at a path, decoded |
| `query_all(&str)` | `Result<Vec<String>, Error>` | every match, decoded |
| `query_path(&Path)` | `Vec<String>` | every match, decoded, path pre-parsed |
| `query_path_raw(&Path)` | `Vec<String>` | every match, exactly as sent |
| `message_code()` | `Option<String>` | MSH-9.1 |
| `trigger_event()` | `Option<String>` | MSH-9.2 |
| `message_structure()` | `Option<String>` | MSH-9.3 |
| `control_id()` | `Option<String>` | MSH-10 |
| `version()` | `Option<String>` | MSH-12.1 |
| `to_er7()` | `String` | write with default options |
| `to_er7_with(RenderOptions)` | `String` | write, choosing the terminator |

Also `impl Display for Message`, equivalent to `to_er7()`.

### `Segment`

| Method | Returns |
| ------ | ------- |
| `field(usize)` / `field_mut(usize)` | `Option<&Field>` / `Option<&mut Field>` |
| `component(usize, usize)` | `Option<&Component>` — field, then component, first repetition |
| `is_header()` | `bool` — `MSH`, `FHS`, or `BHS` |
| `to_er7(&Separators)` / `to_text(&Separators)` | `String` |

### `Field`

| Method | Returns |
| ------ | ------- |
| `repetition(usize)` / `repetition_mut(usize)` | `Option<&Repetition>` / `Option<&mut Repetition>` |
| `component(usize)` | `Option<&Component>` — of the first repetition |
| `is_empty()` / `is_null()` | `bool` ([§5.3](../../spec/05-value-tree.md)) |
| `to_er7(&Separators)` / `to_text(&Separators)` | `String` |

### `Repetition`

| Method | Returns |
| ------ | ------- |
| `component(usize)` / `component_mut(usize)` | `Option<&Component>` / `Option<&mut Component>` |
| `is_empty()` / `is_null()` | `bool` |
| `to_er7(&Separators)` / `to_text(&Separators)` | `String` |

### `Component`

| Method | Returns |
| ------ | ------- |
| `subcomponent(usize)` / `subcomponent_mut(usize)` | `Option<&Subcomponent>` / `Option<&mut Subcomponent>` |
| `is_empty()` / `is_null()` | `bool` |
| `to_er7(&Separators)` / `to_text(&Separators)` | `String` |

### `Subcomponent`

| Method | Returns | Purpose |
| ------ | ------- | ------- |
| `new(impl Into<String>)` | `Subcomponent` | wrap already-encoded text |
| `value(&Separators)` | `Cow<'_, str>` | the decoded text |
| `set(&str, &Separators)` | `()` | write a value, encoding delimiters |
| `is_empty()` / `is_null()` | `bool` |
| `to_er7(&Separators)` / `to_text(&Separators)` | `String` |

Also `impl From<&str> for Subcomponent`, and the constant
`er7::message::NULL` (`"\"\""`), the explicit-null literal.

## Configuration

### `Separators`

Fields: `field`, `component`, `repetition`, `escape`, `subcomponent`: `char`;
`truncation`: `Option<char>`. Derives `Copy`.

| Item | Returns | Purpose |
| ---- | ------- | ------- |
| `Separators::default()` | `Separators` | the HL7-recommended `\|^~\&` |
| `from_header(&str)` | `Result<Separators, Error>` | read from an `MSH`/`FHS`/`BHS` line |
| `validate()` | `Result<(), Error>` | reject an ambiguous set ([§3.3](../../spec/03-delimiters.md)) |
| `is_delimiter(char)` | `bool` | does this character play a structural role |
| `encoding_characters()` | `String` | the MSH-2 text |

Also `impl Display for Separators`, writing `\|^~\&`.

### `Terminator`

`Cr` (default), `Lf`, `CrLf`. Method `as_str() -> &'static str`.

### `RenderOptions`

Fields: `terminator: Terminator`, `trailing_terminator: bool`. Derives
`Default` (`Cr`, `false`).

### `Path`

Fields: `segment: String`; `segment_occurrence`, `field`, `repetition`,
`component`, `subcomponent`: `Option<usize>`. Derives `Debug`, `Clone`,
`PartialEq`, `Eq`, `Hash`.

| Item | Returns |
| ---- | ------- |
| `Path::parse(&str)` | `Result<Path, Error>` |
| `impl FromStr for Path` | the same, via `.parse()` |
| `impl Display for Path` | the canonical spelling, e.g. `OBX[2]-5.1` |

## Escape sequences

Module `er7::escape` ([§6](../../spec/06-escape-sequences.md)).

| Item | Signature |
| ---- | --------- |
| `escapes` | `fn<'a>(&'a str, &Separators) -> Escapes<'a>` |
| `Escapes<'a>` | `impl Iterator<Item = Escape<'a>>` |
| `unescape` | `fn<'a>(&'a str, &Separators) -> Cow<'a, str>` |
| `escape` | `fn<'a>(&'a str, &Separators) -> Cow<'a, str>` |
| `decode_hex` | `fn(&str) -> Option<String>` |

`Escape<'a>` variants: `Text`, `Field`, `Component`, `Subcomponent`,
`Repetition`, `EscapeCharacter`, `Hex`, `Highlight`, `Normal`,
`Formatting`, `Local`, `SingleByteCharacterSet`, `MultiByteCharacterSet`,
`Unknown`, `Unterminated`. Method
`write_er7(&mut String, &Separators)` writes a token back as it was
written.

## Errors

`Error` has four variants: `Empty`, `MissingHeader(String)`,
`BadHeader(String)`, `BadPath(String)`. Derives `Debug`, `Clone`,
`PartialEq`, `Eq`; implements `Display` and `std::error::Error`.
See [§11](../../spec/11-error-handling.md).

## Not in the API

No dictionary, no validator, no transport, no dependencies. See
[§1.3](../../spec/01-purpose-and-scope.md) for what that means and
[§18.1](../../spec/18-open-questions-and-divergences.md) for why.
