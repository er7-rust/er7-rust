//! Stable stand-ins for identifiers.
//!
//! Clearing an identifier destroys the message as test data: nothing joins
//! the patient in this message to the same patient in the next one. A
//! pseudonym replaces the identifier with a token that is the same
//! everywhere the identifier was, for a given key.
//!
//! **This is not a cryptographic guarantee** (D12). Read
//! [`pseudonym()`]'s documentation, and spec §7.3, before using it on data
//! that leaves your control.
//!
//! Specified by spec §7.

/// The FNV-1a 64-bit offset basis and prime.
const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;

/// A stable pseudonym for `value` under `key`: sixteen lowercase
/// hexadecimal characters.
///
/// # Stability
///
/// The value returned is **frozen** (spec §13.2). The same key and value
/// give the same pseudonym on every platform and in every future release
/// of this crate, major versions included, because a pseudonym is a join
/// key: a data set redacted last year and a message redacted today have to
/// still agree about which patient is which.
///
/// # What it does not give you
///
/// The construction is FNV-1a over the key bytes followed by the value
/// bytes. It is a hash, not a message authentication code, and it leaks
/// two things on purpose and one by accident:
///
/// - **Equality**, by construction. Anyone can see which messages concern
///   the same patient and count how many each generated — which, joined
///   with an outside data set, can re-identify the largest one.
/// - **Everything, to anyone who can guess.** Record numbers come from
///   small spaces. Given the key, an attacker computes the pseudonym of
///   every candidate in seconds and inverts the mapping completely.
///
/// So: use this inside your own trust boundary — test environments,
/// reproductions, CI fixtures. For data leaving it, use
/// [`Action::Clear`](crate::Action::Clear) or
/// [`Action::Replace`](crate::Action::Replace), which leak nothing but the
/// fact that a value was there.
///
/// Example:
///
/// ```
/// use er7_redact::pseudonym::pseudonym;
///
/// // Stable: the same identifier maps the same way, every time.
/// assert_eq!(pseudonym(0, "PATID1234"), pseudonym(0, "PATID1234"));
/// assert_eq!(pseudonym(0, "PATID1234").len(), 16);
///
/// // Keyed: two data sets redacted under different keys cannot be joined.
/// assert_ne!(pseudonym(0, "PATID1234"), pseudonym(1, "PATID1234"));
///
/// // Distinct: different identifiers do not collide into one patient.
/// assert_ne!(pseudonym(0, "PATID1234"), pseudonym(0, "PATID1235"));
/// ```
pub fn pseudonym(key: u64, value: &str) -> String {
    let mut hash = OFFSET;
    for byte in key.to_be_bytes().iter().chain(value.as_bytes()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pseudonyms_are_stable_and_keyed() {
        // D12: stable for a given key and value, different across keys,
        // and pinned to exact text so that a change to the construction
        // fails here rather than silently splitting somebody's data set
        // (spec §13.2).
        assert_eq!(pseudonym(0, "PATID1234"), "11a9d74f8a6a54a7");
        assert_eq!(pseudonym(0, ""), "a8c7f832281a39c5");
        assert_eq!(pseudonym(7, "PATID1234"), "da0cc7ad590d56fc");

        // Same key and value, every time.
        assert_eq!(pseudonym(0, "PATID1234"), pseudonym(0, "PATID1234"));
        // Different keys do not share pseudonyms, so two data sets
        // redacted under different keys cannot be joined.
        assert_ne!(pseudonym(0, "PATID1234"), pseudonym(1, "PATID1234"));
        // Different values do not collapse into one patient.
        assert_ne!(pseudonym(0, "PATID1234"), pseudonym(0, "PATID1235"));
    }

    #[test]
    fn a_pseudonym_is_sixteen_hexadecimal_characters() {
        for value in ["", "1", "PATID1234", "a much longer identifier than that"] {
            let pseudonym = pseudonym(0, value);
            assert_eq!(pseudonym.len(), 16, "{value:?}");
            assert!(
                pseudonym.chars().all(|c| c.is_ascii_hexdigit()),
                "{pseudonym:?}"
            );
            assert!(!pseudonym.chars().any(|c| c.is_ascii_uppercase()));
        }
    }

    #[test]
    fn the_key_is_mixed_in_before_the_value() {
        // Guards against the classic bug where the key is appended, which
        // would make pseudonym(0, "AB") and pseudonym(hash_of("A"), "B")
        // interact. Two keys over the same value must be unrelated.
        let a: Vec<String> = (0..4).map(|key| pseudonym(key, "PATID1234")).collect();
        let mut sorted = a.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), a.len());
    }
}
