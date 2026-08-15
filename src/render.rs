//! Writing the value tree back out, either as ER7 or as readable text.
//!
//! Every level offers the same pair: `to_er7` reproduces the message
//! exactly as a receiver would need it, escape sequences intact, while
//! `to_text` decodes the leaves so the result reads like data rather than
//! like wire format. The structural delimiters stay in both, because
//! `SMITH^JOHN` without its caret is no longer a name and a surname (R17).
//!
//! A consequence worth knowing: `to_text` output is **not** re-parseable. A
//! decoded `\F\` becomes a literal `|`, which a parser would read as a
//! field separator. Use `to_er7` for anything that will be sent, stored, or
//! parsed again; use `to_text` for display, logging, and database writes.
//!
//! Specified by spec §7.

use crate::message::is_header_name;
use crate::{Component, Field, Message, Repetition, Segment, Separators, Subcomponent};
use std::fmt;

/// Whether the leaves are written as sent or escape-decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Exactly as sent — the ER7 a receiver parses.
    Er7,
    /// Escape sequences resolved — the values a human or a database wants.
    Text,
}

/// How [`Message::to_er7_with`] writes a whole message.
///
/// The [`Default`] is a carriage return with no trailing terminator, which
/// is what [`Message::to_er7`] uses.
///
/// Example:
///
/// ```
/// use er7::{RenderOptions, Terminator};
///
/// let options = RenderOptions { terminator: Terminator::CrLf, trailing_terminator: true };
/// assert_eq!(options.terminator.as_str(), "\r\n");
/// assert_eq!(RenderOptions::default().terminator, Terminator::Cr);
/// assert!(!RenderOptions::default().trailing_terminator);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RenderOptions {
    /// What ends each segment. HL7 allows only a carriage return on the
    /// wire, which is the default; the other choices exist for messages
    /// kept in text files.
    pub terminator: crate::Terminator,
    /// Whether to end the last segment too.
    ///
    /// HL7 terminates every segment, the last one included, so set this for
    /// strict wire output. It defaults to `false` because a trailing
    /// terminator surprises callers that compare or concatenate the result,
    /// and because the transport — MLLP, or a file — already marks where
    /// the message ends.
    pub trailing_terminator: bool,
}

impl Message {
    /// Write the message as ER7 with the default [`RenderOptions`]:
    /// carriage-return terminators and no trailing terminator.
    ///
    /// Parsing and writing round-trip: for any message this crate parsed,
    /// the output differs from the input only where the input was not
    /// already canonical — that is, where it had blank lines or a different
    /// terminator, the two things parsing normalizes (R16, spec §7.2).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// // Canonical input comes back byte for byte.
    /// let text = "MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN";
    /// assert_eq!(er7::parse(text)?.to_er7(), text);
    ///
    /// // Including unusual delimiters, empty positions, and escapes the
    /// // crate does not decode.
    /// for text in [
    ///     "MSH#*!?@#LAB#*A*B#C!D",
    ///     "MSH|^~\\&|LAB\rPID||A~~B|^^C|||D",
    ///     "MSH|^~\\&|LAB\rNTE|1||line\\.br\\next\rZPD|1|LOCAL",
    /// ] {
    ///     assert_eq!(er7::parse(text)?.to_er7(), text);
    /// }
    ///
    /// // Non-canonical input is normalized, and then round-trips.
    /// let messy = "MSH|^~\\&|LAB\r\n\r\nPID|1\n";
    /// assert_eq!(er7::parse(messy)?.to_er7(), "MSH|^~\\&|LAB\rPID|1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_er7(&self) -> String {
        self.to_er7_with(RenderOptions::default())
    }

    /// Write the message as ER7, choosing the segment terminator and
    /// whether the last segment gets one.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// use er7::{RenderOptions, Terminator};
    ///
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1")?;
    ///
    /// // Readable in a terminal.
    /// let readable = RenderOptions { terminator: Terminator::Lf, ..Default::default() };
    /// assert_eq!(message.to_er7_with(readable), "MSH|^~\\&|LAB\nPID|1");
    ///
    /// // Strict wire output terminates every segment, the last included.
    /// let wire = RenderOptions { terminator: Terminator::Cr, trailing_terminator: true };
    /// assert_eq!(message.to_er7_with(wire), "MSH|^~\\&|LAB\rPID|1\r");
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_er7_with(&self, options: RenderOptions) -> String {
        let mut out = String::new();
        for (index, segment) in self.segments.iter().enumerate() {
            if index > 0 {
                out.push_str(options.terminator.as_str());
            }
            write_segment(&mut out, segment, &self.separators, Mode::Er7);
        }
        if options.trailing_terminator && !self.segments.is_empty() {
            out.push_str(options.terminator.as_str());
        }
        out
    }
}

