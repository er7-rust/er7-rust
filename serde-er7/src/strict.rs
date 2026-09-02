//! [`Strict`]: an opt-in wrapper that rejects unknown fields on
//! deserialize.

use std::ops::{Deref, DerefMut};

use serde::{Serialize, Serializer};

use crate::{Message, Segment, Separators};

/// Deserialize `T` strictly: an unrecognized field is a `serde::de::Error`
/// instead of being ignored.
///
/// `T::deserialize` alone stays tolerant of unknown fields, unconditionally
/// (rule S8, [§5](../spec/05-error-handling/index.md)) — that default
/// does not change. `Strict<T>` is a separate, additive entry point for a
/// caller who wants the opposite for one particular call, such as
/// validating a hand-written JSON fixture for typos. See
/// [§11](../spec/11-strict-mode/index.md) (rule S13) for the full
/// rationale, including why this is a distinct type rather than a flag on
/// the existing one.
///
/// A `Deserialize` implementation exists for `Strict<Message>`,
/// `Strict<Segment>`, and `Strict<Separators>` — the three object-shaped
/// types in [§2](../spec/02-wire-shapes/index.md)'s table. `Field`,
/// `Repetition`, `Component`, and `Subcomponent` serialize as bare arrays
/// or strings (S4), where "unknown field" has no meaning, so there is no
/// `Strict<Field>`; `Terminator` already rejects an unrecognized variant
/// either way, so there is no `Strict<Terminator>` either.
///
/// Strictness nests: `Strict<Message>` also rejects an unrecognized key
/// inside each segment it contains and inside its separators object, not
/// only at the top level.
///
/// Like every wrapper type in this crate ([§6](../spec/06-ergonomics/index.md),
/// rule S11), `Strict<T>` implements `Deref`, `DerefMut`, and `From` both
/// ways, plus a `Serialize` impl that delegates to `T`'s own — strictness
/// is a deserialize-only concept, but a `Strict<T>` should still be usable
/// anywhere a `T` is, including as the input to `serialize`.
///
/// Example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_er7::{Message, Strict};
///
/// let good = r#"{"separators":{"field":"|","component":"^","repetition":"~",
///     "escape":"\\","subcomponent":"&","truncation":null},"segments":[]}"#;
/// let typo = r#"{"separatros":{},"segments":[]}"#;
///
/// // The plain type ignores an unrecognized top-level key.
/// // `Strict<Message>` reports it instead:
/// assert!(serde_json::from_str::<Strict<Message>>(typo).is_err());
/// assert!(serde_json::from_str::<Strict<Message>>(good).is_ok());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strict<T>(pub T);

impl<T> From<T> for Strict<T> {
    fn from(inner: T) -> Strict<T> {
        Strict(inner)
    }
}

impl<T> Deref for Strict<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Strict<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Serialize> Serialize for Strict<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

// The reverse `From<Strict<T>> for T` cannot be written generically over
// `T` — Rust's orphan rule rejects `impl<T> From<Strict<T>> for T` because
// the `Self` type, `T`, would be a completely uncovered impl parameter.
// Each of the three types `Strict` supports gets its own concrete impl
// instead, which the orphan rule allows: `Self` is then a local, concrete
// type.
impl From<Strict<Message>> for Message {
    fn from(outer: Strict<Message>) -> Message {
        outer.0
    }
}

impl From<Strict<Segment>> for Segment {
    fn from(outer: Strict<Segment>) -> Segment {
        outer.0
    }
}

impl From<Strict<Separators>> for Separators {
    fn from(outer: Strict<Separators>) -> Separators {
        outer.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_reaches_the_inner_value() {
        let strict = Strict(Message::parse("MSH|^~\\&|LAB").unwrap());
        assert_eq!(strict.segments.len(), 1);
    }

    #[test]
    fn from_and_back_round_trip() {
        let message = Message::parse("MSH|^~\\&|LAB").unwrap();
        let strict: Strict<Message> = message.clone().into();
        let back: Message = strict.into();
        assert_eq!(back, message);
    }

    #[test]
    fn serialize_delegates_to_the_inner_type() {
        let message = Message::parse("MSH|^~\\&|LAB").unwrap();
        let plain = serde_json::to_string(&message).unwrap();
        let wrapped = serde_json::to_string(&Strict(message)).unwrap();
        assert_eq!(plain, wrapped);
    }
}
