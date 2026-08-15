# Specification: the `er7` crate

This is the single source of truth for what the `er7` crate does. It
describes observable behavior, not implementation details; where a rule is
implemented, the module is named so that the two stay in sync.
[`README.md`](../README.md) summarizes this document for newcomers, and
[`er7-format.md`](er7-format.md) describes the ER7 format itself,
independent of this crate — if any of the three disagree, this document
wins for behavior and should be corrected only alongside the code.

Status: describes `er7` 0.1.0 as implemented. Every rule below is pinned by
a unit test next to the code that implements it, or by an integration test
in `tests/integration.rs`. A rule here that is not backed by a test, or a
code change not reflected here, is a bug.

## 1. Scope

Read, query, edit, and write HL7 v2 messages in the **ER7** encoding — the
pipe-hat, positional text encoding described in
[`er7-format.md`](er7-format.md) and defined by chapter 2 of every HL7 v2
release.

This crate is an **encoding**, not a dictionary. It deliberately does not
know:

- which fields a given segment has, or what data type each one carries;
- which message structures exist, or how segments group into them;
- what any code table value means;
- whether a message is valid, complete, or acceptable.

All of that is version-specific and belongs in a layer above; §9 is the one
narrow exception. What this crate guarantees instead is **fidelity**: any
message it reads, it can write back unchanged (§6.2), and any value it
reports is the value that was actually at that position.

The crate has **no dependencies**, and is meant to keep it that way.

## 2. Delimiters (`src/separators.rs`)

### 2.1 The set

A `Separators` holds six characters. Five are structural; the sixth is
informational.

| Role         | Declared by       | Recommended | Purpose |
|--------------|-------------------|:-----------:|---------|
| field        | MSH-1 — the character right after the segment name | `\|` | separates fields, and the segment name from field 1 |
| component    | MSH-2 position 1  | `^`  | separates components within a repetition |
| repetition   | MSH-2 position 2  | `~`  | separates repetitions within a field |
| escape       | MSH-2 position 3  | `\`  | opens and closes an escape sequence |
| subcomponent | MSH-2 position 4  | `&`  | separates subcomponents within a component |
| truncation   | MSH-2 position 5  | `#`  | marks a value the sender truncated; optional, HL7 v2.7 and later |

`Separators::default()` is the recommended set with no truncation
character. Nothing in the crate assumes it: every function that needs
delimiters takes them.

### 2.2 Reading them from a message

`Separators::from_header(line)` reads the set from an `MSH`, `FHS`, or
`BHS` line:

- the field separator is the character immediately after the
  three-character segment name;
- the encoding characters are the characters from there up to the next
  field separator, taken positionally, at most five;
- an encoding character the sender omitted falls back to its recommended
  value. A sender that writes `MSH|^~\|` has supplied three encoding
  characters, not four: reading stops at the field separator.

### 2.3 Validation

`Separators::validate` rejects a set that cannot encode a message
unambiguously, and `from_header` applies it to every message:

- no delimiter may be alphanumeric — this is also what distinguishes a real
  header from a line that merely starts with those three letters;
- no delimiter may be a carriage return or a line feed, which end a
  segment;
- no two delimiters may be the same character.

A failing set is `Error::BadHeader`. Nothing else about a message can fail
this way.

## 3. Parsing (`src/parse.rs`)

### 3.1 Lines

- A leading byte-order mark (`U+FEFF`) is removed.
- Input is divided at `\r`, `\n`, or `\r\n`; each division is one segment.
- Lines that are empty or entirely whitespace are dropped.
- **Nothing else is trimmed.** A value that really ended in a space keeps
  it, because the crate cannot know whether that space is data.

### 3.2 `parse`

`parse(text)` requires the first surviving line to be an `MSH`, `FHS`, or
`BHS` segment; its delimiters (§2.2) govern the whole message. A different
first segment is `Error::MissingHeader`, and no lines at all is
`Error::Empty`.

Below the header nothing can fail. Unknown segments, local `Z` segments,
ragged field counts, segments with no fields, and stray empty positions are
all data.

### 3.3 `parse_with`

