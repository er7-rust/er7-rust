//! ER7 — the pipe-hat encoding that carries HL7 v2 messages between
//! healthcare systems — parsed, queried, edited, and written back, with no
//! dependencies.
//!
//! ER7 is compact and everywhere, and it is also unforgiving: a value's
//! meaning comes entirely from its position, so one misplaced `|` silently
//! shifts everything after it. This crate exists to make that structure
//! explicit and to keep it intact — text is stored exactly as it arrived,
//! and decoded only when you ask for a value.
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
//! - [`parse()`] and [`parse_with`] read text into a [`Message`]; the tree
//!   below it is [`Segment`], [`Field`], [`Repetition`], [`Component`],
//!   [`Subcomponent`].
//! - [`split_messages`] cuts a batch file or a run of concatenated messages
//!   into individual ones.
//! - [`Message::query`] and [`Message::query_all`] read values by HL7
//!   [`Path`], e.g. `PID-5.1` or `OBX[2]-5`.
//! - [`Message::to_er7`] writes the tree back out; [`Segment::to_text`] and
//!   its siblings write it with escape sequences resolved.
//! - [`escape::unescape`] and [`escape::escape`] convert between a value
//!   and its encoded form; [`escape::escapes`] exposes the whole
//!   escape-sequence vocabulary for callers that need more.
//! - [`Separators`] is the delimiter set, read from each message's own
//!   header rather than assumed.
//!
//! # What is deliberately not here
//!
//! This crate is an encoding, not a dictionary. It does not know which
//! fields a segment should have, what data type each one carries, which
//! message structures exist, or what any code table means — all of that is
//! version-specific and belongs in a layer above. The one exception is a
//! handful of `MSH` accessors such as [`Message::control_id`], because
//! routing a message requires reading them and their positions have never
//! moved.
//!
//! `spec/index.md` in the repository is the normative specification of
//! everything above; where this documentation and that document disagree,
//! that document is right.

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
/// Only two things can: a message with no usable header, and a path that is
/// not a path. Everything else about ER7 is recoverable, and this crate
/// recovers rather than refusing, because a receiver that rejects a message
/// it could have read is worse than one that reads it as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input held no segments at all.
    Empty,
    /// The first segment is not `MSH`, `FHS`, or `BHS`, so the message
    /// never declared its delimiters. Carries the name that was found.
    MissingHeader(String),
    /// The header segment declared a delimiter set that cannot be used —
    /// missing, alphanumeric, or reusing one character for two roles.
    BadHeader(String),
    /// A path such as `PID-5.1` could not be read; carries the reason.
    BadPath(String),
}

impl fmt::Display for Error {
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
