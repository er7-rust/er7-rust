//! The ER7 delimiter set, read from a message's own header segment.
//!
//! ER7 hardcodes nothing: a message declares its delimiters in MSH-1 (the
//! field separator, which is literally the fourth character of the message)
//! and MSH-2 (the remaining encoding characters). Everything in this crate
//! therefore takes a [`Separators`] rather than assuming `|^~\&`.

use crate::Error;
use std::fmt;

/// The characters that end a segment when a message is written back out.
///
/// Parsing always accepts any of these; this only chooses what
/// [`Message::to_er7_with`](crate::Message::to_er7_with) emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Terminator {
    /// A single carriage return, `\r` — the only terminator HL7 permits, and
    /// the default here.
    #[default]
    Cr,
    /// A single line feed, `\n` — common when messages are stored in text
    /// files rather than sent over the wire.
    Lf,
    /// A carriage return followed by a line feed, `\r\n`.
    CrLf,
}

impl Terminator {
    /// The characters this terminator writes.
    pub fn as_str(self) -> &'static str {
        match self {
            Terminator::Cr => "\r",
            Terminator::Lf => "\n",
            Terminator::CrLf => "\r\n",
        }
    }
}

/// The ER7 delimiter set: one field separator plus the encoding characters.
///
/// The HL7-recommended values are the [`Default`] (`|`, `^`, `~`, `\`, `&`,
/// and no truncation character). A real message may choose others, so parse
/// them from the message with [`Separators::from_header`] instead of
/// assuming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Separators {
    /// Separates fields within a segment, and the segment name from the
    /// first field. Declared by MSH-1, i.e. the character right after `MSH`.
    /// Recommended `|`.
    pub field: char,
    /// Separates components within a field repetition. MSH-2 position 1.
    /// Recommended `^`.
    pub component: char,
    /// Separates repetitions within a field. MSH-2 position 2.
    /// Recommended `~`.
    pub repetition: char,
    /// Opens and closes an escape sequence (see [`crate::escape`]). MSH-2
    /// position 3. Recommended `\`.
    pub escape: char,
    /// Separates subcomponents within a component. MSH-2 position 4.
    /// Recommended `&`.
    pub subcomponent: char,
    /// Marks a value the sender truncated to fit a length limit. MSH-2
    /// position 5, added in HL7 v2.7 and absent from most messages, hence
    /// [`Option`]. Recommended `#`.
    pub truncation: Option<char>,
}

impl Default for Separators {
    /// The delimiters HL7 recommends, and that essentially every real
    /// message uses: `|^~\&`, with no truncation character.
    fn default() -> Self {
        Separators {
            field: '|',
            component: '^',
            repetition: '~',
            escape: '\\',
            subcomponent: '&',
            truncation: None,
        }
    }
}

impl Separators {
    /// Read the delimiter set from a header segment line — `MSH`, `FHS`, or
    /// `BHS`, all of which declare delimiters the same way.
    ///
    /// The character after the three-character segment name is the field
    /// separator; the characters from there to the next field separator are
    /// the encoding characters, taken positionally. Encoding characters the
    /// message omits fall back to their [`Default`].
    ///
    /// ```
    /// # use er7::Separators;
    /// let seps = Separators::from_header(r"MSH|^~\&|LAB").unwrap();
    /// assert_eq!(seps, Separators::default());
    /// ```
    pub fn from_header(line: &str) -> Result<Separators, Error> {
        let mut chars = line.chars().skip(3);
        let field = chars
            .next()
            .ok_or_else(|| Error::BadHeader("no field separator after the segment name".into()))?;
        let encoding: Vec<char> = chars.take_while(|&c| c != field).take(5).collect();
        let default = Separators::default();
        let separators = Separators {
            field,
            component: encoding.first().copied().unwrap_or(default.component),
            repetition: encoding.get(1).copied().unwrap_or(default.repetition),
            escape: encoding.get(2).copied().unwrap_or(default.escape),
            subcomponent: encoding.get(3).copied().unwrap_or(default.subcomponent),
            truncation: encoding.get(4).copied(),
        };
        separators.validate()?;
        Ok(separators)
    }

