//! [`Component`]: a sequence of [`Subcomponent`]s, serialized as an array.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Subcomponent;

/// A Serde-enabled [`er7::Component`].
///
/// A component is nothing but its subcomponents in order
/// ([`er7::Component::subcomponents`]), so it serializes as a plain array
/// rather than as a one-field object: `ACME&1.2.3&ISO` becomes
/// `["ACME", "1.2.3", "ISO"]`. The same shape [`Subcomponent`] chose for
/// itself, one level up.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Component;
///
/// let message = er7::parse(r"MSH|^~\&|LAB|ACME&1.2.3&ISO")?;
/// let component = er7::Component {
///     subcomponents: vec![
///         er7::Subcomponent::new("ACME"),
///         er7::Subcomponent::new("1.2.3"),
///         er7::Subcomponent::new("ISO"),
///     ],
/// };
/// let wrapped = Component(component);
///
/// assert_eq!(serde_json::to_string(&wrapped)?, r#"["ACME","1.2.3","ISO"]"#);
/// let back: Component = serde_json::from_str(r#"["ACME","1.2.3","ISO"]"#)?;
/// // Subcomponents join on `&`, one level below where components join on `^`.
/// assert_eq!(back.to_er7(&message.separators), "ACME&1.2.3&ISO");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Component(pub er7::Component);

impl From<er7::Component> for Component {
    fn from(inner: er7::Component) -> Component {
        Component(inner)
    }
}

impl From<Component> for er7::Component {
    fn from(outer: Component) -> er7::Component {
        outer.0
    }
}

impl Deref for Component {
    type Target = er7::Component;

    fn deref(&self) -> &er7::Component {
        &self.0
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut er7::Component {
        &mut self.0
    }
}

impl Serialize for Component {
    /// Write each subcomponent as one array element.
    ///
    /// This clones every subcomponent to borrow it as a [`Subcomponent`],
    /// which is the price of keeping one owning wrapper type per tree level
    /// instead of a second, borrowing family — a fair trade for an HL7®
    /// message, which is kilobytes, not gigabytes.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let subcomponents = &self.0.subcomponents;
        let mut seq = serializer.serialize_seq(Some(subcomponents.len()))?;
        for subcomponent in subcomponents {
            seq.serialize_element(&Subcomponent(subcomponent.clone()))?;
        }
        seq.end()
    }
}

struct ComponentVisitor;

impl<'de> Visitor<'de> for ComponentVisitor {
    type Value = Component;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array of subcomponent strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Component, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut subcomponents = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(subcomponent) = seq.next_element::<Subcomponent>()? {
            subcomponents.push(subcomponent.0);
        }
        Ok(Component(er7::Component { subcomponents }))
    }
}

impl<'de> Deserialize<'de> for Component {
    /// Read an array of strings into `subcomponents`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ComponentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_several_subcomponents() {
        let component = Component(er7::Component {
            subcomponents: vec![
                er7::Subcomponent::new("ACME"),
                er7::Subcomponent::new("1.2.3"),
                er7::Subcomponent::new("ISO"),
            ],
        });
        let json = serde_json::to_string(&component).unwrap();
        assert_eq!(json, r#"["ACME","1.2.3","ISO"]"#);
        let back: Component = serde_json::from_str(&json).unwrap();
        assert_eq!(back, component);
    }

    #[test]
    fn round_trips_an_empty_array() {
        let component = Component::default();
        let json = serde_json::to_string(&component).unwrap();
        assert_eq!(json, "[]");
        let back: Component = serde_json::from_str(&json).unwrap();
        assert!(back.subcomponents.is_empty());
    }
}
