//! Rules, policies, the four built-in policies, and the policy file format.
//!
//! A [`Rule`] is one HL7® path and one [`Action`]. A [`Policy`] is an
//! ordered list of rules plus the two things it does by default: its
//! [`Posture`] — accept or reject every leaf no rule named — and what it
//! does with a payload that is not ER7 at all ([`Unrecognised`]).
//!
//! Specified by spec §5 (the built-in policies) and §6 (the file format).

use crate::{Action, Error};
use er7::Path;
use std::fmt;

/// One HL7 path and what to do at it.
///
/// The path is an [`er7::Path`], so its notation and semantics are `er7`'s
/// (that crate's spec §8.1): an omitted occurrence index means *every*
/// segment of that name and *every* repetition of that field, which is
/// what lets `OBX-5` cover a message with forty results.
///
/// A rule whose action is [`Action::Keep`] **accepts** the position it
/// names; a rule with any other action **rejects** it. Where both name the
/// same leaf, the rejecting one wins, whichever order they are in (D19,
/// spec §2.4).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Action, Rule};
///
/// let rule = Rule::new("PID-5", Action::redacted())?;
/// assert_eq!(rule.path.segment, "PID");
/// assert_eq!(rule.to_string(), "PID-5 replace REDACTED");
///
/// // Or read one as a policy file spells it.
/// assert_eq!(Rule::parse("PID-7 first 4")?.action, Action::First(4));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// Where the rule applies.
    pub path: Path,
    /// What it does there.
    pub action: Action,
}

impl Rule {
    /// A rule from a path and an action.
    ///
    /// # Errors
    ///
    /// [`Error::Er7`] wrapping [`er7::Error::BadPath`] when `path` is not
    /// an HL7 path — a zero index, a missing field number, trailing text.
    pub fn new(path: &str, action: Action) -> Result<Rule, Error> {
        Ok(Rule {
            path: Path::parse(path)?,
            action,
        })
    }

    /// Read one policy line: a path, whitespace, an action (spec §6).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Rule};
    ///
    /// let rule = Rule::parse("  OBX[2]-5   replace NOT ON FILE  ")?;
    /// assert_eq!(rule.path.to_string(), "OBX[2]-5");
    /// assert_eq!(rule.action, Action::Replace("NOT ON FILE".to_string()));
    ///
    /// assert!(Rule::parse("PID-5").is_err());       // no action
    /// assert!(Rule::parse("PID-0 clear").is_err()); // not a path
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// [`Error::BadPolicy`] naming the rule and the problem: a line with
    /// no action, an action that does not exist, or a path that is not a
    /// path.
    pub fn parse(line: &str) -> Result<Rule, Error> {
        let at = |e: Error| Error::BadPolicy(format!("rule {:?}: {e}", line.trim()));
        let Some((path, action)) = split_line(line) else {
            return Err(at(Error::BadPolicy(
                "expected a path and an action".to_string(),
            )));
        };
        let action = Action::parse(action).map_err(at)?;
        Rule::new(path, action).map_err(at)
    }
}

impl fmt::Display for Rule {
    /// The path and the action, separated by one space — the policy file
    /// spelling, so a rule written out reads back as itself.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.path, self.action)
    }
}

/// The first word that accepts by default rather than naming a position.
const ACCEPT: &str = "accept";

/// The first word that rejects by default.
const REJECT: &str = "reject";

/// The first word that says what an unrecognised payload gets.
const UNRECOGNISED: &str = "unrecognised";

/// The spelling of [`UNRECOGNISED`] this crate also reads. Both are in the
/// field, and a policy file that fails over one letter helps nobody.
const UNRECOGNIZED: &str = "unrecognized";

/// The first word that turns the known-values sweep on or off (D23, spec
/// §2.10).
const KNOWN_VALUES: &str = "known-values";

/// The path that set the fallback before 0.2, kept only to be refused with
/// a sentence naming its replacement (spec §6.3).
const REMOVED_FALLBACK: &str = "*";

/// The column the default lines pad their first word to. `KNOWN_VALUES` is
/// the same length as `UNRECOGNISED`, so one width serves both.
const DEFAULT_WIDTH: usize = UNRECOGNISED.len();

/// What a policy does with every leaf that no rule named (D9, spec §2.6).
///
/// A policy has exactly one of these and cannot leave it unstated: "redact
/// what is listed" and "redact everything except what is listed" are
/// different enough that guessing between them is not something a
/// redaction crate may do.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Action, Policy, Posture, Redactor};
///
/// // Accept by default: only what a rule names is redacted.
/// let listed = Policy::accept_all().with("PID-5", Action::redacted())?;
///
/// // Reject by default: only what a `keep` rule names survives.
/// let all_but = Policy::reject_all().with("PID-5", Action::Keep)?;
///
/// assert_eq!(listed.posture, Posture::Accept);
/// assert_eq!(all_but.posture, Posture::Reject(Action::redacted()));
///
/// let text = "MSH|^~\\&|LAB\rPID|1||9||SMITH";
/// let mut one = er7::parse(text)?;
/// let mut two = er7::parse(text)?;
/// Redactor::new(listed).redact(&mut one);
/// Redactor::new(all_but).redact(&mut two);
///
/// assert_eq!(one.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||REDACTED");
/// assert_eq!(two.to_er7(), "MSH|^~\\&|REDACTED\rPID|REDACTED||REDACTED||SMITH");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Posture {
    /// Accept by default: a leaf no rule named is left exactly as it is.
    Accept,
    /// Reject by default: a leaf no rule named gets this action.
    ///
    /// `Reject(Action::Keep)` is a contradiction — rejecting a value by
    /// leaving it alone — and [`Policy::posture`] normalises it to
    /// [`Posture::Accept`], which is what it means.
    Reject(Action),
}

