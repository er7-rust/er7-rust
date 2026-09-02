//! [`Message`]: the whole tree, serialized as an object.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Segment, Separators, Strict};

/// A Serde-enabled [`er7::Message`] — the crate's main entry point.
///
/// This is the type most callers reach for: parse ER7 with
/// [`Message::parse`] (or wrap an [`er7::Message`] you already have), hand
/// it to any `Serializer` — `serde_json::to_string`, a YAML or CBOR writer,
/// anything Serde-compatible — and get it back the same way on the other
/// end.
///
/// It serializes as an object with two fields, `"separators"` and
/// `"segments"`, exactly the shape [`er7::Message`] itself has
/// ([`er7::Message::separators`], [`er7::Message::segments`]) and the shape
/// [serde's own manual-implementation
/// guide](https://docs.rs/serde/latest/serde/) walks through for a struct
/// with named fields.
///
/// # What round-trips and what does not
///
/// Every subcomponent serializes as its `raw` text — escape sequences
/// intact, not decoded — so `Message::parse(text)?` through any Serde
/// format and back out through [`er7::Message::to_er7`] reproduces the
/// original bytes wherever `er7::parse(text)?.to_er7()` already would (see
/// [`er7::Message::to_er7`] for exactly when that is: canonical input
/// round-trips unchanged; non-canonical terminators and blank lines are
/// normalized once, at the first parse, same as in plain `er7`).
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Message;
///
/// let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
///             PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M\r\
///             OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";
///
/// let message = Message::parse(text)?;
///
/// // Any Serde format works; this crate never mentions JSON itself.
/// let json = serde_json::to_string_pretty(&message)?;
/// assert!(json.contains(r#""name": "PID""#));
///
/// // ...and it comes back the same message.
/// let back: Message = serde_json::from_str(&json)?;
/// assert_eq!(back.to_er7(), text);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message(pub er7::Message);

impl Message {
    /// Parse ER7 text directly into a Serde-enabled [`Message`].
    ///
    /// A thin wrapper over [`er7::parse()`], so the crate's flagship path —
    /// text in, any Serde format out — needs only this one call plus
    /// whichever format's `to_string`/`to_writer`.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_er7::Message;
    ///
    /// let message = Message::parse("MSH|^~\\&|LAB\rPID|1")?;
    /// assert_eq!(message.segments.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `er7::Error` exactly as [`er7::parse()`] does: the input held
    /// no segments, the first segment is not a header, or the header
    /// declared an unusable delimiter set. Nothing is added here.
    pub fn parse(text: &str) -> Result<Message, er7::Error> {
        er7::parse(text).map(Message)
    }
}

impl From<er7::Message> for Message {
    fn from(inner: er7::Message) -> Message {
        Message(inner)
    }
}

impl From<Message> for er7::Message {
    fn from(outer: Message) -> er7::Message {
        outer.0
    }
}

impl Deref for Message {
    type Target = er7::Message;

    fn deref(&self) -> &er7::Message {
        &self.0
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut er7::Message {
        &mut self.0
    }
}

impl fmt::Display for Message {
    /// The message as ER7; see [`er7::Message::to_er7`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_er7())
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let segments: Vec<Segment> = self.0.segments.iter().map(|s| Segment(s.clone())).collect();
        let mut state = serializer.serialize_struct("Message", 2)?;
        state.serialize_field("separators", &Separators(self.0.separators))?;
        state.serialize_field("segments", &segments)?;
        state.end()
    }
}

const FIELDS: &[&str] = &["separators", "segments"];

