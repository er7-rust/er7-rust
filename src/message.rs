//! The ER7 value tree: message, segment, field, repetition, component,
//! subcomponent.
//!
//! Text is held exactly as it arrived — escape sequences and all — and
//! decoded on demand. That is what lets a message survive a round trip
//! through this crate unchanged, and it keeps the two questions a receiver
//! asks separate: what did the sender write (`raw`), and what does it mean
//! ([`Subcomponent::value`]).
//!
//! Specified by spec §5 (the tree), §8 (queries), and §10 (the MSH
//! accessors). A tutorial is in `docs/usage/index.md`.

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
///
/// Every field is `pub`, so a message can be built from literals as well as
/// parsed (spec §5.1). Nothing is memoized, which is what makes mutating
/// those fields safe: there is no cache to invalidate.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7::Error> {
/// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
///
/// // By path...
/// assert_eq!(message.query("PID-5.1")?.as_deref(), Some("SMITH"));
/// // ...or by walking the tree.
/// assert_eq!(message.segment("PID").unwrap().fields.len(), 5);
/// // And back out again, unchanged.
/// assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN");
/// # Ok(())
/// # }
/// ```
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
/// blank (R7, spec §4.4.1).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), er7::Error> {
/// let message = er7::parse("MSH|^~\\&|LAB\rPID||A~~B")?;
/// let pid = message.segment("PID").unwrap();
///
/// assert_eq!(pid.field(1).unwrap().repetitions.len(), 0);  // `||`
/// assert_eq!(pid.field(2).unwrap().repetitions.len(), 3);  // `A~~B`
/// # Ok(())
/// # }
/// ```
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
///
/// Text is stored exactly as it arrived and decoded on demand, which is
/// what makes the round-trip guarantee possible (R9, spec §5.2).
///
/// Example:
///
/// ```
/// use er7::{Separators, Subcomponent};
///
/// let separators = Separators::default();
/// let mut leaf = Subcomponent::new(r"Smith \T\ Jones");
///
/// assert_eq!(leaf.raw, r"Smith \T\ Jones");        // what the sender wrote
/// assert_eq!(leaf.value(&separators), "Smith & Jones"); // what it means
///
/// leaf.set("O'Brien & Sons", &separators);
/// assert_eq!(leaf.raw, r"O'Brien \T\ Sons");       // encoded on the way in
/// ```
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
    /// database write, it always does (spec §5.3).
    ///
    /// Example:
    ///
    /// ```
    /// use er7::{Separators, Subcomponent};
    ///
    /// let separators = Separators::default();
    ///
    /// assert_eq!(Subcomponent::new(r"a\T\b").value(&separators), "a&b");
    /// // A sequence with no plain-text meaning is kept as written.
    /// assert_eq!(Subcomponent::new(r"a\.br\b").value(&separators), r"a\.br\b");
    /// // The explicit null reads as empty; ask `is_null` to tell them apart.
    /// assert_eq!(Subcomponent::new(r#""""#).value(&separators), "");
    /// ```
    pub fn value(&self, separators: &Separators) -> Cow<'_, str> {
        if self.is_null() {
            return Cow::Borrowed("");
        }
        unescape(&self.raw, separators)
    }

    /// Replace the text with `value`, encoding any delimiters it contains.
    ///
    /// This is the recommended way to write a value. Assigning
    /// [`Subcomponent::raw`] directly is allowed, but then the escaping is
    /// yours to get right: an unescaped `&` would split the component in two
    /// the next time the message was parsed, shifting every value after it
    /// (spec §5.5).
    ///
    /// Example:
    ///
    /// ```
    /// use er7::{Separators, Subcomponent};
    ///
    /// let separators = Separators::default();
    /// let mut leaf = Subcomponent::default();
    ///
    /// leaf.set("Smith & Jones", &separators);
    /// assert_eq!(leaf.raw, r"Smith \T\ Jones");
    /// assert_eq!(leaf.value(&separators), "Smith & Jones");
    /// ```
    pub fn set(&mut self, value: &str, separators: &Separators) {
        self.raw = crate::escape::escape(value, separators).into_owned();
    }

    /// True when this is the explicit HL7 null `""`, meaning "the sender is
    /// clearing this value", not "the sender had nothing to say".
    ///
    /// Getting this wrong is a patient-safety bug: treating a null as empty
    /// leaves a withdrawn allergy on the record (spec §5.3, R10).
    ///
    /// Example:
    ///
    /// ```
    /// use er7::Subcomponent;
    ///
    /// let null = Subcomponent::new(r#""""#);
    /// assert!(null.is_null());
    /// assert!(!null.is_empty());   // the null is text, so never both
    ///
    /// let empty = Subcomponent::new("");
    /// assert!(empty.is_empty());
    /// assert!(!empty.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        self.raw == NULL
    }

    /// True when no text was sent here at all. The explicit null is text,
    /// so `is_empty` and [`Subcomponent::is_null`] are never both true —
    /// together they separate "nothing to say" from "clear this" (R11).
    ///
    /// See [`Subcomponent::is_null`] for a worked example of both.
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
    ///
    /// Index `0` returns `None` rather than element 1: HL7 numbering starts
    /// at 1, so a `0` is a caller's off-by-one and must not read as a valid
    /// position (spec §5.4).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9^^^ACME&1.2.3&ISO")?;
    /// let assigning = message.segment("PID").unwrap().component(3, 4).unwrap();
    ///
    /// assert_eq!(assigning.subcomponent(1).unwrap().raw, "ACME");
    /// assert_eq!(assigning.subcomponent(2).unwrap().raw, "1.2.3");
    /// assert!(assigning.subcomponent(9).is_none());
    /// assert!(assigning.subcomponent(0).is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See also [`Component::subcomponent_mut`] to edit one.
    pub fn subcomponent(&self, n: usize) -> Option<&Subcomponent> {
        self.subcomponents.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based subcomponent `n`, for editing it with
    /// [`Subcomponent::set`].
    pub fn subcomponent_mut(&mut self, n: usize) -> Option<&mut Subcomponent> {
        self.subcomponents.get_mut(n.checked_sub(1)?)
    }

    /// True when every subcomponent is empty. See [`Component::is_null`]
    /// for the distinction that matters.
    pub fn is_empty(&self) -> bool {
        self.subcomponents.iter().all(Subcomponent::is_empty)
    }

    /// True when this component is exactly the explicit null `""` — one
    /// subcomponent, and that subcomponent null.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|\"\"^X")?;
    /// let repetition = message.segment("PID").unwrap()
    ///     .field(1).unwrap().repetition(1).unwrap();
    ///
    /// assert!(repetition.component(1).unwrap().is_null());
    /// assert!(!repetition.component(2).unwrap().is_null());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self.subcomponents.as_slice(), [only] if only.is_null())
    }
}

