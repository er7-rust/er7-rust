[er7](../../index.md) → [docs](../) → escapes

# Escape sequences

How a value carries characters that would otherwise be read as structure —
and, just as importantly, which sequences this crate leaves alone.

Normative rules are [spec §6](../../spec/06-escape-sequences/index.md); the
format background is [spec §2.5](../../spec/02-er7-encoding/index.md).

## The shape

A sequence is the message's escape character, a body, and the escape
character again:

```
\F\        \X0D\        \.br\        \Z0102\
^ ^        ^    ^
| body     | body
escape     escape
```

The escape character is whatever MSH-2 position 3 declared — conventionally
`\`, but a message using `?` writes `?F?`. Everything here takes a
`&Separators` for exactly that reason.

## The full table

| Sequence | Meaning | Decoded by `unescape`? |
| -------- | ------- | :--------------------: |
| `\F\` | the field separator as data | ✔ |
| `\S\` | the component separator as data | ✔ |
| `\T\` | the subcomponent separator as data | ✔ |
| `\R\` | the repetition separator as data | ✔ |
| `\E\` | the escape character as data | ✔ |
| `\Xdd..\` | hexadecimal data; pairs of digits, each pair one byte | ✔ when the body is whole hex pairs |
| `\H\` | start highlighting | ✘ kept literally |
| `\N\` | normal text, ending highlighting | ✘ |
| `\Zdd..\` | locally defined, agreed between the two ends | ✘ |
| `\Cxxyy\` | switch to a single-byte character set | ✘ |
| `\Mxxyyzz\` | switch to a multi-byte character set | ✘ |
| `\.br\`, `\.sp 2\`, … | formatted-text display commands | ✘ |
| anything else, well-formed | unrecognized | ✘ |
| an escape character with no partner | unterminated | ✘ |

The display commands, used inside `FT` fields: `.sp <n>`, `.br`, `.fi`,
`.nf`, `.in <n>`, `.ti <n>`, `.sk <n>`, `.ce`.

## Why half the table is "kept literally"

The sequences marked ✘ say something about **presentation or encoding** that
a plain `String` cannot carry. There are three options and only one of them
is honest:

| Option | Result |
| ------ | ------ |
| drop them | loses information the sender sent |
| guess at them | invents information the sender did not send |
| keep them as written | the caller sees exactly what arrived |

So `unescape(r"line\.br\next")` is `line\.br\next`, unchanged. A caller who
*does* understand display commands can still act on them, via
[`escapes`](#the-tokenizer) below. A caller who does not sees text that is
at worst ugly rather than wrong.

## Decoding

```rust
use er7::{Separators, escape::unescape};

let separators = Separators::default();