impl Posture {
    /// How strict this posture is, for D20; see [`Policy::append`].
    fn strictness(&self) -> u8 {
        match self {
            Posture::Accept => 0,
            Posture::Reject(_) => 1,
        }
    }

    /// Read `accept` or `reject [ACTION]` (spec §6.3).
    fn parse(word: &str, argument: &str) -> Result<Posture, Error> {
        if word == ACCEPT {
            if argument.is_empty() {
                Ok(Posture::Accept)
            } else {
                Err(Error::BadPolicy(format!(
                    "{ACCEPT:?} takes no argument, but got {argument:?}"
                )))
            }
        } else if argument.is_empty() {
            // A bare `reject` means the placeholder the built-in policies
            // write, the same way a bare `replace` does (spec §6.2).
            Ok(Posture::Reject(Action::redacted()))
        } else {
            Ok(normalise_posture(Posture::Reject(Action::parse(argument)?)))
        }
    }
}

impl fmt::Display for Posture {
    /// The policy file spelling, so a policy written out re-reads as
    /// itself (D18, spec §6.5).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Posture::Accept => write!(f, "{ACCEPT}"),
            Posture::Reject(action) => write!(f, "{REJECT:<DEFAULT_WIDTH$}  {action}"),
        }
    }
}

/// What a policy does with a payload that is not ER7 (D21, spec §2.8).
///
/// A payload with no header, or one that `er7` cannot parse, has no
/// positions in it: no rule can name anything, and the posture has no leaf
/// to reach. This is the only thing a policy can say about it.
///
/// Example:
///
/// ```
/// use er7_redact::{Action, Policy, Redactor, Unrecognised};
///
/// let junk = "{\"patient\": \"EVERYWOMAN\"}";
///
/// // The curated policies refuse: nothing is written, and the caller says so.
/// assert_eq!(Redactor::default().unrecognised(junk), None);
///
/// // `accept_all` passes it through, because it redacts nothing at all.
/// let passed = Redactor::new(Policy::accept_all()).unrecognised(junk);
/// assert_eq!(passed.as_deref(), Some(junk));
///
/// // `reject_all` masks it whole, because it rejects everything else.
/// let masked = Redactor::new(Policy::reject_all()).unrecognised(junk);
/// assert_eq!(masked.as_deref(), Some("*************************"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unrecognised {
    /// Write the payload out unchanged.
    Pass,
    /// Apply this action to the whole payload, as if it were one value.
    ///
    /// `Apply(Action::Keep)` and `Apply(Action::Null)` write nothing in
    /// the payload's place, which is [`Unrecognised::Pass`];
    /// [`Policy::on_unrecognised`] normalises both to it.
    Apply(Action),
    /// Write nothing, and tell the caller the payload did not parse.
    ///
    /// The library does not raise this as an error — [`crate::Redactor`]
    /// cannot fail (spec §9.2). It returns `None`, and the caller decides
    /// what a refusal costs; the CLI makes it a diagnostic and exit 1
    /// (spec §10.4).
    Refuse,
}

impl Unrecognised {
    /// Read `refuse`, `pass`, or an action (spec §6.3).
    fn parse(argument: &str) -> Result<Unrecognised, Error> {
        match argument.to_ascii_lowercase().as_str() {
            "" => Err(Error::BadPolicy(format!(
                "{UNRECOGNISED:?} wants \"refuse\", \"pass\", or an action"
            ))),
            "refuse" => Ok(Unrecognised::Refuse),
            "pass" => Ok(Unrecognised::Pass),
            _ => Ok(normalise_unrecognised(Unrecognised::Apply(Action::parse(
                argument,
            )?))),
        }
    }
}

impl fmt::Display for Unrecognised {
    /// The policy file spelling, after the `unrecognised` keyword.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unrecognised::Pass => write!(f, "pass"),
            Unrecognised::Apply(action) => write!(f, "{action}"),
            Unrecognised::Refuse => write!(f, "refuse"),
        }
    }
}

/// A posture that rejects by keeping is an accepting one; see
/// [`Posture::Reject`].
fn normalise_posture(posture: Posture) -> Posture {
    match posture {
        Posture::Reject(Action::Keep) => Posture::Accept,
        posture => posture,
    }
}

/// A disposition that writes nothing in the payload's place passes it
/// through; see [`Unrecognised::Apply`].
fn normalise_unrecognised(unrecognised: Unrecognised) -> Unrecognised {
    match unrecognised {
        Unrecognised::Apply(Action::Keep | Action::Null) => Unrecognised::Pass,
        unrecognised => unrecognised,
    }
}

/// Read `on` or `off` for the `known-values` line (D23, spec §6.3).
fn parse_known_values(argument: &str) -> Result<bool, Error> {
    match argument.to_ascii_lowercase().as_str() {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(Error::BadPolicy(format!(
            "{KNOWN_VALUES:?} wants \"on\" or \"off\""
        ))),
    }
}