impl fmt::Display for Message {
    /// The message as ER7 with default options; see [`Message::to_er7`].
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1")?;
    /// assert_eq!(message.to_string(), "MSH|^~\\&|LAB\rPID|1");
    /// # Ok(())
    /// # }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_er7())
    }
}

/// Generate the `to_er7` / `to_text` pair for one level of the tree.
macro_rules! writers {
    ($type:ty, $write:ident, $what:literal) => {
        impl $type {
            #[doc = concat!("Write this ", $what, " as ER7, exactly as a receiver would read it.")]
            ///
            /// Escape sequences are left intact, so the result can be sent,
            /// stored, or parsed again. This is the form the round-trip
            /// guarantee applies to (R16).
            ///
            /// Example:
            ///
            /// ```
            /// # fn main() -> Result<(), er7::Error> {
            /// let message = er7::parse(r"MSH|^~\&|LAB|Smith \T\ Jones^X")?;
            /// let separators = &message.separators;
            /// let field = message.segment("MSH").unwrap().field(4).unwrap();
            ///
            /// assert_eq!(field.to_er7(separators), r"Smith \T\ Jones^X");
            /// # Ok(())
            /// # }
            /// ```
            pub fn to_er7(&self, separators: &Separators) -> String {
                let mut out = String::new();
                $write(&mut out, self, separators, Mode::Er7);
                out
            }

            #[doc = concat!("Write this ", $what, " with its leaf text escape-decoded.")]
            ///
            /// Structural delimiters remain, so the result shows the shape
            /// of the value as well as its content (R17). That also means
            /// the result is **not** re-parseable: a decoded `\F\` becomes a
            /// literal field separator. Use this for display, logging, and
            /// database writes; use `to_er7` for anything that goes back
            /// into a message.
            ///
            /// Example:
            ///
            /// ```
            /// # fn main() -> Result<(), er7::Error> {
            /// let message = er7::parse(r"MSH|^~\&|LAB|Smith \T\ Jones^X")?;
            /// let separators = &message.separators;
            /// let field = message.segment("MSH").unwrap().field(4).unwrap();
            ///
            /// // The escape decodes; the component separator stays.
            /// assert_eq!(field.to_text(separators), "Smith & Jones^X");
            /// # Ok(())
            /// # }
            /// ```
            pub fn to_text(&self, separators: &Separators) -> String {
                let mut out = String::new();
                $write(&mut out, self, separators, Mode::Text);
                out
            }
        }
    };
}

writers!(Segment, write_segment, "segment");
writers!(Field, write_field, "field");
writers!(Repetition, write_repetition, "repetition");
writers!(Component, write_component, "component");
writers!(Subcomponent, write_subcomponent, "subcomponent");

