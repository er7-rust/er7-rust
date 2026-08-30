//! The eight built-in things redaction can do to a value, and a ninth a
//! caller can supply.
//!
//! An action reads a leaf's **decoded** text and returns the text that
//! should replace it, so `Mask` and `First` count characters the way a
//! reader would rather than the way the wire spells them. What comes back
//! is written through [`er7::Subcomponent::set`], which encodes any
//! delimiter it contains (D11).
//!
//! Specified by spec §3.

use crate::Error;
use crate::pseudonym::pseudonym;
use std::fmt;
use std::sync::Arc;

/// The placeholder every built-in policy writes.
const REDACTED: &str = "REDACTED";

/// The mask character `mask` uses when a policy file names none.
const MASK: char = '*';

/// What to do to a value the policy selected.
///
/// Every variant except [`Action::Null`] rewrites leaf text and leaves the
/// shape of the message alone (D1); `Null` collapses the position it names
/// to the explicit HL7® null, because that is what a null means (D6, spec
/// §3.4).
///
/// Example:
///
/// ```
/// use er7_redact::Action;
///
/// // Applied to a decoded value, with the pseudonym key.
/// assert_eq!(Action::redacted().apply("EVERYWOMAN", 0).as_deref(), Some("REDACTED"));
/// assert_eq!(Action::Mask('*').apply("EVERYWOMAN", 0).as_deref(), Some("**********"));
/// assert_eq!(Action::First(4).apply("19610615", 0).as_deref(), Some("1961"));
/// assert_eq!(Action::Last(4).apply("444333222", 0).as_deref(), Some("3222"));
/// assert_eq!(Action::Clear.apply("EVERYWOMAN", 0).as_deref(), Some(""));
///
/// // `Keep` changes nothing, and so returns nothing to write.
/// assert_eq!(Action::Keep.apply("EVERYWOMAN", 0), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Leave the value exactly as it is.
    ///
    /// This is the action that **accepts** a position: its use is to
    /// exempt one from a policy that rejects by default (spec §2.6).
    ///
    /// It does **not** undo an earlier rule: rules apply in order to the
    /// message as it stands, so once a value has been replaced there is
    /// nothing left to restore it from (D7, spec §2.4). That is also why a
    /// rejecting rule beats a `Keep` for the same leaf whichever order the
    /// two are in (D19).
    Keep,
    /// Empty the value, leaving the position in place.
    ///
    /// This is what redaction usually means. A receiver reads an empty
    /// field as "the sender said nothing about this", which leaves its
    /// stored value alone — see [`Action::Null`] for the other reading.
    Clear,
    /// Replace the named position with the explicit HL7 null `""`.
    ///
    /// A receiver reads this as "clear your stored value", which is a
    /// different instruction from [`Action::Clear`] and a much stronger
    /// one. It is also the only action that changes the shape of a
    /// message: everything beneath the named position is replaced by one
    /// subcomponent holding `""` (D6, spec §3.4).
    Null,
    /// Replace the value with fixed text, e.g. `REDACTED`.
    ///
    /// Delimiters in the text are encoded on the way in, so a replacement
    /// can never break the message (D11).
    Replace(String),
    /// Replace each character of the value with this one, preserving the
    /// length.
    ///
    /// The length is exactly what this leaks; use [`Action::Clear`] or
    /// [`Action::Replace`] where that matters (spec §5.5).
    Mask(char),
    /// Keep the first `n` characters and drop the rest — a birth date
    /// reduced to its year, say. `First(0)` is legal and equivalent to
    /// [`Action::Clear`] (spec §3.7).
    First(usize),
    /// Keep the last `n` characters and drop the rest — an account number
    /// reduced to the digits a human matches on.
    Last(usize),
    /// Replace the value with a stable pseudonym, so that equal values stay
    /// equal across every message redacted with the same key.
    ///
    /// Read [`crate::pseudonym()`] before using this: a pseudonym preserves
    /// linkage on purpose, and it is not a cryptographic guarantee (D12,
    /// spec §7.3).
    Pseudonym,
    /// Run the caller's own function instead of one of the eight built-ins
    /// above — a real MAC, a lookup table, a per-patient date shift.
    ///
    /// Not part of the closed set of eight (spec §3.1); admitted by spec
    /// §16.11 under the rule that a ninth action needs a section saying why
    /// the eight were not enough. See [`CustomAction`] and spec §3.8 (D24)
    /// for what it costs: `Action` keeps its ordinary `Debug`, `Clone`,
    /// `PartialEq`, and `Eq`, and there is no policy-file spelling, ever.
    Custom(CustomAction),
}