/// An ordered list of rules, plus what to do with everything they do not
/// name.
///
/// Rules apply **in order**, each to the message as it stands (D7, spec
/// §2.4). The [`Posture`] then runs over every leaf that no rule named
/// (D9, spec §2.6), and [`Unrecognised`] covers a payload that is not ER7
/// at all (D21, spec §2.8).
///
/// # A reject beats an accept (D19)
///
/// A rule whose action is [`Action::Keep`] **accepts** the position it
/// names; any other action **rejects** it. Where a leaf is named by both,
/// the rejecting rule wins — **whichever order the two rules are in**, and
/// at whatever depth, so a reject naming a whole segment beats an accept
/// naming one field inside it.
///
/// A leaf named by both is a policy somebody got wrong, and redacting it
/// is the direction that fails safely (spec §1.5, priority 1): a value
/// redacted by mistake costs a policy edit, and a value left behind by
/// mistake cannot be recalled.
///
/// The mirror of that rule: an accept naming a whole segment is **not**
/// narrowed by the posture. `MSH keep` exempts every leaf of the header,
/// including ones the policy's author never saw. Only a reject rule
/// reaches back into it.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Action, Policy, Redactor};
///
/// // Redact what is listed...
/// let listed = Policy::accept_all()
///     .with("PID-5", Action::redacted())?
///     .with("PID-7", Action::First(4))?;
///
/// // ...or redact everything that is not.
/// let everything_else = Policy::reject_all().with("MSH", Action::Keep)?;
///
/// let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN||19610615")?;
/// Redactor::new(listed).redact(&mut message);
/// assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||REDACTED^REDACTED||1961");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// The rules, in the order they apply.
    pub rules: Vec<Rule>,
    /// What every leaf no rule named gets.
    pub posture: Posture,
    /// What a payload that is not ER7 gets.
    pub unrecognised: Unrecognised,
    /// Whether a value found at a named position is redacted wherever
    /// else it appears (D23, spec §2.10). Defaults to `true`.
    pub search_known_values: bool,
}