assert_eq!(unescape(r"Smith \T\ Jones", &separators), "Smith & Jones");
assert_eq!(unescape(r"a\F\b", &separators), "a|b");
assert_eq!(unescape(r"\X0D\", &separators), "\r");
assert_eq!(unescape(r"\X4142\", &separators), "AB");

// Not decoded, and not damaged.
assert_eq!(unescape(r"line\.br\next", &separators), r"line\.br\next");
assert_eq!(unescape(r"\H\loud\N\", &separators), r"\H\loud\N\");
assert_eq!(unescape(r"a\Fb", &separators), r"a\Fb");   // unterminated
```

`\X..\` decodes only when its body is whole pairs of ASCII hexadecimal
digits; `\XZZ\` and `\X123\` are kept literally, because a body that is not
hex was not hex data. `decode_hex` is public if you want to make that
judgement yourself:

```rust
assert_eq!(er7::escape::decode_hex("0D0A"), Some("\r\n".to_string()));
assert_eq!(er7::escape::decode_hex("GG"), None);
```

Bytes are read as UTF-8 with the usual lossy replacement, which is the best
a receiver can do for a sender that meant some other repertoire.

`unescape` returns `Cow::Borrowed` when the text contains no escape
character at all — the overwhelmingly common case, and free.

## Encoding

```rust
use er7::escape::escape;

let separators = er7::Separators::default();

assert_eq!(escape("Smith & Jones", &separators), r"Smith \T\ Jones");
assert_eq!(escape("a|b^c~d&e", &separators), r"a\F\b\S\c\R\d\T\e");
assert_eq!(escape(r"a\b", &separators), r"a\E\b");
assert_eq!(escape("line\r\nnext", &separators), r"line\X0D\\X0A\next");
```

Two details worth knowing:

- **The escape character is encoded first**, so a value containing it is
  encoded once, not twice.
- **`\r` and `\n` become `\X0D\` and `\X0A\`.** A literal carriage return in
  a value would end the segment and truncate the message — the one
  corruption an ER7 writer must never commit.

The truncation character (`#`, HL7 v2.7) is **not** encoded: it is
structural only inside MSH-2, so a `#` in a value is just a `#`.

`unescape(escape(value)) == value` for every value. The converse does not
hold, and cannot: `escape(unescape(text))` differs from `text` when `text`
held a sequence that `unescape` leaves literal.

### Always encode when you write

```rust
// Right: `set` encodes for you.
subcomponent.set("Smith & Jones", &separators);

// Wrong: the `&` will split the component next time the message is parsed,
// shifting every value after it.
subcomponent.raw = "Smith & Jones".to_string();
```

## The tokenizer

`escapes` is the layer both functions above are built on. It turns text
into a stream of classified tokens, which is what you want when you need
more than "decode or don't".

```rust
use er7::{Separators, escape::{escapes, Escape}};

let separators = Separators::default();
let tokens: Vec<_> = escapes(r"Dr\S\Who\.br\", &separators).collect();

assert_eq!(tokens, vec![
    Escape::Text("Dr"),
    Escape::Component,
    Escape::Text("Who"),
    Escape::Formatting("br"),
]);
```

Bodies come back without the surrounding escape characters and without the
selector letter, so `\X0D\` is `Escape::Hex("0D")` and `\.sp 2\` is
`Escape::Formatting("sp 2")`.

Two properties make it usable as a foundation:

- **It never fails.** Text that does not form a valid sequence comes back as
  `Escape::Unknown` or `Escape::Unterminated`, not an error.
- **It is lossless.** Writing every token back with `Escape::write_er7`
  reproduces the input exactly.

Classification is structural, not semantic: `\XZZ\` is `Hex("ZZ")` even
though `ZZ` is not hexadecimal. Deciding whether a body is *decodable* is
`decode_hex`'s job, which is what keeps the tokenizer total.

### Rendering formatted text

The crate will not render `\.br\` for you — that is a presentation concern
and out of scope — but `escapes` gives you everything needed to do it:

```rust
fn to_plain_text(value: &str, separators: &er7::Separators) -> String {
    let mut out = String::new();
    for token in escapes(value, separators) {
        match token {
            Escape::Text(run) => out.push_str(run),
            Escape::Formatting("br") => out.push('\n'),
            Escape::Field => out.push(separators.field),
            Escape::Component => out.push(separators.component),
            Escape::Subcomponent => out.push(separators.subcomponent),
            Escape::Repetition => out.push(separators.repetition),
            Escape::EscapeCharacter => out.push(separators.escape),
            Escape::Hex(body) => {
                out.push_str(&er7::escape::decode_hex(body).unwrap_or_default())
            }
            // Highlighting, character sets, local sequences: drop them.
            _ => {}
        }
    }
    out
}
```

## Custom delimiters

Everything above takes a `&Separators`, so a message that declared `?` as
its escape character behaves identically:

```rust
let separators = er7::Separators {
    field: '#', component: '*', repetition: '!',
    escape: '?', subcomponent: '@', truncation: None,
};
assert_eq!(er7::escape::unescape("a?F?b", &separators), "a#b");
assert_eq!(er7::escape::escape("a#b", &separators), "a?F?b");
```

The selector letters — `X`, `Z`, `C`, `M`, and the leading `.` — are fixed
by the standard and do not vary with the delimiter set.

## One known divergence

HL7 scopes escaping to `ST`, `TX`, and `FT` fields and to the fourth
component of `ED`. This crate decodes sequences **wherever they appear**,
because applying the standard's scope requires knowing each field's data
type, and that requires a dictionary this crate deliberately does not have.

The risk is a false positive on a value that legitimately contains a
backslash in a field where escaping does not apply. The mitigations:
unrecognized sequences stay literal, so such a value usually round-trips
unchanged anyway; and `Subcomponent::raw` always holds exactly what
arrived, so a caller who knows the data type can override.

Recorded as
[spec §18.2](../../spec/18-open-questions-and-divergences/index.md).
