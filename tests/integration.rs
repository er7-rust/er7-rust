//! Black-box tests through the public API only, exercising the same kind of
//! ER7 fixtures the sibling `er7` crate tests itself against — including
//! its own `samples/` files, read straight from the sibling checkout, so a
//! round trip through this crate is checked against real messages, not
//! just literals written for this crate.

use serde_er7::Message;

// The sample files end with a trailing segment terminator, which `er7`
// itself normalizes away at parse time (there is no trailing terminator by
// default — see `er7::RenderOptions`). Trim it here so the text we compare
// against is already the canonical form a plain `er7::parse(..).to_er7()`
// would produce, and the round-trip assertions below test this crate's
// JSON round trip specifically, not `er7`'s own normalization.
fn sample(text: &str) -> &str {
    text.trim_end_matches(['\r', '\n'])
}

const ORU: &str = include_str!("../../er7-rust/samples/oru_r01.er7");
const ADT: &str = include_str!("../../er7-rust/samples/adt_a08.er7");
const BATCH: &str = include_str!("../../er7-rust/samples/batch.er7");

fn round_trips_through_json(text: &str) {
    let text = sample(text);
    let message = Message::parse(text).expect("sample parses");
    let json = serde_json::to_string(&message).expect("serializes");
    let back: Message = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(
        back.to_er7(),
        text,
        "ER7 text changed after a JSON round trip"
    );
    assert_eq!(
        back, message,
        "wrapper equality changed after a JSON round trip"
    );
}

#[test]
fn round_trips_every_sample_from_the_er7_crate() {
    for sample in [ORU, ADT, BATCH] {
        round_trips_through_json(sample);
    }
}

#[test]
fn round_trips_pretty_printed_json_too() {
    // Pretty-printing changes only whitespace between JSON tokens, never
    // the values, so this must round-trip exactly like compact JSON.
    let message = Message::parse(sample(ORU)).unwrap();
    let json = serde_json::to_string_pretty(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), sample(ORU));
}

#[test]
fn keeps_absent_empty_and_null_distinct_through_json() {
    // R10/R11 in the `er7` spec: the explicit `""` means "clear this
    // value", never conflated with "not sent" or "sent blank". This has to
    // survive a JSON round trip exactly as it survives an ER7 one.
    let text = "MSH|^~\\&|LAB\rPID|1||\"\"|X";
    let message = Message::parse(text).unwrap();
    let pid = message.segment("PID").unwrap();
    assert!(pid.field(2).unwrap().is_empty()); // sent as `||`
    assert!(pid.field(3).unwrap().is_null()); // sent as `""`
    assert!(pid.field(9).is_none()); // never sent

    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    let pid = back.segment("PID").unwrap();
    assert!(pid.field(2).unwrap().is_empty());
    assert!(!pid.field(2).unwrap().is_null());
    assert!(pid.field(3).unwrap().is_null());
    assert!(!pid.field(3).unwrap().is_empty());
    assert!(pid.field(9).is_none());
}

#[test]
fn keeps_escape_sequences_raw_through_json() {
    // Subcomponents serialize `raw`, not `value`-decoded, so a JSON round
    // trip must not resolve `\T\` into `&` along the way.
    let text = r"MSH|^~\&|LAB|Smith \T\ Jones^X";
    let message = Message::parse(text).unwrap();
    let json = serde_json::to_string(&message).unwrap();
    assert!(json.contains(r"Smith \\T\\ Jones"), "raw text: {json}");
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
}

#[test]
fn round_trips_unusual_delimiters() {
    let text = "MSH#*!?@#LAB#*A*B#C!D";
    let message = Message::parse(text).unwrap();
    assert_eq!(message.separators.field, '#');
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
}

#[test]
fn round_trips_the_truncation_character() {
    let text = r"MSH|^~\&#|LAB";
    let message = Message::parse(text).unwrap();
    assert_eq!(message.separators.truncation, Some('#'));
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.separators.truncation, Some('#'));
    assert_eq!(back.to_er7(), text);
}

#[test]
fn preserves_repeated_fields_and_ragged_segments() {
    // Nothing below the header may fail, and that carries through JSON:
    // unknown segments and stray positions are data, both before and after
    // the round trip.
    let text = "MSH|^~\\&|LAB\rZZZ|1|LOCAL^EXTENSION\rOBX|1||A~~B";
    let message = Message::parse(text).unwrap();
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
    assert_eq!(
        back.query("OBX-3").unwrap().as_deref(),
        Some("A~~B"),
        "the repetition separator is preserved because the query stops above it"
    );
    assert_eq!(back.query("ZZZ-2.2").unwrap().as_deref(), Some("EXTENSION"));
}

#[test]
fn error_messages_still_name_the_missing_field() {
    let err = serde_json::from_str::<Message>(r#"{"segments":[]}"#).unwrap_err();
    assert!(err.to_string().contains("separators"), "{err}");
}
