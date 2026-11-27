//! # ER7 redact
//!
//! **[website](https://er7-rust.github.io/er7-redact/)**
//! •
//! **[documentation](https://docs.rs/er7-redact/)**
//! •
//! **[source](https://github.com/er7-rust/er7-rust/tree/main/er7-redact)**
//! •
//! **[crate](https://crates.io/crates/er7-redact)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! Remove patient detail from HL7® v2 messages in the ER7 pipe-hat
//! encoding — without breaking the message.
//!
//! A redacted message still parses, still declares the same delimiters,
//! and still holds a value in every position that held one before, so
//! everything downstream of it behaves the way it did on the original.
//! That is the whole design: redaction rewrites leaf text and nothing else
//! (D1).
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
//! # fn main() -> Result<(), er7_redact::Error> {
//! use er7_redact::{Policy, Redactor};
//!
//! let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ADT^A08|MSG9|P|2.5\r\
//!             PID|1||PATID1234^^^ACME^MR||EVERYWOMAN^EVE^E||19610615|F|||\
//!             12 ELM ST^^BOSTON^MA^02101|||555-555-1111";
//!
//! let mut message = er7::parse(text)?;
//! let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message)?;
//!
//! // The name is a placeholder, the birth date is a year, the address and
//! // the phone number are gone, and the record number is a pseudonym.
//! assert_eq!(message.query("PID-5")?.as_deref(), Some("REDACTED^REDACTED^REDACTED"));
//! assert_eq!(message.query("PID-7")?.as_deref(), Some("1961"));
//! assert_eq!(message.query("PID-11")?.as_deref(), Some("^^^^"));
//! assert_eq!(message.query("PID-13")?.as_deref(), Some(""));
//! assert_ne!(message.query("PID-3.1")?.as_deref(), Some("PATID1234"));
//!
//! // The shape did not move: the assigning authority is still there, in
//! // the component it was in, and the message still parses.
//! assert_eq!(message.query("PID-3.4")?.as_deref(), Some("ACME"));
//! assert!(er7::parse(&message.to_er7()).is_ok());
//!
//! // And there is a record of exactly what changed.
//! assert_eq!(report.changes[0].path.to_string(), "PID[1]-3[1].1.1");
//! # Ok(())
//! # }
//! ```
//!
//! # Accept by default, or reject by default
//!
//! Every policy is one of the two, and says which ([`Posture`], spec §2.6):
//!
//! ```
//! # fn main() -> Result<(), er7_redact::Error> {
//! use er7_redact::{Action, Policy, Redactor};
//!
//! // Accept by default — redact the positions the policy lists.
//! let listed = Policy::patient_identifiers();
//!
//! // Reject by default — redact everything except what a `keep` rule names.
//! let all_but = Policy::all_but_the_header().with("OBX-5", Action::Keep)?;
//! # let _ = (listed, all_but);
//! # Ok(())
//! # }
//! ```
//!
//! Accepting by default is the safer *message* — it leaves the clinical
//! content a test needs. Rejecting by default is the safer *posture* — it
//! is the only one that covers a `Z` segment nobody documented, or a field
//! an interface started sending last week. Where the two disagree inside
//! one policy, the reject wins (D19).
//!
//! # What is here
//!
//! | Item | Purpose |
//! |------|---------|
//! | [`Redactor`] | a policy and a pseudonym key; the only thing that edits a message |
//! | [`Policy`], [`Rule`], [`Action`] | what to redact, where, and how |
//! | [`Posture`] | accept by default, or reject by default |
//! | [`Unrecognised`] | what a payload that is not ER7 gets |
//! | [`Policy::patient_identifiers`] | the curated default: `PID`, `NK1`, `PV1`, `GT1`, `IN1` |
//! | [`Policy::all_but_the_header`] | the other posture, curated: redact all but what you name |
//! | [`Policy::accept_all`], [`Policy::reject_all`] | the two bare postures, with no rules at all |
//! | [`Policy::parse`] | read a policy file |
//! | [`Report`], [`Change`] | what a redaction did, with no values in it |
//! | [`pseudonym()`] | the stable stand-in an identifier is replaced by |
//!
//! # What is deliberately not here
//!
//! This crate is a **positional editor, not a compliance tool**. It does
//! not know whether the positions it redacts are the ones your senders
//! use; it cannot tell you whether what remains is de-identified, because
//! that is a judgement about a whole data set made by a person who is
//! accountable for it; and it cannot find an identifier written into free
//! text, because no positional rule can.
//!
//! There is also no way back. No mapping table, no key escrow, no undo.
//!
//! A message this crate has redacted is a message with less in it, which
//! is progress, and is not the same thing as a safe one.
//!
//! # Documentation
//!
//! `spec/index.md` in the repository is the normative specification of
//! everything above; where this documentation and that document disagree,
//! that document is right. Section references such as "spec §5.1" and rule
//! IDs such as "D1" throughout these docs point into it.
//!
//! The repository also holds a tutorial (`docs/usage/`), the policy
//! reference (`docs/policies/`), an FAQ (`docs/faq/`), and runnable
//! programs (`examples/`).
//!
//! For the encoding layer underneath — parsing, queries, escape sequences,
//! the absent/empty/null distinction — see the [`er7`] crate, which is
//! this crate's only dependency.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod action;
pub mod policy;
pub mod pseudonym;
pub mod redact;

pub use crate::action::{Action, CustomAction};
pub use crate::policy::{Policy, Posture, Rule, Unrecognised};
pub use crate::pseudonym::pseudonym;
pub use crate::redact::{Change, Redactor, Report, is_standard_segment_id, shareable_path};

