//! Criterion benchmarks for the four operations that dominate any ER7
//! workload: reading a message, writing one back, decoding a value, and
//! looking one up by path.
//!
//! The inputs are two synthetic messages — never real patient data (family
//! policy §1.4) — chosen to bracket what production traffic looks like: a
//! short ADT, and a lab result with many repeating `OBX` segments.
//!
//! Run with `cargo bench -p er7-bench`; compare against a baseline with
//! `cargo bench -p er7-bench -- --save-baseline before`.

use criterion::{Criterion, criterion_group, criterion_main};
use er7::escape::{escape, escapes, unescape};
use er7::{RenderOptions, Separators};
use std::hint::black_box;

/// A short ADT: the shape most interfaces move in bulk.
fn small_message() -> String {
    "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260814080000||ADT^A08|MSG00042|P|2.5\r\
     EVN|A08|20260814080000\r\
     PID|1||444333222^^^ACME&1.2.3.4&ISO^MR||EVERYWOMAN^EVE^E||19620320|F\r\
     PV1|1|O|OP^^^ACME"
        .to_string()
}

/// A lab result with 200 `OBX` segments: the shape that decides whether a
/// parser is fast enough for a day's traffic.
fn large_message() -> String {
    let mut text =
        String::from("MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260814080000||ORU^R01|MSG00042|P|2.5\r");
    text.push_str("PID|1||444333222^^^ACME&1.2.3.4&ISO^MR||EVERYWOMAN^EVE^E||19620320|F\r");
    for i in 1..=200 {
        use std::fmt::Write as _;
        let _ = write!(
            text,
            "OBX|{i}|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F\r\
             NTE|{i}||Fasting sample, drawn \\T\\ processed at ACME\r"
        );
    }
    text.pop();
    text
}

fn bench_parse(c: &mut Criterion) {
    let small = small_message();
    let large = large_message();
    let mut group = c.benchmark_group("parse");
    group.throughput(criterion::Throughput::Bytes(small.len() as u64));
    group.bench_function("small", |b| b.iter(|| er7::parse(black_box(&small))));
    group.throughput(criterion::Throughput::Bytes(large.len() as u64));
    group.bench_function("large", |b| b.iter(|| er7::parse(black_box(&large))));
    group.finish();
}

fn bench_render(c: &mut Criterion) {
    let small = er7::parse(&small_message()).unwrap();
    let large = er7::parse(&large_message()).unwrap();
    let mut group = c.benchmark_group("render");
    group.bench_function("small", |b| b.iter(|| black_box(&small).to_er7()));
    group.bench_function("large", |b| b.iter(|| black_box(&large).to_er7()));
    group.bench_function("large_crlf_trailing", |b| {
        let options = RenderOptions {
            terminator: er7::Terminator::CrLf,
            trailing_terminator: true,
        };
        b.iter(|| black_box(&large).to_er7_with(options));
    });
    group.finish();
}

fn bench_escape(c: &mut Criterion) {
    let separators = Separators::default();
    // Three shapes with very different costs: nothing to do (the common
    // case, which should not allocate), delimiters throughout, and a value
    // already full of sequences to tokenize.
    let plain = "EVERYWOMAN^EVE^E".replace('^', "");
    let delimited = "Smith & Jones | Radiology ^ Main ~ Ward";
    let sequenced = r"line\.br\next \T\ more \X0D\ and \H\loud\N\";
    let mut group = c.benchmark_group("escape");
    group.bench_function("escape_plain", |b| {
        b.iter(|| escape(black_box(&plain), &separators));
    });
    group.bench_function("escape_delimited", |b| {
        b.iter(|| escape(black_box(delimited), &separators));
    });
    group.bench_function("unescape_sequenced", |b| {
        b.iter(|| unescape(black_box(sequenced), &separators));
    });
    group.bench_function("tokenize_sequenced", |b| {
        b.iter(|| escapes(black_box(sequenced), &separators).count());
    });
    group.finish();
}

fn bench_query(c: &mut Criterion) {
    let large = er7::parse(&large_message()).unwrap();
    let mut group = c.benchmark_group("query");
    group.bench_function("field", |b| {
        b.iter(|| black_box(&large).query("PID-3").unwrap());
    });
    group.bench_function("subcomponent", |b| {
        b.iter(|| black_box(&large).query("PID-3.4.2").unwrap());
    });
    group.bench_function("last_segment", |b| {
        b.iter(|| black_box(&large).query("NTE-3").unwrap());
    });
    group.bench_function("all_segments", |b| {
        b.iter(|| black_box(&large).query_all("OBX-5").unwrap());
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_parse,
    bench_render,
    bench_escape,
    bench_query
);
criterion_main!(benches);
