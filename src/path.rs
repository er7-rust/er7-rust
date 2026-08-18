//! HL7 paths: the short notation interface engineers use to name one place
//! in a message, such as `PID-5.1` or `OBX[2]-5.1.2`.
//!
//! The notation is a de-facto standard rather than part of HL7 itself, so
//! this module accepts the two spellings that are common in the field —
//! `PID-5.1` and `PID.5.1` — and writes the first.
//!
//! Specified by spec §8.1. A full reference, including the two behaviours
//! that surprise people, is in `docs/paths/index.md`.

use crate::Error;
use std::fmt;
use std::str::FromStr;

/// A parsed HL7 path: a segment name, then optionally a field, and within
/// it a component and subcomponent, with optional 1-based occurrence
/// indices for the segment and for the field's repetitions.
///
/// Every index is 1-based, matching the standard's own numbering: `PID-5.1`
/// is the first component of the fifth field, not the sixth (R18).
///
/// The two occurrence indices mean different things: the one after the
/// **segment name** picks which segment of that name, and the one after the
/// **field number** picks which repetition. Leaving either out means "every
/// one" (R19).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7::Error> {
/// let path: er7::Path = "OBX[2]-5[1].1".parse()?;
/// assert_eq!(path.segment, "OBX");
/// assert_eq!(path.segment_occurrence, Some(2));  // the second OBX
/// assert_eq!(path.field, Some(5));
/// assert_eq!(path.repetition, Some(1));          // its first repetition
/// assert_eq!(path.component, Some(1));
/// assert_eq!(path.subcomponent, None);
///
/// // Leaving an occurrence out means every one.
/// let all: er7::Path = "OBX-5".parse()?;
/// assert_eq!(all.segment_occurrence, None);
/// assert_eq!(all.repetition, None);
/// # Ok(())
/// # }
/// ```
///
/// `Path` is `Hash`, so a set of paths can be map keys or deduplicated.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    /// The segment name, e.g. `PID`. Matched case-sensitively against the
    /// names in the message.
    pub segment: String,
    /// Which segment of that name, 1-based. `None` means every one of them,
    /// which is what makes `OBX-5` useful on a message with many results.
    pub segment_occurrence: Option<usize>,
    /// The 1-based field number. `None` names the whole segment.
    pub field: Option<usize>,
    /// Which repetition of the field, 1-based. `None` means every one.
    pub repetition: Option<usize>,
    /// The 1-based component number. `None` names the whole repetition.
    pub component: Option<usize>,
    /// The 1-based subcomponent number. `None` names the whole component.
    pub subcomponent: Option<usize>,
}

impl Path {
    /// Parse a path, accepting either `PID-5.1` or `PID.5.1` for the step
    /// from segment to field, because both are in common use. Surrounding
    /// whitespace is ignored.
    ///
    /// An index of `0` is rejected rather than clamped: HL7 numbering starts
    /// at 1, so a `0` is almost always a caller's off-by-one, and silently
    /// reading it as `1` would return a plausible wrong answer (R18).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// use er7::Path;
    ///
    /// // Both spellings, and surrounding whitespace, are accepted.
    /// assert_eq!(Path::parse("PID.5.1")?, Path::parse("PID-5.1")?);
    /// assert_eq!(Path::parse(" PID-5 ")?, Path::parse("PID-5")?);
    ///
    /// // Rejected: a zero index, and anything malformed.
    /// for text in ["", "-5", "PID-", "PID-5.", "PID-0", "PID[0]-5", "PID[2-5", "PID-5x"] {
    ///     assert!(Path::parse(text).is_err(), "expected {text:?} to be rejected");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See also [`FromStr`], which is the same thing via `.parse()`.
    ///
    /// # Errors
    ///
    /// [`Error::BadPath`] naming the path and the problem: no segment name,
    /// an index of `0`, a missing number, an unclosed `[`, or trailing
    /// text (R18).
    pub fn parse(text: &str) -> Result<Path, Error> {
        let text = text.trim();
        let mut rest = text;
        let bad = |detail: &str| Error::BadPath(format!("{text:?}: {detail}"));

        let end = rest
            .find(|c: char| !c.is_ascii_alphanumeric())
            .unwrap_or(rest.len());
        let segment = rest[..end].to_string();
        if segment.is_empty() {
            return Err(bad("expected a segment name"));
        }
        rest = &rest[end..];
        let segment_occurrence = take_occurrence(&mut rest, text)?;

        if rest.is_empty() {
            return Ok(Path {
                segment,
                segment_occurrence,
                field: None,
                repetition: None,
                component: None,
                subcomponent: None,
            });
        }
        if !rest.starts_with(['-', '.']) {
            return Err(bad("expected '-' or '.' after the segment name"));
        }
        rest = &rest[1..];
        let field = Some(take_index(&mut rest, text)?);
        let repetition = take_occurrence(&mut rest, text)?;
        let component = take_step(&mut rest, text)?;
        let subcomponent = if component.is_some() {
            take_step(&mut rest, text)?
        } else {
            None
        };
        if !rest.is_empty() {
            return Err(bad(&format!("unexpected trailing {rest:?}")));
        }
        Ok(Path {
            segment,
            segment_occurrence,
            field,
            repetition,
            component,
            subcomponent,
        })
    }
}

