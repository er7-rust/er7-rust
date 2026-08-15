//! Decode, encode, and classify ER7 escape sequences.
//!
//! Run with: `cargo run --example escape_sequences`
//!
//! Shows which sequences decode and which are deliberately kept literal,
//! why writing a value must go through `escape`, and how to use the token
//! stream to render formatted text yourself.
//!
//! See `docs/escapes/index.md` and spec §6.

use er7::Separators;
use er7::escape::{Escape, decode_hex, escape, escapes, unescape};

fn main() -> Result<(), er7::Error> {
    let separators = Separators::default();

    // --- Sequences that stand for characters decode -----------------------
    assert_eq!(unescape(r"Smith \T\ Jones", &separators), "Smith & Jones");
    assert_eq!(unescape(r"a\F\b", &separators), "a|b");
    assert_eq!(unescape(r"a\S\b", &separators), "a^b");
    assert_eq!(unescape(r"a\R\b", &separators), "a~b");
    assert_eq!(unescape(r"a\E\b", &separators), r"a\b");
    assert_eq!(unescape(r"\X0D\", &separators), "\r");
    assert_eq!(unescape(r"\X4142\", &separators), "AB");
    println!("decoded: {}", unescape(r"Smith \T\ Jones", &separators));

    // --- Everything else is kept exactly as written -----------------------
    // Display commands, highlighting, character-set switches, and local
    // extensions say something a plain String cannot carry. Dropping them
    // would lose information; guessing would invent it.
    assert_eq!(unescape(r"line\.br\next", &separators), r"line\.br\next");
    assert_eq!(unescape(r"\H\loud\N\", &separators), r"\H\loud\N\");
    assert_eq!(unescape(r"\Z0102\", &separators), r"\Z0102\");
    assert_eq!(unescape(r"\C2842\", &separators), r"\C2842\");
    // A `\X..\` body that is not whole hex pairs was not hex data.
    assert_eq!(unescape(r"\XZZ\", &separators), r"\XZZ\");
    // An escape character with no partner is data, not a broken sequence.
    assert_eq!(unescape(r"a\Fb", &separators), r"a\Fb");
    println!("preserved: {}", unescape(r"line\.br\next", &separators));

    // --- Encoding ---------------------------------------------------------
    assert_eq!(escape("Smith & Jones", &separators), r"Smith \T\ Jones");
    assert_eq!(escape("a|b^c~d&e", &separators), r"a\F\b\S\c\R\d\T\e");
    // The escape character is encoded first, so it is encoded once.
    assert_eq!(escape(r"a\b", &separators), r"a\E\b");
    // A literal carriage return would end the segment and truncate the
    // message, so it becomes hex. This is the one corruption an ER7 writer
    // must never commit.
    assert_eq!(escape("line\r\nnext", &separators), r"line\X0D\\X0A\next");

    // Encoding then decoding is the identity, for every value.
    for value in ["plain", r"a|b^c~d&e\f", "with\rcr", "Smith & Jones"] {
        assert_eq!(unescape(&escape(value, &separators), &separators), value);
    }
    println!("escape/unescape round trip holds");

    // --- Hex bodies -------------------------------------------------------
    assert_eq!(decode_hex("0D0A"), Some("\r\n".to_string()));
    assert_eq!(decode_hex("4142"), Some("AB".to_string()));
    assert_eq!(decode_hex("A"), None); // odd length
    assert_eq!(decode_hex("GG"), None); // not hexadecimal

    // --- The token stream -------------------------------------------------
    // `escapes` is the layer the two functions above are built on. Bodies
    // come back without the escape characters and without the selector
    // letter.
    let tokens: Vec<_> = escapes(r"Dr\S\Who\.br\", &separators).collect();
    assert_eq!(
        tokens,
        vec![
            Escape::Text("Dr"),
            Escape::Component,
            Escape::Text("Who"),
            Escape::Formatting("br"),
        ]
    );

    // It is lossless: writing every token back reproduces the input.
    let source = r"a\F\b\.sp 2\c\Q\d";
    let mut rebuilt = String::new();
    for token in escapes(source, &separators) {
        token.write_er7(&mut rebuilt, &separators);
    }
    assert_eq!(rebuilt, source);
    println!("tokenizing is lossless");

    // --- Rendering formatted text yourself --------------------------------
    // The crate will not render `\.br\` for you — that is presentation, and
    // out of scope — but the token stream gives you everything you need.
    let note = r"Line one\.br\Line two \S\ more";
    assert_eq!(
        to_plain_text(note, &separators),
        "Line one\nLine two ^ more"
    );
    println!("--- rendered note ---");
    println!("{}", to_plain_text(note, &separators));

    // --- Custom delimiters ------------------------------------------------
    // A message that declared `?` as its escape character behaves the same.
    let custom = Separators {
        field: '#',
        component: '*',
        repetition: '!',
        escape: '?',
        subcomponent: '@',
        truncation: None,
    };
    assert_eq!(unescape("a?F?b", &custom), "a#b");
    assert_eq!(escape("a#b", &custom), "a?F?b");

    println!("ok");
    Ok(())
}

/// Flatten a formatted-text value into plain text, honouring `\.br\` and
/// dropping the sequences that have no plain-text equivalent.
fn to_plain_text(value: &str, separators: &Separators) -> String {
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
            Escape::Hex(body) => out.push_str(&decode_hex(body).unwrap_or_default()),
            // Highlighting, character sets, local and unknown sequences:
            // nothing to render, so drop them.
            _ => {}
        }
    }
    out
}