impl Action {
    /// [`Action::Replace`] with the placeholder the built-in policies use.
    ///
    /// Example:
    ///
    /// ```
    /// use er7_redact::Action;
    ///
    /// assert_eq!(Action::redacted(), Action::Replace("REDACTED".to_string()));
    /// assert_eq!(Action::redacted().to_string(), "replace REDACTED");
    /// ```
    #[must_use]
    pub fn redacted() -> Action {
        Action::Replace(REDACTED.to_string())
    }

    /// Shorthand for `Action::Custom(CustomAction::new(f))` (D24, spec
    /// §3.8).
    ///
    /// `f` receives the leaf's decoded text and the redactor's pseudonym
    /// key — the same two things [`Action::apply`] always receives — and
    /// returns the replacement the same way every other action does:
    /// `Some(text)` to write `text`, `None` to leave the leaf as it is.
    ///
    /// Example:
    ///
    /// ```
    /// use er7_redact::Action;
    ///
    /// let shout = Action::custom(|value, _key| Some(value.to_uppercase()));
    /// assert_eq!(shout.apply("everywoman", 0).as_deref(), Some("EVERYWOMAN"));
    ///
    /// // There is no policy-file spelling for it.
    /// assert_eq!(shout.to_string(), "<custom>");
    /// ```
    #[must_use]
    pub fn custom(f: impl Fn(&str, u64) -> Option<String> + Send + Sync + 'static) -> Action {
        Action::Custom(CustomAction::new(f))
    }

