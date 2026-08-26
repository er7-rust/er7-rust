[`er7` specification](../index.md) — section 6 of 19. Section numbers
(§6.x) are stable and cited from code, tests, and commit messages.

# 6. Escape sequences

Implemented in `src/escape.rs`. Background in
[§2.5](../02-er7-encoding/index.md); a worked tutorial is in
[`docs/escapes/index.md`](../../docs/escapes/index.md).

## 6.1 The vocabulary [R12]

```rust
pub fn escapes<'a>(text: &'a str, separators: &Separators) -> Escapes<'a>
```

Tokenizes text into `Escape` values: literal runs, plus one token per
sequence, classified as HL7® defines them.

| Token | From | Body holds |
| ----- | ---- | ---------- |
| `Text` | a run with no escape character | the run |
| `Field`, `Component`, `Subcomponent`, `Repetition`, `EscapeCharacter` | `\F\ \S\ \T\ \R\ \E\` | — |
| `Highlight`, `Normal` | `\H\ \N\` | — |
| `Hex` | `\Xdd..\` | the digits, without the `X` |
| `Local` | `\Zdd..\` | the body, without the `Z` |
| `SingleByteCharacterSet` | `\Cxxyy\` | the body, without the `C` |
| `MultiByteCharacterSet` | `\Mxxyyzz\` | the body, without the `M` |
| `Formatting` | `\.br\`, `\.sp 2\` | the command, without the `.` |
| `Unknown` | a well-formed sequence matching nothing above | the whole body |
| `Unterminated` | an escape character with no closing partner | the remainder, escape character included |

**[R12]** Tokenizing never fails, and it is lossless: writing every token
back with `Escape::write_er7` reproduces the input exactly. That property
is what lets `unescape` leave sequences it does not decode untouched
without a second copy of the source text.

Classification is structural, not semantic: `\XZZ\` is `Hex("ZZ")` even
though `ZZ` is not hexadecimal. Deciding whether a body is *decodable* is
`decode_hex`'s job, which keeps the tokenizer total.

## 6.2 Decoding [R13]

```rust
pub fn unescape<'a>(text: &'a str, separators: &Separators) -> Cow<'a, str>
```

Resolves **only the sequences that stand for characters**:

| Sequence | Becomes |
| -------- | ------- |
| `\F\ \S\ \T\ \R\ \E\` | the corresponding delimiter |
| `\Xdd..\` | the bytes given by the digits, read as UTF-8 with lossy replacement — **only** when the body is whole pairs of ASCII hexadecimal digits |

**Every other sequence is kept literally**, escape characters included:
highlighting, formatting commands, character-set switches, local
extensions, unrecognized bodies, an undecodable `\X..\`, and an
unterminated escape character.

The reasoning: these sequences say something about presentation or
encoding that a plain `String` cannot carry. Dropping them would lose
information; guessing at them would invent it. Keeping them literal means
a caller who *does* understand them can still act on them via
[`escapes`](#61-the-vocabulary-r12), and a caller who does not sees text
that is at worst ugly rather than wrong.

`unescape` returns `Cow::Borrowed` when the text contains no escape
character at all, which is the overwhelmingly common case.

`decode_hex(body)` is public, and returns `None` for a body that is empty,
of odd length, non-ASCII, or holding a non-hexadecimal character.

## 6.3 Encoding [R14] [R15]

```rust
pub fn escape<'a>(text: &'a str, separators: &Separators) -> Cow<'a, str>
```

The inverse of §6.2 for text containing no sequences of its own.

| Character | Becomes | Why |
| --------- | ------- | --- |
| the escape character | `\E\` | encoded first, so a value holding it is encoded once, not twice |
| field, component, repetition, subcomponent separators | `\F\ \S\ \R\ \T\` | they would otherwise split the value |
| `\r` | `\X0D\` | it would otherwise end the segment |
| `\n` | `\X0A\` | many readers treat it as a terminator too |

The **truncation character is deliberately not encoded**: it is structural
only inside MSH-2, so a `#` in a value is just a `#`. It appears in
`Separators::is_delimiter` all the same, because a message may not reuse it
for another role ([§3.3](../03-delimiters/index.md)).

**[R15]** `unescape(escape(value), seps) == value` for every `value`. Note
the converse does *not* hold, and cannot: `escape(unescape(text))` is not
`text` when `text` held a sequence §6.2 leaves literal.

## 6.4 Custom delimiters

Every function here takes `&Separators`, so a message that declares
`#*!?@` escapes and unescapes with `?F?` where a conventional one uses
`\F\`. The `\X..\` selector letters (`X`, `Z`, `C`, `M`, and the leading
`.`) are fixed by the standard and do not vary with the delimiter set.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
