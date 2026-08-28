//! Criterion benchmarks for the operation this crate exists to do:
//! `Redactor::redact` on a parsed message. Closes `er7-redact` spec §15
//! task T4 — "there is no measurement, only the argument that redaction
//! is one pass over a small message." This measures that argument.
//!
//! The inputs are synthetic — never real patient data (workspace family
//! policy §1.4) — chosen to bracket what a real export looks like: one
//! ADT message, the shape most interfaces move one at a time, and a
//! 50-message batch, the shape a de-identification run over an export
//! actually processes.
//!
//! Run with `cargo bench -p er7-redact-bench`; compare against a baseline
//! with `cargo bench -p er7-redact-bench -- --save-baseline before`.
#![forbid(unsafe_code)]

use criterion::{Criterion, criterion_group, criterion_main};
use er7::Message;
use er7_redact::{Policy, Redactor};
use std::hint::black_box;

/// The HL7® v2 reference `ADT^A08` example — the same shape every sample
/// and doc-test in this workspace already uses. Carries the identifiers a
/// redaction policy actually targets: `PID`, `NK1`, `PV1`, `GT1`, `IN1`.
fn adt_a08() -> String {
    "MSH|^~\\&|ADT1|MCM|LABADT|MCM|20260815140000||ADT^A08^ADT_A01|MSG00001|P|2.5\r\
     EVN|A08|20260815140000\r\
     PID|1||PATID1234^5^M11^ADT1^MR^MCM~123456789^^^USSSA^SS||JONES^WILLIAM^A^III||19610615|M||C|1200 N ELM STREET^^GREENSBORO^NC^27401-1020|GL|(919)379-1212|(919)271-3434||S||PATID12345001^2^M10^ADT1^AN^A|444333222|987654^NC\r\
     NK1|1|JONES^BARBARA^K|SPO^Spouse^HL70063|1200 N ELM STREET^^GREENSBORO^NC^27401-1020|(919)379-1212\r\
     PV1|1|I|2000^2012^01||||004777^ATTEND^AARON^A|||SUR||||ADM|A0||||V00001\r\
     GT1|1|1122334^^^MCM^AN|JONES^WILLIAM^A^III||1200 N ELM STREET^^GREENSBORO^NC^27401-1020|(919)379-1212||19610615|\r\
     IN1|1|BC1^BLUE CROSS|BC001|BLUE CROSS OF NC||||||||||||JONES^WILLIAM^A^III|SEL|19610615|1200 N ELM STREET^^GREENSBORO^NC^27401-1020\r\
     AL1|1|DA|1605^ACETAMINOPHEN^L|MO|HEADACHE"
        .to_string()
}

/// Fifty independently parsed copies of the reference message, standing in
/// for a batch export. Redaction cost is what is measured, not parsing
/// cost — that is `er7`'s own benchmark's concern — so every message is
/// parsed once up front, outside the timed closure.
fn batch() -> Vec<Message> {
    let text = adt_a08();
    (0..50)
        .map(|_| er7::parse(&text).expect("the reference message parses"))
        .collect()
}

fn bench_redact(c: &mut Criterion) {
    let small = er7::parse(&adt_a08()).unwrap();
    let batch = batch();
    let mut group = c.benchmark_group("redact");
    group.bench_function("small", |b| {
        b.iter(|| {
            let mut message = black_box(&small).clone();
            Redactor::new(Policy::patient_identifiers()).redact(&mut message)
        });
    });
    group.bench_function("batch_of_50", |b| {
        b.iter(|| {
            let redactor = Redactor::new(Policy::patient_identifiers());
            for message in black_box(&batch) {
                let mut message = message.clone();
                let _ = redactor.redact(&mut message);
            }
        });
    });
    group.finish();
}

/// The two postures cost different amounts of work by construction: accept
/// by default only touches the leaves a rule names, reject by default
/// walks and judges every leaf in the message. Worth measuring
/// side by side rather than assuming the gap is small.
fn bench_posture(c: &mut Criterion) {
    let small = er7::parse(&adt_a08()).unwrap();
    let mut group = c.benchmark_group("posture");
    group.bench_function("accept_by_default", |b| {
        b.iter(|| {
            let mut message = black_box(&small).clone();
            Redactor::new(Policy::patient_identifiers()).redact(&mut message)
        });
    });
    group.bench_function("reject_by_default", |b| {
        b.iter(|| {
            let mut message = black_box(&small).clone();
            Redactor::new(Policy::all_but_the_header()).redact(&mut message)
        });
    });
    group.finish();
}

criterion_group!(benches, bench_redact, bench_posture);
criterion_main!(benches);
