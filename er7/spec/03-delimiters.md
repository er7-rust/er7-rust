[`er7` specification](index.md) — section 3 of 19. Section numbers (§3.x) are stable and cited from code, tests, and commit messages.

# 3. Delimiters

Implemented in `src/separators.rs`. Background in
[§2.3](02-er7-encoding.md).

## 3.1 The set

A `Separators` holds six characters. Five are structural; the sixth is
informational.

| Role | Declared by | Recommended | Purpose |
|------|-------------|:-----------:|---------|
| field | MSH-1 — the character right after the segment name | `\|` | separates fields, and the segment name from field 1 |
| component | MSH-2 position 1 | `^` | separates components within a repetition |
| repetition | MSH-2 position 2 | `~` | separates repetitions within a field |
| escape | MSH-2 position 3 | `\` | opens and closes an escape sequence |
| subcomponent | MSH-2 position 4 | `&` | separates subcomponents within a component |
| truncation | MSH-2 position 5 | `#` | marks a value the sender truncated; optional, HL7 v2.7 and later |

`Separators::default()` is the recommended set with no truncation
character. **[R1]** Nothing in the crate assumes it: every function that
needs delimiters takes them as a parameter.

The truncation character is modelled as `Option<char>` rather than
defaulting to `#`, because "the sender declared a truncation character" and
"the sender did not" are different facts, and only the first licenses a
receiver to read `#` as a truncation marker.

## 3.2 Reading them from a message

`Separators::from_header(line)` reads the set from an `MSH`, `FHS`, or
`BHS` line:

- the field separator is the character immediately after the
  three-character segment name;
- the encoding characters are the characters from there up to the next
  field separator, taken positionally, at most five;
- **[R3]** an encoding character the sender omitted falls back to its
  recommended value.

Worked example — a sender that writes `MSH|^~\|` has supplied *three*
encoding characters, not four, because reading stops at the field
separator. The escape character is `\` (as declared) and the subcomponent
separator falls back to `&`. A sender cannot declare `|` as the
subcomponent separator this way, and no ER7 message can: the field
separator terminates MSH-2.

## 3.3 Validation

**[R2]** `Separators::validate` rejects a set that cannot encode a message
unambiguously, and `from_header` applies it to every message:

| Rejected | Why |
|----------|-----|
| an alphanumeric delimiter | a real header never has one; this also distinguishes a header from a line that merely starts with those three letters, such as `MSHX` |
| `\r` or `\n` as a delimiter | it would end the segment instead |
| the same character in two roles | a value split on it could not be reassembled |

A failing set is `Error::BadHeader` ([§11](11-error-handling.md)) carrying
a sentence naming the two roles or the offending character. Nothing else
about a message can fail this way.

Validation is deliberately narrow. It rejects only sets that make parsing
*ambiguous*, never sets that are merely unusual: `MSH#*!?@#` parses as
happily as `MSH|^~\&|`.

## 3.4 Writing them back

- `Separators::encoding_characters()` returns the MSH-2 text (`^~\&`, or
  `^~\&#` when a truncation character is present). It excludes the field
  separator, which is MSH-1.
- `Display for Separators` writes the field separator followed by the
  encoding characters (`|^~\&`) — the first five characters of the message
  after the segment name.
- `Separators::is_delimiter(c)` reports whether `c` plays any structural
  role in this set, including truncation.

## 3.5 Segment terminators

`Terminator` chooses what ends a segment when a message is written:

| Variant | Writes | When to use |
|---------|--------|-------------|
| `Cr` (default) | `\r` | the only terminator HL7 permits on the wire |
| `Lf` | `\n` | messages kept in text files, or shown in a terminal |
| `CrLf` | `\r\n` | messages exchanged with Windows-oriented tooling |

Parsing always accepts all three (R4); this type governs output only
([§7](07-writing.md)).
