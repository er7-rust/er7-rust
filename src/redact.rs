//! The engine: applying a policy to a message, and reporting what changed.
//!
//! A [`Redactor`] is the only thing in this crate that edits a message. It
//! walks the tree once per rule, in order, and then once more for the
//! policy's fallback, rewriting leaf text and leaving the shape alone.
//!
//! Specified by spec §2 (the model), §4 (what is preserved), and §8 (the
//! report).

use std::collections::HashSet;
use std::fmt;

use er7::message::NULL;
use er7::{Component, Field, Message, Path, Repetition, Segment, Separators, Subcomponent};

use crate::{Action, Policy};

/// One leaf's coordinates: the segment's index in the message, then the
/// 1-based field, repetition, component, and subcomponent numbers.
type Position = (usize, usize, usize, usize, usize);

/// One position a redaction changed, and what changed it.
///
/// The path is fully qualified — every index present, even where it would
/// be unambiguous — so that a row is a valid `er7 --query` argument and an
/// audit trail nobody has to interpret (spec §8.3).
///
/// A change carries **no values** (D13): not the text that was there, and
/// not the text that replaced it. A log line quoting the old value puts
/// the patient's name into the log, the scrollback, and the CI transcript
/// (spec §8.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// Where it happened, e.g. `PID[1]-5[1].2.1`.
    pub path: Path,
    /// What happened there.
    pub action: Action,
}

impl fmt::Display for Change {
    /// The path and the action, separated by one space.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.path, self.action)
    }
}

/// What a redaction did: one entry per position that changed.
///
/// Entries are in the order the changes were made — rule by rule, and in
/// message order within each rule (spec §8.4). A rule that matched nothing
/// contributes none, and neither does an [`Action::Keep`], an empty leaf,
/// or a null one.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Action, Policy, Redactor};
///
/// let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN")?;
/// let policy = Policy::new().with("PID-5", Action::redacted())?;
/// let report = Redactor::new(policy).redact(&mut message);
///
/// // One row per leaf that actually changed.
/// let rows: Vec<String> = report.changes.iter().map(|c| c.to_string()).collect();
/// assert_eq!(rows, [
///     "PID[1]-5[1].1.1 replace REDACTED",
///     "PID[1]-5[1].2.1 replace REDACTED",
/// ]);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report {
    /// The changes, in the order they were made.
    pub changes: Vec<Change>,
}

impl Report {
    /// True when nothing changed — which means either that the message
    /// carried none of the positions the policy names, or that the policy
    /// is wrong. The crate does not presume to say which (spec §2.5).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// How many positions changed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.changes.len()
    }
}

impl fmt::Display for Report {
    /// One change per line. See [`Change`] for the row format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for change in &self.changes {
            writeln!(f, "{change}")?;
        }
        Ok(())
    }
}

/// A policy, plus the key its pseudonyms are derived from.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7_redact::Error> {
/// use er7_redact::{Policy, Redactor};
///
/// let text = "MSH|^~\\&|LAB\rPID|1||PATID1234||EVERYWOMAN^EVE||19610615|F";
/// let mut message = er7::parse(text)?;
///
/// let redactor = Redactor::new(Policy::patient_identifiers()).with_key(42);
/// let report = redactor.redact(&mut message);
///
/// assert_eq!(message.query("PID-5")?.as_deref(), Some("REDACTED^REDACTED"));
/// assert_eq!(message.query("PID-7")?.as_deref(), Some("1961"));
/// assert_eq!(report.len(), 4);
///
/// // The same key maps the same identifier the same way, in every message.
/// let mut other = er7::parse("MSH|^~\\&|LAB\rPID|1||PATID1234")?;
/// redactor.redact(&mut other);
/// assert_eq!(other.query("PID-3")?, message.query("PID-3")?);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redactor {
    policy: Policy,
    key: u64,
}

impl Redactor {
    /// A redactor for this policy, with the default pseudonym key `0`.
    #[must_use]
    pub fn new(policy: Policy) -> Redactor {
        Redactor { policy, key: 0 }
    }

    /// Set the pseudonym key (spec §7.2).
    ///
    /// Two data sets redacted under different keys share no pseudonyms and
    /// so cannot be joined; two under the same key can. The key is a
    /// number in a configuration file, not a managed secret — read spec
    /// §7.3 before treating it as one.
    #[must_use]
    pub fn with_key(mut self, key: u64) -> Redactor {
        self.key = key;
        self
    }

