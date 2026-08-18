//! Black-box tests through the public API only, exercising the same kind of
//! ER7 fixtures the sibling `er7` crate tests itself against — including
//! its own `samples/` files, read straight from the sibling checkout, so a
//! round trip through this crate is checked against real messages, not
//! just literals written for this crate.

use serde_er7::Message;

// The sample files end with a trailing segment terminator, which `er7`
// itself normalizes away at parse time (there is no trailing terminator by
// default — see `er7::RenderOptions`). Trim it here so the text we compare
// against is already the canonical form a plain `er7::parse(..).to_er7()`
// would produce, and the round-trip assertions below test this crate's
// JSON round trip specifically, not `er7`'s own normalization.
fn sample(text: &str) -> &str {
    text.trim_end_matches(['\r', '\n'])
}

const ORU: &str = include_str!("../../er7-rust/samples/oru_r01.er7");
const ADT: &str = include_str!("../../er7-rust/samples/adt_a08.er7");
const BATCH: &str = include_str!("../../er7-rust/samples/batch.er7");

fn round_trips_through_json(text: &str) {
    let text = sample(text);
    let message = Message::parse(text).expect("sample parses");
    let json = serde_json::to_string(&message).expect("serializes");
    let back: Message = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(
        back.to_er7(),
        text,
        "ER7 text changed after a JSON round trip"
    );
    assert_eq!(
        back, message,
        "wrapper equality changed after a JSON round trip"
    );
}

#[test]
fn round_trips_every_sample_from_the_er7_crate() {
    for sample in [ORU, ADT, BATCH] {
        round_trips_through_json(sample);
    }
}

#[test]
fn round_trips_pretty_printed_json_too() {
    // Pretty-printing changes only whitespace between JSON tokens, never
    // the values, so this must round-trip exactly like compact JSON.
    let message = Message::parse(sample(ORU)).unwrap();
    let json = serde_json::to_string_pretty(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), sample(ORU));
}

#[test]
fn keeps_absent_empty_and_null_distinct_through_json() {
    // R10/R11 in the `er7` spec: the explicit `""` means "clear this
    // value", never conflated with "not sent" or "sent blank". This has to
    // survive a JSON round trip exactly as it survives an ER7 one.
    let text = "MSH|^~\\&|LAB\rPID|1||\"\"|X";
    let message = Message::parse(text).unwrap();
    let pid = message.segment("PID").unwrap();
    assert!(pid.field(2).unwrap().is_empty()); // sent as `||`
    assert!(pid.field(3).unwrap().is_null()); // sent as `""`
    assert!(pid.field(9).is_none()); // never sent

    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    let pid = back.segment("PID").unwrap();
    assert!(pid.field(2).unwrap().is_empty());
    assert!(!pid.field(2).unwrap().is_null());
    assert!(pid.field(3).unwrap().is_null());
    assert!(!pid.field(3).unwrap().is_empty());
    assert!(pid.field(9).is_none());
}

#[test]
fn keeps_escape_sequences_raw_through_json() {
    // Subcomponents serialize `raw`, not `value`-decoded, so a JSON round
    // trip must not resolve `\T\` into `&` along the way.
    let text = r"MSH|^~\&|LAB|Smith \T\ Jones^X";
    let message = Message::parse(text).unwrap();
    let json = serde_json::to_string(&message).unwrap();
    assert!(json.contains(r"Smith \\T\\ Jones"), "raw text: {json}");
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
}

#[test]
fn round_trips_unusual_delimiters() {
    let text = "MSH#*!?@#LAB#*A*B#C!D";
    let message = Message::parse(text).unwrap();
    assert_eq!(message.separators.field, '#');
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
}

#[test]
fn round_trips_the_truncation_character() {
    let text = r"MSH|^~\&#|LAB";
    let message = Message::parse(text).unwrap();
    assert_eq!(message.separators.truncation, Some('#'));
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.separators.truncation, Some('#'));
    assert_eq!(back.to_er7(), text);
}

#[test]
fn preserves_repeated_fields_and_ragged_segments() {
    // Nothing below the header may fail, and that carries through JSON:
    // unknown segments and stray positions are data, both before and after
    // the round trip.
    let text = "MSH|^~\\&|LAB\rZZZ|1|LOCAL^EXTENSION\rOBX|1||A~~B";
    let message = Message::parse(text).unwrap();
    let json = serde_json::to_string(&message).unwrap();
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back.to_er7(), text);
    assert_eq!(
        back.query("OBX-3").unwrap().as_deref(),
        Some("A~~B"),
        "the repetition separator is preserved because the query stops above it"
    );
    assert_eq!(back.query("ZZZ-2.2").unwrap().as_deref(), Some("EXTENSION"));
}