/// `strict` distinguishes the ordinary tolerant `Deserialize` entry point
/// (`strict: false`, rule S8) from [`Strict`]`<Message>`'s (`strict: true`,
/// rule S13, spec §11). When `strict`, `"separators"` and `"segments"` are
/// each read through a [`DeserializeSeed`] that carries the flag down into
/// [`crate::segment::SegmentVisitor`] and
/// [`crate::separators::SeparatorsVisitor`] directly, rather than through
/// `Segment`'s or `Separators`' own (always-tolerant) `Deserialize` impl —
/// see [§11.2](../spec/11-strict-mode/index.md) for why an unknown key
/// nested inside a segment or the separators object needs to be caught the
/// same way as one at the top level.
struct MessageVisitor {
    strict: bool,
}

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Message object with \"separators\" and \"segments\"")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Message, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut separators: Option<Separators> = None;
        let mut segments: Option<Vec<Segment>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "separators" => {
                    if separators.is_some() {
                        return Err(de::Error::duplicate_field("separators"));
                    }
                    separators = Some(if self.strict {
                        map.next_value_seed(SeparatorsSeed { strict: true })?
                    } else {
                        map.next_value()?
                    });
                }
                "segments" => {
                    if segments.is_some() {
                        return Err(de::Error::duplicate_field("segments"));
                    }
                    segments = Some(if self.strict {
                        map.next_value_seed(SegmentsSeed { strict: true })?
                    } else {
                        map.next_value()?
                    });
                }
                _ if self.strict => {
                    return Err(de::Error::unknown_field(&key, FIELDS));
                }
                _ => {
                    let _ = map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        let separators = separators.ok_or_else(|| de::Error::missing_field("separators"))?;
        let segments = segments.ok_or_else(|| de::Error::missing_field("segments"))?;
        Ok(Message(er7::Message {
            separators: separators.0,
            segments: segments.into_iter().map(|s| s.0).collect(),
        }))
    }
}

/// Deserializes one [`Segment`], strictly or not — used from
/// [`MessageVisitor`] via `next_value_seed` so a nested segment's own
/// unknown-field check runs with the same `strict` flag as the message
/// around it, which a plain `map.next_value::<Segment>()?` (always
/// `strict: false`) could not do.
struct SegmentSeed {
    strict: bool,
}

impl<'de> DeserializeSeed<'de> for SegmentSeed {
    type Value = Segment;

    fn deserialize<D>(self, deserializer: D) -> Result<Segment, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Segment",
            crate::segment::FIELDS,
            crate::segment::SegmentVisitor {
                strict: self.strict,
            },
        )
    }
}

/// Deserializes `"segments"` as a `Vec<Segment>`, using [`SegmentSeed`] for
/// each element so strictness reaches every segment in the sequence.
struct SegmentsSeed {
    strict: bool,
}

impl<'de> DeserializeSeed<'de> for SegmentsSeed {
    type Value = Vec<Segment>;

    fn deserialize<D>(self, deserializer: D) -> Result<Vec<Segment>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentsVisitor {
            strict: bool,
        }

        impl<'de> Visitor<'de> for SegmentsVisitor {
            type Value = Vec<Segment>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of segments")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Segment>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut segments = Vec::new();
                while let Some(segment) = seq.next_element_seed(SegmentSeed {
                    strict: self.strict,
                })? {
                    segments.push(segment);
                }
                Ok(segments)
            }
        }

        deserializer.deserialize_seq(SegmentsVisitor {
            strict: self.strict,
        })
    }
}

/// Deserializes one [`Separators`], strictly or not — the same reasoning
/// as [`SegmentSeed`].
struct SeparatorsSeed {
    strict: bool,
}

impl<'de> DeserializeSeed<'de> for SeparatorsSeed {
    type Value = Separators;

    fn deserialize<D>(self, deserializer: D) -> Result<Separators, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Separators",
            crate::separators::FIELDS,
            crate::separators::SeparatorsVisitor {
                strict: self.strict,
            },
        )
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Message", FIELDS, MessageVisitor { strict: false })
    }
}

