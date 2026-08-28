//! `parse_with` must be total, the same way `parse` is (R6, spec §11.2):
//! given arbitrary bytes as text, it never panics, overflows the stack, or
//! hangs, whatever arrives.
//!
//! `parse_roundtrip.rs` already fuzzes `parse`, but `parse` insists on a
//! valid `MSH`/`FHS`/`BHS` header before it reaches the below-the-header
//! parsing logic R6 actually claims total-ness for — most arbitrary byte
//! strings fail that check and return `Err(Error::MissingHeader(_))`
//! before touching a single segment body. `parse_with` skips the header
//! entirely and hands its input straight to that same segment-parsing
//! code, so this target is what actually exercises R6 on inputs that do
//! not happen to start with a recognisable header — which is most of
//! what a fuzzer generates.
#![forbid(unsafe_code)]
#![no_main]

use er7::Separators;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let _ = er7::parse_with(text, Separators::default());
});
