//! The escape layer's three guarantees, on arbitrary text.
//!
//! 1. **Tokenizing is lossless** (R12): concatenating every token's
//!    `write_er7` reproduces the input exactly.
//! 2. **Encode-then-decode is the identity** (R14, R15): `unescape` of
//!    `escape` of any value returns that value. This is the property that
//!    keeps a value from changing meaning in transit.
//! 3. **Escaped text is structurally safe**: no delimiter and no bare
//!    carriage return survives encoding, so an encoded value can never
//!    split the field, component, or segment it sits in.
#![forbid(unsafe_code)]
#![no_main]

use er7::Separators;
use er7::escape::{escape, escapes, unescape};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let separators = Separators::default();

    let mut rebuilt = String::new();
    for token in escapes(text, &separators) {
        token.write_er7(&mut rebuilt, &separators);
    }
    assert_eq!(rebuilt, text, "tokenizing lost or invented characters");

    let encoded = escape(text, &separators);
    assert_eq!(
        unescape(&encoded, &separators),
        text,
        "encode then decode changed the value"
    );
    for structural in [
        separators.field,
        separators.component,
        separators.repetition,
        separators.subcomponent,
        '\r',
        '\n',
    ] {
        assert!(
            !encoded.contains(structural),
            "encoded value still carries {structural:?}, which would split the message"
        );
    }
});
