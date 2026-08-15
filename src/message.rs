//! The ER7 value tree: message, segment, field, repetition, component,
//! subcomponent.
//!
//! Text is held exactly as it arrived — escape sequences and all — and
//! decoded on demand. That is what lets a message survive a round trip
//! through this crate unchanged, and it keeps the two questions a receiver
//! asks separate: what did the sender write (`raw`), and what does it mean
//! ([`Subcomponent::value`]).

use crate::escape::unescape;
use crate::{Error, Path, Separators};
use std::borrow::Cow;

/// The literal text HL7 uses for an explicit null: a value the sender is
/// deliberately clearing, as opposed to one they simply did not send.
pub const NULL: &str = "\"\"";

/// One ER7 message: the delimiters it declared, and its segments in order.
///
/// The first segment is the header (`MSH`, or `FHS`/`BHS` in a batch) and
/// is where [`Message::separators`] came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The delimiter set this message declared in MSH-1 and MSH-2.
    pub separators: Separators,
    /// Every segment, in the order it appeared.
    pub segments: Vec<Segment>,
}

/// One segment: a three-character name and its fields.
///
/// Fields are stored 1-based-in-spirit: `fields[0]` is field 1. For a
/// header segment, field 1 is the field separator itself and field 2 is the
/// encoding characters, exactly as HL7 numbers them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    /// The segment name, e.g. `PID`. Names beginning with `Z` are local
    /// extensions; this crate treats them like any other.
    pub name: String,
    /// The fields, in order; `fields[n - 1]` is field `n`.
    pub fields: Vec<Field>,
}

/// One field: its repetitions, split on the repetition separator.
///
/// A field that was sent empty (`||`) has no repetitions at all, which is
/// how "not sent" is distinguished from a repetition that is present but
/// blank.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Field {
    /// The repetitions, in order.
    pub repetitions: Vec<Repetition>,
}

/// One repetition of a field: its components, split on the component
/// separator.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Repetition {
    /// The components, in order.
    pub components: Vec<Component>,
}

/// One component: its subcomponents, split on the subcomponent separator.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Component {
    /// The subcomponents, in order.
    pub subcomponents: Vec<Subcomponent>,
}

/// One subcomponent: the leaf of the tree, and the only place text lives.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Subcomponent {
    /// The text exactly as it appeared in the message, escape sequences
    /// included. Use [`Subcomponent::value`] to read it decoded, and
    /// [`Subcomponent::set`] to write a value that needs encoding.
    pub raw: String,
}

impl Subcomponent {
    /// Wrap already-encoded ER7 text. The text must not contain unescaped
    /// delimiters; [`Subcomponent::set`] handles that for you.
    pub fn new(raw: impl Into<String>) -> Subcomponent {
        Subcomponent { raw: raw.into() }
    }

    /// The decoded text: escape sequences that stand for characters are
    /// resolved, and the explicit null `""` reads as the empty string.
    ///
    /// Because null and empty both read as empty here, ask
    /// [`Subcomponent::is_null`] when the difference matters — for a
    /// database write, it usually does.
    pub fn value(&self, separators: &Separators) -> Cow<'_, str> {
        if self.is_null() {
            return Cow::Borrowed("");
        }
        unescape(&self.raw, separators)
    }

    /// Replace the text with `value`, encoding any delimiters it contains.
    pub fn set(&mut self, value: &str, separators: &Separators) {
        self.raw = crate::escape::escape(value, separators).into_owned();
    }

    /// True when this is the explicit HL7 null `""`, meaning "the sender is
    /// clearing this value", not "the sender had nothing to say".
    pub fn is_null(&self) -> bool {
        self.raw == NULL
    }

    /// True when no text was sent here at all. The explicit null is text,
    /// so `is_empty` and [`Subcomponent::is_null`] are never both true —
    /// together they separate "nothing to say" from "clear this".
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
}

impl From<&str> for Subcomponent {
    fn from(raw: &str) -> Subcomponent {
        Subcomponent::new(raw)
    }
}