/// Consume `.` followed by an index, if the next character is `.`.
fn take_step(rest: &mut &str, text: &str) -> Result<Option<usize>, Error> {
    if !rest.starts_with('.') {
        return Ok(None);
    }
    *rest = &rest[1..];
    take_index(rest, text).map(Some)
}

/// Consume a bracketed 1-based occurrence such as `[2]`, if present.
fn take_occurrence(rest: &mut &str, text: &str) -> Result<Option<usize>, Error> {
    if !rest.starts_with('[') {
        return Ok(None);
    }
    let close = rest
        .find(']')
        .ok_or_else(|| Error::BadPath(format!("{text:?}: unclosed '['")))?;
    let mut inner = &rest[1..close];
    let index = take_index(&mut inner, text)?;
    if !inner.is_empty() {
        return Err(Error::BadPath(format!(
            "{text:?}: unexpected {inner:?} inside brackets"
        )));
    }
    *rest = &rest[close + 1..];
    Ok(Some(index))
}

/// Consume a run of digits as a 1-based index; `0` is rejected because HL7
/// numbering starts at 1 and a `0` here is nearly always an off-by-one.
fn take_index(rest: &mut &str, text: &str) -> Result<usize, Error> {
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    let digits = &rest[..end];
    let index: usize = digits
        .parse()
        .map_err(|_| Error::BadPath(format!("{text:?}: expected a number")))?;
    if index == 0 {
        return Err(Error::BadPath(format!(
            "{text:?}: indices are 1-based, so 0 is not a position"
        )));
    }
    *rest = &rest[end..];
    Ok(index)
}

impl FromStr for Path {
    type Err = Error;

    /// Parse a path with `.parse()`; see [`Path::parse`].
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let path: er7::Path = "PID-5.1".parse()?;
    /// assert_eq!(path.field, Some(5));
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(text: &str) -> Result<Path, Error> {
        Path::parse(text)
    }
}

impl fmt::Display for Path {
    /// Write the canonical spelling, e.g. `OBX[2]-5.1`. Occurrence indices
    /// the path left open are left out rather than defaulted to 1, so a
    /// round trip through `Display` and `parse` preserves meaning.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// for text in ["MSH", "PID-5", "PID-5.1", "PID-5.1.2", "OBX[2]-5[1].1.2"] {
    ///     assert_eq!(text.parse::<er7::Path>()?.to_string(), text);
    /// }
    /// // The `.` spelling is normalized to `-`.
    /// assert_eq!("PID.5.1".parse::<er7::Path>()?.to_string(), "PID-5.1");
    /// # Ok(())
    /// # }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.segment)?;
        if let Some(occurrence) = self.segment_occurrence {
            write!(f, "[{occurrence}]")?;
        }
        if let Some(field) = self.field {
            write!(f, "-{field}")?;
        }
        if let Some(repetition) = self.repetition {
            write!(f, "[{repetition}]")?;
        }
        if let Some(component) = self.component {
            write!(f, ".{component}")?;
        }
        if let Some(subcomponent) = self.subcomponent {
            write!(f, ".{subcomponent}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(text: &str) -> Path {
        Path::parse(text).unwrap()
    }

    #[test]
    fn parses_each_depth() {
        assert_eq!(path("MSH").field, None);
        assert_eq!(path("PID-5").field, Some(5));
        assert_eq!(path("PID-5.1").component, Some(1));
        assert_eq!(path("PID-5.1.2").subcomponent, Some(2));
    }

    #[test]
    fn parses_occurrences() {
        assert_eq!(path("OBX[3]-5").segment_occurrence, Some(3));
        assert_eq!(path("PID-13[2].4").repetition, Some(2));
        assert_eq!(path("PID-13").repetition, None);
    }

    #[test]
    fn accepts_both_spellings() {
        assert_eq!(path("PID.5.1"), path("PID-5.1"));
        assert_eq!(path(" PID-5 "), path("PID-5"));
    }

    #[test]
    fn round_trips_through_display() {
        for text in ["MSH", "PID-5", "PID-5.1", "PID-5.1.2", "OBX[2]-5[1].1.2"] {
            assert_eq!(path(text).to_string(), text);
        }
    }

    #[test]
    fn rejects_malformed_paths() {
        for text in [
            "", "-5", "PID-", "PID-5.", "PID-0", "PID[0]-5", "PID[2-5", "PID-5x", "PID/5",
        ] {
            assert!(
                Path::parse(text).is_err(),
                "expected {text:?} to be rejected"
            );
        }
    }
}
