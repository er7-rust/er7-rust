//! Redact a message with the built-in policy, and see what survived.
//!
//! Run with: `cargo run --example redact_a_message`

use er7_redact::{Policy, Redactor};

fn main() -> Result<(), er7_redact::Error> {
    let text = "MSH|^~\\&|ADT1|MCM|LABADT|MCM|20260815140000||ADT^A08|MSG00001|P|2.5\r\
                PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE^E||19610615|F|||\
                1200 N ELM STREET^^GREENSBORO^NC^27401-1020||(919)379-1212\r\
                OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|100-199|H|||F";

    let mut message = er7::parse(text)?;
    let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);

    println!("{}\n", message.to_er7().replace('\r', "\n"));
    println!("{} positions changed:\n{report}", report.len());

    // The identifiers are gone.
    assert_eq!(
        message.query("PID-5")?.as_deref(),
        Some("REDACTED^REDACTED^REDACTED")
    );
    assert_eq!(message.query("PID-7")?.as_deref(), Some("1961"));
    assert_eq!(message.query("PID-11")?.as_deref(), Some("^^^^"));
    assert_eq!(message.query("PID-13")?.as_deref(), Some(""));
    assert_ne!(message.query("PID-3.1")?.as_deref(), Some("PATID1234"));

    // The message is not.
    assert_eq!(message.control_id().as_deref(), Some("MSG00001"));
    assert_eq!(message.query("PID-3.4")?.as_deref(), Some("ADT1")); // assigning authority
    assert_eq!(message.query("PID-8")?.as_deref(), Some("F")); // not an identifier
    assert_eq!(message.query("OBX-5")?.as_deref(), Some("187")); // the clinical content
    assert!(er7::parse(&message.to_er7()).is_ok());

    // And the shape did not move: every position that held a value still
    // exists, so a test that asserted on PID-11.3 still finds it.
    assert_eq!(message.query("PID-11.3")?.as_deref(), Some(""));

    Ok(())
}
