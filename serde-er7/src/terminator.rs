//! [`Terminator`]: a three-variant enum, serialized as its Rust identifier.

use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A Serde-enabled [`er7::Terminator`].
///
/// [`er7::Terminator`] is a render option, not part of a message's own
/// data, but it is a public [`Copy`] enum with no fields, so it is included
/// here for completeness and as the crate's one example of the "simple
/// C-like enum" case in [serde's manual-implementation
/// guide](https://docs.rs/serde/latest/serde/).
///
/// It serializes as one of the strings `"Cr"`, `"Lf"`, `"CrLf"` — the
/// variant's own Rust identifier — via [`Serializer::serialize_str`] rather
/// than [`Serializer::serialize_unit_variant`]. That trades the compact
/// index a binary format could use for a value that reads the same way in
/// every format, including the JSON and YAML this crate is mainly built
/// for, where a terminator choice is never on a hot path.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::Terminator;
///
/// assert_eq!(serde_json::to_string(&Terminator(er7::Terminator::Lf))?, r#""Lf""#);
///
/// let back: Terminator = serde_json::from_str(r#""CrLf""#)?;
/// assert_eq!(back.0, er7::Terminator::CrLf);
///
/// assert!(serde_json::from_str::<Terminator>(r#""Sixteen""#).is_err());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Terminator(pub er7::Terminator);

impl From<er7::Terminator> for Terminator {
    fn from(inner: er7::Terminator) -> Terminator {
        Terminator(inner)
    }
}

impl From<Terminator> for er7::Terminator {
    fn from(outer: Terminator) -> er7::Terminator {
        outer.0
    }
}

impl Deref for Terminator {
    type Target = er7::Terminator;

    fn deref(&self) -> &er7::Terminator {
        &self.0
    }
}

impl DerefMut for Terminator {
    fn deref_mut(&mut self) -> &mut er7::Terminator {
        &mut self.0
    }
}

impl Serialize for Terminator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = match self.0 {
            er7::Terminator::Cr => "Cr",
            er7::Terminator::Lf => "Lf",
            er7::Terminator::CrLf => "CrLf",
        };
        serializer.serialize_str(name)
    }
}

struct TerminatorVisitor;

impl Visitor<'_> for TerminatorVisitor {
    type Value = Terminator;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#""Cr", "Lf", or "CrLf""#)
    }

    fn visit_str<E>(self, value: &str) -> Result<Terminator, E>
    where
        E: de::Error,
    {
        match value {
            "Cr" => Ok(Terminator(er7::Terminator::Cr)),
            "Lf" => Ok(Terminator(er7::Terminator::Lf)),
            "CrLf" => Ok(Terminator(er7::Terminator::CrLf)),
            other => Err(de::Error::unknown_variant(other, &["Cr", "Lf", "CrLf"])),
        }
    }
}

impl<'de> Deserialize<'de> for Terminator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TerminatorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_variant() {
        for terminator in [
            er7::Terminator::Cr,
            er7::Terminator::Lf,
            er7::Terminator::CrLf,
        ] {
            let wrapped = Terminator(terminator);
            let json = serde_json::to_string(&wrapped).unwrap();
            let back: Terminator = serde_json::from_str(&json).unwrap();
            assert_eq!(back.0, terminator);
        }
    }

    #[test]
    fn rejects_an_unknown_variant() {
        assert!(serde_json::from_str::<Terminator>(r#""Sixteen""#).is_err());
    }
}
