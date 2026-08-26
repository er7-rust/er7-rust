//! Work with a message that chose its own delimiters.
//!
//! Run with: `cargo run --example custom_delimiters`
//!
//! Nothing in this crate hardcodes `|^~\&`. Shows a message using a
//! completely different set, the HL7® v2.7 truncation character, the
//! fallbacks for encoding characters a sender omits, and the one thing that
//! is rejected: an ambiguous set.
//!
//! See spec §3.
#![forbid(unsafe_code)]

use er7::{Error, Separators};

fn main() -> Result<(), Error> {
    // --- A message that uses none of the usual characters -----------------
    // MSH-1 is `#`; MSH-2 declares `*` component, `!` repetition,
    // `?` escape, `@` subcomponent.
    let text = "MSH#*!?@#LAB#*ACME#SMITH*JOHN@JR!DOE*JANE";
    let message = er7::parse(text)?;

    assert_eq!(message.separators.field, '#');
    assert_eq!(message.separators.component, '*');
    assert_eq!(message.separators.repetition, '!');
    assert_eq!(message.separators.escape, '?');
    assert_eq!(message.separators.subcomponent, '@');
    println!("delimiters: {}", message.separators);

    // Everything works exactly as it does for a conventional message.
    assert_eq!(message.query("MSH-5.1")?.as_deref(), Some("SMITH"));
    assert_eq!(message.query("MSH-5.2.2")?.as_deref(), Some("JR"));
    assert_eq!(message.query_all("MSH-5[2].1")?, vec!["DOE"]);

    // Including the round trip.
    assert_eq!(message.to_er7(), text);
    println!("round trip holds for custom delimiters");

    // --- The truncation character (HL7 v2.7) ------------------------------
    // A fifth encoding character marks values the sender cut short to fit a
    // length limit. It is `Option<char>`, because "declared" and "not
    // declared" are different facts.
    let v27 = er7::parse(r"MSH|^~\&#|LAB|ACME|||20260815||ADT^A08|N1|P|2.7")?;
    assert_eq!(v27.separators.truncation, Some('#'));
    assert_eq!(v27.separators.encoding_characters(), r"^~\&#");
    assert_eq!(v27.version().as_deref(), Some("2.7"));

    // Most messages omit it.
    assert_eq!(Separators::default().truncation, None);

    // --- Omitted encoding characters fall back ----------------------------
    // A sender writing `MSH|^~\|` supplied three encoding characters, not
    // four: reading stops at the field separator. The subcomponent
    // separator falls back to the recommended `&`.
    let partial = Separators::from_header(r"MSH|^~\|LAB")?;
    assert_eq!(partial.escape, '\\');
    assert_eq!(partial.subcomponent, '&');
    assert_eq!(partial.truncation, None);
    println!("partial header resolved to: {partial}");

    // --- What is rejected --------------------------------------------------
    // Only sets that make parsing *ambiguous*, never sets that are merely
    // unusual. The same character cannot mean two things.
    assert!(matches!(
        Separators::from_header(r"MSH|^^\&|LAB"),
        Err(Error::BadHeader(_))
    ));
    assert!(matches!(
        Separators::from_header("MSH|^~&&|LAB"),
        Err(Error::BadHeader(_))
    ));

    // An alphanumeric field separator means this is not really a header.
    assert!(matches!(
        Separators::from_header("MSHX^~\\&"),
        Err(Error::BadHeader(_))
    ));

    // A delimiter that would end a segment is unusable.
    assert!(
        Separators {
            component: '\r',
            ..Separators::default()
        }
        .validate()
        .is_err()
    );

    if let Err(e) = Separators::from_header(r"MSH|^^\&|LAB") {
        println!("rejected: {e}");
    }

    // --- Escaping follows the message -------------------------------------
    // Every escape function takes the delimiter set, so a message using `?`
    // as its escape character writes `?F?` where a conventional one writes
    // `\F\`.
    let separators = message.separators;
    assert_eq!(er7::escape::escape("a#b", &separators), "a?F?b");
    assert_eq!(er7::escape::unescape("a?F?b", &separators), "a#b");

    println!("ok");
    Ok(())
}
