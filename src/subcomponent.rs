//! [`Subcomponent`]: the leaf of the tree, serialized as a bare string.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A Serde-enabled [`er7::Subcomponent`].
///
/// A subcomponent holds only text — [`er7::Subcomponent::raw`] — so it
/// serializes as a bare string rather than as a one-field object. That
/// choice is the one that makes the whole tree read naturally in JSON:
/// `PID-5.1` becomes `"SMITH"`, not `{"raw": "SMITH"}`.
///
/// The text serialized is `raw`, exactly as the sender wrote it — escape
/// sequences included, not [`er7::Subcomponent::value`]-decoded. That keeps
/// the crate's core promise: `Message::parse(text)?` followed by
/// `.to_er7()` on the other end of any Serde format reproduces the
/// original bytes. Decode with [`er7::Subcomponent::value`] yourself where
/// you want the resolved text instead.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Subcomponent;
///
/// let leaf = Subcomponent(er7::Subcomponent::new(r"Smith \T\ Jones"));
/// let json = serde_json::to_string(&leaf)?;
/// assert_eq!(json, r#""Smith \\T\\ Jones""#);
///
/// let back: Subcomponent = serde_json::from_str(&json)?;
/// assert_eq!(back.raw, r"Smith \T\ Jones");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Subcomponent(pub er7::Subcomponent);

impl From<er7::Subcomponent> for Subcomponent {
    fn from(inner: er7::Subcomponent) -> Subcomponent {
        Subcomponent(inner)
    }
}

impl From<Subcomponent> for er7::Subcomponent {
    fn from(outer: Subcomponent) -> er7::Subcomponent {
        outer.0
    }
}

impl Deref for Subcomponent {
    type Target = er7::Subcomponent;

    fn deref(&self) -> &er7::Subcomponent {
        &self.0
    }
}

impl DerefMut for Subcomponent {
    fn deref_mut(&mut self) -> &mut er7::Subcomponent {
        &mut self.0
    }
}

impl Serialize for Subcomponent {
    /// Write `raw` as a string, via [`Serializer::serialize_str`].
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.raw)
    }
}

/// Reads any string into a [`Subcomponent`]. There is no way for a string
/// to be malformed here — every byte sequence is valid `raw` text — so this
/// visitor never returns an error.
struct SubcomponentVisitor;

impl Visitor<'_> for SubcomponentVisitor {
    type Value = Subcomponent;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string holding one ER7 subcomponent, escape sequences as sent")
    }

    fn visit_str<E>(self, value: &str) -> Result<Subcomponent, E>
    where
        E: serde::de::Error,
    {
        Ok(Subcomponent(er7::Subcomponent::new(value)))
    }
}

impl<'de> Deserialize<'de> for Subcomponent {
    /// Read a string into `raw`, via [`Deserializer::deserialize_str`].
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SubcomponentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_a_bare_string() {
        let leaf = Subcomponent(er7::Subcomponent::new("SMITH"));
        assert_eq!(serde_json::to_string(&leaf).unwrap(), r#""SMITH""#);
    }

    #[test]
    fn round_trips_the_explicit_null() {
        let null = Subcomponent(er7::Subcomponent::new(er7::message::NULL));
        let json = serde_json::to_string(&null).unwrap();
        let back: Subcomponent = serde_json::from_str(&json).unwrap();
        assert!(back.is_null());
    }

    #[test]
    fn round_trips_an_empty_subcomponent() {
        let empty = Subcomponent::default();
        let json = serde_json::to_string(&empty).unwrap();
        let back: Subcomponent = serde_json::from_str(&json).unwrap();
        assert!(back.is_empty());
        assert!(!back.is_null());
    }

    #[test]
    fn deref_reaches_the_inner_api() {
        let separators = er7::Separators::default();
        let leaf = Subcomponent(er7::Subcomponent::new(r"a\T\b"));
        // `.value(...)` is a method on `er7::Subcomponent`, reached through Deref.
        assert_eq!(leaf.value(&separators), "a&b");
    }
}
