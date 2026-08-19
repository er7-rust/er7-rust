//! [`Repetition`]: a sequence of [`Component`]s, serialized as an array.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Component;

/// A Serde-enabled [`er7::Repetition`].
///
/// A repetition is nothing but its components in order
/// ([`er7::Repetition::components`]), so — like [`Component`] one level
/// down — it serializes as a plain array: `SMITH^JOHN` becomes
/// `["SMITH", "JOHN"]`.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Repetition;
///
/// let message = er7::parse(r"MSH|^~\&|LAB|SMITH^JOHN")?;
/// let repetition = message
///     .segment("MSH")
///     .unwrap()
///     .field(4)
///     .unwrap()
///     .repetition(1)
///     .unwrap()
///     .clone();
///
/// let json = serde_json::to_string(&Repetition(repetition))?;
/// let back: Repetition = serde_json::from_str(&json)?;
/// assert_eq!(back.to_er7(&message.separators), "SMITH^JOHN");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Repetition(pub er7::Repetition);

impl From<er7::Repetition> for Repetition {
    fn from(inner: er7::Repetition) -> Repetition {
        Repetition(inner)
    }
}

impl From<Repetition> for er7::Repetition {
    fn from(outer: Repetition) -> er7::Repetition {
        outer.0
    }
}

impl Deref for Repetition {
    type Target = er7::Repetition;

    fn deref(&self) -> &er7::Repetition {
        &self.0
    }
}

impl DerefMut for Repetition {
    fn deref_mut(&mut self) -> &mut er7::Repetition {
        &mut self.0
    }
}

impl Serialize for Repetition {
    /// Write each component as one array element. See [`Component::serialize`]
    /// for the cost of the clone this involves.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let components = &self.0.components;
        let mut seq = serializer.serialize_seq(Some(components.len()))?;
        for component in components {
            seq.serialize_element(&Component(component.clone()))?;
        }
        seq.end()
    }
}

struct RepetitionVisitor;

impl<'de> Visitor<'de> for RepetitionVisitor {
    type Value = Repetition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array of components, each an array of subcomponent strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Repetition, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut components = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(component) = seq.next_element::<Component>()? {
            components.push(component.0);
        }
        Ok(Repetition(er7::Repetition { components }))
    }
}

impl<'de> Deserialize<'de> for Repetition {
    /// Read an array of components into `components`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RepetitionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_nested_components() {
        let repetition = Repetition(er7::Repetition {
            components: vec![
                er7::Component {
                    subcomponents: vec![er7::Subcomponent::new("SMITH")],
                },
                er7::Component {
                    subcomponents: vec![er7::Subcomponent::new("JOHN")],
                },
            ],
        });
        let json = serde_json::to_string(&repetition).unwrap();
        assert_eq!(json, r#"[["SMITH"],["JOHN"]]"#);
        let back: Repetition = serde_json::from_str(&json).unwrap();
        assert_eq!(back, repetition);
    }
}
