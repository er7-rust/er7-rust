//! Change a message and write it back out.
//!
//! Run with: `cargo run --example edit_a_value`
//!
//! Shows why edits go through `Subcomponent::set` rather than assigning
//! `raw`, how to make a structural edit through the public `Vec` fields,
//! and the round-trip guarantee that makes any of it safe.
//!
//! See `docs/usage/index.md` §7–8 and spec §5.5, §7.
#![forbid(unsafe_code)]

use er7::{Component, RenderOptions, Repetition, Subcomponent, Terminator};

fn main() -> Result<(), er7::Error> {
    let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ADT^A08|MSG1|P|2.5\r\
                PID|1||444333222^^^ACME^MR||SMITH^JOHN||19800101|M";

    // --- The round trip ---------------------------------------------------
    // Canonical input comes back byte for byte. This is what makes editing
    // safe: whatever you did not touch is untouched.
    let mut message = er7::parse(text)?;
    assert_eq!(message.to_er7(), text);

    // --- Editing a value --------------------------------------------------
    // `set` encodes delimiters on the way in, so a value can never break the
    // structure that holds it. Here the `&` becomes `\T\`.
    let separators = message.separators;
    message
        .segment_at_mut("PID", 1)
        .and_then(|segment| segment.field_mut(5))
        .and_then(|field| field.repetition_mut(1))
        .and_then(|repetition| repetition.component_mut(1))
        .and_then(|component| component.subcomponent_mut(1))
        .expect("PID-5.1 was sent")
        .set("O'BRIEN & SONS", &separators);

    assert!(message.to_er7().contains(r"O'BRIEN \T\ SONS^JOHN"));
    assert_eq!(message.query("PID-5.1")?.as_deref(), Some("O'BRIEN & SONS"));
    println!("edited name: {}", message.query("PID-5")?.unwrap());

    // The edited message still parses to the same tree it was written from.
    assert_eq!(er7::parse(&message.to_er7())?, message);

    // --- Why not assign `raw` directly ------------------------------------
    // You may, but then the escaping is your problem. An unescaped `&` here
    // would split the component in two the next time the message was
    // parsed, shifting every value after it.
    let mut careless = er7::parse(text)?;
    careless
        .segment_at_mut("PID", 1)
        .and_then(|segment| segment.field_mut(5))
        .and_then(|field| field.repetition_mut(1))
        .and_then(|repetition| repetition.component_mut(1))
        .and_then(|component| component.subcomponent_mut(1))
        .expect("PID-5.1 was sent")
        .raw = "O'BRIEN & SONS".to_string();

    // One component before the round trip...
    assert_eq!(
        careless
            .segment("PID")
            .and_then(|segment| segment.component(5, 1))
            .map(|component| component.subcomponents.len()),
        Some(1)
    );
    // ...and two after it. The message was silently corrupted.
    let reparsed = er7::parse(&careless.to_er7())?;
    assert_eq!(
        reparsed
            .segment("PID")
            .and_then(|segment| segment.component(5, 1))
            .map(|component| component.subcomponents.len()),
        Some(2)
    );
    println!("careless edit split one component into two — use `set`");

    // --- A structural edit ------------------------------------------------
    // Every field of every type is public, so adding a repetition is just a
    // `Vec::push`. There is no builder API; `Vec` already has one.
    message
        .segment_at_mut("PID", 1)
        .and_then(|segment| segment.field_mut(5))
        .expect("PID-5 was sent")
        .repetitions
        .push(Repetition {
            components: vec![Component {
                subcomponents: vec![Subcomponent::new("OBRIEN")],
            }],
        });
    assert_eq!(
        message.query("PID-5")?.as_deref(),
        Some(r"O'BRIEN & SONS^JOHN~OBRIEN")
    );

    // --- Adding a segment -------------------------------------------------
    let note = er7::parse_with("NTE|1||Name corrected.", separators)
        .segments
        .remove(0);
    message.segments.push(note);
    assert_eq!(message.query("NTE-3")?.as_deref(), Some("Name corrected."));

    // --- Writing it out ---------------------------------------------------
    // The default is carriage returns and no trailing terminator. For strict
    // wire output, terminate every segment including the last.
    let wire = message.to_er7_with(RenderOptions {
        terminator: Terminator::Cr,
        trailing_terminator: true,
    });
    assert!(wire.ends_with('\r'));

    // For something a terminal can show, use line feeds.
    let readable = message.to_er7_with(RenderOptions {
        terminator: Terminator::Lf,
        trailing_terminator: true,
    });
    print!("{readable}");

    println!("ok");
    Ok(())
}