fn write_segment(out: &mut String, segment: &Segment, separators: &Separators, mode: Mode) {
    out.push_str(&segment.name);
    if is_header_name(&segment.name) {
        // Fields 1 and 2 of a header are the field separator and the
        // encoding characters. They are the delimiters, not values encoded
        // with them, so they are written literally and no separator is
        // inserted before field 1 — the separator *is* field 1.
        for field in segment.fields.iter().take(2) {
            write_field(out, field, separators, Mode::Er7);
        }
        for field in segment.fields.iter().skip(2) {
            out.push(separators.field);
            write_field(out, field, separators, mode);
        }
        return;
    }
    for field in &segment.fields {
        out.push(separators.field);
        write_field(out, field, separators, mode);
    }
}

fn write_field(out: &mut String, field: &Field, separators: &Separators, mode: Mode) {
    join(
        out,
        &field.repetitions,
        separators.repetition,
        |out, item| write_repetition(out, item, separators, mode),
    );
}

fn write_repetition(
    out: &mut String,
    repetition: &Repetition,
    separators: &Separators,
    mode: Mode,
) {
    join(
        out,
        &repetition.components,
        separators.component,
        |out, item| write_component(out, item, separators, mode),
    );
}

fn write_component(out: &mut String, component: &Component, separators: &Separators, mode: Mode) {
    join(
        out,
        &component.subcomponents,
        separators.subcomponent,
        |out, item| write_subcomponent(out, item, separators, mode),
    );
}

fn write_subcomponent(
    out: &mut String,
    subcomponent: &Subcomponent,
    separators: &Separators,
    mode: Mode,
) {
    match mode {
        Mode::Er7 => out.push_str(&subcomponent.raw),
        Mode::Text => out.push_str(&subcomponent.value(separators)),
    }
}

/// Write `items` with `separator` between them.
fn join<T>(out: &mut String, items: &[T], separator: char, mut write: impl FnMut(&mut String, &T)) {
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push(separator);
        }
        write(out, item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Terminator, parse};

    #[test]
    fn round_trips_a_canonical_message() {
        let text = "MSH|^~\\&|LAB|ACME|EHR||20260815||ORU^R01|9|P|2.5\r\
                    PID|1||12345^^^ACME&1.2.3&ISO^MR||SMITH^JOHN\r\
                    OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";
        assert_eq!(parse(text).unwrap().to_er7(), text);
    }

    #[test]
    fn round_trips_custom_delimiters() {
        let text = "MSH#*!?@#LAB#*A*B#C!D";
        assert_eq!(parse(text).unwrap().to_er7(), text);
    }

    #[test]
    fn normalizes_terminators_and_blank_lines() {
        let text = "MSH|^~\\&|LAB\r\n\r\nPID|1\n";
        assert_eq!(parse(text).unwrap().to_er7(), "MSH|^~\\&|LAB\rPID|1");
    }

    #[test]
    fn writes_the_chosen_terminator() {
        let message = parse("MSH|^~\\&|LAB\rPID|1").unwrap();
        let options = RenderOptions {
            terminator: Terminator::CrLf,
            trailing_terminator: true,
        };
        assert_eq!(message.to_er7_with(options), "MSH|^~\\&|LAB\r\nPID|1\r\n");
    }

    #[test]
    fn decodes_only_the_leaves() {
        let message = parse(r"MSH|^~\&|LAB|Smith \T\ Jones").unwrap();
        let separators = &message.separators;
        let segment = message.segment("MSH").unwrap();
        assert_eq!(
            segment.field(4).unwrap().to_er7(separators),
            r"Smith \T\ Jones"
        );
        assert_eq!(
            segment.field(4).unwrap().to_text(separators),
            "Smith & Jones"
        );
        // The header's own delimiter fields stay literal in either mode.
        assert!(segment.to_text(separators).starts_with(r"MSH|^~\&|"));
    }

    #[test]
    fn keeps_empty_positions() {
        // Empty repetitions and components hold their places, because the
        // position of a value is what gives it meaning.
        let text = "MSH|^~\\&|LAB\rPID|A~~B|^^C|||D";
        assert_eq!(parse(text).unwrap().to_er7(), text);
    }
}
