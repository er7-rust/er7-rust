//! `escape_roundtrip`'s three guarantees, under a delimiter set the fuzzer
//! chooses rather than `|^~\&` (R12, R14, R15, spec §6.4).
//!
//! The first five distinct characters of the input become the field,
//! component, repetition, escape, and subcomponent characters. A set that
//! `Separators::validate` refuses is skipped, and the rest of the input is
//! the text. Sequences the text happens to contain are often invalid under
//! the chosen set, which is the point: tokenizing must still be lossless.
#![forbid(unsafe_code)]
#![no_main]

use er7::Separators;
use er7::escape::{escape, escapes, unescape};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };
    let mut chosen: Vec<char> = Vec::with_capacity(5);
    let mut rest = input;
    for c in input.chars() {
        rest = &rest[c.len_utf8()..];
        if !chosen.contains(&c) {
            chosen.push(c);
        }
        if chosen.len() == 5 {
            break;
        }
    }
    let [field, component, repetition, escape_char, subcomponent] = chosen[..] else {
        return;
    };
    let separators = Separators {
        field,
        component,
        repetition,
        escape: escape_char,
        subcomponent,
        truncation: None,
    };
    if separators.validate().is_err() {
        return;
    }
    let text = rest;

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
    for structural in [field, component, repetition, subcomponent, '\r', '\n'] {
        assert!(
            !encoded.contains(structural),
            "{structural:?} survived encoding: {encoded:?}"
        );
    }
});
