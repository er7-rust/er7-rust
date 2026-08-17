//! Why an identifier becomes a pseudonym rather than a blank, what that
//! buys, and what it costs.
//!
//! Run with: `cargo run --example pseudonyms_and_linkage`

use er7_redact::{Action, Policy, Redactor, pseudonym};

fn main() -> Result<(), er7_redact::Error> {
    let admit = "MSH|^~\\&|ADT1|MCM||||||ADT^A01|MSG1|P|2.5\r\
                 PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE";
    let result = "MSH|^~\\&|LAB|ACME||||||ORU^R01|MSG2|P|2.5\r\
                  PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE\r\
                  OBX|1|NM|2093-3^Cholesterol^LN||187";

    let redactor = Redactor::new(Policy::patient_identifiers()).with_key(20260815);

    let mut admit = er7::parse(admit)?;
    let mut result = er7::parse(result)?;
    redactor.redact(&mut admit);
    redactor.redact(&mut result);

    // The record number is gone from both...
    let one = admit.query("PID-3.1")?.expect("a value");
    let two = result.query("PID-3.1")?.expect("a value");
    assert_ne!(one, "PATID1234");

    // ...and the two messages still agree that this is the same patient,
    // which is what makes them useful as a test case.
    assert_eq!(one, two);
    println!("both messages now say PID-3.1 = {one}");

    // A different key produces an unrelated mapping, so two data sets
    // redacted under different keys cannot be joined.
    let mut other = er7::parse("MSH|^~\\&|ADT1|MCM\rPID|1||PATID1234^^^ADT1^MR")?;
    Redactor::new(Policy::patient_identifiers())
        .with_key(1)
        .redact(&mut other);
    assert_ne!(other.query("PID-3.1")?.expect("a value"), one);

    // The function is available directly, for building an expected value
    // in a test.
    assert_eq!(pseudonym(20260815, "PATID1234"), one);

    // What it costs. A pseudonym preserves equality on purpose, so anyone
    // holding the redacted data can count how many messages each patient
    // generated — and anyone holding the key can invert the mapping by
    // trying every candidate identifier, because record numbers come from
    // small spaces. Inside your own trust boundary, that is a fair trade;
    // for data leaving it, clear the value instead.
    let mut leaving = er7::parse("MSH|^~\\&|ADT1|MCM\rPID|1||PATID1234^^^ADT1^MR")?;
    let policy = Policy::patient_identifiers().with("PID-3.1", Action::Clear)?;
    Redactor::new(policy).redact(&mut leaving);
    assert_eq!(leaving.query("PID-3.1")?.as_deref(), Some(""));

    Ok(())
}
