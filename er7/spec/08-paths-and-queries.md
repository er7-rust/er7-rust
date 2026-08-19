[`er7` specification](index.md) — section 8 of 19. Section numbers (§8.x) are stable and cited from code, tests, and commit messages.

# 8. Paths and queries

Implemented in `src/path.rs` (notation) and `src/message.rs` (resolution).
A worked tutorial is in [`docs/paths/index.md`](../docs/paths/index.md).

## 8.1 Notation [R18]

An HL7 path names one place in a message. The notation is a de-facto
standard among interface engineers rather than part of HL7 itself, so this
crate accepts the two spellings that are common in the field and writes the
first.

```
path       = name occurrence? ( ("-" | ".") index occurrence? ( "." index ( "." index )? )? )?
name       = one or more ASCII letters and digits
occurrence = "[" index "]"
index      = a decimal number, 1 or greater
```

| Path | Names |
| ---- | ----- |
| `PID` | every `PID` segment, whole |
| `PID-5` | field 5 of every `PID` |
| `PID-5.1` | component 1 of that field |
| `PID-5.1.2` | subcomponent 2 of that component |
| `OBX[2]-5` | field 5 of the **second** `OBX` only |
| `PID-13[2]` | the **second repetition** of field 13 |
| `PID-13[2].1` | component 1 of that repetition |
| `PID.5.1` | the same as `PID-5.1` |

Surrounding whitespace is ignored. `Display` writes the canonical form,
using `-` before the field number and omitting occurrence indices the path
left open, so `parse` ∘ `Display` preserves meaning.

**[R18]** Every index is 1-based. A `0` is rejected with `Error::BadPath`
rather than clamped, because it is almost always a caller's off-by-one and
silently reading it as `1` would return a plausible wrong answer.

`Path` implements `FromStr`, `Display`, `Clone`, `PartialEq`, `Eq`, and
`Hash`, so a set of paths can be used as map keys or deduplicated.

## 8.2 Resolution [R19] [R20]

| Method | Returns |
| ------ | ------- |
| `Message::query(&str)` | `Result<Option<String>, Error>` — the first match, decoded |
| `Message::query_all(&str)` | `Result<Vec<String>, Error>` — every match, decoded |
| `Message::query_path(&Path)` | `Vec<String>` — every match, decoded |
| `Message::query_path_raw(&Path)` | `Vec<String>` — every match, exactly as sent |

The `&str` forms parse the path each call and can fail with
`Error::BadPath`; the `&Path` forms take an already-parsed path, which is
what you want when applying one path to many messages.

Rules:

- **[R19]** An omitted **segment occurrence** matches every segment of that
  name. `OBX-5` on a result with three `OBX` segments returns three
  values, in message order.
- **[R19]** An omitted **repetition** matches every repetition — *unless*
  the path stops at the field, in which case the whole field is returned
  with its repetition separators intact. `PID-13` gives
  `555-1111~555-2222`; `PID-13.1` gives two values.
- A path naming a level **above a subcomponent** returns that subtree
  written back per [§7.1](07-writing.md). `PID-5` on `SMITH^JOHN` gives
  `SMITH^JOHN`; `PID-5.1` gives `SMITH`.
- **[R20]** A position the message does not have contributes **nothing**:
  no entry in the vector. It is not an error and not an empty string, so
  `query_all("OBX-5").len()` counts the `OBX` segments that actually
  carried a fifth field.
- Fields 1 and 2 of a header are returned literally in either mode
  ([§4.4.2](04-parsing.md)), so `MSH-1` gives `|` and `MSH-2` gives `^~\&`.

### 8.2.1 Reading an explicit null through a query

A null field decodes to the empty string ([§5.3](05-value-tree.md)), so
`query("PID-5")` on `|""|` returns `Some("")` — present, and empty. To tell
that apart from a field sent as `||`, either use `query_path_raw`, which
returns `""` verbatim, or reach for the node and ask `is_null()`. The CLI
outline does the former ([§12.3](12-command-line-interface.md)).

## 8.3 Choosing between paths and accessors

Both reach the same data; they differ in what they optimize for.

| Use | When |
| --- | ---- |
| paths (`query`) | the position is known at compile time as a string, or comes from configuration or a user; you want every match without writing a loop |
| accessors (`segment`/`field`/`component`) | you need the node itself — to edit it ([§5.5](05-value-tree.md)), to ask `is_null()`, or to avoid the intermediate `String` |
