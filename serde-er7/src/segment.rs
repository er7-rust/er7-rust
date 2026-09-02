//! [`Segment`]: a name and its [`Field`]s, serialized as an object.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Field, Strict};

/// A Serde-enabled [`er7::Segment`].
///
/// Unlike the levels below it, a segment carries two different kinds of
/// information — its name and its fields — so it serializes as an object
/// with two named fields, `"name"` and `"fields"`, the same shape
/// [`serde`'s own manual-implementation guide](https://docs.rs/serde)
/// walks through for a struct with named fields.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Segment;
///
/// let message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
/// let pid = message.segment("PID").unwrap().clone();
///
/// let json = serde_json::to_string(&Segment(pid))?;
/// assert!(json.starts_with(r#"{"name":"PID","fields":"#));
///
/// let back: Segment = serde_json::from_str(&json)?;
/// assert_eq!(back.to_er7(&message.separators), "PID|1||9|4|SMITH^JOHN");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment(pub er7::Segment);

impl From<er7::Segment> for Segment {
    fn from(inner: er7::Segment) -> Segment {
        Segment(inner)
    }
}

impl From<Segment> for er7::Segment {
    fn from(outer: Segment) -> er7::Segment {
        outer.0
    }
}

impl Deref for Segment {
    type Target = er7::Segment;

    fn deref(&self) -> &er7::Segment {
        &self.0
    }
}

impl DerefMut for Segment {
    fn deref_mut(&mut self) -> &mut er7::Segment {
        &mut self.0
    }
}

impl Serialize for Segment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields: Vec<Field> = self.0.fields.iter().map(|f| Field(f.clone())).collect();
        let mut state = serializer.serialize_struct("Segment", 2)?;
        state.serialize_field("name", &self.0.name)?;
        state.serialize_field("fields", &fields)?;
        state.end()
    }
}

pub(crate) const FIELDS: &[&str] = &["name", "fields"];

/// `strict` selects between the two `Deserialize` entry points below: the
/// ordinary tolerant one (`strict: false`, rule S8) and [`Strict`]`<Segment>`'s
/// (`strict: true`, rule S13, spec §11). `message.rs` also constructs this
/// visitor directly, with `strict` carried down from `Strict<Message>`'s
/// own deserialization, so a typo inside a segment nested in a strict
/// message is caught too.
pub(crate) struct SegmentVisitor {
    pub(crate) strict: bool,
}

impl<'de> Visitor<'de> for SegmentVisitor {
    type Value = Segment;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a segment object with \"name\" and \"fields\"")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Segment, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut name: Option<String> = None;
        let mut fields: Option<Vec<Field>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "fields" => {
                    if fields.is_some() {
                        return Err(de::Error::duplicate_field("fields"));
                    }
                    fields = Some(map.next_value()?);
                }
                _ if self.strict => {
                    return Err(de::Error::unknown_field(&key, FIELDS));
                }
                _ => {
                    let _ = map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let fields = fields.ok_or_else(|| de::Error::missing_field("fields"))?;
        Ok(Segment(er7::Segment {
            name,
            fields: fields.into_iter().map(|f| f.0).collect(),
        }))
    }
}

impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Segment", FIELDS, SegmentVisitor { strict: false })
    }
}

impl<'de> Deserialize<'de> for Strict<Segment> {
    /// See [`Strict`] (rule S13, [§11](../spec/11-strict-mode/index.md)):
    /// an unrecognized key is a `serde::de::Error::unknown_field` rather
    /// than being ignored.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_struct("Segment", FIELDS, SegmentVisitor { strict: true })
            .map(Strict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_segment() {
        let segment = Segment(er7::Segment {
            name: "PID".into(),
            fields: vec![er7::Field {
                repetitions: vec![er7::Repetition {
                    components: vec![er7::Component {
                        subcomponents: vec![er7::Subcomponent::new("1")],
                    }],
                }],
            }],
        });
        let json = serde_json::to_string(&segment).unwrap();
        let back: Segment = serde_json::from_str(&json).unwrap();
        assert_eq!(back, segment);
    }

    #[test]
    fn rejects_a_missing_name() {
        let err = serde_json::from_str::<Segment>(r#"{"fields":[]}"#).unwrap_err();
        assert!(err.to_string().contains("name"));
    }

    #[test]
    fn ignores_unknown_fields() {
        let json = r#"{"name":"ZZZ","fields":[],"extra":true}"#;
        let back: Segment = serde_json::from_str(json).unwrap();
        assert_eq!(back.name, "ZZZ");
    }

    #[test]
    fn strict_rejects_an_unknown_field() {
        let json = r#"{"name":"ZZZ","fields":[],"extra":true}"#;
        let err = serde_json::from_str::<Strict<Segment>>(json).unwrap_err();
        assert!(err.to_string().contains("extra"));
    }

    #[test]
    fn strict_still_accepts_a_segment_with_no_extra_keys() {
        let json = r#"{"name":"ZZZ","fields":[]}"#;
        let back = serde_json::from_str::<Strict<Segment>>(json).unwrap();
        assert_eq!(back.0.name, "ZZZ");
    }

    #[test]
    fn strict_still_requires_every_field_the_plain_type_does() {
        let err = serde_json::from_str::<Strict<Segment>>(r#"{"fields":[]}"#).unwrap_err();
        assert!(err.to_string().contains("name"));
    }
}
