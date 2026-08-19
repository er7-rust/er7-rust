//! Read a batch file, one message at a time.
//!
//! Run with: `cargo run --example split_a_batch`
//!
//! Shows `split_messages` on a full `FHS`/`BHS`/`BTS`/`FTS` envelope, that
//! the results borrow from the input rather than copying, and how to keep
//! going when one message in a batch is malformed.
//!
//! See `docs/usage/index.md` §9 and spec §9.

fn main() -> Result<(), er7::Error> {
    // A batch file: a file header, a batch header, two messages, and the
    // matching trailers.
    let batch = "FHS|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000\r\
                 BHS|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000\r\
                 MSH|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000||ACK^A08^ACK|B1|P|2.5\r\
                 MSA|AA|MSG00001\r\
                 MSH|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090005||ACK^A08^ACK|B2|P|2.5\r\
                 MSA|AE|MSG00002|Unknown patient identifier\r\
                 ERR||PID^1^3|101^Required field missing^HL70357|E\r\
                 BTS|2\r\
                 FTS|1";

    // The envelope segments describe the file, not any message in it, so
    // they are dropped. Two messages come out.
    let sources = er7::split_messages(batch);
    assert_eq!(sources.len(), 2);

    // The results borrow from the input and keep its original terminators,
    // so each can go straight to `parse` with no copy.
    assert!(sources[0].starts_with("MSH|^~\\&|SENDER"));
    assert!(sources[0].ends_with("MSA|AA|MSG00001"));

    for (index, source) in sources.iter().enumerate() {
        let message = er7::parse(source)?;
        let acknowledged = message.query("MSA-2")?.unwrap_or_default();
        let code = message.query("MSA-1")?.unwrap_or_default();
        let outcome = match code.as_str() {
            "AA" => "accepted",
            "AE" => "rejected: application error",
            "AR" => "rejected: application reject",
            other => other,
        };
        println!(
            "{}. {} acknowledges {acknowledged}: {outcome}",
            index + 1,
            message.control_id().unwrap_or_default(),
        );
        if let Some(text) = message.query("ERR-3.2")? {
            println!("   error: {text}");
        }
    }

    assert_eq!(
        er7::parse(sources[0])?.query("MSA-1")?.as_deref(),
        Some("AA")
    );
    assert_eq!(
        er7::parse(sources[1])?.query("ERR-2.1")?.as_deref(),
        Some("PID")
    );

    // --- Carrying on past a bad message ------------------------------------
    // A batch can hold one message that will not parse. Handle each
    // independently rather than failing the whole file.
    let mixed = "PID|1|this message has no header\r\
                 MSH|^~\\&|A||||1||ACK|GOOD|P|2.5\rMSA|AA|1";
    let mut good = 0;
    let mut bad = 0;
    for (index, source) in er7::split_messages(mixed).iter().enumerate() {
        match er7::parse(source) {
            Ok(message) => {
                good += 1;
                println!("message {}: {:?}", index + 1, message.control_id());
            }
            Err(e) => {
                bad += 1;
                eprintln!("message {}: skipping: {e}", index + 1);
            }
        }
    }
    assert_eq!((good, bad), (1, 1));

    // Note the malformed message was still reported, not silently dropped.
    // A caller reconciling against a `BTS` count needs to know it was there.

    // --- A local segment is not a batch trailer ---------------------------
    // Envelope names are matched exactly, so `BTSX` is a local segment.
    let local = "MSH|^~\\&|A\rBTSX|1";
    assert_eq!(er7::split_messages(local), vec![local]);

    // --- What this does not do ---------------------------------------------
    // MLLP framing (the 0x0B / 0x1C 0x0D bytes around a message on a socket)
    // is transport, and out of scope. Strip it before parsing.

    println!("ok");
    Ok(())
}