// `Default` is deliberately not implemented, and neither is a `new`. Both
// would have to choose a posture without being asked: an accepting empty
// policy silently redacts nothing, and a curated one silently redacts
// forty positions. A caller names the policy they mean (spec §5).
impl Policy {
    /// Accept everything: no rules, nothing redacted, and a payload that
    /// is not ER7 passed through unchanged (spec §5.6).
    ///
    /// This is the starting point for building a policy rule by rule. On
    /// its own it does nothing at all, and it says so: a policy named
    /// "accept all" that quietly replaced an unparseable payload with
    /// `***` would be the one surprise it has no excuse for.
    ///
    /// A policy *file* that states no defaults is not quite this: it
    /// accepts by default too, but it refuses an unrecognised payload,
    /// because it was written by somebody who did not think about one
    /// (spec §6.1). Ask for [`Unrecognised::Pass`] in the file to get it.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Posture, Redactor, Unrecognised};
    ///
    /// let policy = Policy::accept_all();
    /// assert_eq!(policy.posture, Posture::Accept);
    /// assert_eq!(policy.unrecognised, Unrecognised::Pass);
    /// assert!(policy.is_empty());
    ///
    /// // It changes nothing, and reports nothing.
    /// let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH")?;
    /// let report = Redactor::new(policy).redact(&mut message);
    /// assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||SMITH");
    /// assert!(report.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn accept_all() -> Policy {
        Policy {
            rules: Vec::new(),
            posture: Posture::Accept,
            unrecognised: Unrecognised::Pass,
            search_known_values: true,
        }
    }

    /// Reject everything: no rules, `replace REDACTED` over every leaf,
    /// and a payload that is not ER7 masked whole (spec §5.6).
    ///
    /// The strictest thing in the crate, and it takes the header with it:
    /// everything from `MSH-3` on reads `REDACTED`, so the message is no
    /// longer routable or identifiable. [`Policy::all_but_the_header`] is
    /// the same posture with the header kept, and is usually what is
    /// wanted.
    ///
    /// `MSH-1` and `MSH-2` survive, as they survive everything: they are
    /// the delimiters themselves (D5, spec §4.4).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Redactor};
    ///
    /// let policy = Policy::reject_all().with("OBX-2", Action::Keep)?;
    /// let mut message = er7::parse("MSH|^~\\&|LAB\rOBX|1|NM|2093-3||187")?;
    /// Redactor::new(policy).redact(&mut message);
    ///
    /// assert_eq!(
    ///     message.to_er7(),
    ///     "MSH|^~\\&|REDACTED\rOBX|REDACTED|NM|REDACTED||REDACTED",
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn reject_all() -> Policy {
        Policy {
            rules: Vec::new(),
            posture: Posture::Reject(Action::redacted()),
            unrecognised: Unrecognised::Apply(Action::Mask('*')),
            search_known_values: true,
        }
    }

    /// The curated policy: the positions that carry a patient identifier
    /// in `PID`, `NK1`, `PV1`, `GT1`, and `IN1`.
    ///
    /// It **accepts by default**, so a position the table does not name is
    /// left as it is, and it **refuses** a payload that is not ER7: a list
    /// of positions has no opinion about input with no positions in it,
    /// and refusing is the fail-closed answer (spec §2.8).
    ///
    /// The whole table is written out in spec §5.1, with a reason for each
    /// action. **It is a starting point, not a compliance certification**
    /// (D14): it does not touch free text, quasi-identifiers, or local `Z`
    /// segments, and it does not know which positions your senders
    /// actually use. Read spec §5.4 and §5.5 before relying on it.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Policy, Redactor};
    ///
    /// let mut message = er7::parse(
    ///     "MSH|^~\\&|LAB\rPID|1||PATID1234||EVERYWOMAN^EVE||19610615|F|||12 ELM ST^^BOSTON",
    /// )?;
    /// let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);
    ///
    /// assert_eq!(message.query("PID-5.1")?.as_deref(), Some("REDACTED"));
    /// assert_eq!(message.query("PID-7")?.as_deref(), Some("1961"));
    /// assert_eq!(message.query("PID-11.1")?.as_deref(), Some(""));
    /// assert_ne!(message.query("PID-3")?.as_deref(), Some("PATID1234"));
    ///
    /// // The sex is not an identifier, so the default policy leaves it.
    /// assert_eq!(message.query("PID-8")?.as_deref(), Some("F"));
    /// assert!(!report.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Only if the table below is edited to hold something that is not an
    /// HL7 path — every entry is a literal, and
    /// `the_documented_positions_match_the_built_in_policy` in
    /// `tests/integration.rs` checks the whole table against spec §5.1.
    #[must_use]
    pub fn patient_identifiers() -> Policy {
        // Spec §5.1 is the normative table; this list is its executable
        // form, in the same order, and the two are changed together.
        let table: &[(&str, Action)] = &[
            // PID — patient identification. An identifier field names its
            // first component, which is the ID number itself: the
            // assigning authority and identifier type beside it identify
            // the interface rather than the patient, and a message that
            // keeps them still looks like the one it came from (spec §5.1).
            ("PID-2.1", Action::Pseudonym),
            ("PID-3.1", Action::Pseudonym),
            ("PID-4.1", Action::Pseudonym),
            ("PID-5", Action::redacted()),
            ("PID-6", Action::redacted()),
            ("PID-7", Action::First(4)),
            ("PID-9", Action::redacted()),
            ("PID-11", Action::Clear),
            ("PID-12", Action::Clear),
            ("PID-13", Action::Clear),
            ("PID-14", Action::Clear),
            ("PID-18.1", Action::Pseudonym),
            ("PID-19", Action::Clear),
            ("PID-20", Action::Clear),
            ("PID-21.1", Action::Pseudonym),
            ("PID-23", Action::Clear),
            ("PID-29", Action::First(4)),
            // NK1 — next of kin.
            ("NK1-2", Action::redacted()),
            ("NK1-4", Action::Clear),
            ("NK1-5", Action::Clear),
            ("NK1-6", Action::Clear),
            // PV1 — patient visit.
            ("PV1-5.1", Action::Pseudonym),
            ("PV1-7", Action::redacted()),
            ("PV1-8", Action::redacted()),
            ("PV1-9", Action::redacted()),
            ("PV1-17", Action::redacted()),
            ("PV1-19.1", Action::Pseudonym),
            // GT1 — guarantor.
            ("GT1-2.1", Action::Pseudonym),
            ("GT1-3", Action::redacted()),
            ("GT1-4", Action::redacted()),
            ("GT1-5", Action::Clear),
            ("GT1-6", Action::Clear),
            ("GT1-7", Action::Clear),
            ("GT1-8", Action::First(4)),
            ("GT1-12", Action::Clear),
            // IN1 — insurance.
            ("IN1-16", Action::redacted()),
            ("IN1-18", Action::First(4)),
            ("IN1-19", Action::Clear),
            ("IN1-36", Action::Pseudonym),
            ("IN1-49.1", Action::Pseudonym),
        ];
        let rules = table
            .iter()
            .map(|(path, action)| {
                Rule::new(path, action.clone()).expect("built-in paths are well-formed")
            })
            .collect();
        Policy {
            rules,
            posture: Posture::Accept,
            unrecognised: Unrecognised::Refuse,
            search_known_values: true,
        }
    }

    /// The other posture, curated: reject every value, and keep the `MSH`
    /// header so the message stays routable (spec §5.2).
    ///
    /// Use it when the message is unfamiliar, or when the answer to "is
    /// there anything else in here?" has to be "no" rather than "not that
    /// I listed". The cost is that nothing below `MSH` is clinically
    /// meaningful afterwards; add `Keep` rules for what a test needs.
    ///
    /// Like [`Policy::patient_identifiers`] it **refuses** a payload that
    /// is not ER7 rather than guessing (spec §2.8).
    ///
    /// The header exception is an ordinary accept rule, so an ordinary
    /// reject rule overrides it (D19) — `.with("MSH", Action::redacted())`
    /// takes the header too.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Redactor};
    ///
    /// let policy = Policy::all_but_the_header().with("OBX-2", Action::Keep)?;
    /// let mut message = er7::parse("MSH|^~\\&|LAB\rOBX|1|NM|2093-3||187")?;
    /// Redactor::new(policy).redact(&mut message);
    ///
    /// assert_eq!(
    ///     message.to_er7(),
    ///     "MSH|^~\\&|LAB\rOBX|REDACTED|NM|REDACTED||REDACTED",
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Only if the `MSH` literal below stops being an HL7 path, which no
    /// caller can cause.
    #[must_use]
    pub fn all_but_the_header() -> Policy {
        Policy::reject_all()
            .with("MSH", Action::Keep)
            .expect("built-in paths are well-formed")
            .on_unrecognised(Unrecognised::Refuse)
    }

    /// Add a rule, for building a policy in one expression.
    ///
    /// # Errors
    ///
    /// [`Error::Er7`] when `path` is not an HL7 path; see [`Rule::new`].
    pub fn with(mut self, path: &str, action: Action) -> Result<Policy, Error> {
        self.rules.push(Rule::new(path, action)?);
        Ok(self)
    }

    /// Set what every leaf no rule named gets (spec §2.6).
    ///
    /// [`Posture::Reject`] with [`Action::Keep`] is normalised to
    /// [`Posture::Accept`], so that the policy file's `reject keep` and
    /// this method agree about what they mean.
    ///
    /// This is the only way to make a policy *less* strict: appending one
    /// policy to another never weakens it (D20, [`Policy::append`]).
    #[must_use]
    pub fn posture(mut self, posture: Posture) -> Policy {
        self.posture = normalise_posture(posture);
        self
    }

    /// Set what a payload that is not ER7 gets (spec §2.8).
    ///
    /// [`Unrecognised::Apply`] with an action that writes nothing —
    /// [`Action::Keep`] or [`Action::Null`] — is normalised to
    /// [`Unrecognised::Pass`], which is what it does.
    #[must_use]
    pub fn on_unrecognised(mut self, unrecognised: Unrecognised) -> Policy {
        self.unrecognised = normalise_unrecognised(unrecognised);
        self
    }

    /// Set whether a value found at a named position is redacted wherever
    /// else it appears (D23, spec §2.10).
    ///
    /// Every built-in policy defaults to `true`. There is no normalising
    /// to do here — unlike [`Policy::posture`] and
    /// [`Policy::on_unrecognised`], a plain `bool` has no contradictory
    /// spelling to collapse.
    #[must_use]
    pub fn search_known_values(mut self, search_known_values: bool) -> Policy {
        self.search_known_values = search_known_values;
        self
    }

    /// Read a policy file (spec §6).
    ///
    /// Blank lines and `#` comments are ignored; every other line is
    /// either a path, whitespace, and an action, in the order they apply,
    /// or one of the three reserved first words — `accept`, `reject`, and
    /// `unrecognised` — that set what the policy does by default.
    ///
    /// A file that states no defaults accepts by default and **refuses** a
    /// payload that is not ER7: unlike [`Policy::accept_all`], a file was
    /// written by somebody who may simply not have considered one, and
    /// refusing is the answer that cannot lose a value quietly.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Posture, Unrecognised};
    ///
    /// let policy = Policy::parse("
    ///     MSH    keep      # everything but the header...
    ///     OBX-5  keep      # ...and the numbers the test asserts on
    ///
    ///     reject replace REDACTED
    ///     unrecognised mask *
    /// ")?;
    ///
    /// assert_eq!(policy.rules.len(), 2);
    /// assert_eq!(policy.posture, Posture::Reject(Action::redacted()));
    /// assert_eq!(policy.unrecognised, Unrecognised::Apply(Action::Mask('*')));
    ///
    /// // A file that says nothing accepts, and refuses what it cannot read.
    /// let quiet = Policy::parse("PID-5 clear")?;
    /// assert_eq!(quiet.posture, Posture::Accept);
    /// assert_eq!(quiet.unrecognised, Unrecognised::Refuse);
    ///
    /// // A malformed line names itself.
    /// let e = Policy::parse("PID-5 obfuscate").unwrap_err();
    /// assert_eq!(e.to_string(), "policy line 1: \"PID-5 obfuscate\": unknown action \"obfuscate\"");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// [`Error::BadPolicy`] naming the **line number**, the line, and the
    /// problem. Reading a policy is the one place this crate is strict,
    /// because a typo means a value that silently was not redacted (spec
    /// §6.4).
    pub fn parse(text: &str) -> Result<Policy, Error> {
        let mut policy = Policy::accept_all().on_unrecognised(Unrecognised::Refuse);
        for (index, line) in text.lines().enumerate() {
            // A `#` starts a comment wherever it appears, so replacement
            // text cannot contain one (spec §16.4).
            let line = match line.split_once('#') {
                Some((before, _comment)) => before,
                None => line,
            }
            .trim();
            if line.is_empty() {
                continue;
            }
            let number = index + 1;
            let at = |e: Error| Error::BadPolicy(format!("policy line {number}: {line:?}: {e}"));

            // The reserved words come first, and a line is one or the
            // other: three characters is the whole of a segment name, so
            // none of them can be a path (spec §6.3).
            let (word, argument) = match split_line(line) {
                Some((word, argument)) => (word, argument),
                None => (line, ""),
            };
            let lowercase = word.to_ascii_lowercase();
            match lowercase.as_str() {
                ACCEPT | REJECT => {
                    // A second one replaces the first rather than being
                    // ignored: a policy has one posture, and quietly
                    // keeping the earlier one would hide an editing
                    // mistake (spec §6.3).
                    policy.posture = Posture::parse(&lowercase, argument).map_err(at)?;
                    continue;
                }
                UNRECOGNISED | UNRECOGNIZED => {
                    policy.unrecognised = Unrecognised::parse(argument).map_err(at)?;
                    continue;
                }
                KNOWN_VALUES => {
                    policy.search_known_values = parse_known_values(argument).map_err(at)?;
                    continue;
                }
                REMOVED_FALLBACK => {
                    // Refused rather than read as a synonym: `*` never
                    // said which of the two postures it meant (spec §6.3).
                    let replacement = match argument {
                        "" | "keep" => ACCEPT.to_string(),
                        action => format!("{REJECT} {action}"),
                    };
                    return Err(at(Error::BadPolicy(format!(
                        "the default line is now {replacement:?}, not {REMOVED_FALLBACK:?}"
                    ))));
                }
                _ => {}
            }

            if argument.is_empty() {
                return Err(at(Error::BadPolicy(
                    "expected a path and an action".to_string(),
                )));
            }
            let action = Action::parse(argument).map_err(at)?;
            policy.rules.push(Rule::new(word, action).map_err(at)?);
        }
        Ok(policy)
    }

    /// Append another policy's rules, take the stricter posture, and
    /// take the appended policy's disposition for an unrecognised payload
    /// (D20, spec §2.6).
    ///
    /// This is how the command line concatenates several `--policy` files
    /// and `--rule` arguments; order is significant for the rules (D7).
    ///
    /// The **posture** cannot be weakened by appending, and deliberately:
    /// a file of extra rules says nothing about its posture, so it accepts
    /// by default, and adopting that would switch redaction off for
    /// everything the file did not name. Silence is indistinguishable from
    /// a decision, so silence is not trusted. To relax a posture, say so
    /// with [`Policy::posture`].
    ///
    /// The disposition for an **unrecognised payload** is different, and
    /// the appended policy's wins outright. Nothing is silent there:
    /// [`Policy::parse`] gives a file that says nothing the strictest
    /// disposition there is, [`Unrecognised::Refuse`], so every value one
    /// carries is somebody's decision — and a file that goes to the
    /// trouble of writing `unrecognised pass` should not be quietly
    /// overruled by a default it never saw.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Posture};
    ///
    /// let mut policy = Policy::all_but_the_header();
    /// policy.append(Policy::parse("OBX-2 keep")?);
    ///
    /// // The appended file accepts by default; the strict policy still rejects.
    /// assert_eq!(policy.posture, Posture::Reject(Action::redacted()));
    ///
    /// // And a stricter action in the appended policy does win.
    /// policy.append(Policy::parse("reject clear")?);
    /// assert_eq!(policy.posture, Posture::Reject(Action::Clear));
    /// # Ok(())
    /// # }
    /// ```
    pub fn append(&mut self, other: Policy) {
        self.rules.extend(other.rules);
        if other.posture.strictness() >= self.posture.strictness() {
            self.posture = other.posture;
        }
        self.unrecognised = other.unrecognised;
        // Only ever turns on, for the same reason the posture can only
        // get stricter (D20, spec §2.6): a file of extra rules that says
        // nothing about it is not a decision to turn the sweep off.
        self.search_known_values |= other.search_known_values;
    }

    /// True when the policy would redact nothing at all: no rules, and it
    /// accepts by default.
    ///
    /// What it does with an unrecognised payload is not part of this: a
    /// policy that refuses one still redacts nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.posture == Posture::Accept
    }
}

