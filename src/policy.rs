//! Rules, policies, the two built-in policies, and the policy file format.
//!
//! A [`Rule`] is one HL7 path and one [`Action`]. A [`Policy`] is an
//! ordered list of rules, plus an optional fallback action that covers
//! every leaf no rule named — which is what turns "redact these positions"
//! into "redact everything except these positions".
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
    /// The `Err` case is a path that is not a path; see [`er7::Path`].
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

/// The path that sets a policy's fallback rather than adding a rule.
const FALLBACK: &str = "*";

/// An ordered list of rules, plus the action for everything else.
///
/// Rules apply **in order**, each to the message as it stands (D7, spec
/// §2.4). The fallback, if there is one, runs last over every leaf that no
/// rule named (D9, spec §2.6).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Action, Policy, Redactor};
///
/// // Redact what is listed...
/// let listed = Policy::new()
///     .with("PID-5", Action::redacted())?
///     .with("PID-7", Action::First(4))?;
///
/// // ...or redact everything that is not.
/// let everything_else = Policy::new()
///     .with("MSH", Action::Keep)?
///     .fallback(Action::redacted());
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
    /// What to do with every leaf no rule named, if anything.
    pub fallback: Option<Action>,
}

// `Default` is deliberately not implemented. An empty default would
// silently redact nothing, and a curated default would silently redact
// forty positions; both are surprises a redaction crate cannot afford, so
// a caller names the policy they mean (spec §5).
#[allow(clippy::new_without_default)]
impl Policy {
    /// An empty policy: no rules, no fallback, and so no effect.
    ///
    /// This is the starting point for building one. The curated policy is
    /// [`Policy::patient_identifiers`]; there is deliberately no `Default`
    /// implementation, because either choice of default would be a silent
    /// one.
    pub fn new() -> Policy {
        Policy {
            rules: Vec::new(),
            fallback: None,
        }
    }

    /// The curated policy: the positions that carry a patient identifier
    /// in `PID`, `NK1`, `PV1`, `GT1`, and `IN1`.
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
            fallback: None,
        }
    }

    /// The other posture: replace every value in the message, except the
    /// `MSH` header that keeps it routable (spec §5.2).
    ///
    /// Use it when the message is unfamiliar, or when the answer to "is
    /// there anything else in here?" has to be "no" rather than "not that
    /// I listed". The cost is that nothing below `MSH` is clinically
    /// meaningful afterwards; add `Keep` rules for what a test needs.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy, Redactor};
    ///
    /// let policy = Policy::everything().with("OBX-2", Action::Keep)?;
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
    pub fn everything() -> Policy {
        Policy::new()
            .with("MSH", Action::Keep)
            .expect("built-in paths are well-formed")
            .fallback(Action::redacted())
    }

    /// Add a rule, for building a policy in one expression.
    ///
    /// The `Err` case is a path that is not a path.
    pub fn with(mut self, path: &str, action: Action) -> Result<Policy, Error> {
        self.rules.push(Rule::new(path, action)?);
        Ok(self)
    }

    /// Set the action for every leaf no rule named (spec §2.6).
    ///
    /// [`Action::Keep`] means "no fallback", which is also the default,
    /// so that the policy file's `* keep` and this method agree.
    pub fn fallback(mut self, action: Action) -> Policy {
        self.fallback = match action {
            Action::Keep => None,
            action => Some(action),
        };
        self
    }

    /// Read a policy file (spec §6).
    ///
    /// Blank lines and `#` comments are ignored; every other line is a
    /// path, whitespace, and an action, in the order they apply. A path of
    /// `*` sets the fallback rather than adding a rule.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy};
    ///
    /// let policy = Policy::parse("
    ///     MSH    keep      # everything but the header...
    ///     OBX-5  keep      # ...and the numbers the test asserts on
    ///
    ///     *      replace REDACTED
    /// ")?;
    ///
    /// assert_eq!(policy.rules.len(), 2);
    /// assert_eq!(policy.fallback, Some(Action::redacted()));
    ///
    /// // A malformed line names itself.
    /// let e = Policy::parse("PID-5 obfuscate").unwrap_err();
    /// assert_eq!(e.to_string(), "policy line 1: \"PID-5 obfuscate\": unknown action \"obfuscate\"");
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse(text: &str) -> Result<Policy, Error> {
        let mut policy = Policy::new();
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
            let Some((path, action)) = split_line(line) else {
                return Err(at(Error::BadPolicy(
                    "expected a path and an action".to_string(),
                )));
            };
            let action = Action::parse(action).map_err(at)?;
            if path == FALLBACK {
                // A second fallback replaces the first rather than being
                // ignored: a policy has one, and quietly keeping the
                // earlier one would hide an editing mistake (spec §6.3).
                policy = policy.fallback(action);
                continue;
            }
            policy.rules.push(Rule::new(path, action).map_err(at)?);
        }
        Ok(policy)
    }

    /// Append another policy's rules, and its fallback if it has one.
    ///
    /// This is how the command line concatenates several `--policy` files
    /// and `--rule` arguments; order is significant (D7).
    pub fn append(&mut self, other: Policy) {
        self.rules.extend(other.rules);
        if let Some(fallback) = other.fallback {
            self.fallback = Some(fallback);
        }
    }

    /// True when the policy would do nothing at all: no rules, no
    /// fallback.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.fallback.is_none()
    }
}

