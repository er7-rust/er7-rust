//! How redaction treats the three states HL7 keeps apart: a value that was
//! never sent, one sent blank, and one the sender is clearing.
//!
//! Getting this wrong is a patient-safety bug rather than a privacy one,
//! so it is worth seeing worked through.
//!
//! Run with: `cargo run --example redact_absent_empty_null`

use er7_redact::{Action, Policy, Redactor};

fn main() -> Result<(), er7_redact::Error> {
    // PID-1 has a value, PID-2 was sent blank, PID-3 is the explicit null,
    // and PID-4 onwards was never sent at all.
    let text = "MSH|^~\\&|LAB\rPID|1||\"\"";

    let policy = Policy::new()
        .with("PID-1", Action::redacted())?
        .with("PID-2", Action::redacted())?
        .with("PID-3", Action::redacted())?
        .with("PID-9", Action::redacted())?;

    let mut message = er7::parse(text)?;
    let report = Redactor::new(policy).redact(&mut message);

    // Only the field that carried a value changed.
    assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|REDACTED||\"\"");
    assert_eq!(report.len(), 1);

    // Why each of the other three was left alone:
    //
    // PID-2 was empty. Writing REDACTED into it would invent a value, and
    // would announce that one used to be there — which is a disclosure.
    assert!(message.segment("PID").unwrap().field(2).unwrap().is_empty());
    //
    // PID-3 is the explicit null: an instruction to the receiver to clear
    // its stored value, not patient data. Overwriting it would turn
    // "clear this" into a value, and leave a withdrawn record standing.
    assert!(message.segment("PID").unwrap().field(3).unwrap().is_null());
    //
    // PID-9 was never sent. Redaction does not lengthen a segment to reach
    // a position that is not there: padding would change what the message
    // says, and eleven new trailing pipes would announce the redaction.
    assert!(message.segment("PID").unwrap().field(9).is_none());

    // To *make* a position null — to tell the receiver to clear it — ask
    // for that, which is the one action that changes the shape of a
    // message, because an HL7 null is a single `""`.
    let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN")?;
    let policy = Policy::new().with("PID-5", Action::Null)?;
    Redactor::new(policy).redact(&mut message);
    assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||\"\"");

    // Compare with `clear`, which says nothing rather than saying "delete".
    let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN")?;
    let policy = Policy::new().with("PID-5", Action::Clear)?;
    Redactor::new(policy).redact(&mut message);
    assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1||9||^");

    println!("absent, empty, and null all survived redaction unchanged");
    Ok(())
}
