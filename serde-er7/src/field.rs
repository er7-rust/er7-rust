//! [`Field`]: a sequence of [`Repetition`]s, serialized as an array.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Repetition;

/// A Serde-enabled [`er7::Field`].
///
/// A field is nothing but its repetitions in order
/// ([`er7::Field::repetitions`]), so — like [`Repetition`] one level down —
/// it serializes as a plain array: `555-1111~555-2222` becomes
/// `[["555-1111"], ["555-2222"]]`. A field that was sent empty (`||`) has
/// no repetitions at all, and serializes as `[]`; this is what
/// distinguishes it from a present-but-empty repetition ([`er7::Field`]
/// spec).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Field;
///
/// let message = er7::parse(r"MSH|^~\&|LAB|555-1111~555-2222")?;
/// let field = message.segment("MSH").unwrap().field(4).unwrap().clone();
///
/// let json = serde_json::to_string(&Field(field))?;
/// let back: Field = serde_json::from_str(&json)?;
/// assert_eq!(back.to_er7(&message.separators), "555-1111~555-2222");
///
/// // Absent (`||`) round-trips as an empty array, not `[[]]`.
/// let empty: Field = serde_json::from_str("[]")?;
/// assert!(empty.repetitions.is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Field(pub er7::Field);

impl From<er7::Field> for Field {
    fn from(inner: er7::Field) -> Field {
        Field(inner)
    }
}

impl From<Field> for er7::Field {
    fn from(outer: Field) -> er7::Field {
        outer.0
    }
}

impl Deref for Field {
    type Target = er7::Field;

    fn deref(&self) -> &er7::Field {
        &self.0
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut er7::Field {
        &mut self.0
    }
}

impl Serialize for Field {
    /// Write each repetition as one array element. See
    /// [`Component::serialize`](crate::Component#impl) for the cost of the
    /// clone this involves.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let repetitions = &self.0.repetitions;
        let mut seq = serializer.serialize_seq(Some(repetitions.len()))?;
        for repetition in repetitions {
            seq.serialize_element(&Repetition(repetition.clone()))?;
        }
        seq.end()
    }
}

struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array of repetitions, each an array of components")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Field, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut repetitions = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(repetition) = seq.next_element::<Repetition>()? {
            repetitions.push(repetition.0);
        }
        Ok(Field(er7::Field { repetitions }))
    }
}

impl<'de> Deserialize<'de> for Field {
    /// Read an array of repetitions into `repetitions`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(FieldVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_repeated_values() {
        let field = Field(er7::Field {
            repetitions: vec![
                er7::Repetition {
                    components: vec![er7::Component {
                        subcomponents: vec![er7::Subcomponent::new("555-1111")],
                    }],
                },
                er7::Repetition {
                    components: vec![er7::Component {
                        subcomponents: vec![er7::Subcomponent::new("555-2222")],
                    }],
                },
            ],
        });
        let json = serde_json::to_string(&field).unwrap();
        assert_eq!(json, r#"[[["555-1111"]],[["555-2222"]]]"#);
        let back: Field = serde_json::from_str(&json).unwrap();
        assert_eq!(back, field);
    }

    #[test]
    fn an_absent_field_is_an_empty_array() {
        let field = Field::default();
        assert_eq!(serde_json::to_string(&field).unwrap(), "[]");
    }
}
