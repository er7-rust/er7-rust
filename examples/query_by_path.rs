//! Read values out of a message by HL7 path.
//!
//! Run with: `cargo run --example query_by_path`
//!
//! Shows the four query methods, occurrence indices for repeated segments
//! and repeated fields, and the two behaviours that surprise people: a
//! field-level path keeps its `~`, and a missing position yields nothing at
//! all.
//!
//! See `docs/paths/index.md` and spec §8.

fn main() -> Result<(), er7::Error> {
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01|MSG00042|P|2.5\r\
                PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E\
                ||19620320|F|||||555-1111~555-2222\r\
                OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL\r\
                OBX|2|NM|2571-8^Triglycerides^LN||102|mg/dL\r\
                OBX|3|ST|X-NOTE^Comment^L";

    let message = er7::parse(text)?;

    // --- The four levels -------------------------------------------------
    // A path that stops above the leaf returns that subtree as written,
    // structural delimiters intact. Only leaf text is decoded.
    assert_eq!(message.query("PID-5")?.as_deref(), Some("EVERYWOMAN^EVE^E"));
    assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
    assert_eq!(
        message.query("PID-3.4.2")?.as_deref(),
        Some("1.2.840.114398.1.100")
    );
    println!("patient: {}", message.query("PID-5.1")?.unwrap());

    // --- Repeated segments -----------------------------------------------
    // `query` gives the first match; `query_all` gives every one.
    assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));
    assert_eq!(
        message.query_all("OBX-3.2")?,
        vec!["Cholesterol", "Triglycerides", "Comment"]
    );

    // An occurrence index after the segment name pins one down.
    assert_eq!(message.query_all("OBX[2]-3.2")?, vec!["Triglycerides"]);
    for (index, name) in message.query_all("OBX-3.2")?.iter().enumerate() {
        println!("observation {}: {name}", index + 1);
    }

    // --- A missing position yields nothing --------------------------------
    // The third OBX carried no fifth field, so only two values come back.
    // This is not an error and not an empty string: there is no entry.
    assert_eq!(message.query_all("OBX-5")?, vec!["187", "102"]);
    assert_eq!(message.query("PID-99")?, None);
    assert_eq!(message.query("ZZZ-1")?, None);

    // --- Repeated fields --------------------------------------------------
    // Stopping at the field keeps the repetition separator...
    assert_eq!(
        message.query("PID-13")?.as_deref(),
        Some("555-1111~555-2222")
    );
    // ...going deeper splits into one answer per repetition.
    assert_eq!(message.query_all("PID-13.1")?, vec!["555-1111", "555-2222"]);
    // An occurrence index after the field number picks one.
    assert_eq!(message.query_all("PID-13[2].1")?, vec!["555-2222"]);

    // --- Reusing a parsed path -------------------------------------------
    // Parse once, apply many times. `Path` is also `Hash`, so a set of
    // paths can be map keys.
    let path: er7::Path = "OBX-3.2".parse()?;
    assert_eq!(path.to_string(), "OBX-3.2");
    assert_eq!(message.query_path(&path).len(), 3);

    // --- Raw versus decoded ----------------------------------------------
    // `query_path_raw` gives exactly what the sender wrote.
    let escaped = er7::parse(r"MSH|^~\&|LAB|Smith \T\ Jones")?;
    let path: er7::Path = "MSH-4".parse()?;
    assert_eq!(escaped.query_path(&path), vec!["Smith & Jones"]);
    assert_eq!(escaped.query_path_raw(&path), vec![r"Smith \T\ Jones"]);

    // --- Both spellings ---------------------------------------------------
    assert_eq!("PID.5.1".parse::<er7::Path>()?, "PID-5.1".parse()?);

    // --- Indices are 1-based ----------------------------------------------
    // A `0` is rejected rather than clamped: it is almost always an
    // off-by-one, and reading it as 1 would return a plausible wrong answer.
    assert!(matches!(
        "PID-0".parse::<er7::Path>(),
        Err(er7::Error::BadPath(_))
    ));

    println!("ok");
    Ok(())
}
