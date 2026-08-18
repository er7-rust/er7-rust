//! Parse ER7, serialize to JSON, deserialize back, and confirm the ER7 text
//! that comes out the other end is byte-for-byte identical to what went in.
//!
//! Run with: `cargo run --example round_trip_via_json`
//!
//! This is the crate's flagship path: [`serde_er7::Message`] never mentions
//! JSON itself, so `serde_json` here is interchangeable with any other
//! Serde format — YAML, CBOR, `MessagePack`, whatever your pipeline already
//! uses. See `docs/usage/index.md` §1.

use serde_er7::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5\r\
                PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
                OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F";

    let message = Message::parse(text)?;
    println!("parsed {} segments", message.segments.len());

    // Out to JSON. `Message`'s Serialize impl is the only thing this line
    // depends on this crate for; `serde_json` does the rest.
    let json = serde_json::to_string_pretty(&message)?;
    println!("--- as JSON ---\n{json}\n");

    // Back from JSON, into a fresh Message.
    let back: Message = serde_json::from_str(&json)?;

    // What went in comes back out, byte for byte — the same guarantee
    // `er7::Message::to_er7` makes for plain ER7, now carried through a
    // JSON round trip too.
    assert_eq!(back.to_er7(), text);
    println!("--- round-tripped through JSON, unchanged ---");
    println!("{}", back.to_er7());

    Ok(())
}