    /// The policy this redactor applies.
    #[must_use]
    pub fn policy(&self) -> &Policy {
        &self.policy
    }

    /// The pseudonym key.
    #[must_use]
    pub fn key(&self) -> u64 {
        self.key
    }

    /// Redact `message` in place, and report what changed.
    ///
    /// This cannot fail (spec §9.2): a rule that matches nothing does
    /// nothing, a position that is not there is not created, and an empty
    /// or null leaf is left alone.
    pub fn redact(&self, message: &mut Message) -> Report {
        // The segment name and occurrence of every segment, taken before
        // anything is borrowed mutably, so that a change can be labelled
        // with the path that names it.
        let mut counts: Vec<usize> = Vec::with_capacity(message.segments.len());
        let names: Vec<String> = message.segments.iter().map(|s| s.name.clone()).collect();
        for (index, name) in names.iter().enumerate() {
            counts.push(names[..index].iter().filter(|n| *n == name).count() + 1);
        }

        let mut pass = Pass {
            key: self.key,
            separators: message.separators,
            named: HashSet::new(),
            report: Report::default(),
        };

        for rule in &self.policy.rules {
            for index in 0..message.segments.len() {
                if names[index] != rule.path.segment {
                    continue;
                }
                if rule
                    .path
                    .segment_occurrence
                    .is_some_and(|wanted| wanted != counts[index])
                {
                    continue;
                }
                let at = At {
                    name: &names[index],
                    index,
                    occurrence: counts[index],
                };
                pass.segment(&mut message.segments[index], at, &rule.path, &rule.action);
            }
        }

        if let Some(action) = &self.policy.fallback {
            for index in 0..message.segments.len() {
                let at = At {
                    name: &names[index],
                    index,
                    occurrence: counts[index],
                };
                pass.fallback(&mut message.segments[index], at, action);
            }
        }

        pass.report
    }
}

impl Default for Redactor {
    /// The curated policy ([`Policy::patient_identifiers`]) with key `0` —
    /// the same thing the command line does when no policy is given.
    fn default() -> Redactor {
        Redactor::new(Policy::patient_identifiers())
    }
}

/// Which segment a walk is in: enough to label a change with its path.
#[derive(Debug, Clone, Copy)]
struct At<'a> {
    name: &'a str,
    index: usize,
    occurrence: usize,
}

/// One run of a policy over one message: the state that outlives a single
/// rule, and the descent that every rule shares.
struct Pass {
    key: u64,
    separators: Separators,
    /// Every leaf position some rule named, so that the fallback can skip
    /// them. A leaf a `Keep` rule named is in here too — that is what
    /// `Keep` is for (spec §2.6).
    named: HashSet<Position>,
    report: Report,
}

impl Pass {
    /// Apply one rule to one segment.
    fn segment(&mut self, segment: &mut Segment, at: At, path: &Path, action: &Action) {
        let header = segment.is_header();
        let numbers: Vec<usize> = match path.field {
            Some(number) => vec![number],
            None => (1..=segment.fields.len()).collect(),
        };
        for number in numbers {
            // D5: the header's first two fields are the delimiters
            // themselves. Redacting them would leave a message that either
            // does not parse or parses into different values.
            if header && number <= 2 {
                continue;
            }
            // D2: a position the message does not carry is not created.
            let Some(field) = segment.field_mut(number) else {
                continue;
            };
            if action == &Action::Null && path.repetition.is_none() && path.component.is_none() {
                if !field.is_null() {
                    *field = null_field();
                    self.record(at, number, 1, 1, 1, action);
                }
                continue;
            }
            let repetitions: Vec<usize> = match path.repetition {
                Some(number) => vec![number],
                None => (1..=field.repetitions.len()).collect(),
            };
            for repetition in repetitions {
                let Some(node) = field.repetition_mut(repetition) else {
                    continue;
                };
                if action == &Action::Null && path.component.is_none() {
                    if !node.is_null() {
                        *node = null_repetition();
                        self.record(at, number, repetition, 1, 1, action);
                    }
                    continue;
                }
                self.repetition(node, at, (number, repetition), path, action);
            }
        }
    }