use std::fmt;

/// What can go wrong.
///
/// Four variants, arising from four situations: a policy that cannot be
/// read, a path that is not a path (D15, spec §9), and an action that writes
/// nothing, for an unrecognised payload or for a leaf (spec §9.1, §9.2). A
/// rule that matches nothing does nothing, and a position that is not there
/// is not created; neither is an error. But an action that writes nothing
/// where text must be redacted fails closed, so [`Redactor::redact`] returns
/// a `Result<Report, Error>`.
///
/// Example:
///
/// ```
/// use er7_redact::{Error, Policy};
///
/// assert!(matches!(Policy::parse("PID-5 obfuscate"), Err(Error::BadPolicy(_))));
/// assert!(matches!(Policy::parse("PID-0 clear"), Err(Error::BadPolicy(_))));
///
/// // A path handed straight to the API reports as `er7` phrased it.
/// use er7_redact::{Action, Rule};
/// assert!(matches!(Rule::new("PID-0", Action::Clear), Err(Error::Er7(_))));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A policy could not be read: an unknown action, a missing one, a
    /// count that is not a number, or a line naming something that is not
    /// a path. Carries a sentence naming the line and the problem (spec
    /// §6.4).
    BadPolicy(String),
    /// A path handed to the API is not a path. Carries `er7`'s own error,
    /// so that the message reads the same whichever crate reported it.
    Er7(er7::Error),
    /// The policy's `unrecognised` disposition is `Apply(action)` and the
    /// action writes nothing (`keep`, `null`, or a custom action returning
    /// `None`), so there is nothing to put in the payload's place.
    /// [`Redactor::unrecognised`] fails closed rather than return the
    /// payload unredacted (RCA-IN-2, RCA-IN-6). `Policy::on_unrecognised`
    /// and policy files normalise `keep` and `null` to `pass`, so only the
    /// public field or a custom action reaches this.
    NoReplacement {
        /// The action as it displays (`keep`, `null`, `<custom>`).
        action: String,
    },
    /// An action wrote nothing for a leaf that carries text (a custom
    /// action returning `None`), so [`Redactor::redact`] fails closed
    /// rather than leave the text, possibly PHI, in the message (spec
    /// §9.2; RCA-IN-2, RCA-IN-6). `Keep` is not this: it leaves the text
    /// on purpose.
    NoOutput {
        /// The action as it displays (`<custom>`).
        action: String,
        /// The leaf's fully qualified path, such as `PID[1]-5[1].1.1`.
        path: String,
    },
    /// A `null` action would write the explicit null `""` into a message
    /// that uses `"` as one of its delimiters. There the two quotes are
    /// structure, so writing them would split the position and shift every
    /// value after it. [`Redactor::redact`] fails closed instead (D25,
    /// spec §9.2).
    NullUnrepresentable {
        /// The position the null would have been written to, fully
        /// qualified, such as `ZXX[1]-1[1].1.1`.
        path: String,
    },
}

impl fmt::Display for Error {
    /// One complete sentence, with no trailing period and no error prefix,
    /// so it reads correctly whether a caller writes `{e}`, wraps it, or
    /// prefixes it as the CLI does.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadPolicy(detail) => write!(f, "{detail}"),
            Error::Er7(e) => write!(f, "{e}"),
            Error::NoReplacement { action } => write!(
                f,
                "the unrecognised-payload action {action:?} writes nothing, \
                 so the payload cannot be redacted"
            ),
            Error::NoOutput { action, path } => write!(
                f,
                "the action {action:?} writes nothing at {path}, \
                 so the value cannot be redacted"
            ),
            Error::NullUnrepresentable { path } => write!(
                f,
                "a null cannot be written at {path}, because the message \
                 uses \" as a delimiter"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::BadPolicy(_)
            | Error::NoReplacement { .. }
            | Error::NoOutput { .. }
            | Error::NullUnrepresentable { .. } => None,
            Error::Er7(e) => Some(e),
        }
    }
}

impl From<er7::Error> for Error {
    fn from(e: er7::Error) -> Error {
        Error::Er7(e)
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use std::error::Error as _;

    #[test]
    fn error_sources_distinguish_policy_text_from_er7_errors() {
        let policy = Error::BadPolicy("bad policy".into());
        assert_eq!(policy.to_string(), "bad policy");
        assert!(policy.source().is_none());

        let wrapped = Error::from(er7::Error::Empty);
        assert_eq!(wrapped.to_string(), "input contains no HL7 segments");
        assert!(wrapped.source().is_some());

        let none = Error::NoReplacement {
            action: "keep".into(),
        };
        assert_eq!(
            none.to_string(),
            "the unrecognised-payload action \"keep\" writes nothing, so the payload cannot be redacted"
        );
        assert!(none.source().is_none());

        let nothing = Error::NoOutput {
            action: "<custom>".into(),
            path: "PID[1]-5[1].1.1".into(),
        };
        assert_eq!(
            nothing.to_string(),
            "the action \"<custom>\" writes nothing at PID[1]-5[1].1.1, so the value cannot be redacted"
        );
        assert!(nothing.source().is_none());

        let unrepresentable = Error::NullUnrepresentable {
            path: "ZXX[1]-1[1].1.1".into(),
        };
        assert_eq!(
            unrepresentable.to_string(),
            "a null cannot be written at ZXX[1]-1[1].1.1, because the message uses \" as a delimiter"
        );
        assert!(unrepresentable.source().is_none());
    }
}
