//! The engine: applying a policy to a message, and reporting what changed.
//!
//! A [`Redactor`] is the only thing in this crate that edits a message. It
//! walks the tree once per rule, in order, and then once more where the
//! policy rejects by default, rewriting leaf text and leaving the shape
//! alone.
//!
//! Specified by spec §2 (the model), §4 (what is preserved), and §8 (the
//! report).

use std::collections::HashSet;
use std::fmt;

use er7::message::NULL;
use er7::{Component, Field, Message, Path, Repetition, Segment, Separators, Subcomponent};

use crate::{Action, Policy, Posture, Unrecognised};

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
/// let policy = Policy::accept_all().with("PID-5", Action::redacted())?;
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

    /// What to write in place of `payload`, which did not parse as ER7
    /// (D21, spec §2.8).
    ///
    /// `None` means the policy **refuses** it: nothing should be written,
    /// and the caller reports that the payload did not parse. That is not
    /// an error this crate raises — [`Redactor::redact`] cannot fail — so
    /// the caller decides what a refusal costs. The CLI makes it a
    /// diagnostic and exit 1 (spec §10.4).
    ///
    /// `Some(text)` is the payload itself where the policy passes it
    /// through, or the policy's action applied to the whole payload as if
    /// it were one value.
    ///
    /// Example:
    ///
    /// ```
    /// use er7_redact::{Action, Policy, Redactor, Unrecognised};
    ///
    /// let junk = "not a message";
    ///
    /// // The curated policies refuse a payload they cannot read.
    /// assert_eq!(Redactor::default().unrecognised(junk), None);
    ///
    /// // The bare postures each do what their name says.
    /// assert_eq!(
    ///     Redactor::new(Policy::accept_all()).unrecognised(junk).as_deref(),
    ///     Some("not a message"),
    /// );
    /// assert_eq!(
    ///     Redactor::new(Policy::reject_all()).unrecognised(junk).as_deref(),
    ///     Some("*************"),
    /// );
    ///
    /// // And any of it is overridable.
    /// let policy = Policy::accept_all().on_unrecognised(Unrecognised::Apply(Action::redacted()));
    /// assert_eq!(
    ///     Redactor::new(policy).unrecognised(junk).as_deref(),
    ///     Some("REDACTED"),
    /// );
    /// ```
    #[must_use]
    pub fn unrecognised(&self, payload: &str) -> Option<String> {
        match &self.policy.unrecognised {
            Unrecognised::Refuse => None,
            Unrecognised::Pass => Some(payload.to_string()),
            // An action that writes nothing leaves the payload as it is:
            // `Policy::on_unrecognised` normalises those away, so this
            // arm is only reachable through the public field.
            Unrecognised::Apply(action) => Some(
                action
                    .apply(payload, self.key)
                    .unwrap_or_else(|| payload.to_string()),
            ),
        }
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

        if let Posture::Reject(action) = &self.policy.posture {
            for index in 0..message.segments.len() {
                let at = At {
                    name: &names[index],
                    index,
                    occurrence: counts[index],
                };
                pass.reject_the_rest(&mut message.segments[index], at, action);
            }
        }

        pass.report
    }

    /// Every leaf that carries text and is named by no rule in this
    /// policy — the inverse of redaction: not what changed, but what a
    /// rule never looked at (D22, spec §2.9).
    ///
    /// Independent of the policy's posture: this reports what no *rule*
    /// names, not what the policy will eventually do about it. Under an
    /// accepting default this is the leak surface — what
    /// [`redact`](Redactor::redact) will leave exactly as it arrived.
    /// Under a rejecting default it is what the posture is about to blank
    /// on the caller's behalf; reporting it here is not a defect.
    ///
    /// Takes `&Message`, not `&mut Message`, and never mutates it: the
    /// positions a rule would name are computed against a disposable copy
    /// and discarded, so calling this before, after, or instead of
    /// [`redact`](Redactor::redact) sees the same message either way.
    ///
    /// A leaf **carries text** when it is neither empty nor the explicit
    /// null (D3, D4) — the same test a rejecting posture already applies
    /// before it acts. Paths are fully qualified and in message order,
    /// the same convention [`Change::path`] uses.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::{Policy, Redactor};
    ///
    /// let message = er7::parse(
    ///     "MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN\rNTE|1||spoke with the patient",
    /// )?;
    /// let redactor = Redactor::new(Policy::patient_identifiers());
    ///
    /// let gaps: Vec<String> = redactor
    ///     .uncovered(&message)
    ///     .iter()
    ///     .map(std::string::ToString::to_string)
    ///     .collect();
    ///
    /// // The default policy names PID-5 (patient name) and PID-3.1
    /// // (patient ID) — neither is a gap.
    /// assert!(!gaps.iter().any(|path| path.starts_with("PID[1]-5")));
    /// assert!(!gaps.contains(&"PID[1]-3[1].1.1".to_string()));
    ///
    /// // It names no free text, and no set ID (spec §5.4) — both are gaps.
    /// assert!(gaps.contains(&"NTE[1]-3[1].1.1".to_string()));
    /// assert!(gaps.contains(&"PID[1]-1[1].1.1".to_string()));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn uncovered(&self, message: &Message) -> Vec<Path> {
        let names: Vec<String> = message.segments.iter().map(|s| s.name.clone()).collect();
        let mut counts: Vec<usize> = Vec::with_capacity(message.segments.len());
        for (index, name) in names.iter().enumerate() {
            counts.push(names[..index].iter().filter(|n| *n == name).count() + 1);
        }
        let covered = self.named_positions(message, &names, &counts);

        let mut paths = Vec::new();
        for (index, segment) in message.segments.iter().enumerate() {
            let header = segment.is_header();
            for field in 1..=segment.fields.len() {
                // D5: the header's delimiter fields are never a gap —
                // no rule can name them, and neither can this.
                if header && field <= 2 {
                    continue;
                }
                let Some(node) = segment.field(field) else {
                    continue;
                };
                for repetition in 1..=node.repetitions.len() {
                    let Some(node) = node.repetition(repetition) else {
                        continue;
                    };
                    for component in 1..=node.components.len() {
                        let Some(node) = node.component(component) else {
                            continue;
                        };
                        for subcomponent in 1..=node.subcomponents.len() {
                            let position = (index, field, repetition, component, subcomponent);
                            if covered.contains(&position) {
                                continue;
                            }
                            let Some(leaf) = node.subcomponent(subcomponent) else {
                                continue;
                            };
                            if leaf.is_empty() || leaf.is_null() {
                                continue;
                            }
                            paths.push(Path {
                                segment: names[index].clone(),
                                segment_occurrence: Some(counts[index]),
                                field: Some(field),
                                repetition: Some(repetition),
                                component: Some(component),
                                subcomponent: Some(subcomponent),
                            });
                        }
                    }
                }
            }
        }
        paths
    }

    /// Every leaf position a rule in this policy would name in `message`,
    /// computed without mutating it.
    ///
    /// Runs the same rule-matching walk [`Redactor::redact`] uses, on a
    /// disposable clone — reusing tested, working mutation logic rather
    /// than re-deriving "what a rule names" as a second, separately
    /// maintained implementation that could quietly drift from the first.
    /// The clone is discarded; only the set of positions it visited
    /// survives.
    fn named_positions(
        &self,
        message: &Message,
        names: &[String],
        counts: &[usize],
    ) -> HashSet<Position> {
        let mut throwaway = message.clone();
        let mut pass = Pass {
            key: self.key,
            separators: throwaway.separators,
            named: HashSet::new(),
            report: Report::default(),
        };
        for rule in &self.policy.rules {
            for index in 0..throwaway.segments.len() {
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
                pass.segment(&mut throwaway.segments[index], at, &rule.path, &rule.action);
            }
        }
        pass.named
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
    /// Every leaf position some rule named, so that a rejecting posture
    /// can skip them. A leaf a `Keep` rule named is in here too — that is
    /// what `Keep` is for, and it is why an accept naming a whole segment
    /// is not narrowed by the posture (spec §2.4, §2.6).
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

    /// Apply a rejecting posture's action to every leaf of one segment
    /// that no rule named (D9, spec §2.6).
    fn reject_the_rest(&mut self, segment: &mut Segment, at: At, action: &Action) {
        let header = segment.is_header();
        for field in 1..=segment.fields.len() {
            // D5 again: the posture reaches no further than a rule does.
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

/// The explicit HL7® null, as a leaf.
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
        let mut policy = Policy::accept_all();
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
        // accepted and applied to nothing, and so is a rejecting posture.
        let mut message = message();
        let mut policy = policy(&["MSH-1 replace X", "MSH-2 clear", "MSH-3 replace X"]);
        policy = policy.posture(Posture::Reject(Action::Mask('#')));
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
    fn rejecting_by_default_covers_what_no_rule_named() {
        // D9: rejecting by default inverts the model — redact everything
        // except what a rule named — and a `Keep` rule is how a position
        // is exempted (spec §2.6).
        let mut message = er7::parse("MSH|^~\\&|LAB\rOBX|1|NM|2093-3||187").unwrap();
        let policy =
            policy(&["MSH keep", "OBX-2 keep"]).posture(Posture::Reject(Action::redacted()));
        redact(policy, &mut message);
        assert_eq!(
            message.to_er7(),
            "MSH|^~\\&|LAB\rOBX|REDACTED|NM|REDACTED||REDACTED"
        );
    }

    #[test]
    fn a_segment_wide_accept_is_not_narrowed() {
        // D9, spec §2.4: an accept naming a whole segment exempts every
        // leaf of it from the posture — including the ones the policy's
        // author never saw, which is the point of writing `MSH keep`
        // rather than a rule per field.
        let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000\rOBX|1|NM|2093-3||187";
        let mut message = er7::parse(text).unwrap();
        let policy = policy(&["MSH keep"]).posture(Posture::Reject(Action::redacted()));
        redact(policy, &mut message);
        assert_eq!(
            message.to_er7(),
            "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000\rOBX|REDACTED|REDACTED|REDACTED||REDACTED"
        );
    }

    #[test]
    fn reject_beats_accept_for_the_same_field() {
        // D19: a leaf named by an accept rule and a reject rule is a
        // policy somebody got wrong, and redacting it is the direction
        // that fails safely (spec §2.4, §1.5 priority 1). It does not
        // depend on the order the two were written in.
        for rules in [
            vec!["PID-5 keep", "PID-5 replace REDACTED"],
            vec!["PID-5 replace REDACTED", "PID-5 keep"],
        ] {
            let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH").unwrap();
            redact(policy(&rules), &mut message);
            assert_eq!(
                message.query("PID-5").unwrap().as_deref(),
                Some("REDACTED"),
                "{rules:?} let the name through"
            );
        }

        // And the accept still does its own job: exempting the position
        // from the posture, for every leaf no reject rule reached.
        let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN").unwrap();
        let policy = policy(&["MSH keep", "PID-5 keep", "PID-5.1 replace REDACTED"])
            .posture(Posture::Reject(Action::Clear));
        redact(policy, &mut message);
        assert_eq!(
            message.query("PID-5").unwrap().as_deref(),
            Some("REDACTED^JOHN")
        );
    }

    #[test]
    fn reject_segment_beats_a_narrower_accept() {
        // D19 across depths: a reject naming a whole segment beats an
        // accept naming one field inside it, and the other way round —
        // neither order carves the field back out (spec §2.4).
        for rules in [
            vec!["PID replace REDACTED", "PID-5 keep"],
            vec!["PID-5 keep", "PID replace REDACTED"],
        ] {
            let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH").unwrap();
            redact(policy(&rules), &mut message);
            assert_eq!(
                message.query("PID-5").unwrap().as_deref(),
                Some("REDACTED"),
                "{rules:?} carved the name out of a rejected segment"
            );
        }
    }

    #[test]
    fn an_unrecognised_payload_follows_the_policy() {
        // D21: a payload with no positions in it is the one thing rules
        // and the posture cannot speak to, so the policy says outright
        // what happens to it (spec §2.8).
        let junk = "{\"name\": \"EVERYWOMAN\"}";

        // The curated policies refuse: nothing is written, and the caller
        // decides what that costs.
        assert_eq!(
            Redactor::new(Policy::patient_identifiers()).unrecognised(junk),
            None
        );
        assert_eq!(
            Redactor::new(Policy::all_but_the_header()).unrecognised(junk),
            None
        );

        // The bare postures each do what their name claims.
        assert_eq!(
            Redactor::new(Policy::accept_all())
                .unrecognised(junk)
                .as_deref(),
            Some(junk)
        );
        let masked = Redactor::new(Policy::reject_all())
            .unrecognised(junk)
            .expect("reject_all writes something");
        assert_eq!(masked, "*".repeat(junk.chars().count()));
        assert!(!masked.contains("EVERYWOMAN"));

        // And every one of them is overridable, in either direction.
        let policy = Policy::patient_identifiers().on_unrecognised(Unrecognised::Pass);
        assert_eq!(
            Redactor::new(policy).unrecognised(junk).as_deref(),
            Some(junk)
        );
        let policy = Policy::accept_all().on_unrecognised(Unrecognised::Refuse);
        assert_eq!(Redactor::new(policy).unrecognised(junk), None);
        let policy = Policy::accept_all().on_unrecognised(Unrecognised::Apply(Action::Clear));
        assert_eq!(
            Redactor::new(policy).unrecognised(junk).as_deref(),
            Some("")
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

    #[test]
    fn uncovered_lists_every_position_no_rule_names() {
        // D22: the inverse of redaction — every leaf with text that no
        // rule in the policy names, regardless of what the posture would
        // eventually do about it (spec §2.9).
        let mut message =
            er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN\rNTE|1||free text").unwrap();
        let redactor = Redactor::new(policy(&["PID-5 replace REDACTED"]));
        let gaps: Vec<String> = redactor
            .uncovered(&message)
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        // Named by the rule: not a gap.
        assert!(!gaps.contains(&"PID[1]-5[1].1.1".to_string()));
        assert!(!gaps.contains(&"PID[1]-5[1].2.1".to_string()));
        // Named by nothing: a gap.
        assert!(gaps.contains(&"PID[1]-1[1].1.1".to_string()));
        assert!(gaps.contains(&"PID[1]-3[1].1.1".to_string()));
        assert!(gaps.contains(&"NTE[1]-3[1].1.1".to_string()));

        // Read-only: redact() still finds PID-5 to change afterward, so
        // uncovered() did not consume or alter anything.
        let report = redactor.redact(&mut message);
        assert!(!report.is_empty());
    }

    #[test]
    fn uncovered_ignores_empty_and_null_leaves() {
        // D3, D4: an empty or null leaf is not a gap — a rule would find
        // nothing to redact there either way (spec §2.9).
        let message = er7::parse("MSH|^~\\&|LAB\rPID|1||\"\"").unwrap();
        let redactor = Redactor::new(Policy::accept_all());
        let gaps: Vec<String> = redactor
            .uncovered(&message)
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        // PID-1 carries "1": a real gap under a policy naming nothing.
        assert!(gaps.contains(&"PID[1]-1[1].1.1".to_string()));
        // PID-2 is empty, and PID-3 is the explicit null: neither is one.
        assert!(!gaps.contains(&"PID[1]-2[1].1.1".to_string()));
        assert!(!gaps.contains(&"PID[1]-3[1].1.1".to_string()));
    }
}
