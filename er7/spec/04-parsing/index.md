[`er7` specification](../index.md) — section 4 of 19. Section numbers
(§4.x) are stable and cited from code, tests, and commit messages.

# 4. Parsing

Implemented in `src/parse.rs`. Produces the value tree of
[§5](../05-value-tree/index.md).

## 4.1 Lines

**[R4]**

- A leading byte-order mark (`U+FEFF`) is removed. Text editors add it;
  it is not part of the message.
- Input is divided at `\r`, `\n`, or `\r\n`; each division is one segment.
- Lines that are empty or entirely whitespace are dropped.
- **Nothing else is trimmed.** A value that really ended in a space keeps
  it, because the crate cannot know whether that space is data. This is a
  deliberate divergence from the sibling crate
  `hl7-2-from-er7-into-xml`, which trims; there, fidelity is not a goal,
  and here it is (see [§1.5](../01-purpose-and-scope/index.md) priority 1).

The scan is done at the byte level, which is safe because `\r` and `\n`
cannot appear inside a multi-byte UTF-8 sequence.

## 4.2 `parse`

```rust
pub fn parse(text: &str) -> Result<Message, Error>
```

**[R5]** The first surviving line must be an `MSH`, `FHS`, or `BHS`
segment; its delimiters ([§3.2](../03-delimiters/index.md)) govern the
whole message.

| Input | Result |
| ----- | ------ |
| no non-blank lines | `Error::Empty` |
| a first segment that is not a header | `Error::MissingHeader(name)` |
| a header whose delimiters are unusable | `Error::BadHeader(detail)` |
| anything else | `Ok(Message)` |

**[R6]** Below the header nothing can fail. Unknown segments, local `Z`
segments, ragged field counts, segments with no fields at all, and stray
empty positions are data, not errors.

The segment name used for the header check is the leading run of letters
and digits. This is exact rather than a guess: a field separator is never
alphanumeric ([§3.3](../03-delimiters/index.md)), so the run ends precisely
where the name does. `MSHX|...` therefore has the name `MSHX`, not `MSH`.

## 4.3 `parse_with`

```rust
pub fn parse_with(text: &str, separators: Separators) -> Message
```

Parses a fragment that has no header of its own — one segment lifted from a
log file, or a message body whose `MSH` was read separately. It cannot
fail: whatever the text is, it becomes a tree.

Use this when you already know the delimiters. Use `parse` when the text
carries its own.

## 4.4 Splitting into the tree

Each segment line splits into fields on the field separator; each field
into repetitions; each repetition into components; each component into
subcomponents. **[R9]** Leaf text is stored exactly as it arrived.

Two exceptions:

### 4.4.1 An empty field has no repetitions [R7]

A field the sender left empty has **zero** repetitions, which is what
separates "not sent" from a repetition that is present and blank.

| Input | Repetitions | Note |
| ----- | ----------- | ---- |
| `\|\|` | 0 | nothing was sent |
| `\|A\|` | 1 | one value |
| `\|A~~B\|` | 3 | the middle one is present and blank |
| `\|~\|` | 2 | both present, both blank |

Empty positions below the field level are always kept, because position is
what gives a value meaning: `^^C` is three components, not one.

### 4.4.2 Header fields 1 and 2 are the delimiters [R8]

MSH-1 *is* the field separator and MSH-2 *is* the encoding characters. They
are stored whole — never split on any delimiter, never escape-decoded —
because doing either would be circular. The same applies to `FHS` and
`BHS`.

This keeps HL7®'s own numbering intact: `msh.field(1)` is the field
separator, `msh.field(9)` is the message type, exactly as the standard
numbers them.

## 4.5 Complexity and allocation

Parsing is a single pass, O(n) in the input length. One `String` is
allocated per subcomponent and per segment name; nothing else is copied.
Callers who want to avoid even that for a read-only pass should use
[`split_messages`](../09-batch-input/index.md), which borrows, and query
the resulting `&str` directly.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
