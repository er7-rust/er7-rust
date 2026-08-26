//! The report: what a redaction did, in a form that can be pasted into a
//! ticket without a second thought.
//!
//! Run with: `cargo run --example read_the_report`
#![forbid(unsafe_code)]

use er7_redact::{Policy, Redactor};

fn main() -> Result<(), er7_redact::Error> {
    let text = "MSH|^~\\&|ADT1|MCM||||||ADT^A08|MSG1|P|2.5\r\
                PID|1||PATID1234^^^ADT1^MR||EVERYWOMAN^EVE^E||19610615|F|||\
                12 ELM ST^^BOSTON^MA^02101||555-555-1111~555-555-2222\r\
                NK1|1|EVERYMAN^ADAM|SPO";

    let mut message = er7::parse(text)?;
    let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);

    for change in &report.changes {
        println!("{:<18} {}", change.path.to_string(), change.action);
    }

    // One row per leaf that actually changed — so a name in three
    // components is three rows, and a repeated field is one row per
    // repetition.
    let paths: Vec<String> = report.changes.iter().map(|c| c.path.to_string()).collect();
    assert!(paths.contains(&"PID[1]-5[1].1.1".to_string()));
    assert!(paths.contains(&"PID[1]-5[1].3.1".to_string()));
    assert!(paths.contains(&"PID[1]-13[2].1.1".to_string()));

    // Every path is fully qualified, which means it is also a valid
    // `er7 --query` argument: paste one in to see what is there now.
    for change in &report.changes {
        assert!(message.query(&change.path.to_string()).is_ok());
    }

    // A report carries no values — not the old text, and not the new. A
    // log line quoting the old value puts the patient's name into the log.
    let printed = report.to_string();
    for value in ["EVERYWOMAN", "PATID1234", "19610615", "BOSTON"] {
        assert!(!printed.contains(value));
    }

    // And nothing that was not there contributes a row: this message has
    // no GT1 segment, so the guarantor rules matched nothing, which is not
    // an error.
    assert!(!paths.iter().any(|p| p.starts_with("GT1")));

    println!("\n{} positions changed", report.len());
    Ok(())
}