impl fmt::Display for Policy {
    /// The canonical policy file (spec §6.5): one rule per line, paths
    /// padded to a common width, the fallback last.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Action, Policy};
    ///
    /// let policy = Policy::new()
    ///     .with("PID-5", Action::redacted())?
    ///     .with("PID-7", Action::First(4))?
    ///     .fallback(Action::Clear);
    ///
    /// assert_eq!(policy.to_string(), "\
    /// PID-5  replace REDACTED
    /// PID-7  first 4
    /// *      clear
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
            .chain(self.fallback.iter().map(|_| FALLBACK.len()))
            .max()
            .unwrap_or(0);
        for rule in &self.rules {
            let path = rule.path.to_string();
            writeln!(f, "{path:<width$}  {}", rule.action)?;
        }
        if let Some(fallback) = &self.fallback {
            writeln!(f, "{FALLBACK:<width$}  {fallback}")?;
        }
        Ok(())
    }
}

/// Split a policy line into its path and its action, or `None` when it has
/// only one of the two.
fn split_line(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    let (path, action) = line.split_once(char::is_whitespace)?;
    let action = action.trim();
    match action.is_empty() {
        true => None,
        false => Some((path, action)),
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
            Policy::new(),
            Policy::patient_identifiers(),
            Policy::everything(),
        ] {
            assert_eq!(Policy::parse(&policy.to_string()).unwrap(), policy);
        }
    }

    #[test]
    fn parses_comments_blank_lines_and_the_fallback() {
        let policy = Policy::parse(
            "\
            # a comment on its own line\n\
            \n\
            PID-5   replace REDACTED   # and one after a rule\n\
            \t OBX-5  keep \n\
            *       clear\n",
        )
        .unwrap();
        assert_eq!(policy.rules.len(), 2);
        assert_eq!(policy.rules[0].action, Action::redacted());
        assert_eq!(policy.rules[1].path.to_string(), "OBX-5");
        assert_eq!(policy.fallback, Some(Action::Clear));

        // A later fallback replaces an earlier one, and `* keep` means
        // none at all (spec §6.3).
        let replaced = Policy::parse("* clear\n* replace X").unwrap();
        assert_eq!(replaced.fallback, Some(Action::Replace("X".to_string())));
        assert_eq!(Policy::parse("* keep").unwrap().fallback, None);
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
        assert_eq!(policy.fallback, None);

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

        assert!(Policy::new().is_empty());
    }

    #[test]
    fn appends_in_order() {
        let mut policy = Policy::new().with("PID-5", Action::redacted()).unwrap();
        policy.append(Policy::parse("PID-7 first 4\n* clear").unwrap());
        assert_eq!(policy.rules.len(), 2);
        assert_eq!(policy.rules[1].action, Action::First(4));
        assert_eq!(policy.fallback, Some(Action::Clear));
    }
}
