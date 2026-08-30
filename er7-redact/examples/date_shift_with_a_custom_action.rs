//! Shifting dates by a per-patient offset, with `Action::custom` — the
//! resolution of T5 ("date shifting"), not a built-in action.
//!
//! Run with: `cargo run --example date_shift_with_a_custom_action`
//!
//! `first 4` on a birth date keeps the year and destroys everything else,
//! including the interval to the next event — which is exactly what
//! longitudinal test data needs. Shifting every date in a message by the
//! same offset keeps every interval exact while still changing the actual
//! dates, and a *per-patient* offset means two different patients' dates
//! do not move by the same amount, which a single global shift would
//! otherwise leak.
//!
//! This crate does not parse HL7® timestamps itself — that is dictionary-
//! adjacent knowledge in the sense spec §5.3 draws the line, and pattern
//! matching on a value's *shape* was already declined for the same reason
//! (spec §16.2). `Action::custom` (D24, spec §3.8) means it does not have
//! to: the caller parses, shifts, and re-formats, and the crate only runs
//! the closure at the position a rule names.
//!
//! See spec §16.12 for the decision this example demonstrates.
#![forbid(unsafe_code)]

use er7_redact::{Action, Policy, Redactor, pseudonym};

fn main() -> Result<(), er7_redact::Error> {
    // Two messages for the same patient: an admission and a lab result,
    // each carrying a date at a different precision. `EVN-2` is
    // seconds-precision; `PID-7` is date-only.
    let admit = "MSH|^~\\&|ADT1|MCM|||||ADT^A01|MSG1|P|2.5\r\
                 EVN|A01|20260815140000\r\
                 PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE||19610615";
    let result = "MSH|^~\\&|LAB|ACME|||||ORU^R01|MSG2|P|2.5\r\
                  PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE||19610615\r\
                  OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL||||||||20260817093000";

    #[allow(clippy::unreadable_literal, reason = "the digits are a date")]
    let key = 20260815;

    for text in [admit, result] {
        let mut message = er7::parse(text)?;

        // The per-patient offset: where the "per-patient" part of T5's
        // question is answered. It comes from the message's own patient
        // identifier, computed once, before the policy for *this* message
        // is built — which is also the answer to T5's other open
        // question, "where the per-patient key comes from when the policy
        // is applied to one message at a time." There is no shared state
        // across messages: a second message for the same patient computes
        // the same offset from the same identifier, independently.
        let patient_id = message.query("PID-3.1")?.expect("sample carries one");
        let offset = day_offset_for(key, &patient_id);

        let policy = Policy::accept_all()
            .with("PID-7", Action::custom(move |v, _k| shift_date(v, offset)))?
            .with("EVN-2", Action::custom(move |v, _k| shift_date(v, offset)))?
            .with("OBX-14", Action::custom(move |v, _k| shift_date(v, offset)))?;

        Redactor::new(policy).redact(&mut message);

        let birth = message.query("PID-7")?;
        let event = message.query("EVN-2")?.or(message.query("OBX-14")?);

        println!(
            "{}: birth {birth:?}, event {event:?}",
            message.query("MSH-9")?.unwrap()
        );

        // Every date moved.
        assert_ne!(birth.as_deref(), Some("19610615"));

        // Precision survived: the date-only field is still 8 digits, and
        // the seconds-precision field is still 14, unchanged in shape even
        // though its content moved.
        assert_eq!(birth.as_deref().unwrap().len(), 8);
        if let Some(event) = &event {
            assert_eq!(event.len(), 14);
        }
    }

    // Both messages are the same patient, so both get the same offset —
    // computed independently each time, not carried over.
    let one = day_offset_for(key, "PATID1234");
    let two = day_offset_for(key, "PATID1234");
    assert_eq!(one, two);
    // A different patient gets a different offset (not guaranteed for
    // every possible identifier, but true for this pair, which is what
    // the assertion below is actually checking).
    assert_ne!(one, day_offset_for(key, "PATID9999"));

    // The interval between two dates in the same message is exactly
    // preserved: shifting is uniform, so the gap does not change even
    // though both endpoints moved.
    let before = days_from_civil(2026, 8, 15) - days_from_civil(1961, 6, 15);
    let shifted_admit = shift_date("20260815140000", one).unwrap();
    let shifted_birth = shift_date("19610615", one).unwrap();
    let after = days_from_civil(
        shifted_admit[0..4].parse().unwrap(),
        shifted_admit[4..6].parse().unwrap(),
        shifted_admit[6..8].parse().unwrap(),
    ) - days_from_civil(
        shifted_birth[0..4].parse().unwrap(),
        shifted_birth[4..6].parse().unwrap(),
        shifted_birth[6..8].parse().unwrap(),
    );
    assert_eq!(before, after);

    // What happens to a timestamp the action cannot parse: this example's
    // closure returns `None`, which leaves the leaf exactly as it was
    // sent — the same choice `Keep` makes, and the caller's to change. A
    // caller who wants the opposite default writes `Some(String::new())`
    // in the `None` branch instead, which clears it like `Action::Clear`.
    assert_eq!(shift_date("not a date", 5), None);
    let mut garbled = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH||not-a-date")?;
    let policy = Policy::accept_all().with("PID-7", Action::custom(|v, _k| shift_date(v, 5)))?;
    Redactor::new(policy).redact(&mut garbled);
    assert_eq!(garbled.query("PID-7")?.as_deref(), Some("not-a-date"));

    // `shift_date` keeps whatever precision it was given.
    assert_eq!(shift_date("20260815", 1).as_deref(), Some("20260816"));
    assert_eq!(
        shift_date("202608151400", 1).as_deref(),
        Some("202608161400")
    );
    assert_eq!(
        shift_date("20260815140030", -1).as_deref(),
        Some("20260814140030")
    );

    // And goes through a real calendar: 2028 is a leap year, so a naive
    // month/day-only shift would land on 2028-02-29 one day early or late.
    assert_eq!(shift_date("20280228", 1).as_deref(), Some("20280229"));
    assert_eq!(shift_date("20280229", 1).as_deref(), Some("20280301"));

    // The calendar conversion itself is worth checking harder than any one
    // message will: every day, not a sample, across two centuries either
    // side of the epoch.
    for days in -73000..73000 {
        let (y, m, d) = civil_from_days(days);
        assert_eq!(days_from_civil(y, m, d), days, "{y:04}-{m:02}-{d:02}");
    }

    println!("ok");
    Ok(())
}