`parse_with(text, separators)` parses a fragment that has no header of its
own — one segment lifted from a log, or a body whose `MSH` was read
separately. It cannot fail: whatever the text is, it becomes a tree.

### 3.4 Splitting

Each segment line splits into fields on the field separator; each field
into repetitions; each repetition into components; each component into
subcomponents. Leaf text is stored exactly as it arrived (§4.1).

Two exceptions:

- **A field the sender left empty has no repetitions at all**, which is
  what separates "not sent" from a repetition that is present and blank.
  `A~~B` is three repetitions, the middle one blank; `||` is none.
- **Fields 1 and 2 of a header segment are the delimiters themselves** —
  MSH-1 *is* the field separator and MSH-2 *is* the encoding characters.
  They are stored whole, never split and never escape-decoded, because
  doing either would be circular.

## 4. The value tree (`src/message.rs`)

### 4.1 Shape

`Message` → `Segment` → `Field` → `Repetition` → `Component` →
`Subcomponent`. Only `Subcomponent` holds text, in its public `raw` field,
exactly as sent. Decoding happens on demand (§5.2), which is what makes
§6.2 possible.

Every level exposes 1-based accessors matching HL7's own numbering:
`segment.field(5)`, `field.repetition(2)`, `repetition.component(1)`,
`component.subcomponent(2)`, plus `_mut` variants and the shortcuts
`field.component(n)` (first repetition) and `segment.component(f, c)`.

### 4.2 Absent, empty, and null

ER7 distinguishes three states, and so does this crate:

| State  | On the wire | How to ask |
|--------|-------------|------------|
| absent | the field was never sent | the accessor returns `None` |
| empty  | `\|\|` — sent with no value | `is_empty()` |
| null   | `\|""\|` — sent to clear the value | `is_null()` |

`is_empty` and `is_null` are never both true: the explicit null is text.
The distinction matters, because a receiver updating a record must leave an
absent or empty field alone and must clear a null one.

`Subcomponent::value` reports the explicit null as the empty string, since
that is the value being conveyed; ask `is_null` when the difference
matters.

### 4.3 Editing

`Subcomponent::raw` is public and may be assigned directly, in which case
the caller is responsible for the text containing no unescaped delimiters.
`Subcomponent::set(value, separators)` does that encoding for you (§5.3),
and is the recommended way to write a value.

## 5. Escape sequences (`src/escape.rs`)

### 5.1 The vocabulary