    /// Read an action as a policy file spells it (spec §6.2).
    ///
    /// The action name is matched case-insensitively; an argument, where
    /// one is allowed, is the rest of the text as written.
    ///
    /// Example:
    ///
    /// ```
    /// # fn main() -> Result<(), er7_redact::Error> {
    /// use er7_redact::Action;
    ///
    /// assert_eq!(Action::parse("clear")?, Action::Clear);
    /// assert_eq!(Action::parse("first 4")?, Action::First(4));
    /// assert_eq!(Action::parse("replace NOT ON FILE")?,
    ///            Action::Replace("NOT ON FILE".to_string()));
    ///
    /// // The two arguments that may be left out have a default.
    /// assert_eq!(Action::parse("replace")?, Action::redacted());
    /// assert_eq!(Action::parse("mask")?, Action::Mask('*'));
    ///
    /// assert!(Action::parse("obfuscate").is_err());
    /// assert!(Action::parse("first three").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// [`Error::BadPolicy`] naming the problem: an unknown action name, an
    /// argument where none belongs, a `mask` argument that is not one
    /// character, or a `first`/`last` count that is not a number (spec
    /// §6.4).
    pub fn parse(text: &str) -> Result<Action, Error> {
        let text = text.trim();
        let (name, argument) = match text.split_once(char::is_whitespace) {
            Some((name, argument)) => (name, argument.trim()),
            None => (text, ""),
        };
        let bad = |detail: String| Err(Error::BadPolicy(detail));
        // An argument where none belongs is a typo worth reporting, not
        // something to ignore: `clear PID-5` is a rule missing a newline.
        let none = |action: Action| {
            if argument.is_empty() {
                Ok(action)
            } else {
                Err(Error::BadPolicy(format!(
                    "action {name:?} takes no argument, but got {argument:?}"
                )))
            }
        };
        let count = |what: &str| match argument.parse::<usize>() {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::BadPolicy(format!(
                "action {what:?} wants a number of characters, not {argument:?}"
            ))),
        };
        match name.to_ascii_lowercase().as_str() {
            "keep" => none(Action::Keep),
            "clear" => none(Action::Clear),
            "null" => none(Action::Null),
            "pseudonym" => none(Action::Pseudonym),
            "replace" if argument.is_empty() => Ok(Action::redacted()),
            "replace" => Ok(Action::Replace(argument.to_string())),
            "mask" if argument.is_empty() => Ok(Action::Mask(MASK)),
            "mask" => {
                let mut characters = argument.chars();
                match (characters.next(), characters.next()) {
                    (Some(mask), None) => Ok(Action::Mask(mask)),
                    _ => bad(format!(
                        "action \"mask\" wants one character, not {argument:?}"
                    )),
                }
            }
            "first" => Ok(Action::First(count("first")?)),
            "last" => Ok(Action::Last(count("last")?)),
            "" => bad("expected an action".to_string()),
            _ => bad(format!("unknown action {name:?}")),
        }
    }

    /// The text this action writes in place of `value`, or `None` to leave
    /// the value alone.
    ///
    /// `value` is the leaf's decoded text, and `key` is the redactor's
    /// pseudonym key, used only by [`Action::Pseudonym`].
    ///
    /// [`Action::Keep`] returns `None` because it writes nothing, and so
    /// does [`Action::Null`], which the redactor applies structurally
    /// rather than as text (spec §3.4).
    ///
    /// Example:
    ///
    /// ```
    /// use er7_redact::Action;
    ///
    /// // Counting is by character, so an escape that stands for one
    /// // character counts as one.
    /// assert_eq!(Action::First(3).apply("O'BRIEN", 0).as_deref(), Some("O'B"));
    ///
    /// // Asking for more characters than there are is not an error.
    /// assert_eq!(Action::First(99).apply("MR", 0).as_deref(), Some("MR"));
    /// assert_eq!(Action::Last(99).apply("MR", 0).as_deref(), Some("MR"));
    /// ```
    #[must_use]
    pub fn apply(&self, value: &str, key: u64) -> Option<String> {
        match self {
            Action::Keep | Action::Null => None,
            Action::Clear => Some(String::new()),
            Action::Replace(text) => Some(text.clone()),
            Action::Mask(mask) => Some(value.chars().map(|_| *mask).collect()),
            Action::First(n) => Some(value.chars().take(*n).collect()),
            Action::Last(n) => {
                let skip = value.chars().count().saturating_sub(*n);
                Some(value.chars().skip(skip).collect())
            }
            Action::Pseudonym => Some(pseudonym(key, value)),
            Action::Custom(custom) => custom.apply(value, key),
        }
    }
}

impl fmt::Display for Action {
    /// The spelling a policy file uses, so that a policy written out
    /// re-reads as the same policy (D18, spec §6.5).
    ///
    /// One value that does not survive the trip is `Replace` with empty
    /// text, written as `clear`, because nothing downstream can tell the
    /// two apart. `Custom` never survives it at all: it writes the fixed
    /// placeholder `<custom>`, which is not a keyword [`Action::parse`]
    /// recognises, on purpose (D24, spec §3.8, §6.5).
    ///
    /// Example:
    ///
    /// ```
    /// use er7_redact::Action;
    ///
    /// assert_eq!(Action::Pseudonym.to_string(), "pseudonym");
    /// assert_eq!(Action::First(4).to_string(), "first 4");
    /// assert_eq!(Action::Mask('#').to_string(), "mask #");
    /// assert_eq!(Action::Replace(String::new()).to_string(), "clear");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Keep => write!(f, "keep"),
            Action::Clear => write!(f, "clear"),
            Action::Null => write!(f, "null"),
            Action::Replace(text) if text.is_empty() => write!(f, "clear"),
            Action::Replace(text) => write!(f, "replace {text}"),
            Action::Mask(mask) => write!(f, "mask {mask}"),
            Action::First(n) => write!(f, "first {n}"),
            Action::Last(n) => write!(f, "last {n}"),
            Action::Pseudonym => write!(f, "pseudonym"),
            Action::Custom(_) => write!(f, "<custom>"),
        }
    }
}

