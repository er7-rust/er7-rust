//! # ER7
//!
//! **[website](https://er7-rust.github.io/)**
//! •
//! **[documentation](https://docs.rs/er7/)**
//! •
//! **[source](https://github.com/er7-rust/er7-rust)**
//! •
//! **[crate](https://crates.io/crates/er7)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! ER7 — the pipe-hat encoding that carries HL7® v2 messages between
//! healthcare systems — parsed, queried, edited, and written back, with no
//! dependencies.
//!
//! ER7 is compact and everywhere, and it is also unforgiving: a value's
//! meaning comes entirely from its position, so one misplaced `|` silently
//! shifts everything after it. This crate exists to make that structure
//! explicit and to keep it intact — text is stored exactly as it arrived,
//! and decoded only when you ask for a value.
//!
//! # Trademarks
//!
//! HL7®, and FHIR® are the registered trademarks of Health Level Seven
//! International and their use of these trademarks does not constitute an
//! endorsement by HL7.
//!
//! # Example
//!
//! ```
//! # fn main() -> Result<(), er7::Error> {
//! let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
//!             PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M\r\
//!             OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";
//!
//! let message = er7::parse(text)?;
//! assert_eq!(message.control_id().as_deref(), Some("MSG9"));
//! assert_eq!(message.query("PID-5.1")?.as_deref(), Some("SMITH"));
//! assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));
//!
//! // What went in comes back out, byte for byte.
//! assert_eq!(message.to_er7(), text);
//! # Ok(())
//! # }
//! ```
//!
//! # What is here
//!
//! | Item | Purpose |
//! |------|---------|
//! | [`parse()`], [`parse_with`] | read text into a [`Message`] |
//! | [`Message`], [`Segment`], [`Field`], [`Repetition`], [`Component`], [`Subcomponent`] | the six-level value tree |
//! | [`Message::query`], [`Message::query_all`] | read values by HL7 [`Path`], e.g. `PID-5.1` or `OBX[2]-5` |
//! | [`Message::to_er7`], [`Segment::to_text`] and siblings | write the tree back out, as sent or decoded |
//! | [`split_messages`] | cut a batch file or concatenated messages into individual ones |
//! | [`escape::unescape`], [`escape::escape`], [`escape::escapes`] | the escape-sequence vocabulary |
//! | [`Separators`] | the delimiter set, read from each message's own header rather than assumed |
//!
//! # What is deliberately not here
//!
//! This crate is an encoding, not a dictionary. It does not know which
//! fields a segment should have, what data type each one carries, which
//! message structures exist, or what any code table means — all of that is
//! version-specific and belongs in a layer above. It performs no
//! validation and no transport.
//!
//! The one exception is a handful of `MSH` accessors such as
//! [`Message::control_id`], because routing a message requires reading
//! them and their positions have never moved in any HL7 v2 release.
//!
//! # The crate family
//!
//! Each layer above this one is its own crate, so a caller pays only for
//! what they use:
//!
//! | Crate | Adds |
//! |-------|------|
//! | [`er7-redact`](https://crates.io/crates/er7-redact) | redaction: remove patient detail without changing the shape of the message |
//! | [`serde-er7`](https://crates.io/crates/serde-er7) | Serde support for every type in this tree |
//! | [`hl7-2-from-er7-into-xml`](https://crates.io/crates/hl7-2-from-er7-into-xml), [`hl7-2-from-er7-into-json`](https://crates.io/crates/hl7-2-from-er7-into-json) | the HL7 v2.5 dictionary |
//!
//! `spec/01-purpose-and-scope/index.md` §1.3.1 is the source of truth for that
//! list, and <https://er7-rust.github.io/ecosystem/> presents it.
//!
//! # Documentation
//!
//! `spec/index.md` in the repository is the normative specification of
//! everything above; where this documentation and that document disagree,
//! that document is right. Section references such as "spec §6.2" and rule
//! IDs such as "R16" throughout these docs point into it.
//!
//! The repository also holds a tutorial (`docs/usage/`), references for
//! [paths](Path) (`docs/paths/`) and [escape sequences](escape)
//! (`docs/escapes/`), an FAQ (`docs/faq/`), and runnable programs
//! (`examples/`).
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod escape;
pub mod message;
pub mod parse;
pub mod path;
pub mod render;
pub mod separators;

pub use crate::message::{Component, Field, Message, Repetition, Segment, Subcomponent};
pub use crate::parse::{parse, parse_with, split_messages};
pub use crate::path::Path;
pub use crate::render::RenderOptions;
pub use crate::separators::{Separators, Terminator};

use std::fmt;

/// What can go wrong.
///
/// Four variants, arising from exactly two situations: a message with no
/// usable header, and a path that is not a path. Everything else about ER7
/// is recoverable, and this crate recovers rather than refusing, because a
/// receiver that rejects a message it could have read is worse than one
/// that reads it as written (spec §11, rules R6 and R23).
///
/// Example:
///
/// ```
/// use er7::Error;
///
/// assert!(matches!(er7::parse(""), Err(Error::Empty)));
/// assert!(matches!(er7::parse("PID|1"), Err(Error::MissingHeader(_))));
/// assert!(matches!(er7::parse("MSH"), Err(Error::BadHeader(_))));
/// assert!(matches!("PID-0".parse::<er7::Path>(), Err(Error::BadPath(_))));
///
/// // Everything below the header parses, however odd it is.
/// assert!(er7::parse("MSH|^~\\&|LAB\rZZZ\rPID||||||||||").is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input held no segments at all — it was empty, or held only
    /// blank lines.
    Empty,
    /// The first segment is not `MSH`, `FHS`, or `BHS`, so the message
    /// never declared its delimiters. Carries the segment name that was
    /// found instead.
    MissingHeader(String),
    /// The header segment declared a delimiter set that cannot be used:
    /// missing, alphanumeric, a line ending, or reusing one character for
    /// two roles (spec §3.3). Carries a sentence naming the problem.
    BadHeader(String),
    /// A path such as `PID-5.1` could not be read (spec §8.1). Carries the
    /// path and the reason.
    BadPath(String),
}

impl fmt::Display for Error {
    /// One complete sentence, with no trailing period and no error prefix,
    /// so it reads correctly whether a caller writes `{e}`, wraps it, or
    /// prefixes it as the CLI does.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => write!(f, "input contains no HL7 segments"),
            Error::MissingHeader(name) => write!(
                f,
                "message starts with a {name} segment, not the MSH, FHS, or BHS header \
                 that declares the delimiters"
            ),
            Error::BadHeader(detail) => write!(f, "unusable delimiters in the header: {detail}"),
            Error::BadPath(detail) => write!(f, "invalid HL7 path {detail}"),
        }
    }
}

impl std::error::Error for Error {}