`escapes(text, separators)` tokenizes text into `Escape` values: literal
runs, plus one token per sequence, classified as HL7 defines them —
`\F\ \S\ \T\ \R\ \E\` for the delimiters, `\H\` and `\N\` for highlighting,
`\Xdd..\` for hexadecimal data, `\Zdd..\` for local extensions,
`\Cxxyy\` and `\Mxxyyzz\` for character-set switches, and `\.cmd\` for
formatted-text display commands. A body matching none of these is
`Unknown`; an escape character with no closing partner is `Unterminated`.

Tokenizing never fails, and it is lossless: writing every token back with
`Escape::write_er7` reproduces the input exactly.

### 5.2 Decoding

`unescape(text, separators)` resolves **only the sequences that stand for
characters**: the five delimiter sequences, and `\Xdd..\` when its body is
whole pairs of hexadecimal digits, decoded as bytes and read as UTF-8 with
the usual lossy replacement.

Every other sequence is **kept literally**, escape characters included:
highlighting, formatting commands, character-set switches, local
extensions, unrecognized bodies, and an unterminated escape character.
These say something about presentation or encoding that a plain string
cannot carry, so dropping them would lose more than keeping them.

### 5.3 Encoding

`escape(text, separators)` is the inverse for text containing no sequences
of its own. It encodes the five structural delimiters, and encodes a
carriage return as `\X0D\` and a line feed as `\X0A\` — those would
otherwise end the segment, the one corruption an ER7 writer must never
commit. The truncation character is not encoded: it is structural only
inside MSH-2.

`unescape(escape(value)) == value` for every `value`.

## 6. Writing (`src/render.rs`)

### 6.1 The two forms

Every level of the tree offers a pair:

- `to_er7(separators)` — exactly what a receiver would read, escape
  sequences intact;
- `to_text(separators)` — the same, with leaf text escape-decoded per §5.2.

Structural delimiters remain in both, because `SMITH^JOHN` without its
caret is no longer a name and a surname. Fields 1 and 2 of a header are
written literally in both forms (§3.4).

`Message::to_er7()` writes a whole message and `Message::to_er7_with`
takes `RenderOptions`:

- `terminator` — `Cr` (the default, and the only terminator HL7 permits on
  the wire), `Lf`, or `CrLf`;
- `trailing_terminator` — whether the last segment gets one too; `false` by
  default, because a trailing terminator surprises callers that compare or
  concatenate the result, and the transport already marks the end.

`Display for Message` is `to_er7()`.

### 6.2 Round trip

For any message this crate parsed, writing it back reproduces the input
**byte for byte**, except where the input was not already canonical.
Writing is canonical when it fixes the two things §3.1 normalizes: blank
lines are gone, and every terminator is the one `RenderOptions` chose.
Canonical text round-trips exactly.

This holds for messages with unusual delimiters, unknown segments, empty
positions, and escape sequences the crate does not decode.

## 7. Paths and queries (`src/path.rs`, `src/message.rs`)

### 7.1 Notation

A path names one place in a message. The grammar:

```
path      = name occurrence? ( ("-" | ".") index occurrence? ( "." index ( "." index )? )? )?
name      = one or more letters and digits
occurrence = "[" index "]"
index     = a number, 1 or greater
```

So `PID`, `PID-5`, `PID-5.1`, `PID-5.1.2`, `OBX[2]-5`, `PID-13[2].1`. Both
`PID-5.1` and `PID.5.1` are accepted, because both are in common use; the
first is what `Display` writes. Surrounding whitespace is ignored.

Every index is 1-based. A `0` is rejected rather than clamped, because it
is almost always an off-by-one: `Error::BadPath`.

### 7.2 What a query returns

`Message::query(path)` returns the first match; `query_all` returns every
match in message order; `query_path` and `query_path_raw` take an
already-parsed `Path`, decoded and as-sent respectively.

- An omitted segment occurrence matches **every** segment of that name, so
  `OBX-5` on a result with three `OBX` segments returns three values.
- An omitted repetition matches **every** repetition, unless the path stops
  at the field, in which case the whole field is returned with its
  repetition separators intact.
- A path that names a level above a subcomponent returns that subtree
  written back per §6.1 — `PID-5` on `SMITH^JOHN` gives `SMITH^JOHN`, and
  `PID-5.1` gives `SMITH`.
- A position the message does not have contributes nothing; it is not an
  error and not an empty string.
- Fields 1 and 2 of a header are returned literally, in either mode.

## 8. Batch and multi-message input (`split_messages`)

`split_messages(text)` cuts input holding several messages, or a whole HL7
batch file, into individual messages. The returned slices borrow from the
input and keep its original terminators, so each can be handed straight to
`parse`.

- A new message begins at each line whose segment name is `MSH`.
- The batch envelope segments `FHS`, `BHS`, `BTS`, and `FTS` are left out:
  they describe the file, not a message. The name is matched exactly, so a
  local segment such as `BTSX` is not mistaken for a batch trailer.
- If the first surviving line is not an `MSH`, it still starts a message —
  which `parse` then rejects on its own. Reporting that is better than
  silently dropping it.

## 9. MSH conveniences

`Message` exposes five accessors that read fixed MSH positions:
`message_code` (MSH-9.1), `trigger_event` (MSH-9.2), `message_structure`
(MSH-9.3), `control_id` (MSH-10), and `version` (MSH-12.1). Each returns
`None` when the position is absent or empty.

These are the only HL7 semantics in the crate. They earn the exception
because every tool that touches a message needs them to route or log it,
and because those positions have not moved in any v2 release. Deriving a
structure from the code and trigger event when MSH-9.3 is absent is
**not** done here: that mapping is version-specific and belongs above this
layer.

## 10. Errors

`Error` has four variants, and only two situations produce them:

| Variant                 | When |
|-------------------------|------|
| `Empty`                 | the input held no segments |
| `MissingHeader(name)`   | the first segment is not `MSH`, `FHS`, or `BHS` |
| `BadHeader(detail)`     | the declared delimiters are unusable (§2.3) |
| `BadPath(detail)`       | a path could not be read (§7.1) |

Everything else about ER7 is recoverable and is recovered rather than
refused, because a receiver that rejects a message it could have read is
worse than one that reads it as written.

## 11. Limitations

These are scope boundaries, not defects:

- **No dictionary.** See §1. Segment definitions, data types, message
  structures, and code tables are out of scope.
- **No validation.** Nothing checks cardinality, length, required fields,
  or table membership.
- **No transport.** MLLP framing, TCP, files, and acknowledgement
  workflows are out of scope; this crate handles the bytes between the
  frames.
- **Formatting escapes are text.** `\.br\`, `\H\`, `\Cxxyy\`, and the rest
  are preserved but not interpreted (§5.2). Rendering formatted text is a
  presentation concern.
- **Character sets.** Input and output are Rust strings, so UTF-8.
  `\Xdd..\` bytes are decoded as UTF-8 with lossy replacement; a message in
  some other repertoire declared by MSH-18 or `\Cxxyy\` is not transcoded.

## 12. Command-line behavior (`src/main.rs`)

Documented here because it is a contract, not an implementation detail.

```
er7 [OPTIONS] [FILE]
```

`FILE` holds one or more messages, or a batch file; `-` or no argument
reads standard input. Input is split per §8, and every message is parsed
before anything is written.

| Option | Effect |
|--------|--------|
| `-q, --query <PATH>` | print the values at an HL7 path, one per line; may be repeated, and the outputs appear in the order the options were given |
| `-n, --normalize`    | rewrite the input as canonical ER7 (§6.2), with a trailing terminator on every message |
| `-m, --message <N>`  | use only the Nth message of the input, counting from 1 |
| `-r, --raw`          | show text as sent, without decoding escape sequences |
| `-t, --terminator <KIND>` | segment terminator to write: `cr` (default), `lf`, `crlf` |
| `-o, --output <FILE>`| write to `FILE` instead of standard output |
| `-h, --help`         | print usage |
| `-V, --version`      | print the version |

With neither `--query` nor `--normalize`, the command prints an **outline**:
one line per value, labelled with the HL7 path that names it and aligned
into two columns. A path in the outline can be pasted straight back into
`--query`, so repeated segments are labelled `OBX[2]-3.1` and repeated
fields `PID-13[2]`. A level with only one child is not indexed, so a name
sent as a single component reads `NTE-3`, not `NTE-3.1`. Positions with no
value are left out, but an explicit null is shown as the `""` it was sent
as, since that is exactly what distinguishes it. Carriage returns, line
feeds, and tabs inside a decoded value are shown as `\r`, `\n`, and `\t`,
so that one value stays on one line.

When the input holds more than one message, each outline is preceded by a
`# message N` heading naming the message code, trigger event, control ID,
and version where the message supplies them.

Combining `--query` with `--normalize` is an error, since they ask for
different output.

Exit code 0 on success, 1 on any error, with a message on standard error
prefixed `er7: error: `. A query that matches nothing prints nothing and
still succeeds — the message simply did not carry that value.

## 13. References

- [HL7 v2.5 chapter 2, control](https://www.hl7.eu/HL7v2x/v25/std25/ch02.html) — the encoding rules, delimiter table, and escape sequences
- [HL7 v2.8 chapter 2](https://www.hl7.eu/HL7v2x/v28/std28/ch02.html) — the same, with the truncation character
- [HL7 v2+ XML encoding syntax](http://v2plus.hl7.org/2021Jan/xml-encoding-rules.html) — the alternative encoding, and what ER7 is being contrasted with
- [Caristix: HL7 ER7 encoding](https://caristix.com/help-center/v3/test/task/hl7-er7-encoding/)
- [Rhapsody: HL7 escape sequences](https://rhapsody.health/resources/hl7-escape-sequences/)
- [`hl7-2-5-to-xml-using-rust`](https://github.com/joelparkerhenderson/hl7-2-5-to-xml-using-rust) — a sibling crate that adds the v2.5 dictionary this one deliberately omits
