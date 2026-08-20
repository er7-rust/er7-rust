//! Path parsing and querying are total: any path string either fails to
//! parse or returns values, and a position a message does not carry is
//! `None` rather than an error or a panic (R20).
//!
//! The input is split into a path and a message so one run exercises both
//! sides at once.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let (path, body) = text.split_once('\u{0}').unwrap_or((text, ""));
    let message = match er7::parse(body) {
        Ok(message) => message,
        Err(_) => return,
    };
    // Whatever the path is, these must not panic, and `query` must agree
    // with the head of `query_all`.
    if let Ok(first) = message.query(path) {
        let all = message.query_all(path).expect("query_all disagreed on path validity");
        assert_eq!(first, all.into_iter().next(), "query is not the first of query_all");
    }
});