impl fmt::Display for Policy {
    /// The canonical policy file (spec §6.5): one rule per line, paths
    /// padded to a common width, then the three default lines — always
    /// all three, whatever they say, so that a reader never has to know
    /// which default was the quiet one.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy};
    ///
    /// let policy = Policy::accept_all()
    ///     .with("PID-5", Action::redacted())?
    ///     .with("PID-7", Action::First(4))?
    ///     .posture(er7_redact::Posture::Reject(Action::Clear));
    ///
    /// assert_eq!(policy.to_string(), "\
    /// PID-5  replace REDACTED
    /// PID-7  first 4
    ///
    /// reject        clear
    /// unrecognised  pass
    /// known-values  on
    /// ");
    ///
    /// // And it reads back as the same policy.
    /// assert_eq!(Policy::parse(&policy.to_string())?, policy);
    /// # Ok(())
    /// # }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self
            .rules
            .iter()
            .map(|rule| rule.path.to_string().len())
            .max()
            .unwrap_or(0);
        for rule in &self.rules {
            let path = rule.path.to_string();
            writeln!(f, "{path:<width$}  {}", rule.action)?;
        }
        if !self.rules.is_empty() {
            writeln!(f)?;
        }
        writeln!(f, "{}", self.posture)?;
        writeln!(f, "{UNRECOGNISED:<DEFAULT_WIDTH$}  {}", self.unrecognised)?;
        let known_values = if self.search_known_values {
            "on"
        } else {
            "off"
        };
        writeln!(f, "{KNOWN_VALUES:<DEFAULT_WIDTH$}  {known_values}")
    }
}

