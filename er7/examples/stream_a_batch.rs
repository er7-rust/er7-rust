//! Read a batch file one message at a time, without holding the whole file
//! in memory.
//!
//! Run with: `cargo run --example stream_a_batch`
//!
//! Shows `read_messages` on the same `FHS`/`BHS`/`BTS`/`FTS` envelope
//! `split_a_batch` uses, why it hands back owned `String`s rather than
//! borrowed slices, and what happens when a source can fail mid-read.
//!
//! See `docs/usage/index.md` §9 and spec §9.5.
#![forbid(unsafe_code)]

use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The same batch `split_a_batch` reads, but here it stands in for
    // anything that implements `std::io::BufRead` — a file opened with
    // `BufReader::new(File::open(path)?)`, standard input, a socket.
    // `Cursor` is used here only because this example has no file of its
    // own to open.
    let batch = "FHS|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000\r\
                 BHS|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000\r\
                 MSH|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090000||ACK^A08^ACK|B1|P|2.5\r\
                 MSA|AA|MSG00001\r\
                 MSH|^~\\&|SENDER|SENDFAC|RECEIVER|RECFAC|20260815090005||ACK^A08^ACK|B2|P|2.5\r\
                 MSA|AE|MSG00002|Unknown patient identifier\r\
                 BTS|2\r\
                 FTS|1";

    // Memory use is bounded by the message currently being assembled, not
    // by the size of the input — unlike `split_messages`, which needs the
    // whole file in one `&str` before it can cut anything.
    let mut count = 0;
    for source in er7::read_messages(Cursor::new(batch)) {
        let text = source?; // an I/O failure or invalid UTF-8
        count += 1;
        match er7::parse(&text) {
            Ok(message) => println!(
                "{count}. {} acknowledges {}",
                message.control_id().unwrap_or_default(),
                message.query("MSA-2")?.unwrap_or_default(),
            ),
            Err(e) => eprintln!("{count}. skipping malformed message: {e}"),
        }
    }
    assert_eq!(count, 2);

    // --- Owned, not borrowed --------------------------------------------
    // `split_messages` borrows `&str` slices of its input, so cutting a
    // batch costs nothing beyond a `Vec` of spans — possible only because
    // the whole file already sits in one contiguous `&str`. A `BufRead`
    // offers no such thing: its internal buffer is overwritten as bytes
    // are consumed, so `read_messages` copies each message into an owned
    // `String` instead. Segments come back joined with `\r`
    // (`Terminator::Cr`), regardless of which terminator the source used —
    // there is nothing left to copy the original byte from.
    let mut messages = er7::read_messages(Cursor::new("MSH|^~\\&|A\nPID|1\n"));
    assert_eq!(messages.next().unwrap()?, "MSH|^~\\&|A\rPID|1");

    // --- An error ends the iteration, not just one item ------------------
    // Unlike `split_messages` (which cannot fail — cutting text never
    // touches an OS resource), `read_messages` reads from something that
    // can fail mid-stream. Once an item is `Err`, the next call returns
    // `None` rather than trying to resume from an unknown position.
    println!("ok");
    Ok(())
}
