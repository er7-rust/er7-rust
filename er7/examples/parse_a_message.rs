//! Parse an ER7 message and look around it.
//!
//! Run with: `cargo run --example parse_a_message`
//!
//! Shows the two entry points (`parse` and `parse_with`), the delimiters a
//! message declares about itself, and the five MSH accessors every
//! integration needs to route a message.
//!
//! See `docs/usage/index.md` §1 and spec §4.
#![forbid(unsafe_code)]

fn main() -> Result<(), er7::Error> {
    // A lab result. Segments end with a carriage return, which is what HL7
    // specifies; `\n` and `\r\n` are accepted too.
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5\r\
                PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
                OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F";

    let message = er7::parse(text)?;

    // Structure.
    assert_eq!(message.segments.len(), 3);
    assert_eq!(message.header().unwrap().name, "MSH");
    assert_eq!(message.segment("OBX").unwrap().name, "OBX");
    println!("segments: {}", message.segments.len());

    // The delimiters came from the message itself, not from a default.
    assert_eq!(message.separators, er7::Separators::default());
    println!("delimiters: {}", message.separators);

    // The five routing fields. Each is `None` when the position is absent
    // or empty, because for these five "sent blank" and "not sent" mean the
    // same thing to a caller.
    assert_eq!(message.message_code().as_deref(), Some("ORU"));
    assert_eq!(message.trigger_event().as_deref(), Some("R01"));
    assert_eq!(message.message_structure().as_deref(), Some("ORU_R01"));
    assert_eq!(message.control_id().as_deref(), Some("MSG00042"));
    assert_eq!(message.version().as_deref(), Some("2.5"));
    println!(
        "message: {}^{} {} (HL7 {})",
        message.message_code().unwrap_or_default(),
        message.trigger_event().unwrap_or_default(),
        message.control_id().unwrap_or_default(),
        message.version().unwrap_or_default(),
    );

    // A fragment with no header of its own: supply the delimiters yourself.
    // This cannot fail, so it returns `Message` rather than `Result`.
    let fragment = er7::parse_with("NTE|1||Fasting sample.", er7::Separators::default());
    assert_eq!(fragment.query("NTE-3")?.as_deref(), Some("Fasting sample."));
    println!("fragment note: {:?}", fragment.query("NTE-3")?.unwrap());

    // Nothing below the header can fail: unknown segments, ragged fields,
    // and stray positions are all data.
    let odd = "MSH|^~\\&|LAB\rZPD|1|LOCAL^EXTENSION\rXYZ";
    let odd = er7::parse(odd)?;
    assert_eq!(odd.segments.len(), 3);
    assert_eq!(odd.query("ZPD-2.2")?.as_deref(), Some("EXTENSION"));
    println!("local segment: {:?}", odd.query("ZPD-2.2")?.unwrap());

    println!("ok");
    Ok(())
}