/// Split a policy line into its first word and the rest, or `None` when it
/// has only the one.
fn split_line(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    let (path, action) = line.split_once(char::is_whitespace)?;
    let action = action.trim();
    if action.is_empty() {
        None
    } else {
        Some((path, action))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_policy_round_trips_through_display() {
        // D18: the file format is a compatibility surface, so a policy
        // written out must read back as the same policy (spec §6.5).
        for policy in [
            Policy::accept_all(),
            Policy::reject_all(),
            Policy::patient_identifiers(),
            Policy::all_but_the_header(),
            Policy::accept_all()
                .posture(Posture::Reject(Action::Null))
                .on_unrecognised(Unrecognised::Apply(Action::First(4))),
            Policy::accept_all().search_known_values(false),
        ] {
            assert_eq!(Policy::parse(&policy.to_string()).unwrap(), policy);
        }
    }

    #[test]
    fn parses_comments_blank_lines_and_the_defaults() {
        let policy = Policy::parse(
            "\
            # a comment on its own line\n\
            \n\
            PID-5   replace REDACTED   # and one after a rule\n\
            \t OBX-5  keep \n\
            REJECT  clear\n\
            Unrecognized  pass\n",
        )
        .unwrap();
        assert_eq!(policy.rules.len(), 2);
        assert_eq!(policy.rules[0].action, Action::redacted());
        assert_eq!(policy.rules[1].path.to_string(), "OBX-5");
        assert_eq!(policy.posture, Posture::Reject(Action::Clear));
        assert_eq!(policy.unrecognised, Unrecognised::Pass);

        // A later default line replaces an earlier one (spec §6.3).
        let replaced = Policy::parse("reject clear\nreject replace X").unwrap();
        assert_eq!(
            replaced.posture,
            Posture::Reject(Action::Replace("X".to_string()))
        );

        // A bare `reject` is the placeholder the built-ins write, and
        // rejecting by keeping is accepting.
        assert_eq!(
            Policy::parse("reject").unwrap().posture,
            Posture::Reject(Action::redacted())
        );
        assert_eq!(
            Policy::parse("reject keep").unwrap().posture,
            Posture::Accept
        );
        assert_eq!(
            Policy::parse("unrecognised null").unwrap().unrecognised,
            Unrecognised::Pass
        );

        // A file that says nothing accepts, and refuses what it cannot read.
        let quiet = Policy::parse("PID-5 clear").unwrap();
        assert_eq!(quiet.posture, Posture::Accept);
        assert_eq!(quiet.unrecognised, Unrecognised::Refuse);
    }

    #[test]
    fn reports_a_bad_policy_line() {
        // D15: reading a policy is the one place this crate is strict,
        // because a typo means a value that silently was not redacted
        // (spec §6.4).
        let cases = [
            (
                "PID-5 obfuscate",
                "policy line 1: \"PID-5 obfuscate\": unknown action \"obfuscate\"",
            ),
            (
                "MSH keep\nPID-0 clear",
                "policy line 2: \"PID-0 clear\": invalid HL7 path \"PID-0\": \
                 indices are 1-based, so 0 is not a position",
            ),
            (
                "PID-5",
                "policy line 1: \"PID-5\": expected a path and an action",
            ),
            (
                "# comment\n\nPID-7 first three",
                "policy line 3: \"PID-7 first three\": action \"first\" wants a number \
                 of characters, not \"three\"",
            ),
            (
                "accept everything",
                "policy line 1: \"accept everything\": \"accept\" takes no argument, \
                 but got \"everything\"",
            ),
            (
                "unrecognised",
                "policy line 1: \"unrecognised\": \"unrecognised\" wants \"refuse\", \
                 \"pass\", or an action",
            ),
            (
                "unrecognised sideways",
                "policy line 1: \"unrecognised sideways\": unknown action \"sideways\"",
            ),
            // The `*` line of 0.1, refused with its replacement (spec §6.3).
            (
                "MSH keep\n* replace REDACTED",
                "policy line 2: \"* replace REDACTED\": the default line is now \
                 \"reject replace REDACTED\", not \"*\"",
            ),
            (
                "* keep",
                "policy line 1: \"* keep\": the default line is now \"accept\", not \"*\"",
            ),
        ];
        for (text, expected) in cases {
            let error = Policy::parse(text).unwrap_err();
            assert_eq!(error.to_string(), expected, "parsing {text:?}");
        }
    }

    #[test]
    fn the_default_policy_names_the_documented_positions() {
        // D14: the table in spec §5.1 is normative, and this is its
        // executable form; the two change together. A test cannot say the
        // list is *sufficient* — no test can (spec §5.5) — only that it is
        // the list the spec documents.
        let policy = Policy::patient_identifiers();
        assert_eq!(policy.rules.len(), 40);
        assert_eq!(policy.posture, Posture::Accept);
        assert_eq!(policy.unrecognised, Unrecognised::Refuse);

        let named: Vec<String> = policy.rules.iter().map(|r| r.path.to_string()).collect();
        for path in [
            "PID-3.1", "PID-5", "PID-7", "PID-11", "PID-19", "NK1-2", "GT1-3", "IN1-16",
        ] {
            assert!(named.contains(&path.to_string()), "missing {path}");
        }
        // Deliberately absent: free text and quasi-identifiers (spec §5.4).
        for path in ["NTE-3", "OBX-5", "PID-8", "PID-10", "MSH-4"] {
            assert!(!named.contains(&path.to_string()), "unexpected {path}");
        }

        assert!(Policy::accept_all().is_empty());
        assert!(!Policy::reject_all().is_empty());
    }

    #[test]
    fn the_two_bare_postures_say_what_they_are() {
        // Spec §5.6: no rules and no field table, and the two defaults
        // that match what each one claims about itself.
        let accept = Policy::accept_all();
        assert!(accept.rules.is_empty());
        assert_eq!(accept.posture, Posture::Accept);
        assert_eq!(accept.unrecognised, Unrecognised::Pass);

        let reject = Policy::reject_all();
        assert!(reject.rules.is_empty());
        assert_eq!(reject.posture, Posture::Reject(Action::redacted()));
        assert_eq!(reject.unrecognised, Unrecognised::Apply(Action::Mask('*')));

        // The curated one is the same posture, with the header kept and a
        // refusal in place of the mask (spec §5.2).
        let curated = Policy::all_but_the_header();
        assert_eq!(curated.posture, reject.posture);
        assert_eq!(curated.unrecognised, Unrecognised::Refuse);
        assert_eq!(curated.rules.len(), 1);
        assert_eq!(curated.rules[0].to_string(), "MSH keep");
    }

    #[test]
    fn appends_in_order() {
        let mut policy = Policy::accept_all()
            .with("PID-5", Action::redacted())
            .unwrap();
        policy.append(Policy::parse("PID-7 first 4\nreject clear").unwrap());
        assert_eq!(policy.rules.len(), 2);
        assert_eq!(policy.rules[1].action, Action::First(4));
        assert_eq!(policy.posture, Posture::Reject(Action::Clear));
    }

    #[test]
    fn appending_never_weakens_the_defaults() {
        // D20: a file of extra rules says nothing about its posture, so it
        // accepts by default. Adopting that would switch redaction off for
        // everything the file did not name — the one failure spec §1.5
        // puts first (spec §2.6).
        let mut strict = Policy::all_but_the_header();
        strict.append(Policy::parse("OBX-2 keep").unwrap());
        assert_eq!(strict.posture, Posture::Reject(Action::redacted()));
        assert_eq!(strict.unrecognised, Unrecognised::Refuse);

        // Even when the appended policy says `accept` outright.
        let mut strict = Policy::all_but_the_header();
        strict.append(Policy::parse("accept").unwrap());
        assert_eq!(strict.posture, Posture::Reject(Action::redacted()));

        // A file that says nothing about an unrecognised payload is given
        // the strictest disposition when it is read, so adopting it can
        // only tighten a rejecting policy that masked one.
        let mut masking = Policy::reject_all();
        masking.append(Policy::parse("OBX-2 keep").unwrap());
        assert_eq!(masking.unrecognised, Unrecognised::Refuse);

        // But a file that asks for one outright is not overruled: nothing
        // else in the run said anything about it.
        let mut masking = Policy::reject_all();
        masking.append(Policy::parse("unrecognised pass").unwrap());
        assert_eq!(masking.unrecognised, Unrecognised::Pass);

        // Strictening works, in both directions of travel.
        // `mask #` is unwritable in a file — a `#` starts a comment
        // wherever it appears (spec §16.4) — so this uses another one.
        let mut lax = Policy::accept_all();
        lax.append(Policy::parse("reject mask X\nunrecognised refuse").unwrap());
        assert_eq!(lax.posture, Posture::Reject(Action::Mask('X')));
        assert_eq!(lax.unrecognised, Unrecognised::Refuse);

        // And relaxing one is done deliberately, which is the only way.
        let relaxed = Policy::all_but_the_header().posture(Posture::Accept);
        assert_eq!(relaxed.posture, Posture::Accept);
    }

    #[test]
    fn known_values_line_parses_and_displays() {
        // D23, spec §6.3: on by default, on every built-in, and a file
        // that never mentions it gets the same default.
        for policy in [
            Policy::accept_all(),
            Policy::reject_all(),
            Policy::patient_identifiers(),
            Policy::all_but_the_header(),
        ] {
            assert!(policy.search_known_values, "{policy:?}");
        }
        assert!(Policy::parse("PID-5 clear").unwrap().search_known_values);

        let off = Policy::parse("PID-5 clear\nknown-values off").unwrap();
        assert!(!off.search_known_values);
        assert!(off.to_string().ends_with("known-values  off\n"));

        let on = Policy::parse("PID-5 clear\nKNOWN-VALUES ON").unwrap();
        assert!(on.search_known_values);

        // A second line replaces the first, like the other two defaults.
        let last = Policy::parse("known-values off\nknown-values on").unwrap();
        assert!(last.search_known_values);

        let error = Policy::parse("known-values sideways").unwrap_err();
        assert_eq!(
            error.to_string(),
            "policy line 1: \"known-values sideways\": \"known-values\" wants \"on\" or \"off\""
        );
    }

    #[test]
    fn appending_only_turns_known_values_on() {
        // Symmetric with D20's posture rule: appending can only turn the
        // sweep on, never off, so a checked-in "known-values off" cannot
        // be silently defeated by a --rule the caller adds at the command
        // line, and cannot silently switch it off either.
        let mut off = Policy::accept_all().search_known_values(false);
        off.append(Policy::parse("PID-5 clear").unwrap());
        assert!(off.search_known_values, "appending must not stay off");

        let mut off = Policy::accept_all().search_known_values(false);
        off.append(Policy::accept_all().search_known_values(false));
        assert!(!off.search_known_values, "off appended to off stays off");
    }
}
