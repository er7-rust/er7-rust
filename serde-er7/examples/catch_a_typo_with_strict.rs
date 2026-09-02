//! Catch a typo in a hand-written JSON fixture with `Strict<Message>`,
//! instead of it either failing with a generic error or, worse, silently
//! accepting the mistake.
//!
//! Run with: `cargo run --example catch_a_typo_with_strict`
//!
//! See `spec/11-strict-mode/index.md` (rule S13) and `docs/api/index.md`
//! §"Strict deserialization".
#![forbid(unsafe_code)]

use serde_er7::{Message, Strict};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // "feilds" — a typo of "fields" — on the PID segment. This is a
    // *required* key, so the plain type already refuses this: it just
    // cannot say why, since it never sees "feilds" as anything other than
    // an ignored, unrecognized key (S8).
    let required_key_typo = r#"
    {
      "separators": {
        "field": "|", "component": "^", "repetition": "~",
        "escape": "\\", "subcomponent": "&", "truncation": null
      },
      "segments": [
        {"name": "PID", "feilds": [[[["1"]]], [], [[["555-44-4444"]]]]}
      ]
    }"#;

    let plain: Result<Message, _> = serde_json::from_str(required_key_typo);
    let strict: Result<Strict<Message>, _> = serde_json::from_str(required_key_typo);
    println!("A typo on a required key (\"feilds\" for \"fields\"):");
    println!("  Message::deserialize:        {}", describe(&plain));
    println!("  Strict<Message>::deserialize: {}", describe(&strict));
    assert!(plain.is_err());
    assert!(strict.is_err());

    // "truncatoin" — a typo of "truncation" — the one *optional* key this
    // crate has. This is the case S8's tolerance genuinely hides: the plain
    // type does not merely give an unhelpful error, it gives no error at
    // all, silently treating the key as absent and defaulting to `None`.
    let optional_key_typo = r#"
    {
      "separators": {
        "field": "|", "component": "^", "repetition": "~",
        "escape": "\\", "subcomponent": "&", "truncatoin": "%"
      },
      "segments": []
    }"#;

    let plain: Result<Message, _> = serde_json::from_str(optional_key_typo);
    let strict: Result<Strict<Message>, _> = serde_json::from_str(optional_key_typo);
    println!("\nA typo on the one optional key (\"truncatoin\" for \"truncation\"):");
    println!("  Message::deserialize:        {}", describe(&plain));
    println!("  Strict<Message>::deserialize: {}", describe(&strict));
    assert!(plain.is_ok(), "the plain type accepts this silently");
    assert_eq!(
        plain?.separators.truncation, None,
        "and the typo'd '%' is lost"
    );
    assert!(strict.is_err(), "Strict<Message> reports it instead");

    Ok(())
}

fn describe<T>(result: &Result<T, serde_json::Error>) -> String {
    match result {
        Ok(_) => "Ok(_)".to_string(),
        Err(e) => format!("Err({e})"),
    }
}