/// A caller-supplied action, wrapped so [`Action`] can keep its ordinary
/// `Debug`, `Clone`, `PartialEq`, and `Eq` (D24, spec §3.8).
///
/// A bare closure supports none of the four. This newtype carries
/// hand-written versions instead of deriving them: `Debug` prints a fixed
/// placeholder, `Clone` clones the `Arc` (cheap; every clone runs the same
/// closure), and `PartialEq`/`Eq` compare **identity** — two
/// `CustomAction`s are equal exactly when they wrap the same closure,
/// never merely because two different closures compute the same values.
/// There is no general way to compare closures for behavioral equality, so
/// identity is the only comparison that is not lying about what it checked.
#[derive(Clone)]
pub struct CustomAction(Arc<Closure>);

/// The shape of a caller-supplied action: the decoded value and the
/// pseudonym key in, the replacement out — the same as [`Action::apply`].
type Closure = dyn Fn(&str, u64) -> Option<String> + Send + Sync;

impl CustomAction {
    /// Wrap `f` as a caller-supplied action. [`Action::custom`] is the
    /// usual way to reach this — `Action::Custom(CustomAction::new(f))`
    /// spelled out is rarely needed directly.
    #[must_use]
    pub fn new(f: impl Fn(&str, u64) -> Option<String> + Send + Sync + 'static) -> CustomAction {
        CustomAction(Arc::new(f))
    }

    /// Run the wrapped closure. Same signature and meaning as
    /// [`Action::apply`]: the decoded value and the pseudonym key in,
    /// `Some(text)` to write `text` or `None` to leave the leaf as it is,
    /// out.
    fn apply(&self, value: &str, key: u64) -> Option<String> {
        (self.0)(value, key)
    }
}

impl fmt::Debug for CustomAction {
    /// A fixed placeholder. There is nothing truthful to print about an
    /// opaque closure.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAction(..)")
    }
}