impl Component {
    /// The 1-based subcomponent `n`, if the message carried one.
    pub fn subcomponent(&self, n: usize) -> Option<&Subcomponent> {
        self.subcomponents.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based subcomponent `n`.
    pub fn subcomponent_mut(&mut self, n: usize) -> Option<&mut Subcomponent> {
        self.subcomponents.get_mut(n.checked_sub(1)?)
    }

    /// True when every subcomponent is empty.
    pub fn is_empty(&self) -> bool {
        self.subcomponents.iter().all(Subcomponent::is_empty)
    }

    /// True when this component is exactly the explicit null `""`.
    pub fn is_null(&self) -> bool {
        matches!(self.subcomponents.as_slice(), [only] if only.is_null())
    }
}

impl Repetition {
    /// The 1-based component `n`, if the message carried one.
    pub fn component(&self, n: usize) -> Option<&Component> {
        self.components.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based component `n`.
    pub fn component_mut(&mut self, n: usize) -> Option<&mut Component> {
        self.components.get_mut(n.checked_sub(1)?)
    }

    /// True when every component is empty.
    pub fn is_empty(&self) -> bool {
        self.components.iter().all(Component::is_empty)
    }

    /// True when this repetition is exactly the explicit null `""`.
    pub fn is_null(&self) -> bool {
        matches!(self.components.as_slice(), [only] if only.is_null())
    }
}

impl Field {
    /// The 1-based repetition `n`, if the message carried one.
    pub fn repetition(&self, n: usize) -> Option<&Repetition> {
        self.repetitions.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based repetition `n`.
    pub fn repetition_mut(&mut self, n: usize) -> Option<&mut Repetition> {
        self.repetitions.get_mut(n.checked_sub(1)?)
    }

    /// The 1-based component `n` of the first repetition — the common case,
    /// since most fields do not repeat.
    pub fn component(&self, n: usize) -> Option<&Component> {
        self.repetitions.first()?.component(n)
    }

    /// True when the field was sent with no repetitions, or with only empty
    /// ones. A field holding the explicit null is not empty; see
    /// [`Field::is_null`].
    pub fn is_empty(&self) -> bool {
        self.repetitions.iter().all(Repetition::is_empty)
    }

    /// True when this field is exactly the explicit null `""`.
    pub fn is_null(&self) -> bool {
        matches!(self.repetitions.as_slice(), [only] if only.is_null())
    }
}

impl Segment {
    /// The 1-based field `n`, if the segment carried one.
    ///
    /// For a header segment this follows HL7's own numbering, so
    /// `msh.field(1)` is the field separator and `msh.field(9)` is the
    /// message type.
    pub fn field(&self, n: usize) -> Option<&Field> {
        self.fields.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based field `n`.
    pub fn field_mut(&mut self, n: usize) -> Option<&mut Field> {
        self.fields.get_mut(n.checked_sub(1)?)
    }

    /// The 1-based component `component` of the 1-based field `field`,
    /// taking the first repetition — the shape most lookups want.
    pub fn component(&self, field: usize, component: usize) -> Option<&Component> {
        self.field(field)?.component(component)
    }

    /// True when this segment declares the message's delimiters, i.e. it is
    /// an `MSH` header or one of the `FHS`/`BHS` batch headers. Fields 1
    /// and 2 of such a segment are the delimiters themselves and are never
    /// split or escape-decoded.
    pub fn is_header(&self) -> bool {
        is_header_name(&self.name)
    }
}

/// True for the three segment names that declare a delimiter set.
pub(crate) fn is_header_name(name: &str) -> bool {
    matches!(name, "MSH" | "FHS" | "BHS")
}

/// Pick between a node's `to_er7` and its `to_text`, so that a query can be
/// answered either as sent or decoded without writing the descent twice.
fn write<T>(
    node: &T,
    separators: &Separators,
    decode: bool,
    to_er7: fn(&T, &Separators) -> String,
    to_text: fn(&T, &Separators) -> String,
) -> String {
    if decode {
        to_text(node, separators)
    } else {
        to_er7(node, separators)
    }
}

impl Message {
    /// Every segment with this name, in message order.
    pub fn segments_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Segment> {
        self.segments.iter().filter(move |s| s.name == name)
    }

    /// The first segment with this name.
    pub fn segment(&self, name: &str) -> Option<&Segment> {
        self.segments.iter().find(|s| s.name == name)
    }

    /// The 1-based `occurrence`th segment with this name, e.g. the second
    /// `OBX` of a result.
    pub fn segment_at(&self, name: &str, occurrence: usize) -> Option<&Segment> {
        self.segments
            .iter()
            .filter(|s| s.name == name)
            .nth(occurrence.checked_sub(1)?)
    }

    /// Mutable access to the 1-based `occurrence`th segment with this name.
    pub fn segment_at_mut(&mut self, name: &str, occurrence: usize) -> Option<&mut Segment> {
        self.segments
            .iter_mut()
            .filter(|s| s.name == name)
            .nth(occurrence.checked_sub(1)?)
    }

