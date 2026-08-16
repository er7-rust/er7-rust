//! Go the other direction: start from JSON — the shape a web form or an API
//! request might hand you — and produce ER7 text a legacy HL7 receiver can
//! read.
//!
//! Run with: `cargo run --example build_message_from_json`
//!
//! See `docs/usage/index.md` §2.

use serde_er7::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hand-written JSON, the way it might arrive from a JSON API. Every
    // subcomponent is a bare string; every level above it is a plain array,
    // except the two objects: the message itself, and each segment.
    let json = r#"
    {
      "separators": {
        "field": "|", "component": "^", "repetition": "~",
        "escape": "\\", "subcomponent": "&", "truncation": null
      },
      "segments": [
        {
          "name": "MSH",
          "fields": [
            [[["|"]]],
            [[["^~\\&"]]],
            [[["LAB"]]],
            [[["ACME"]]],
            [],
            [],
            [[["20260815090000"]]],
            [],
            [[["ADT"], ["A08"], ["ADT_A01"]]],
            [[["MSG00001"]]],
            [[["P"]]],
            [[["2.5"]]]
          ]
        },
        {
          "name": "PID",
          "fields": [
            [[["1"]]],
            [],
            [[["555-44-4444"]]],
            [],
            [[["SMITH"], ["JOHN"]]]
          ]
        }
      ]
    }"#;

    let message: Message = serde_json::from_str(json)?;

    // Now it is a full er7::Message — Deref reaches straight through to
    // query, edit, and render it, no unwrapping needed.
    assert_eq!(message.query("PID-5.1")?.as_deref(), Some("SMITH"));
    assert_eq!(message.control_id().as_deref(), Some("MSG00001"));

    println!("{}", message.to_er7());
    Ok(())
}