impl<'de> Deserialize<'de> for Strict<Message> {
    /// See [`Strict`] (rule S13, [§11](../spec/11-strict-mode/index.md)):
    /// an unrecognized key — at the top level, inside a segment, or inside
    /// the separators object — is a `serde::de::Error::unknown_field`
    /// rather than being ignored.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_struct("Message", FIELDS, MessageVisitor { strict: true })
            .map(Strict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADT: &str = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ADT^A08^ADT_A01|MSG9|P|2.5\r\
                       PID|1||12345^^^ACME&1.2.3&ISO^MR||SMITH^JOHN^Q||19800101|M|||||\
                       555-1111~555-2222\r\
                       OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL\r\
                       OBX|2|ST|X^Note^L||\"\"";

    #[test]
    fn round_trips_a_full_message_through_json() {
        let message = Message::parse(ADT).unwrap();
        let json = serde_json::to_string(&message).unwrap();
        let back: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(back.to_er7(), ADT);
        assert_eq!(back, message);
    }

    #[test]
    fn round_trips_custom_delimiters() {
        let text = "MSH#*!?@#LAB#*A*B#C!D";
        let message = Message::parse(text).unwrap();
        let json = serde_json::to_string(&message).unwrap();
        let back: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(back.to_er7(), text);
    }

    #[test]
    fn round_trips_a_batch() {
        let text = "FHS|^~\\&|SENDER\rBHS|^~\\&|SENDER\r\
                    MSH|^~\\&|SENDER||RECEIVER||20260815090000||ACK^A08^ACK|B1|P|2.5\r\
                    MSA|AA|MSG00001\rBTS|1\rFTS|1";
        let message = Message::parse(text).unwrap();
        let json = serde_json::to_string(&message).unwrap();
        let back: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(back.to_er7(), text);
    }

    #[test]
    fn deref_reaches_query() {
        let message = Message::parse(ADT).unwrap();
        assert_eq!(message.query("PID-5.1").unwrap().as_deref(), Some("SMITH"));
    }

    #[test]
    fn rejects_a_message_missing_segments() {
        let err = serde_json::from_str::<Message>(r#"{"separators":{"field":"|","component":"^","repetition":"~","escape":"\\","subcomponent":"&"}}"#)
            .unwrap_err();
        assert!(err.to_string().contains("segments"));
    }

    #[test]
    fn pretty_json_keeps_the_tree_shape() {
        let message = Message::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        let pid = &json["segments"][1];
        assert_eq!(pid["name"], "PID");
        // PID-5 (index 4) is field 5 = ["SMITH", "JOHN"] one repetition deep.
        assert_eq!(
            pid["fields"][4][0],
            serde_json::json!([["SMITH"], ["JOHN"]])
        );
    }

    #[test]
    fn strict_rejects_an_unknown_top_level_field() {
        let json = r#"{"separators":{"field":"|","component":"^","repetition":"~",
            "escape":"\\","subcomponent":"&","truncation":null},"segments":[],"extra":true}"#;
        let err = serde_json::from_str::<Strict<Message>>(json).unwrap_err();
        assert!(err.to_string().contains("extra"));
    }

    #[test]
    fn strict_rejects_an_unknown_field_inside_a_nested_segment() {
        let json = r#"{"separators":{"field":"|","component":"^","repetition":"~",
            "escape":"\\","subcomponent":"&","truncation":null},
            "segments":[{"name":"PID","fields":[],"extra":true}]}"#;
        let err = serde_json::from_str::<Strict<Message>>(json).unwrap_err();
        assert!(err.to_string().contains("extra"));
    }

    #[test]
    fn strict_rejects_an_unknown_field_inside_separators() {
        let json = r#"{"separators":{"field":"|","component":"^","repetition":"~",
            "escape":"\\","subcomponent":"&","extra":true},"segments":[]}"#;
        let err = serde_json::from_str::<Strict<Message>>(json).unwrap_err();
        assert!(err.to_string().contains("extra"));
    }

    #[test]
    fn strict_still_requires_every_field_the_plain_type_does() {
        let json = r#"{"separators":{"field":"|","component":"^","repetition":"~",
            "escape":"\\","subcomponent":"&","truncation":null}}"#;
        let err = serde_json::from_str::<Strict<Message>>(json).unwrap_err();
        assert!(err.to_string().contains("segments"));
    }

    #[test]
    fn strict_accepts_a_real_message_with_no_typos() {
        let message = Message::parse(ADT).unwrap();
        let json = serde_json::to_string(&message).unwrap();
        let back = serde_json::from_str::<Strict<Message>>(&json).unwrap();
        assert_eq!(back.0, message);
    }

    #[test]
    fn plain_deserialize_is_unaffected_by_strict_existing() {
        // The whole point of S13 is additive: Strict<Message> existing does
        // not change what plain `Message::deserialize` accepts.
        let json = r#"{"separators":{"field":"|","component":"^","repetition":"~",
            "escape":"\\","subcomponent":"&","truncation":null},"segments":[],"extra":true}"#;
        let back: Message = serde_json::from_str(json).unwrap();
        assert!(back.segments.is_empty());
    }
}
