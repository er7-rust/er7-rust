//! The other posture: reject every value by default, and name what to
//! accept.
//!
//! Use this when the message is unfamiliar — full of `Z` segments nobody
//! documented, say — and the answer to "is there anything else in here?"
//! has to be "no" rather than "not that I listed".
//!
//! Run with: `cargo run --example reject_by_default`

use er7_redact::{Action, Policy, Redactor};

fn main() -> Result<(), er7_redact::Error> {
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
                PID|1||PATID1234||EVERYWOMAN^EVE||19610615|F\r\
                OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|100-199|H|||F\r\
                ZPD|1|LOCAL^EXTENSION^SEGMENT";

    // Rejecting by default covers every leaf no rule named; a `keep` rule
    // accepts a position, exempting it from that. A rejecting rule would
    // beat the `keep` whichever order the two were written in (D19).
    let policy = Policy::all_but_the_header()
        .with("OBX-2", Action::Keep)? // value type
        .with("OBX-3", Action::Keep)? // observation identifier
        .with("OBX-5", Action::Keep)? // the number the test asserts on
        .with("OBX-6", Action::Keep)?; // units

    let mut message = er7::parse(text)?;
    let report = Redactor::new(policy).redact(&mut message);
    println!("{}\n", message.to_er7().replace('\r', "\n"));

    // The header is untouched, so the message still routes and still says
    // which version it is.
    assert_eq!(message.query("MSH-9")?.as_deref(), Some("ORU^R01"));
    assert_eq!(message.version().as_deref(), Some("2.5"));

    // The result is intact, because four rules said so.
    assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));
    assert_eq!(message.query("OBX-5")?.as_deref(), Some("187"));

    // Everything else is gone — including the local segment, which no
    // curated policy could have known about.
    assert_eq!(message.query("PID-5.1")?.as_deref(), Some("REDACTED"));
    assert_eq!(message.query("ZPD-2.1")?.as_deref(), Some("REDACTED"));

    // The cost: values a positional policy would have kept are gone too.
    assert_eq!(message.query("PID-8")?.as_deref(), Some("REDACTED"));
    assert_eq!(message.query("OBX-8")?.as_deref(), Some("REDACTED"));

    println!("{} positions changed", report.len());

    // And what this posture is really for: a payload that is not ER7 at
    // all has no positions to name, so the policy says outright what
    // becomes of it. The curated policy above refuses one, which is what
    // makes the CLI exit non-zero rather than write it out.
    let redactor = Redactor::new(Policy::all_but_the_header());
    assert_eq!(redactor.unrecognised("{\"name\": \"EVERYWOMAN\"}"), None);

    // `Policy::reject_all` masks it whole instead — nothing routable
    // survives, and neither does anything else.
    let redactor = Redactor::new(Policy::reject_all());
    assert_eq!(
        redactor.unrecognised("EVERYWOMAN").as_deref(),
        Some("**********")
    );
    Ok(())
}
