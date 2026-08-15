//! ER7 escape sequences: the way a value carries characters that would
//! otherwise be read as structure.
//!
//! A sequence is the message's escape character, a body, and the escape
//! character again — `\F\`, `\X0D\`, `\.br\`. Five of them stand for the
//! delimiters themselves; the rest are hexadecimal data, character-set
//! switches, display formatting, or locally agreed extensions.
//!
//! [`escapes`] tokenizes a string into [`Escape`] values, which is the whole
//! vocabulary; [`unescape`] and [`escape`] are the two convenience passes
//! built on top of it.
//!
//! Specified by spec §6. A worked tutorial, including how to render
//! formatted text from the token stream, is in `docs/escapes/index.md`.

use crate::Separators;
use std::borrow::Cow;

/// One token of escape-decoded text: either a literal run, or one escape
/// sequence classified by what HL7 says it means.
///
/// Bodies are returned without the surrounding escape characters and
/// without the letter that selects the sequence, so `\X0D\` yields
/// `Escape::Hex("0D")`.
///
/// Classification is structural, not semantic: `\XZZ\` is `Hex("ZZ")` even
/// though `ZZ` is not hexadecimal. Deciding whether a body is *decodable*
/// is [`decode_hex`]'s job, which is what keeps the tokenizer total
/// (spec §6.1).
///
/// Example:
///
/// ```
/// use er7::{Separators, escape::{escapes, Escape}};
///
/// let separators = Separators::default();
/// let tokens: Vec<_> = escapes(r"a\F\b\X0D\c\.br\", &separators).collect();
/// assert_eq!(tokens, vec![
///     Escape::Text("a"),
///     Escape::Field,
///     Escape::Text("b"),
///     Escape::Hex("0D"),
///     Escape::Text("c"),
///     Escape::Formatting("br"),
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Escape<'a> {
    /// A run of text containing no escape character.
    Text(&'a str),
    /// `\F\` — the field separator as data.
    Field,
    /// `\S\` — the component separator as data.
    Component,
    /// `\T\` — the subcomponent separator as data.
    Subcomponent,
    /// `\R\` — the repetition separator as data.
    Repetition,
    /// `\E\` — the escape character itself as data.
    EscapeCharacter,
    /// `\Xdd..\` — hexadecimal data; the body is the digits. Decode with
    /// [`decode_hex`], which rejects a body that is not whole hex pairs.
    Hex(&'a str),
    /// `\H\` — start highlighting the text that follows.
    Highlight,
    /// `\N\` — return to normal text, ending highlighting.
    Normal,
    /// `\.br\`, `\.sp 2\`, … — a formatted-text display command; the body
    /// excludes the leading `.`, so `\.sp 2\` yields `Formatting("sp 2")`.
    Formatting(&'a str),
    /// `\Zdd..\` — a locally defined sequence; the body excludes the `Z`.
    Local(&'a str),
    /// `\Cxxyy\` — a single-byte character-set switch; the body excludes the
    /// `C`.
    SingleByteCharacterSet(&'a str),
    /// `\Mxxyyzz\` — a multi-byte character-set switch; the body excludes
    /// the `M`.
    MultiByteCharacterSet(&'a str),
    /// A well-formed sequence whose body matches nothing above. The body is
    /// returned whole.
    Unknown(&'a str),
    /// An escape character with no closing escape character before the end
    /// of the string. The text is the remainder, escape character included.
    Unterminated(&'a str),
}

impl Escape<'_> {
    /// Write this token back exactly as it appeared in the source string,
    /// escape characters and selector letter included.
    ///
    /// This is what [`unescape`] uses to leave sequences it does not decode
    /// untouched, and it makes `escapes(s, seps)` a lossless tokenizer:
    /// concatenating every token's `write_er7` reproduces `s` (R12).
    ///
    /// Example:
    ///
    /// ```
    /// use er7::{Separators, escape::escapes};
    ///
    /// let separators = Separators::default();
    /// let source = r"a\F\b\.sp 2\c\Q\d";
    /// let mut rebuilt = String::new();
    /// for token in escapes(source, &separators) {
    ///     token.write_er7(&mut rebuilt, &separators);
    /// }
    /// assert_eq!(rebuilt, source);
    /// ```
    pub fn write_er7(&self, out: &mut String, separators: &Separators) {
        let escape = separators.escape;
        let mut sequence = |selector: &str, body: &str| {
            out.push(escape);
            out.push_str(selector);
            out.push_str(body);
            out.push(escape);
        };
        match *self {
            Escape::Text(text) | Escape::Unterminated(text) => out.push_str(text),
            Escape::Field => sequence("F", ""),
            Escape::Component => sequence("S", ""),
            Escape::Subcomponent => sequence("T", ""),
            Escape::Repetition => sequence("R", ""),
            Escape::EscapeCharacter => sequence("E", ""),
            Escape::Highlight => sequence("H", ""),
            Escape::Normal => sequence("N", ""),
            Escape::Hex(body) => sequence("X", body),
            Escape::Local(body) => sequence("Z", body),
            Escape::SingleByteCharacterSet(body) => sequence("C", body),
            Escape::MultiByteCharacterSet(body) => sequence("M", body),
            Escape::Formatting(body) => sequence(".", body),
            Escape::Unknown(body) => sequence("", body),
        }
    }
}

/// Tokenize `text` into literal runs and escape sequences.
///
/// The iterator never fails: text that does not form a valid sequence comes
/// back as [`Escape::Unknown`] or [`Escape::Unterminated`] rather than an
/// error, because a receiver's job is to make sense of what arrived, not to
/// reject it (R12, spec §6.1).
///
/// Reach for this when you need more than "decode or don't" — for instance
/// to render `\.br\` yourself, which [`unescape`] deliberately will not do.
///
/// Example:
///
/// ```
/// use er7::{Separators, escape::{escapes, Escape}};
///
/// let separators = Separators::default();
/// let tokens: Vec<_> = escapes(r"Dr\S\Who\.br\", &separators).collect();
/// assert_eq!(tokens, vec![
///     Escape::Text("Dr"),
///     Escape::Component,
///     Escape::Text("Who"),
///     Escape::Formatting("br"),
/// ]);
/// ```
///
/// See also [`unescape`] and [`escape`], the two passes built on this.
pub fn escapes<'a>(text: &'a str, separators: &Separators) -> Escapes<'a> {
    Escapes {
        rest: text,
        escape: separators.escape,
    }
}

/// Iterator over the [`Escape`] tokens of a string; see [`escapes`].
#[derive(Debug, Clone)]
pub struct Escapes<'a> {
    rest: &'a str,
    escape: char,
}

impl<'a> Iterator for Escapes<'a> {
    type Item = Escape<'a>;

    fn next(&mut self) -> Option<Escape<'a>> {
        if self.rest.is_empty() {
            return None;
        }
        let width = self.escape.len_utf8();
        match self.rest.find(self.escape) {
            // No sequence left: the rest is one literal run.
            None => Some(Escape::Text(self.take_all())),
            // A sequence starts here.
            Some(0) => {
                let after = &self.rest[width..];
                match after.find(self.escape) {
                    None => Some(Escape::Unterminated(self.take_all())),
                    Some(end) => {
                        let body = &after[..end];
                        self.rest = &after[end + width..];
                        Some(classify(body))
                    }
                }
            }
            // Literal text up to the next sequence.
            Some(start) => {
                let text = &self.rest[..start];
                self.rest = &self.rest[start..];
                Some(Escape::Text(text))
            }
        }
    }
}

impl<'a> Escapes<'a> {
    fn take_all(&mut self) -> &'a str {
        std::mem::take(&mut self.rest)
    }
}

/// Classify one sequence body — the text between the two escape characters.
fn classify(body: &str) -> Escape<'_> {
    match body {
        "F" => return Escape::Field,
        "S" => return Escape::Component,
        "T" => return Escape::Subcomponent,
        "R" => return Escape::Repetition,
        "E" => return Escape::EscapeCharacter,
        "H" => return Escape::Highlight,
        "N" => return Escape::Normal,
        _ => {}
    }
    let mut chars = body.chars();
    match (chars.next(), chars.as_str()) {
        (Some('X'), rest) => Escape::Hex(rest),
        (Some('Z'), rest) => Escape::Local(rest),
        (Some('C'), rest) => Escape::SingleByteCharacterSet(rest),
        (Some('M'), rest) => Escape::MultiByteCharacterSet(rest),
        (Some('.'), rest) => Escape::Formatting(rest),
        _ => Escape::Unknown(body),
    }
}