/// Shift an HL7 `TS`-shaped value (`YYYYMMDD`, `YYYYMMDDHHMM`, or
/// `YYYYMMDDHHMMSS`) by `offset_days`, keeping whatever time-of-day
/// component was present unchanged and the same number of digits the
/// input had. `None` for anything else — a timezone suffix, a fractional
/// second, or text that is not a date at all — which is this example's
/// choice, not this crate's: see the `None` branch's caller for the
/// alternative.
fn shift_date(value: &str, offset_days: i64) -> Option<String> {
    if value.len() < 8 || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i64 = value[0..4].parse().ok()?;
    let month: u32 = value[4..6].parse().ok()?;
    let day: u32 = value[6..8].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let (y, m, d) = civil_from_days(days_from_civil(year, month, day) + offset_days);
    if !(0..=9999).contains(&y) {
        return None;
    }
    let mut out = format!("{y:04}{m:02}{d:02}");
    out.push_str(&value[8..]); // time-of-day, if any, is untouched
    Some(out)
}

/// A deterministic, patient-specific day offset in `[-1000, 1000]`, from
/// this crate's own [`pseudonym`] rather than a hand-rolled hash — the
/// same construction `Action::Pseudonym` already uses, reused here for a
/// number instead of a string.
fn day_offset_for(key: u64, patient_id: &str) -> i64 {
    let hash = pseudonym(key, patient_id);
    let n = u64::from_str_radix(&hash[..8], 16).expect("pseudonym is hex");
    #[allow(
        clippy::cast_possible_wrap,
        reason = "n % 2001 is in 0..2001, far below i64::MAX"
    )]
    let bounded = (n % 2001) as i64;
    bounded - 1000
}

/// Days since `1970-01-01` for a proleptic-Gregorian calendar date.
/// Howard Hinnant's widely used `days_from_civil` algorithm (public
/// domain); correct for every year this function's own range checks
/// admit.
#[allow(
    clippy::unreadable_literal,
    reason = "calendar-algorithm constants (days in an era, days in 400 years), not numbers a reader groups by three"
)]
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400; // [0, 399]
    let mp = (i64::from(m) + 9) % 12; // [0, 11]
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468
}

/// The inverse of [`days_from_civil`]: a day count since the epoch back
/// into `(year, month, day)`.
#[allow(
    clippy::unreadable_literal,
    reason = "calendar-algorithm constants, not numbers a reader groups by three"
)]
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = z - era * 146097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "doy and mp are non-negative and small here"
    )]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "mp is in [0, 11] here"
    )]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}