    /// The header segment — the first segment, which declared the
    /// delimiters. A parsed message always has one.
    pub fn header(&self) -> Option<&Segment> {
        self.segments.first()
    }

    /// The decoded text at `path`, or `None` if the message has nothing
    /// there.
    ///
    /// A path that names a level above a subcomponent returns that whole
    /// subtree written back as ER7, with only the leaf text decoded — so
    /// `PID-5` on `SMITH^JOHN` gives `SMITH^JOHN` and `PID-5.1` gives
    /// `SMITH`. Where the path leaves an occurrence open, the first is
    /// taken; use [`Message::query_all`] to get them all.
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
    /// assert_eq!(message.query("PID-5")?.as_deref(), Some("SMITH^JOHN"));
    /// assert_eq!(message.query("PID-5.2")?.as_deref(), Some("JOHN"));
    /// assert_eq!(message.query("PID-99")?, None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query(&self, path: &str) -> Result<Option<String>, Error> {
        Ok(self.query_path(&Path::parse(path)?).into_iter().next())
    }

    /// Every value matching `path`, in message order: one per matching
    /// segment, and one per repetition where the path does not pin one
    /// down. Repeated `OBX-5` across a result is the motivating case.
    pub fn query_all(&self, path: &str) -> Result<Vec<String>, Error> {
        Ok(self.query_path(&Path::parse(path)?))
    }

    /// [`Message::query_all`] against an already-parsed [`Path`], which
    /// saves re-parsing when the same path is applied to many messages.
    pub fn query_path(&self, path: &Path) -> Vec<String> {
        self.query_path_mode(path, true)
    }

    /// [`Message::query_path`] without decoding: every value comes back
    /// exactly as the sender wrote it, escape sequences included. Use this
    /// when you are going to put the text back into a message rather than
    /// read it.
    pub fn query_path_raw(&self, path: &Path) -> Vec<String> {
        self.query_path_mode(path, false)
    }

    fn query_path_mode(&self, path: &Path, decode: bool) -> Vec<String> {
        let separators = &self.separators;
        let mut out = Vec::new();
        let segments: Vec<&Segment> = match path.segment_occurrence {
            Some(occurrence) => self
                .segment_at(&path.segment, occurrence)
                .into_iter()
                .collect(),
            None => self.segments_named(&path.segment).collect(),
        };
        for segment in segments {
            let Some(number) = path.field else {
                out.push(write(
                    segment,
                    separators,
                    decode,
                    Segment::to_er7,
                    Segment::to_text,
                ));
                continue;
            };
            let Some(field) = segment.field(number) else {
                continue;
            };
            // A header segment's first two fields are the delimiters, which
            // are structure rather than data and so are never decoded.
            if segment.is_header() && number <= 2 {
                out.push(field.to_er7(separators));
                continue;
            }
            if path.repetition.is_none() && path.component.is_none() {
                out.push(write(
                    field,
                    separators,
                    decode,
                    Field::to_er7,
                    Field::to_text,
                ));
                continue;
            }
            let repetitions: Vec<&Repetition> = match path.repetition {
                Some(n) => field.repetition(n).into_iter().collect(),
                None => field.repetitions.iter().collect(),
            };
            for repetition in repetitions {
                self.push_below_repetition(&mut out, repetition, path, decode);
            }
        }
        out
    }

    /// Descend from a repetition into the component and subcomponent the
    /// path asks for, appending whatever it lands on.
    fn push_below_repetition(
        &self,
        out: &mut Vec<String>,
        repetition: &Repetition,
        path: &Path,
        decode: bool,
    ) {
        let separators = &self.separators;
        let Some(number) = path.component else {
            out.push(write(
                repetition,
                separators,
                decode,
                Repetition::to_er7,
                Repetition::to_text,
            ));
            return;
        };
        let Some(component) = repetition.component(number) else {
            return;
        };
        let Some(number) = path.subcomponent else {
            out.push(write(
                component,
                separators,
                decode,
                Component::to_er7,
                Component::to_text,
            ));
            return;
        };
        if let Some(subcomponent) = component.subcomponent(number) {
            out.push(match decode {
                true => subcomponent.value(separators).into_owned(),
                false => subcomponent.raw.clone(),
            });
        }
    }

    /// MSH-9.1, the message code, e.g. `ADT`.
    ///
    /// This and the four accessors below are the only HL7 semantics this
    /// crate knows. They earn their place because every tool that touches a
    /// message needs them to route or log it, and because MSH's first
    /// twelve field positions have been stable across every v2 release.
    pub fn message_code(&self) -> Option<String> {
        self.first_value("MSH-9.1")
    }