/// Decode the body of a `\Xdd..\` sequence into text.
///
/// The body must be whole pairs of hexadecimal digits; each pair is one
/// byte, and the bytes are read as UTF-8 with the usual lossy replacement,
/// which is the best a receiver can do for a sender that meant some other
/// character set. Returns `None` for a body that is empty, has an odd
/// length, or holds a non-hexadecimal character. [`unescape`] uses that
/// `None` to keep an undecodable `\X..\` literal rather than guessing.
///
/// Example:
///
/// ```
/// use er7::escape::decode_hex;
///
/// assert_eq!(decode_hex("0D"), Some("\r".to_string()));
/// assert_eq!(decode_hex("4142"), Some("AB".to_string()));
/// assert_eq!(decode_hex(""), None);     // empty
/// assert_eq!(decode_hex("A"), None);    // odd length
/// assert_eq!(decode_hex("XYZ"), None);  // not hexadecimal
/// ```
pub fn decode_hex(body: &str) -> Option<String> {
    if body.is_empty() || !body.len().is_multiple_of(2) || !body.is_ascii() {
        return None;
    }
    let mut bytes = Vec::with_capacity(body.len() / 2);
    let mut digits = body.chars();
    while let (Some(high), Some(low)) = (digits.next(), digits.next()) {
        bytes.push((high.to_digit(16)? * 16 + low.to_digit(16)?) as u8);
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// Decode the escape sequences that stand for characters, leaving every
/// other sequence exactly as written.
///
/// `\F\ \S\ \T\ \R\ \E\` become their delimiters and `\Xdd..\` becomes its
/// bytes. Display formatting (`\H\`, `\.br\`), character-set switches
/// (`\Cxxyy\`), local extensions (`\Zdd..\`), unrecognized bodies, and an
/// unterminated escape character are all kept literally: they say something
/// about presentation or encoding that a plain string cannot carry, so
/// dropping them would lose information and guessing at them would invent
/// it (R13, spec §6.2).
///
/// Returns `Cow::Borrowed` when the text holds no escape character at all,
/// which is the overwhelmingly common case.
///
/// Example:
///
/// ```
/// use er7::{Separators, escape::unescape};
///
/// let separators = Separators::default();
///
/// // Sequences that stand for characters decode.
/// assert_eq!(unescape(r"Smith \T\ Jones", &separators), "Smith & Jones");
/// assert_eq!(unescape(r"\X4142\", &separators), "AB");
///
/// // Everything else is kept exactly as written.
/// assert_eq!(unescape(r"line\.br\next", &separators), r"line\.br\next");
/// assert_eq!(unescape(r"\H\loud\N\", &separators), r"\H\loud\N\");
/// assert_eq!(unescape(r"\XZZ\", &separators), r"\XZZ\");
/// assert_eq!(unescape(r"a\Fb", &separators), r"a\Fb");
/// ```
///
/// See also [`escape`] for the inverse, and [`escapes`] for the token
/// stream both are built on.
pub fn unescape<'a>(text: &'a str, separators: &Separators) -> Cow<'a, str> {
    if !text.contains(separators.escape) {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    for token in escapes(text, separators) {
        match token {
            Escape::Text(run) | Escape::Unterminated(run) => out.push_str(run),
            Escape::Field => out.push(separators.field),
            Escape::Component => out.push(separators.component),
            Escape::Subcomponent => out.push(separators.subcomponent),
            Escape::Repetition => out.push(separators.repetition),
            Escape::EscapeCharacter => out.push(separators.escape),
            Escape::Hex(body) => match decode_hex(body) {
                Some(decoded) => out.push_str(&decoded),
                None => token.write_er7(&mut out, separators),
            },
            _ => token.write_er7(&mut out, separators),
        }
    }
    Cow::Owned(out)
}

/// Encode text so that it can be placed in a subcomponent without changing
/// the structure of the message.
///
/// Every delimiter becomes its sequence, and a carriage return or line feed
/// becomes `\X0D\` or `\X0A\` — those would otherwise end the segment, which
/// is the one corruption an ER7 writer must never commit. This is the
/// inverse of [`unescape`] for text that contains no sequences of its own
/// (R14, R15, spec §6.3).
///
/// Most callers want [`Subcomponent::set`](crate::Subcomponent::set)
/// instead, which does this and stores the result.
///
/// Example:
///
/// ```
/// use er7::{Separators, escape::{escape, unescape}};
///
/// let separators = Separators::default();
///
/// assert_eq!(escape("Smith & Jones", &separators), r"Smith \T\ Jones");
/// assert_eq!(escape("a|b^c~d&e", &separators), r"a\F\b\S\c\R\d\T\e");
/// // The escape character is encoded first, so it is encoded once.
/// assert_eq!(escape(r"a\b", &separators), r"a\E\b");
/// // A literal carriage return would end the segment and truncate the message.
/// assert_eq!(escape("line\rnext", &separators), r"line\X0D\next");
///
/// // Encoding then decoding is the identity, for every value.
/// for value in ["plain", r"a|b^c~d&e\f", "with\rcr"] {
///     assert_eq!(unescape(&escape(value, &separators), &separators), value);
/// }
/// ```
pub fn escape<'a>(text: &'a str, separators: &Separators) -> Cow<'a, str> {
    // The truncation character is deliberately absent here: it is only
    // structural inside MSH-2, so it needs no sequence when it appears in a
    // value.
    let needs_escaping = |c: char| {
        c == separators.field
            || c == separators.component
            || c == separators.repetition
            || c == separators.escape
            || c == separators.subcomponent
            || c == '\r'
            || c == '\n'
    };
    if !text.contains(needs_escaping) {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len() + 8);
    for c in text.chars() {
        // The escape character is written first in this match so that a
        // value containing it is encoded once, not re-encoded.
        let token = if c == separators.escape {
            Escape::EscapeCharacter
        } else if c == separators.field {
            Escape::Field
        } else if c == separators.component {
            Escape::Component
        } else if c == separators.repetition {
            Escape::Repetition
        } else if c == separators.subcomponent {
            Escape::Subcomponent
        } else if c == '\r' {
            Escape::Hex("0D")
        } else if c == '\n' {
            Escape::Hex("0A")
        } else {
            out.push(c);
            continue;
        };
        token.write_er7(&mut out, separators);
    }
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seps() -> Separators {
        Separators::default()
    }

    #[test]
    fn tokenizes_losslessly() {
        let source = r"a\F\b\.sp 2\c\Q\d\E";
        let mut rebuilt = String::new();
        for token in escapes(source, &seps()) {
            token.write_er7(&mut rebuilt, &seps());
        }
        assert_eq!(rebuilt, source);
    }

    #[test]
    fn classifies_every_sequence() {
        let tokens: Vec<_> = escapes(
            r"\F\\S\\T\\R\\E\\H\\N\\X0D\\Z99\\C2842\\M0F2842\\.br\\??\",
            &seps(),
        )
        .collect();
        assert_eq!(
            tokens,
            vec![
                Escape::Field,
                Escape::Component,
                Escape::Subcomponent,
                Escape::Repetition,
                Escape::EscapeCharacter,
                Escape::Highlight,
                Escape::Normal,
                Escape::Hex("0D"),
                Escape::Local("99"),
                Escape::SingleByteCharacterSet("2842"),
                Escape::MultiByteCharacterSet("0F2842"),
                Escape::Formatting("br"),
                Escape::Unknown("??"),
            ]
        );
    }

    #[test]
    fn unescapes_delimiters_and_hex() {
        let seps = seps();
        assert_eq!(unescape(r"a\F\b", &seps), "a|b");
        assert_eq!(unescape(r"a\S\b", &seps), "a^b");
        assert_eq!(unescape(r"a\T\b", &seps), "a&b");
        assert_eq!(unescape(r"a\R\b", &seps), "a~b");
        assert_eq!(unescape(r"a\E\b", &seps), r"a\b");
        assert_eq!(unescape(r"\X0D\", &seps), "\r");
        assert_eq!(unescape(r"\X4142\", &seps), "AB");
    }

    #[test]
    fn keeps_undecodable_sequences_literal() {
        let seps = seps();
        assert_eq!(unescape(r"line\.br\next", &seps), r"line\.br\next");
        assert_eq!(unescape(r"\H\loud\N\", &seps), r"\H\loud\N\");
        assert_eq!(unescape(r"\Z0102\", &seps), r"\Z0102\");
        // A \X\ body that is not whole hexadecimal pairs is not hex data.
        assert_eq!(unescape(r"\XZZ\", &seps), r"\XZZ\");
        assert_eq!(unescape(r"\X123\", &seps), r"\X123\");
        // An unterminated escape character is data, not a broken sequence.
        assert_eq!(unescape(r"a\Fb", &seps), r"a\Fb");
    }

    #[test]
    fn borrows_when_there_is_nothing_to_do() {
        assert!(matches!(unescape("plain text", &seps()), Cow::Borrowed(_)));
        assert!(matches!(escape("plain text", &seps()), Cow::Borrowed(_)));
    }

    #[test]
    fn escapes_delimiters_and_segment_terminators() {
        let seps = seps();
        assert_eq!(escape(r"a\b", &seps), r"a\E\b");
        assert_eq!(escape("a|b^c~d&e", &seps), r"a\F\b\S\c\R\d\T\e");
        assert_eq!(escape("line\r\nnext", &seps), r"line\X0D\\X0A\next");
    }

    #[test]
    fn escape_and_unescape_round_trip() {
        let seps = seps();
        for value in ["plain", r"a|b^c~d&e\f", "with\rcr", "Smith & Jones"] {
            assert_eq!(unescape(&escape(value, &seps), &seps), value);
        }
    }

    #[test]
    fn honors_custom_delimiters() {
        let seps = Separators {
            field: '#',
            component: '*',
            repetition: '!',
            escape: '?',
            subcomponent: '@',
            truncation: None,
        };
        assert_eq!(unescape("a?F?b", &seps), "a#b");
        assert_eq!(escape("a#b", &seps), "a?F?b");
    }

    #[test]
    fn decodes_hex_bodies() {
        assert_eq!(decode_hex("0D0A"), Some("\r\n".to_string()));
        assert_eq!(decode_hex(""), None);
        assert_eq!(decode_hex("A"), None);
        assert_eq!(decode_hex("GG"), None);
    }
}