impl PartialEq for CustomAction {
    /// Identity, not behavior: `Arc::ptr_eq`. Two closures that compute the
    /// same values are still unequal here unless they are the same `Arc`.
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for CustomAction {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_action() {
        // D18: every spelling in spec §6.2 reads, and writes back as
        // itself, because a policy file is a compatibility surface.
        let cases = [
            ("keep", Action::Keep),
            ("clear", Action::Clear),
            ("null", Action::Null),
            ("pseudonym", Action::Pseudonym),
            ("replace REDACTED", Action::redacted()),
            ("mask *", Action::Mask('*')),
            ("first 4", Action::First(4)),
            ("last 4", Action::Last(4)),
        ];
        for (text, action) in cases {
            assert_eq!(Action::parse(text).unwrap(), action, "parsing {text:?}");
            assert_eq!(action.to_string(), text, "writing {action:?}");
        }

        // Case is not significant in an action name.
        assert_eq!(Action::parse("CLEAR").unwrap(), Action::Clear);
        // The two defaults.
        assert_eq!(Action::parse("replace").unwrap(), Action::redacted());
        assert_eq!(Action::parse("mask").unwrap(), Action::Mask('*'));
        // Replacement text may contain spaces, and keeps its case.
        assert_eq!(
            Action::parse("replace Not On File").unwrap(),
            Action::Replace("Not On File".to_string())
        );
    }

    #[test]
    fn rejects_malformed_actions() {
        for text in [
            "",
            "obfuscate",
            "first",
            "first three",
            "mask ab",
            "clear PID-5",
        ] {
            assert!(
                Action::parse(text).is_err(),
                "expected {text:?} to be rejected"
            );
        }
    }

    #[test]
    fn every_action_but_pseudonym_is_idempotent() {
        // D10: applying a policy twice is the same as applying it once,
        // except for `Pseudonym`, which hashes whatever text it finds
        // (spec §3.6).
        let value = "EVERYWOMAN";
        for action in [
            Action::Clear,
            Action::redacted(),
            Action::Mask('*'),
            Action::First(4),
            Action::Last(4),
            Action::First(0),
        ] {
            let once = action.apply(value, 0).expect("writes a value");
            let twice = action.apply(&once, 0).expect("writes a value");
            assert_eq!(once, twice, "{action} is not idempotent");
        }

        // And the documented exception.
        let once = Action::Pseudonym.apply(value, 0).expect("writes a value");
        let twice = Action::Pseudonym.apply(&once, 0).expect("writes a value");
        assert_ne!(once, twice);
    }

    #[test]
    fn counts_characters_not_bytes() {
        // A decoded value can hold anything; `first` and `mask` must not
        // split it mid-character.
        assert_eq!(Action::First(2).apply("naïve", 0).as_deref(), Some("na"));
        assert_eq!(Action::First(3).apply("naïve", 0).as_deref(), Some("naï"));
        assert_eq!(Action::Last(3).apply("naïve", 0).as_deref(), Some("ïve"));
        assert_eq!(
            Action::Mask('*').apply("naïve", 0).as_deref(),
            Some("*****")
        );
    }

    #[test]
    fn zero_counts_are_legal() {
        // Spec §3.7: a policy computed from a table should not have to
        // special-case the boundary.
        assert_eq!(Action::First(0).apply("PATID1234", 0).as_deref(), Some(""));
        assert_eq!(Action::Last(0).apply("PATID1234", 0).as_deref(), Some(""));
    }

    #[test]
    fn custom_action_runs_the_callers_closure() {
        // D24: same signature and meaning as every other action — the
        // decoded value and the key in, `Some`/`None` out.
        let shout = Action::custom(|value, _key| Some(value.to_uppercase()));
        assert_eq!(shout.apply("everywoman", 0).as_deref(), Some("EVERYWOMAN"));

        // `None` leaves the leaf as it is, same as `Keep`.
        let never = Action::custom(|_value, _key| None);
        assert_eq!(never.apply("EVERYWOMAN", 0), None);

        // The key reaches the closure, for a real MAC or a per-patient
        // construction.
        let echo_key = Action::custom(|_value, key| Some(key.to_string()));
        assert_eq!(echo_key.apply("x", 42).as_deref(), Some("42"));
    }

    #[test]
    fn custom_action_equality_is_identity_not_behavior() {
        // D24: two closures that compute the same values are still
        // unequal unless they are the same `Arc` — there is no general way
        // to compare closures for behavioral equality.
        let upper = CustomAction::new(|v: &str, _k| Some(v.to_uppercase()));
        let also_upper = CustomAction::new(|v: &str, _k| Some(v.to_uppercase()));
        assert_ne!(upper, also_upper, "different closures, even identical ones");

        let cloned = upper.clone();
        assert_eq!(upper, cloned, "a clone is the same Arc");
    }

    #[test]
    fn custom_action_writes_a_placeholder_with_no_file_spelling() {
        // D24, spec §6.5: `Custom` is the one action Display does not
        // round-trip through parse, on purpose.
        let action = Action::custom(|v: &str, _k| Some(v.to_string()));
        assert_eq!(action.to_string(), "<custom>");
        assert!(Action::parse("<custom>").is_err());
        assert!(format!("{action:?}").contains("CustomAction"));
    }
}
