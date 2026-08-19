//! [`Separators`]: the delimiter set, serialized as an object of
//! single-character strings.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A Serde-enabled [`er7::Separators`].
///
/// This is the plainest wrapper in the crate: six named, scalar fields, no
/// recursion — the same shape as the `Point { x, y }` example in
/// [serde's own manual-implementation
/// guide](https://docs.rs/serde/latest/serde/), just with six fields
/// instead of two. Each `char` serializes through
/// [`Serializer::serialize_char`], which every common format maps to a
/// one-character string.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Separators;
///
/// let wrapped = Separators(er7::Separators::default());
/// let json = serde_json::to_string(&wrapped)?;
/// assert_eq!(
///     json,
///     r#"{"field":"|","component":"^","repetition":"~","escape":"\\","subcomponent":"&","truncation":null}"#
/// );
///
/// let back: Separators = serde_json::from_str(&json)?;
/// assert_eq!(back.0, er7::Separators::default());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Separators(pub er7::Separators);

impl From<er7::Separators> for Separators {
    fn from(inner: er7::Separators) -> Separators {
        Separators(inner)
    }
}

impl From<Separators> for er7::Separators {
    fn from(outer: Separators) -> er7::Separators {
        outer.0
    }
}

impl Deref for Separators {
    type Target = er7::Separators;

    fn deref(&self) -> &er7::Separators {
        &self.0
    }
}

impl DerefMut for Separators {
    fn deref_mut(&mut self) -> &mut er7::Separators {
        &mut self.0
    }
}

impl Serialize for Separators {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Separators", 6)?;
        state.serialize_field("field", &self.0.field)?;
        state.serialize_field("component", &self.0.component)?;
        state.serialize_field("repetition", &self.0.repetition)?;
        state.serialize_field("escape", &self.0.escape)?;
        state.serialize_field("subcomponent", &self.0.subcomponent)?;
        state.serialize_field("truncation", &self.0.truncation)?;
        state.end()
    }
}

const FIELDS: &[&str] = &[
    "field",
    "component",
    "repetition",
    "escape",
    "subcomponent",
    "truncation",
];

struct SeparatorsVisitor;

impl<'de> Visitor<'de> for SeparatorsVisitor {
    type Value = Separators;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a Separators object with \"field\", \"component\", \"repetition\", \"escape\", \
             \"subcomponent\", and \"truncation\"",
        )
    }

    fn visit_map<V>(self, mut map: V) -> Result<Separators, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut field = None;
        let mut component = None;
        let mut repetition = None;
        let mut escape = None;
        let mut subcomponent = None;
        let mut truncation: Option<Option<char>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "field" => set_once(&mut field, &mut map, "field")?,
                "component" => set_once(&mut component, &mut map, "component")?,
                "repetition" => set_once(&mut repetition, &mut map, "repetition")?,
                "escape" => set_once(&mut escape, &mut map, "escape")?,
                "subcomponent" => set_once(&mut subcomponent, &mut map, "subcomponent")?,
                "truncation" => {
                    if truncation.is_some() {
                        return Err(de::Error::duplicate_field("truncation"));
                    }
                    truncation = Some(map.next_value()?);
                }
                _ => {
                    let _ = map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        let field = field.ok_or_else(|| de::Error::missing_field("field"))?;
        let component = component.ok_or_else(|| de::Error::missing_field("component"))?;
        let repetition = repetition.ok_or_else(|| de::Error::missing_field("repetition"))?;
        let escape = escape.ok_or_else(|| de::Error::missing_field("escape"))?;
        let subcomponent = subcomponent.ok_or_else(|| de::Error::missing_field("subcomponent"))?;
        // `truncation` is genuinely optional in ER7 itself (spec: v2.7+
        // only), so a message that omits the key gets `None` rather than an
        // error — unlike the five delimiters above, which every message has.
        let truncation = truncation.unwrap_or(None);

        Ok(Separators(er7::Separators {
            field,
            component,
            repetition,
            escape,
            subcomponent,
            truncation,
        }))
    }
}

/// Read one `char` field from the map, rejecting a second occurrence of the
/// same key.
fn set_once<'de, V>(
    slot: &mut Option<char>,
    map: &mut V,
    name: &'static str,
) -> Result<(), V::Error>
where
    V: MapAccess<'de>,
{
    if slot.is_some() {
        return Err(de::Error::duplicate_field(name));
    }
    *slot = Some(map.next_value()?);
    Ok(())
}

impl<'de> Deserialize<'de> for Separators {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Separators", FIELDS, SeparatorsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_the_default_delimiters() {
        let separators = Separators::default();
        let json = serde_json::to_string(&separators).unwrap();
        let back: Separators = serde_json::from_str(&json).unwrap();
        assert_eq!(back, separators);
    }

    #[test]
    fn round_trips_custom_delimiters_and_truncation() {
        let separators = Separators(er7::Separators {
            field: '#',
            component: '*',
            repetition: '!',
            escape: '?',
            subcomponent: '@',
            truncation: Some('%'),
        });
        let json = serde_json::to_string(&separators).unwrap();
        let back: Separators = serde_json::from_str(&json).unwrap();
        assert_eq!(back, separators);
    }

    #[test]
    fn treats_a_missing_truncation_key_as_none() {
        let json =
            r#"{"field":"|","component":"^","repetition":"~","escape":"\\","subcomponent":"&"}"#;
        let back: Separators = serde_json::from_str(json).unwrap();
        assert_eq!(back.truncation, None);
    }

    #[test]
    fn rejects_a_missing_required_field() {
        let err = serde_json::from_str::<Separators>(r#"{"field":"|"}"#).unwrap_err();
        assert!(err.to_string().contains("component"));
    }
}