    /// Apply one rule below a repetition.
    fn repetition(
        &mut self,
        repetition: &mut Repetition,
        at: At,
        (field, index): (usize, usize),
        path: &Path,
        action: &Action,
    ) {
        let numbers: Vec<usize> = match path.component {
            Some(number) => vec![number],
            None => (1..=repetition.components.len()).collect(),
        };
        for number in numbers {
            let Some(component) = repetition.component_mut(number) else {
                continue;
            };
            if action == &Action::Null && path.subcomponent.is_none() {
                if !component.is_null() {
                    *component = null_component();
                    self.record(at, field, index, number, 1, action);
                }
                continue;
            }
            let subcomponents: Vec<usize> = match path.subcomponent {
                Some(number) => vec![number],
                None => (1..=component.subcomponents.len()).collect(),
            };
            for subcomponent in subcomponents {
                let Some(leaf) = component.subcomponent_mut(subcomponent) else {
                    continue;
                };
                let position = (at.index, field, index, number, subcomponent);
                self.named.insert(position);
                if self.leaf(leaf, action) {
                    self.record(at, field, index, number, subcomponent, action);
                }
            }
        }
    }

    /// Apply the policy's fallback to every leaf of one segment that no
    /// rule named (D9, spec §2.6).
    fn fallback(&mut self, segment: &mut Segment, at: At, action: &Action) {
        let header = segment.is_header();
        for field in 1..=segment.fields.len() {
            // D5 again: the fallback reaches no further than a rule does.
            if header && field <= 2 {
                continue;
            }
            let Some(node) = segment.field_mut(field) else {
                continue;
            };
            for repetition in 1..=node.repetitions.len() {
                let Some(node) = node.repetition_mut(repetition) else {
                    continue;
                };
                for component in 1..=node.components.len() {
                    let Some(node) = node.component_mut(component) else {
                        continue;
                    };
                    for subcomponent in 1..=node.subcomponents.len() {
                        let position = (at.index, field, repetition, component, subcomponent);
                        if self.named.contains(&position) {
                            continue;
                        }
                        let Some(leaf) = node.subcomponent_mut(subcomponent) else {
                            continue;
                        };
                        if self.leaf(leaf, action) {
                            self.record(at, field, repetition, component, subcomponent, action);
                        }
                    }
                }
            }
        }
    }

    /// Apply an action to one leaf. Returns whether anything changed.
    ///
    /// A leaf is where the two skips live: an empty leaf has nothing to
    /// redact, and writing into it would invent a value; a null leaf is an
    /// instruction to the receiver rather than patient data, and
    /// overwriting it would turn "clear this" into a value (D3, D4).
    fn leaf(&mut self, leaf: &mut Subcomponent, action: &Action) -> bool {
        if action == &Action::Null {
            if leaf.is_null() {
                return false;
            }
            leaf.raw = NULL.to_string();
            return true;
        }
        if leaf.is_empty() || leaf.is_null() {
            return false;
        }
        let value = leaf.value(&self.separators).into_owned();
        let Some(replacement) = action.apply(&value, self.key) else {
            return false;
        };
        if replacement == value {
            // Nothing to do, and nothing to report — and leaving the raw
            // text alone keeps the sender's own spelling of it (D17).
            return false;
        }
        // `set` encodes any delimiter in the replacement, so a redaction
        // can never break the message (D11).
        leaf.set(&replacement, &self.separators);
        true
    }

    /// Add a row to the report, with the path fully qualified (spec §8.3).
    fn record(
        &mut self,
        at: At,
        field: usize,
        repetition: usize,
        component: usize,
        subcomponent: usize,
        action: &Action,
    ) {
        self.report.changes.push(Change {
            path: Path {
                segment: at.name.to_string(),
                segment_occurrence: Some(at.occurrence),
                field: Some(field),
                repetition: Some(repetition),
                component: Some(component),
                subcomponent: Some(subcomponent),
            },
            action: action.clone(),
        });
    }
}

/// The explicit HL7 null, as a leaf.
fn null_subcomponent() -> Subcomponent {
    Subcomponent::new(NULL)
}

/// The explicit HL7 null, as a component.
fn null_component() -> Component {
    Component {
        subcomponents: vec![null_subcomponent()],
    }
}

