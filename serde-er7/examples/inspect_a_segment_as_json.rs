//! Serialize just one piece of a message — a single [`serde_er7::Segment`]
//! — without wrapping the whole [`serde_er7::Message`], to see the tree
//! shape each level chooses for itself.
//!
//! Run with: `cargo run --example inspect_a_segment_as_json`
//!
//! See `docs/usage/index.md` §3 for the full table of shapes.

use serde_er7::Segment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "MSH|^~\\&|LAB\r\
                PID|1||12345^^^ACME&1.2.3&ISO^MR||SMITH^JOHN^Q||19800101|M|||||\
                555-1111~555-2222";
    let message = er7::parse(text)?;
    let pid = message.segment("PID").unwrap().clone();

    // A lone Segment serializes just as it would nested inside a Message:
    // an object with "name" and "fields", fields as arrays all the way
    // down to bare subcomponent strings.
    let json = serde_json::to_string_pretty(&Segment(pid))?;
    println!("{json}");

    // PID-3.4 is a three-subcomponent assigning authority: namespace,
    // universal ID, universal ID type — joined by `&`, one level below
    // where PID-5's two components join by `^`.
    let value: serde_json::Value = serde_json::from_str(&json)?;
    assert_eq!(
        value["fields"][2][0][3],
        serde_json::json!(["ACME", "1.2.3", "ISO"])
    );
    // PID-13 repeats (`~`): two repetitions, each one component.
    assert_eq!(
        value["fields"][12],
        serde_json::json!([[["555-1111"]], [["555-2222"]]])
    );

    Ok(())
}