    /// Check that this set can encode a message unambiguously: no delimiter
    /// may be alphanumeric, a carriage return, or a line feed, and no two
    /// delimiters may be the same character.
    ///
    /// [`Separators::from_header`] applies this to every message it reads,
    /// because a message that reuses one character for two roles cannot be
    /// parsed back into the values the sender meant.
    pub fn validate(&self) -> Result<(), Error> {
        for (role, c) in self.roles() {
            if c.is_alphanumeric() {
                return Err(Error::BadHeader(format!(
                    "{role} separator {c:?} is alphanumeric"
                )));
            }
            if c == '\r' || c == '\n' {
                return Err(Error::BadHeader(format!(
                    "{role} separator {c:?} would end a segment"
                )));
            }
        }
        let roles = self.roles();
        for (i, (role, c)) in roles.iter().enumerate() {
            if let Some((other, _)) = roles[..i].iter().find(|(_, d)| d == c) {
                return Err(Error::BadHeader(format!(
                    "{other} and {role} separators are both {c:?}"
                )));
            }
        }
        Ok(())
    }

    /// True when `c` plays any structural role in this delimiter set, and so
    /// must be escaped to appear in a value.
    pub fn is_delimiter(&self, c: char) -> bool {
        self.roles().iter().any(|&(_, d)| d == c)
    }

    /// The encoding-characters string this set writes as MSH-2, e.g.
    /// `^~\&`. Note this excludes the field separator, which is MSH-1.
    pub fn encoding_characters(&self) -> String {
        let mut s = String::with_capacity(5);
        s.push(self.component);
        s.push(self.repetition);
        s.push(self.escape);
        s.push(self.subcomponent);
        if let Some(truncation) = self.truncation {
            s.push(truncation);
        }
        s
    }

    /// Every delimiter with the name of the role it plays, in MSH-1/MSH-2
    /// order. Used by the checks above and by error messages.
    fn roles(&self) -> Vec<(&'static str, char)> {
        let mut roles = vec![
            ("field", self.field),
            ("component", self.component),
            ("repetition", self.repetition),
            ("escape", self.escape),
            ("subcomponent", self.subcomponent),
        ];
        if let Some(truncation) = self.truncation {
            roles.push(("truncation", truncation));
        }
        roles
    }
}

impl fmt::Display for Separators {
    /// The delimiters as they appear at the start of a message: the field
    /// separator followed by the encoding characters, e.g. `|^~\&`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.field, self.encoding_characters())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_recommended_delimiters() {
        let seps = Separators::from_header(r"MSH|^~\&|LAB|||20260815||ADT^A08").unwrap();
        assert_eq!(seps, Separators::default());
        assert_eq!(seps.encoding_characters(), r"^~\&");
        assert_eq!(seps.to_string(), r"|^~\&");
    }

    #[test]
    fn reads_custom_delimiters() {
        let seps = Separators::from_header("MSH#*!?@#LAB").unwrap();
        assert_eq!(
            seps,
            Separators {
                field: '#',
                component: '*',
                repetition: '!',
                escape: '?',
                subcomponent: '@',
                truncation: None,
            }
        );
    }

    #[test]
    fn reads_the_truncation_character() {
        let seps = Separators::from_header(r"MSH|^~\&#|LAB").unwrap();
        assert_eq!(seps.truncation, Some('#'));
        assert_eq!(seps.encoding_characters(), r"^~\&#");
    }

    #[test]
    fn fills_in_omitted_encoding_characters() {
        let seps = Separators::from_header("MSH|^~|LAB").unwrap();
        assert_eq!(seps.escape, '\\');
        assert_eq!(seps.subcomponent, '&');
    }

    #[test]
    fn rejects_unusable_delimiters() {
        // No field separator at all.
        assert!(Separators::from_header("MSH").is_err());
        // An alphanumeric field separator means this is not really a header.
        assert!(Separators::from_header("MSHX^~\\&").is_err());
        // The same character cannot mean two things.
        assert!(Separators::from_header(r"MSH|^^\&|LAB").is_err());
        assert!(Separators::from_header("MSH|^~&&|LAB").is_err());
        // A delimiter that ends a segment would make the message unreadable.
        assert!(
            Separators {
                component: '\r',
                ..Separators::default()
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn stops_reading_encoding_characters_at_the_field_separator() {
        // A sender that writes only three encoding characters gets the
        // defaults for the rest, rather than swallowing the `|` that ends
        // the field.
        let seps = Separators::from_header(r"MSH|^~\|LAB").unwrap();
        assert_eq!(seps.escape, '\\');
        assert_eq!(seps.subcomponent, '&');
        assert_eq!(seps.truncation, None);
    }

    #[test]
    fn recognizes_its_own_delimiters() {
        let seps = Separators::default();
        assert!(seps.is_delimiter('|'));
        assert!(seps.is_delimiter('&'));
        assert!(!seps.is_delimiter('#'));
        assert!(!seps.is_delimiter('A'));
    }
}
