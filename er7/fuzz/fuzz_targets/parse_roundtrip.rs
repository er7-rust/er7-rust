//! Reading a message and writing it back must be stable and total.
//!
//! Two properties, checked on every input the fuzzer reaches:
//!
//! 1. **Total.** `parse` either returns a message or an error; it never
//!    panics, overflows the stack, or hangs, whatever bytes arrive. A
//!    receiver does not get to choose what a sender sends.
//! 2. **Idempotent.** Rendering a parsed message and parsing that again
//!    yields the same text (R12, spec §5.2). The first render may normalize
//!    — a lone `\n` terminator becomes `\r` — so the fixed point is checked
//!    from the *second* pass on, not against the original input.
#![forbid(unsafe_code)]
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(message) = er7::parse(text) else {
        return;
    };
    let once = message.to_er7();
    let Ok(reparsed) = er7::parse(&once) else {
        panic!("rendered message no longer parses: {once:?}");
    };
    let twice = reparsed.to_er7();
    assert_eq!(once, twice, "rendering is not a fixed point");
});
