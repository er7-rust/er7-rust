//! A header the fuzzer builds, with a field separator and an MSH-2 of any
//! length and content, is either refused or yields a usable delimiter set.
//!
//! `parse_roundtrip` reaches a valid header only by luck. This target builds
//! `MSH` + one separator + up to eight encoding characters + a body, so most
//! runs exercise the header decision itself (spec §3, §4.4):
//!
//! 1. **Total.** `parse` returns a message or an error, never a panic.
//! 2. **Refusals are header errors.** An `Err` is `BadHeader`, since the
//!    input does start with `MSH`. The one exception is an alphanumeric
//!    field separator: then `MSH` is not a segment name at all (`MSHX` is
//!    not a header, spec §3.3, §4.4), so the refusal is `MissingHeader`.
//! 3. **Acceptance is sound.** An `Ok` message's separators validate, are
//!    the ones `Separators::from_header` reads from the same first line, and
//!    the message reaches a rendering fixed point on the second pass.
#![forbid(unsafe_code)]
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let mut chars = text.chars();
    let Some(field) = chars.next() else {
        return;
    };
    let count = chars.next().map_or(0, |c| c as usize % 9);
    let encoding: String = chars.by_ref().take(count).collect();
    let body: String = chars.collect();
    let input = format!("MSH{field}{encoding}{field}LAB\r{body}");

    let first_line = input.split(['\r', '\n']).next().unwrap_or_default();
    match er7::parse(&input) {
        Err(er7::Error::BadHeader(_)) => {}
        Err(er7::Error::MissingHeader(_)) if field.is_alphanumeric() => {}
        Err(other) => panic!("a message starting with MSH was refused as {other:?}"),
        Ok(message) => {
            message
                .separators
                .validate()
                .expect("an accepted header must declare a valid set");
            assert_eq!(
                er7::Separators::from_header(first_line).ok(),
                Some(message.separators),
                "parse and from_header disagree about {first_line:?}"
            );
            let once = message.to_er7();
            let reparsed = er7::parse(&once).expect("a rendered message parses again");
            assert_eq!(reparsed.separators, message.separators);
            assert_eq!(reparsed.to_er7(), once, "rendering is not a fixed point");
        }
    }
});
