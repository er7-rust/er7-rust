//! Build an ACK from scratch — the case a builder API would exist for, if
//! this crate had one.
//!
//! Run with: `cargo run --example build_a_message`
//!
//! There is no `Message::builder()`, and spec §5.5 says why: an ACK almost
//! always echoes fields the inbound message already carries — the sending
//! and receiving application swap places, the control ID being
//! acknowledged is copied — so by the time it is written it is known text
//! with a few values spliced in, and `parse_with` turns text into segments.
//! Parsing *is* the builder, for anything you can already express as ER7.
//!
//! See `docs/usage/index.md` §7–8, `examples/edit_a_value.rs` for the other
//! tool (the public `Vec` fields, for a value that is not text yet), and
//! spec §5.5.
#![forbid(unsafe_code)]

fn main() -> Result<(), er7::Error> {
    // --- The message being acknowledged ------------------------------------
    let inbound = er7::parse(
        "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01|MSG00042|P|2.5\r\
         PID|1||444333222^^^ACME^MR||SMITH^JOHN",
    )?;
    let separators = inbound.separators;

    // --- MSH: mostly copied, one field genuinely new ------------------------
    // Sending and receiving swap places; the timestamp and the ACK's own
    // control ID are the only values this segment did not already have.
    let sending_application = inbound.query("MSH-5")?.unwrap_or_default();
    let sending_facility = inbound.query("MSH-6")?.unwrap_or_default();
    let receiving_application = inbound.query("MSH-3")?.unwrap_or_default();
    let receiving_facility = inbound.query("MSH-4")?.unwrap_or_default();
    let control_id = inbound.query("MSH-10")?.unwrap_or_default();

    let header_text = format!(
        "MSH|^~\\&|{sending_application}|{sending_facility}|{receiving_application}|\
         {receiving_facility}|20260815081501||ACK|ACK00042|P|2.5"
    );
    let msh = er7::parse_with(&header_text, separators).segments.remove(0);

    // --- MSA: entirely new, and still just text ------------------------------
    let msa_text = format!("MSA|AA|{control_id}");
    let msa = er7::parse_with(&msa_text, separators).segments.remove(0);

    // --- Assembling the message -----------------------------------------------
    // Every field of `Message` is `pub`; this literal is the whole
    // constructor there is (spec §5.5).
    let ack = er7::Message {
        separators,
        segments: vec![msh, msa],
    };

    assert_eq!(
        ack.to_er7(),
        "MSH|^~\\&|EHR|CLINIC|LAB|ACME|20260815081501||ACK|ACK00042|P|2.5\r\
         MSA|AA|MSG00042"
    );
    assert_eq!(ack.query("MSA-1")?.as_deref(), Some("AA"));
    assert_eq!(ack.query("MSA-2")?.as_deref(), Some("MSG00042"));
    println!("{}", ack.to_er7());

    // A message built this way is a message like any other: it round-trips.
    assert_eq!(er7::parse(&ack.to_er7())?, ack);

    // --- Why not a builder ------------------------------------------------
    // Nothing above needed `Field::default()`, `Repetition::default()`, or
    // a single nested struct literal — every value here was known text, so
    // `parse_with` did the assembling. A field built from pieces that are
    // not text yet (a repetition assembled from parts already in hand,
    // rather than a string) goes through the public `Vec` fields instead —
    // see the "structural edit" section of `examples/edit_a_value.rs`. The
    // crate offers exactly those two tools and no third; wrapping either
    // in a builder would only add surface without adding meaning.
    println!("ok");
    Ok(())
}