/// The explicit HL7 null, as a repetition.
fn null_repetition() -> Repetition {
    Repetition {
        components: vec![null_component()],
    }
}

/// The explicit HL7 null, as a field.
fn null_field() -> Field {
    Field {
        repetitions: vec![null_repetition()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rule;

    const ADT: &str = "MSH|^~\\&|ADT1|MCM|LABADT|MCM|20260815140000||ADT^A08|MSG00001|P|2.5\r\
                       PID|1||PATID1234^5^M11^ADT1^MR~123456789^^^USSSA^SS||\
                       JONES^WILLIAM^A^III||19610615|M||C|1200 N ELM STREET^^GREENSBORO^NC\r\
                       NK1|1|JONES^BARBARA^K|SPO\r\
                       OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL";

    fn message() -> Message {
        er7::parse(ADT).expect("sample parses")
    }

    fn redact(policy: Policy, message: &mut Message) -> Report {
        Redactor::new(policy).redact(message)
    }

    fn policy(rules: &[&str]) -> Policy {
        let mut policy = Policy::new();
        for rule in rules {
            policy.rules.push(Rule::parse(rule).expect("rule parses"));
        }
        policy
    }

    /// Every count in the tree, so that a test can assert the shape did
    /// not move.
    fn shape(message: &Message) -> Vec<usize> {
        let mut counts = vec![message.segments.len()];
        for segment in &message.segments {
            counts.push(segment.fields.len());
            for field in &segment.fields {
                counts.push(field.repetitions.len());
                for repetition in &field.repetitions {
                    counts.push(repetition.components.len());
                    for component in &repetition.components {
                        counts.push(component.subcomponents.len());
                    }
                }
            }
        }
        counts
    }

    #[test]
    fn preserves_the_shape() {
        // D1: redaction rewrites leaf text and nothing else, so every path
        // that resolved to a value before still resolves to one.
        let before = shape(&message());
        for rules in [
            vec!["PID-5 replace REDACTED"],
            vec!["PID-3 pseudonym", "PID-7 first 4"],
            vec!["PID-11 clear"],
            vec!["OBX-5 mask *"],
        ] {
            let mut message = message();
            redact(policy(&rules), &mut message);
            assert_eq!(shape(&message), before, "{rules:?} changed the shape");
            // And what came out is still a message.
            assert!(er7::parse(&message.to_er7()).is_ok());
        }
    }

    #[test]
    fn does_not_create_a_position() {
        // D2: a rule for a field the segment does not have is a no-op, not
        // a reason to pad the segment out to reach it.
        let mut message = message();
        let report = redact(policy(&["PID-99 replace X", "ZZZ-1 clear"]), &mut message);
        assert!(report.is_empty());
        assert_eq!(message.to_er7(), ADT);
    }

    #[test]
    fn leaves_an_empty_leaf_empty() {
        // D3: writing REDACTED into an empty field would invent a value,
        // and would announce that one used to be there.
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||||^JOHN").unwrap();
        let report = redact(
            policy(&["PID-2 replace X", "PID-5 replace X"]),
            &mut message,
        );
        assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||||^X");
        // One row: the empty positions contributed nothing.
        assert_eq!(report.len(), 1);
    }

    #[test]
    fn leaves_an_explicit_null_alone() {
        // D4: a null is an instruction to the receiver, not patient data.
        // Overwriting it would turn "clear this value" into a value.
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1|\"\"|A").unwrap();
        let report = redact(
            policy(&["PID-2 replace X", "PID-3 replace X"]),
            &mut message,
        );
        assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1|\"\"|X");
        assert_eq!(report.len(), 1);
    }

    #[test]
    fn never_touches_the_delimiter_fields() {
        // D5: MSH-1 and MSH-2 are the delimiters. A rule naming them is
        // accepted and applied to nothing, and neither is a fallback.
        let mut message = message();
        let mut policy = policy(&["MSH-1 replace X", "MSH-2 clear", "MSH-3 replace X"]);
        policy = policy.fallback(Action::Mask('#'));
        redact(policy, &mut message);
        assert!(message.to_er7().starts_with("MSH|^~\\&|X|"));
        assert!(er7::parse(&message.to_er7()).is_ok());
    }

    #[test]
    fn null_collapses_the_named_position() {
        // D6: the one action that changes shape, because an HL7 null is a
        // single `""` and not a `""` in every component (spec §3.4).
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN").unwrap();
        let report = redact(policy(&["PID-5 null"]), &mut message);
        assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||\"\"");
        assert_eq!(report.len(), 1);
        assert_eq!(report.changes[0].path.to_string(), "PID[1]-5[1].1.1");

        // A path that stops deeper nulls only what it names.
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN").unwrap();
        redact(policy(&["PID-5.1 null"]), &mut message);
        assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||\"\"^JOHN");

        // And nulling a null again changes nothing (D10).
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1|\"\"").unwrap();
        assert!(redact(policy(&["PID-2 null"]), &mut message).is_empty());
    }

    #[test]
    fn applies_rules_in_order() {
        // D7: rules run one after another against the message as it
        // stands, so a later rule sees the earlier rule's output — and a
        // `Keep` cannot undo a redaction (spec §2.4).
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||PATID1234").unwrap();
        redact(
            policy(&["PID-3 replace SMITH", "PID-3 first 2"]),
            &mut message,
        );
        assert_eq!(message.query("PID-3").unwrap().as_deref(), Some("SM"));

        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||PATID1234").unwrap();
        redact(
            policy(&["PID-3 replace REDACTED", "PID-3 keep"]),
            &mut message,
        );
        assert_eq!(message.query("PID-3").unwrap().as_deref(), Some("REDACTED"));
    }

    #[test]
    fn a_rule_that_matches_nothing_does_nothing() {
        // D8: a policy is written against a family of messages, not one,
        // so a segment this message does not have is not an error.
        let mut message = message();
        let report = redact(Policy::patient_identifiers(), &mut message);
        assert!(!report.is_empty());
        // No GT1 or IN1 in this message, so no rows name them.
        assert!(!report.changes.iter().any(|c| c.path.segment == "GT1"));
        assert!(!report.changes.iter().any(|c| c.path.segment == "IN1"));
    }

    #[test]
    fn the_fallback_covers_what_no_rule_named() {
        // D9: the fallback inverts the model — redact everything except
        // what a rule named — and a `Keep` rule is how a position is
        // exempted (spec §2.6).
        let mut message = er7::parse("MSH|^~\\&|LAB\rOBX|1|NM|2093-3||187").unwrap();
        let policy = policy(&["MSH keep", "OBX-2 keep"]).fallback(Action::redacted());
        redact(policy, &mut message);
        assert_eq!(
            message.to_er7(),
            "MSH|^~\\&|LAB\rOBX|REDACTED|NM|REDACTED||REDACTED"
        );
    }

    #[test]
    fn a_report_carries_no_values() {
        // D13: a report is meant to be pasted into a ticket, so it holds
        // the path and the action and nothing else (spec §8.2).
        let mut message = message();
        let report = redact(Policy::patient_identifiers(), &mut message);
        let text = report.to_string();
        for value in ["JONES", "WILLIAM", "PATID1234", "19610615", "GREENSBORO"] {
            assert!(!text.contains(value), "the report leaked {value}");
        }
        // Every row is a fully qualified path and an action.
        assert!(text.contains("PID[1]-5[1].1.1 replace REDACTED"));
        assert!(text.contains("NK1[1]-2[1].1.1 replace REDACTED"));
    }

    #[test]
    fn covers_every_repetition_and_occurrence() {
        // A rule that leaves an occurrence open covers all of them, which
        // is `er7`'s R19 doing the work here (spec §2.2).
        let mut message =
            er7::parse("MSH|^~\\&|LAB\rPID|1|555-1111~555-2222\rOBX|1|NM|A\rOBX|2|NM|B").unwrap();
        redact(policy(&["PID-2 clear", "OBX-3 replace X"]), &mut message);
        assert_eq!(message.query("PID-2").unwrap().as_deref(), Some("~"));
        assert_eq!(message.query_all("OBX-3").unwrap(), vec!["X", "X"]);

        // And an occurrence index pins one down.
        let mut message = er7::parse("MSH|^~\\&|LAB\rOBX|1|NM|A\rOBX|2|NM|B").unwrap();
        redact(policy(&["OBX[2]-3 replace X"]), &mut message);
        assert_eq!(message.query_all("OBX-3").unwrap(), vec!["A", "X"]);
    }
}