    /// MSH-9.2, the trigger event, e.g. `A08`.
    pub fn trigger_event(&self) -> Option<String> {
        self.first_value("MSH-9.2")
    }

    /// MSH-9.3, the message structure, e.g. `ADT_A01`. Older senders often
    /// omit it, in which case the structure has to be derived from the code
    /// and trigger event — a mapping this crate deliberately leaves to
    /// callers, since it is version-specific.
    pub fn message_structure(&self) -> Option<String> {
        self.first_value("MSH-9.3")
    }

    /// MSH-10, the message control ID: the sender's unique identifier for
    /// this message, and what an acknowledgement quotes back.
    pub fn control_id(&self) -> Option<String> {
        self.first_value("MSH-10")
    }

    /// MSH-12, the HL7 version ID, e.g. `2.5`.
    pub fn version(&self) -> Option<String> {
        self.first_value("MSH-12.1")
    }

    /// The first value at a path known to be well-formed, dropping empties
    /// so that a field the sender left blank reads as absent.
    fn first_value(&self, path: &str) -> Option<String> {
        let path = Path::parse(path).expect("path literal is well-formed");
        self.query_path(&path)
            .into_iter()
            .next()
            .filter(|value| !value.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const ADT: &str = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ADT^A08^ADT_A01|MSG9|P|2.5\r\
                       PID|1||12345^^^ACME&1.2.3&ISO^MR||SMITH^JOHN^Q||19800101|M|||||\
                       555-1111~555-2222\r\
                       OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL\r\
                       OBX|2|ST|X^Note^L||\"\"";

    fn message() -> Message {
        parse(ADT).unwrap()
    }

    #[test]
    fn reads_the_msh_conveniences() {
        let message = message();
        assert_eq!(message.message_code().as_deref(), Some("ADT"));
        assert_eq!(message.trigger_event().as_deref(), Some("A08"));
        assert_eq!(message.message_structure().as_deref(), Some("ADT_A01"));
        assert_eq!(message.control_id().as_deref(), Some("MSG9"));
        assert_eq!(message.version().as_deref(), Some("2.5"));
    }

    #[test]
    fn queries_each_depth() {
        let message = message();
        assert_eq!(
            message.query("PID-5").unwrap().as_deref(),
            Some("SMITH^JOHN^Q")
        );
        assert_eq!(message.query("PID-5.1").unwrap().as_deref(), Some("SMITH"));
        assert_eq!(
            message.query("PID-3.4.2").unwrap().as_deref(),
            Some("1.2.3")
        );
        assert_eq!(message.query("PID-99").unwrap(), None);
        assert_eq!(message.query("ZZZ-1").unwrap(), None);
    }

    #[test]
    fn queries_repetitions_and_occurrences() {
        let message = message();
        assert_eq!(
            message.query_all("PID-13").unwrap(),
            vec!["555-1111~555-2222"]
        );
        assert_eq!(message.query_all("PID-13[2].1").unwrap(), vec!["555-2222"]);
        assert_eq!(message.query_all("OBX-5").unwrap(), vec!["187", ""]);
        assert_eq!(
            message.query_all("OBX[1]-3.2").unwrap(),
            vec!["Cholesterol"]
        );
    }

    #[test]
    fn queries_header_delimiters_literally() {
        let message = message();
        assert_eq!(message.query("MSH-1").unwrap().as_deref(), Some("|"));
        assert_eq!(message.query("MSH-2").unwrap().as_deref(), Some("^~\\&"));
    }

    #[test]
    fn distinguishes_absent_empty_and_null() {
        let message = message();
        let pid = message.segment("PID").unwrap();
        // Field 2 was sent as `||`: present in the message, but with no value.
        assert!(pid.field(2).unwrap().is_empty());
        assert!(!pid.field(2).unwrap().is_null());
        // Field 99 was never sent at all.
        assert!(pid.field(99).is_none());
        // OBX-5 of the second OBX is the explicit null: text was sent, and
        // it says "clear this", so it is null rather than empty.
        let obx = message.segment_at("OBX", 2).unwrap();
        assert!(obx.field(5).unwrap().is_null());
        assert!(!obx.field(5).unwrap().is_empty());
    }

    #[test]
    fn sets_a_value_with_encoding() {
        let separators = Separators::default();
        let mut subcomponent = Subcomponent::default();
        subcomponent.set("Smith & Jones", &separators);
        assert_eq!(subcomponent.raw, r"Smith \T\ Jones");
        assert_eq!(subcomponent.value(&separators), "Smith & Jones");
    }
}