#[test]
fn error_messages_still_name_the_missing_field() {
    let err = serde_json::from_str::<Message>(r#"{"segments":[]}"#).unwrap_err();
    assert!(err.to_string().contains("separators"), "{err}");
}

/// The `[dependencies]` and `[dev-dependencies]` tables of this crate's
/// own manifest, as raw lines.
fn manifest_dependencies(table: &str) -> Vec<String> {
    let manifest = include_str!("../Cargo.toml");
    let mut inside = false;
    let mut lines = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == table;
            continue;
        }
        if !inside || line.is_empty() || line.starts_with('#') {
            continue;
        }
        lines.push(line.to_string());
    }
    lines
}

#[test]
fn the_crate_has_exactly_two_runtime_dependencies() {
    // S1: `serde` is the trait vocabulary this crate implements against and
    // `er7` is the tree it wraps. Both are the point of the crate; a third
    // would be an audit surface for every downstream healthcare project
    // (spec §3.1).
    assert_eq!(
        manifest_dependencies("[dependencies]"),
        [r#"serde = "1""#, r#"er7 = "0""#]
    );
}

#[test]
fn no_format_crate_is_a_runtime_dependency() {
    // S2: this crate is format-agnostic. `serde_json` appears in the tests,
    // the doctests, and the examples because JSON is the easiest format to
    // read on a page — never in `[dependencies]` (spec §3.2).
    let runtime = manifest_dependencies("[dependencies]").join(" ");
    for format in ["serde_json", "serde_yaml", "toml", "ciborium", "postcard"] {
        assert!(
            !runtime.contains(format),
            "{format} is a runtime dependency"
        );
    }
    assert!(
        manifest_dependencies("[dev-dependencies]")
            .iter()
            .any(|line| line.starts_with("serde_json")),
        "serde_json belongs in dev-dependencies, where the tests use it"
    );
}

/// Every `| S<n> |` cell at the start of a table row in `text`.
fn rule_ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("| S"))
        .filter_map(|rest| rest.split_once(" |"))
        .map(|(number, _)| format!("S{number}"))
        .filter(|id| id[1..].chars().all(|c| c.is_ascii_digit()))
        .collect()
}

#[test]
fn every_rule_has_a_coverage_row() {
    // Spec-driven development only works if the spec is the single source
    // of truth. A rule in the index with no row in the §7.1 coverage table
    // is a rule nobody agreed to test, and a row for a rule that no longer
    // exists is a table nobody re-read.
    let declared = rule_ids(include_str!("../spec/index.md"));
    let covered = rule_ids(include_str!("../spec/07-testing-strategy.md"));

    assert_eq!(declared.len(), 12, "the rule index should hold S1–S12");
    let missing: Vec<&String> = declared.iter().filter(|s| !covered.contains(s)).collect();
    assert!(missing.is_empty(), "no row in §7.1 for {missing:?}");
    let orphan: Vec<&String> = covered.iter().filter(|s| !declared.contains(s)).collect();
    assert!(
        orphan.is_empty(),
        "§7.1 covers {orphan:?}, which the rule index does not declare"
    );
}

#[test]
fn every_spec_section_is_indexed_and_present() {
    // The section files and the table of contents drift apart silently: a
    // new section nobody linked, or a link to a file that was renamed.
    let index = include_str!("../spec/index.md");
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("spec");

    let mut linked: Vec<String> = index
        .match_indices("](")
        .filter_map(|(at, _)| index[at + 2..].split_once(')').map(|(name, _)| name))
        .filter(|name| name.len() > 3 && name.as_bytes()[2] == b'-' && name.ends_with(".md"))
        .map(str::to_string)
        .collect();
    linked.sort();
    linked.dedup();

    let mut on_disk: Vec<String> = std::fs::read_dir(&directory)
        .expect("spec directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md") && name.as_bytes()[0].is_ascii_digit())
        .collect();
    on_disk.sort();

    assert_eq!(linked, on_disk, "spec/index.md and spec/ disagree");
}