impl Repetition {
    /// The 1-based component `n`, if the message carried one.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
    /// let name = message.segment("PID").unwrap()
    ///     .field(5).unwrap().repetition(1).unwrap();
    ///
    /// assert_eq!(name.component(1).unwrap().to_er7(&message.separators), "SMITH");
    /// assert_eq!(name.component(2).unwrap().to_er7(&message.separators), "JOHN");
    /// assert!(name.component(3).is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn component(&self, n: usize) -> Option<&Component> {
        self.components.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based component `n`.
    pub fn component_mut(&mut self, n: usize) -> Option<&mut Component> {
        self.components.get_mut(n.checked_sub(1)?)
    }

    /// True when every component is empty. See [`Repetition::is_null`] for
    /// the distinction that matters.
    pub fn is_empty(&self) -> bool {
        self.components.iter().all(Component::is_empty)
    }

    /// True when this repetition is exactly the explicit null `""` — one
    /// component, and that component null.
    pub fn is_null(&self) -> bool {
        matches!(self.components.as_slice(), [only] if only.is_null())
    }
}

impl Field {
    /// The 1-based repetition `n`, if the message carried one.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|555-1111~555-2222")?;
    /// let phones = message.segment("PID").unwrap().field(1).unwrap();
    ///
    /// assert_eq!(phones.repetitions.len(), 2);
    /// assert_eq!(phones.repetition(2).unwrap().to_er7(&message.separators), "555-2222");
    /// assert!(phones.repetition(3).is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn repetition(&self, n: usize) -> Option<&Repetition> {
        self.repetitions.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based repetition `n`.
    pub fn repetition_mut(&mut self, n: usize) -> Option<&mut Repetition> {
        self.repetitions.get_mut(n.checked_sub(1)?)
    }

    /// The 1-based component `n` of the first repetition — the common case,
    /// since most fields do not repeat.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|SMITH^JOHN")?;
    /// let name = message.segment("PID").unwrap().field(1).unwrap();
    ///
    /// assert_eq!(name.component(1).unwrap().to_er7(&message.separators), "SMITH");
    /// # Ok(())
    /// # }
    /// ```
    pub fn component(&self, n: usize) -> Option<&Component> {
        self.repetitions.first()?.component(n)
    }

