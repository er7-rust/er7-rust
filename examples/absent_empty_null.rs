//! Tell "not sent" from "sent blank" from "clear this value".
//!
//! Run with: `cargo run --example absent_empty_null`
//!
//! The sharpest edge in the whole format. Treating a null as empty means a
//! withdrawn allergy stays on the record; treating an empty as a null means
//! a value that was never sent erases one that was.
//!
//! See `docs/usage/index.md` §5 and spec §5.3.

fn main() -> Result<(), er7::Error> {
    // PID-2 was sent empty, PID-3 was sent as the explicit null, and PID-9
    // was never sent at all.
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ADT^A08|MSG1|P|2.5\r\
                PID|1||\"\"|4|SMITH^JOHN";
    let message = er7::parse(text)?;
    let pid = message.segment("PID").expect("the message has a PID");

    // --- Absent -----------------------------------------------------------
    // The accessor returns None. Nothing was said about this field.
    assert!(pid.field(9).is_none());

    // --- Empty ------------------------------------------------------------
    // Sent as `||`: present in the message, carrying no value.
    let empty = pid.field(2).expect("PID-2 was sent");
    assert!(empty.is_empty());
    assert!(!empty.is_null());

    // --- Null -------------------------------------------------------------
    // Sent as `""`: two quote characters, meaning "clear the stored value".
    let null = pid.field(3).expect("PID-3 was sent");
    assert!(null.is_null());
    // The null is *text*, so it is not empty. These are never both true.
    assert!(!null.is_empty());

    // --- What a receiver should do ----------------------------------------
    for number in [2, 3, 9] {
        let action = match pid.field(number) {
            None => "absent — leave the stored value alone",
            Some(field) if field.is_null() => "null — CLEAR the stored value",
            Some(field) if field.is_empty() => "empty — leave the stored value alone",
            Some(_) => "a value — store it",
        };
        println!("PID-{number}: {action}");
    }

    // --- Why a query alone is not enough ----------------------------------
    // `value` and `query` report a null as the empty string, because that is
    // the value being conveyed. So a query cannot distinguish null from
    // empty, and code that writes to a record must ask the node.
    assert_eq!(message.query("PID-2")?.as_deref(), Some(""));
    assert_eq!(message.query("PID-3")?.as_deref(), Some(""));
    assert_eq!(message.query("PID-9")?, None);

    // The raw form does distinguish, if you would rather stay with paths.
    let path: er7::Path = "PID-3".parse()?;
    assert_eq!(message.query_path_raw(&path), vec![r#""""#]);
    assert_eq!(
        message.query_path_raw(&"PID-2".parse::<er7::Path>()?),
        vec![""]
    );
    println!("PID-3 as sent: {:?}", message.query_path_raw(&path)[0]);

    // --- All three survive a round trip -----------------------------------
    assert_eq!(message.to_er7(), text);

    // --- The distinction runs all the way down ----------------------------
    // `is_empty` and `is_null` are defined at every level. A field is null
    // only for the precise shape `|""|`, never for `|""^X|`.
    let mixed = er7::parse("MSH|^~\\&|LAB\rPID|\"\"^X")?;
    let field = mixed.segment("PID").unwrap().field(1).unwrap();
    assert!(!field.is_null());
    assert!(field.repetition(1).unwrap().component(1).unwrap().is_null());

    // --- Empty positions hold their places --------------------------------
    // Position is what gives a value meaning, so nothing is collapsed. A
    // field left out entirely has no repetitions; a repetition sent blank is
    // still a repetition.
    let positions = er7::parse("MSH|^~\\&|LAB\rPID||A~~B|^^C")?;
    let pid = positions.segment("PID").unwrap();
    assert_eq!(pid.field(1).unwrap().repetitions.len(), 0); // `||`
    assert_eq!(pid.field(2).unwrap().repetitions.len(), 3); // `A~~B`
    assert_eq!(
        pid.field(3)
            .unwrap()
            .repetition(1)
            .unwrap()
            .components
            .len(),
        3
    );
    assert_eq!(positions.to_er7(), "MSH|^~\\&|LAB\rPID||A~~B|^^C");

    println!("ok");
    Ok(())
}