    /// True when the field was sent with no repetitions, or with only empty
    /// ones. A field holding the explicit null is not empty; see
    /// [`Field::is_null`].
    pub fn is_empty(&self) -> bool {
        self.repetitions.iter().all(Repetition::is_empty)
    }

    /// True when this field is exactly the explicit null `""`, meaning the
    /// sender is clearing the stored value.
    ///
    /// This is the level the distinction is usually asked at, and getting
    /// it wrong is a patient-safety bug: an absent or empty field means
    /// "leave the stored value alone", while a null means "clear it"
    /// (R10, R11, spec §5.3).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||\"\"|X")?;
    /// let pid = message.segment("PID").unwrap();
    ///
    /// // Absent: never sent.
    /// assert!(pid.field(9).is_none());
    /// // Empty: sent as `||`, no value.
    /// assert!(pid.field(2).unwrap().is_empty() && !pid.field(2).unwrap().is_null());
    /// // Null: sent as `""`, clear the stored value.
    /// assert!(pid.field(3).unwrap().is_null() && !pid.field(3).unwrap().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self.repetitions.as_slice(), [only] if only.is_null())
    }
}

impl Segment {
    /// The 1-based field `n`, if the segment carried one.
    ///
    /// For a header segment this follows HL7's own numbering, so
    /// `msh.field(1)` is the field separator and `msh.field(9)` is the
    /// message type (R8, spec §4.4.2).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB|ACME|||||ADT^A08|MSG1|P|2.5")?;
    /// let msh = message.header().unwrap();
    /// let separators = &message.separators;
    ///
    /// assert_eq!(msh.field(1).unwrap().to_er7(separators), "|");      // MSH-1
    /// assert_eq!(msh.field(2).unwrap().to_er7(separators), r"^~\&");  // MSH-2
    /// assert_eq!(msh.field(3).unwrap().to_er7(separators), "LAB");
    /// assert_eq!(msh.field(9).unwrap().to_er7(separators), "ADT^A08");
    /// assert!(msh.field(99).is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn field(&self, n: usize) -> Option<&Field> {
        self.fields.get(n.checked_sub(1)?)
    }

    /// Mutable access to the 1-based field `n`.
    pub fn field_mut(&mut self, n: usize) -> Option<&mut Field> {
        self.fields.get_mut(n.checked_sub(1)?)
    }

    /// The 1-based component `component` of the 1-based field `field`,
    /// taking the first repetition — the shape most lookups want.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
    /// let pid = message.segment("PID").unwrap();
    ///
    /// assert_eq!(pid.component(5, 2).unwrap().to_text(&message.separators), "JOHN");
    /// # Ok(())
    /// # }
    /// ```
    pub fn component(&self, field: usize, component: usize) -> Option<&Component> {
        self.field(field)?.component(component)
    }

    /// True when this segment declares the message's delimiters, i.e. it is
    /// an `MSH` header or one of the `FHS`/`BHS` batch headers. Fields 1
    /// and 2 of such a segment are the delimiters themselves and are never
    /// split or escape-decoded.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1")?;
    /// assert!(message.segment("MSH").unwrap().is_header());
    /// assert!(!message.segment("PID").unwrap().is_header());
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rOBX|1\rOBX|2\rNTE|1")?;
    ///
    /// assert_eq!(message.segments_named("OBX").count(), 2);
    /// assert_eq!(message.segments_named("ZZZ").count(), 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn segments_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Segment> {
        self.segments.iter().filter(move |s| s.name == name)
    }

    /// The first segment with this name.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1\rOBX|1\rOBX|2")?;
    ///
    /// assert_eq!(message.segment("OBX").unwrap().field(1).unwrap()
    ///            .to_er7(&message.separators), "1");
    /// assert!(message.segment("ZZZ").is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn segment(&self, name: &str) -> Option<&Segment> {
        self.segments.iter().find(|s| s.name == name)
    }

    /// The 1-based `occurrence`th segment with this name, e.g. the second
    /// `OBX` of a result.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rOBX|1\rOBX|2")?;
    ///
    /// let second = message.segment_at("OBX", 2).unwrap();
    /// assert_eq!(second.field(1).unwrap().to_er7(&message.separators), "2");
    /// assert!(message.segment_at("OBX", 3).is_none());
    /// # Ok(())
    /// # }
    /// ```
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
    /// delimiters. A parsed message always has one; a message built by hand
    /// might not, hence the `Option`.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1")?;
    /// assert_eq!(message.header().unwrap().name, "MSH");
    ///
    /// // A batch envelope header works the same way.
    /// let batch = er7::parse("BHS|^~\\&|SENDER")?;
    /// assert_eq!(batch.header().unwrap().name, "BHS");
    /// # Ok(())
    /// # }
    /// ```
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
    /// A position the message does not carry gives `None` rather than an
    /// error or an empty string (R20, spec §8.2). The `Err` case is only a
    /// malformed path.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
    ///
    /// assert_eq!(message.query("PID-5")?.as_deref(), Some("SMITH^JOHN"));
    /// assert_eq!(message.query("PID-5.2")?.as_deref(), Some("JOHN"));
    /// assert_eq!(message.query("PID-99")?, None);
    /// assert_eq!(message.query("ZZZ-1")?, None);
    /// assert!(message.query("PID-0").is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn query(&self, path: &str) -> Result<Option<String>, Error> {
        Ok(self.query_path(&Path::parse(path)?).into_iter().next())
    }

    /// Every value matching `path`, in message order: one per matching
    /// segment, and one per repetition where the path does not pin one
    /// down. Repeated `OBX-5` across a result is the motivating case.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse(
    ///     "MSH|^~\\&|LAB\r\
    ///      PID|555-1111~555-2222\r\
    ///      OBX|1|NM|2093-3^Cholesterol^LN||187\r\
    ///      OBX|2|NM|2571-8^Triglycerides^LN||102\r\
    ///      OBX|3|ST|X^Note^L",
    /// )?;
    ///
    /// // One value per matching segment.
    /// assert_eq!(message.query_all("OBX-3.2")?, ["Cholesterol", "Triglycerides", "Note"]);
    /// // The third OBX carried no fifth field, so it contributes nothing.
    /// assert_eq!(message.query_all("OBX-5")?, ["187", "102"]);
    /// // Stopping at the field keeps the repetition separator...
    /// assert_eq!(message.query_all("PID-1")?, ["555-1111~555-2222"]);
    /// // ...going deeper splits it.
    /// assert_eq!(message.query_all("PID-1.1")?, ["555-1111", "555-2222"]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_all(&self, path: &str) -> Result<Vec<String>, Error> {
        Ok(self.query_path(&Path::parse(path)?))
    }

    /// [`Message::query_all`] against an already-parsed [`Path`], which
    /// saves re-parsing when the same path is applied to many messages.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let path: er7::Path = "PID-5.1".parse()?;
    /// let messages = [
    ///     er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?,
    ///     er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|JONES^MARY")?,
    /// ];
    ///
    /// let names: Vec<String> = messages
    ///     .iter()
    ///     .flat_map(|message| message.query_path(&path))
    ///     .collect();
    /// assert_eq!(names, ["SMITH", "JONES"]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_path(&self, path: &Path) -> Vec<String> {
        self.query_path_mode(path, true)
    }

    /// [`Message::query_path`] without decoding: every value comes back
    /// exactly as the sender wrote it, escape sequences included.
    ///
    /// Use this when you are going to put the text back into a message
    /// rather than read it, or when you need to tell an explicit null from
    /// an empty value without reaching for the node (spec §8.2.1).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse("MSH|^~\\&|LAB\rPID|Smith \\T\\ Jones|\"\"|")?;
    ///
    /// let name: er7::Path = "PID-1".parse()?;
    /// assert_eq!(message.query_path(&name), ["Smith & Jones"]);
    /// assert_eq!(message.query_path_raw(&name), [r"Smith \T\ Jones"]);
    ///
    /// // Decoded, a null and an empty field look the same; raw, they do not.
    /// let null: er7::Path = "PID-2".parse()?;
    /// let empty: er7::Path = "PID-3".parse()?;
    /// assert_eq!(message.query_path(&null), message.query_path(&empty));
    /// assert_eq!(message.query_path_raw(&null), [r#""""#]);
    /// assert_eq!(message.query_path_raw(&empty), [""]);
    /// # Ok(())
    /// # }
    /// ```
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
    /// crate knows, and they are the documented exception to R24. They earn
    /// their place on two grounds that both have to hold: every tool that
    /// touches a message needs them to route or log it, and these positions
    /// have not moved in any v2 release, so reading them requires no
    /// version knowledge (spec §10.2).
    ///
    /// Each returns `None` when the position is absent *or* empty, because
    /// for these five "sent blank" and "not sent" mean the same thing to a
    /// caller (R22).
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7::Error> {
    /// let message = er7::parse(
    ///     "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ADT^A08^ADT_A01|MSG9|P|2.5",
    /// )?;
    ///
    /// assert_eq!(message.message_code().as_deref(), Some("ADT"));
    /// assert_eq!(message.trigger_event().as_deref(), Some("A08"));
    /// assert_eq!(message.message_structure().as_deref(), Some("ADT_A01"));
    /// assert_eq!(message.control_id().as_deref(), Some("MSG9"));
    /// assert_eq!(message.version().as_deref(), Some("2.5"));
    ///
    /// // Absent or empty both read as None.
    /// let sparse = er7::parse("MSH|^~\\&|LAB")?;
    /// assert_eq!(sparse.message_code(), None);
    /// assert_eq!(sparse.control_id(), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn message_code(&self) -> Option<String> {
        self.first_value("MSH-9.1")
    }

    /// MSH-9.2, the trigger event, e.g. `A08` — what happened that caused
    /// the message. See [`Message::message_code`] for an example.
    pub fn trigger_event(&self) -> Option<String> {
        self.first_value("MSH-9.2")
    }

    /// MSH-9.3, the message structure, e.g. `ADT_A01` — which segments the
    /// message may hold.
    ///
    /// Older senders often omit it, in which case the structure has to be
    /// derived from the code and trigger event. This crate deliberately
    /// does not do that: the mapping differs between HL7 versions, and a
    /// wrong answer routes a message to the wrong handler (spec §10.3).
    /// Derive it in the dictionary layer that knows the version.
    ///
    /// See [`Message::message_code`] for an example.
    pub fn message_structure(&self) -> Option<String> {
        self.first_value("MSH-9.3")
    }

    /// MSH-10, the message control ID: the sender's unique identifier for
    /// this message, and what an acknowledgement quotes back in MSA-2.
    ///
    /// See [`Message::message_code`] for an example.
    pub fn control_id(&self) -> Option<String> {
        self.first_value("MSH-10")
    }

    /// MSH-12.1, the HL7 version ID, e.g. `2.5`.
    ///
    /// This reads the first component rather than the whole field, because
    /// a v2.5.1-and-later sender may write `2.5.1^AUS^2.5.1` and only the
    /// first component is the version ID (spec §10.1).
    ///
    /// See [`Message::message_code`] for an example.
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
    fn a_missing_position_yields_no_value() {
        // R20: a position the message does not have contributes nothing —
        // not an error, and not an empty string. So the count of results
        // reflects the positions that were actually carried.
        let message = message();
        assert_eq!(message.query("PID-99").unwrap(), None);
        assert_eq!(message.query("PID-5.9").unwrap(), None);
        assert_eq!(message.query("PID-5.1.9").unwrap(), None);
        assert_eq!(message.query("ZZZ-1").unwrap(), None);
        assert_eq!(message.query("OBX[9]-1").unwrap(), None);
        assert!(message.query_all("PID-99").unwrap().is_empty());
        // Two OBX segments, but only one carried a sixth field.
        assert_eq!(message.segments_named("OBX").count(), 2);
        assert_eq!(message.query_all("OBX-6").unwrap(), vec!["mg/dL"]);
    }

    #[test]
    fn msh_conveniences_are_none_when_absent() {
        // R22: absent and empty both read as None, because for these five
        // fields "sent blank" and "not sent" mean the same to a caller.
        let absent = parse("MSH|^~\\&|LAB").unwrap();
        assert_eq!(absent.message_code(), None);
        assert_eq!(absent.trigger_event(), None);
        assert_eq!(absent.message_structure(), None);
        assert_eq!(absent.control_id(), None);
        assert_eq!(absent.version(), None);

        // Present but empty reads the same way.
        let empty = parse("MSH|^~\\&|LAB|||||||||").unwrap();
        assert_eq!(empty.message_code(), None);
        assert_eq!(empty.control_id(), None);

        // A trigger event without a structure gives two of three, which is
        // the common shape from a pre-2.3.1 sender.
        let partial = parse("MSH|^~\\&|LAB||||||ADT^A08|MSG1|P|2.3").unwrap();
        assert_eq!(partial.message_code().as_deref(), Some("ADT"));
        assert_eq!(partial.trigger_event().as_deref(), Some("A08"));
        assert_eq!(partial.message_structure(), None);
        assert_eq!(partial.version().as_deref(), Some("2.3"));

        // A versioned MSH-12 reports only its first component.
        let versioned = parse("MSH|^~\\&|LAB|||||||||2.5.1^AUS^2.5.1").unwrap();
        assert_eq!(versioned.version().as_deref(), Some("2.5.1"));
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
