//! Black-box tests through the public API only, exercising the same kind of
//! ER7 fixtures the sibling `er7` crate tests itself against — including
//! its own `samples/` files, read straight from the sibling checkout, so a
//! round trip through this crate is checked against real messages, not
//! just literals written for this crate.

use serde_er7::{
    Component, Field, Message, Repetition, Segment, Separators, Strict, Subcomponent, Terminator,
};

// The sample files end with a trailing segment terminator, which `er7`
// itself normalizes away at parse time (there is no trailing terminator by
// default — see `er7::RenderOptions`). Trim it here so the text we compare
// against is already the canonical form a plain `er7::parse(..).to_er7()`
// would produce, and the round-trip assertions below test this crate's
// JSON round trip specifically, not `er7`'s own normalization.
fn sample(text: &str) -> &str {
    text.trim_end_matches(['\r', '\n'])
}

const ORU: &str = include_str!("../../er7/samples/oru_r01.er7");
const ADT: &str = include_str!("../../er7/samples/adt_a08.er7");
const BATCH: &str = include_str!("../../er7/samples/batch.er7");
const MULTILINGUAL: &str = include_str!("../../er7/samples/multilingual.er7");

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
    for sample in [ORU, ADT, BATCH, MULTILINGUAL] {
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

#[test]
fn wrong_json_kinds_report_each_expected_shape() {
    let cases = [
        serde_json::from_str::<Component>("{}").unwrap_err(),
        serde_json::from_str::<Field>("{}").unwrap_err(),
        serde_json::from_str::<Repetition>("{}").unwrap_err(),
    ];
    for error in cases {
        assert!(error.to_string().contains("array"), "{error}");
    }

    let error = serde_json::from_str::<Subcomponent>("[]").unwrap_err();
    assert!(error.to_string().contains("string"), "{error}");
    let error = serde_json::from_str::<Terminator>("[]").unwrap_err();
    assert!(error.to_string().contains("CrLf"), "{error}");

    let error = serde_json::from_str::<Message>("[]").unwrap_err();
    assert!(error.to_string().contains("separators"), "{error}");
    let error = serde_json::from_str::<Segment>("[]").unwrap_err();
    assert!(error.to_string().contains("name"), "{error}");
    let error = serde_json::from_str::<Separators>("[]").unwrap_err();
    assert!(error.to_string().contains("truncation"), "{error}");
}

#[test]
fn every_wrapper_converts_and_mutates_through() {
    // S11 is a promise about every wrapper, not two representative types.
    // Exercise the public conversion and DerefMut surface from outside the
    // crate so an omitted or incorrectly wired implementation is visible.
    let inner_message = er7::parse("MSH|^~\\&|LAB").unwrap();
    let mut message: Message = inner_message.into();
    message.segments.push(er7::Segment {
        name: "ZZZ".into(),
        fields: Vec::new(),
    });
    let inner_message: er7::Message = message.into();
    assert_eq!(inner_message.segments.last().unwrap().name, "ZZZ");

    let mut segment: Segment = inner_message.segments[0].clone().into();
    segment.name = "HDR".into();
    let inner_segment: er7::Segment = segment.into();
    assert_eq!(inner_segment.name, "HDR");

    let mut field: Field = er7::Field::default().into();
    field.repetitions.push(er7::Repetition::default());
    let inner_field: er7::Field = field.into();
    assert_eq!(inner_field.repetitions.len(), 1);

    let mut repetition: Repetition = er7::Repetition::default().into();
    repetition.components.push(er7::Component::default());
    let inner_repetition: er7::Repetition = repetition.into();
    assert_eq!(inner_repetition.components.len(), 1);

    let mut component: Component = er7::Component::default().into();
    component
        .subcomponents
        .push(er7::Subcomponent::new("VALUE"));
    let inner_component: er7::Component = component.into();
    assert_eq!(inner_component.subcomponents[0].raw, "VALUE");

    let mut subcomponent: Subcomponent = er7::Subcomponent::new("A").into();
    subcomponent.raw.push('B');
    let inner_subcomponent: er7::Subcomponent = subcomponent.into();
    assert_eq!(inner_subcomponent.raw, "AB");

    let mut separators: Separators = er7::Separators::default().into();
    separators.field = '#';
    let inner_separators: er7::Separators = separators.into();
    assert_eq!(inner_separators.field, '#');

    let mut terminator: Terminator = er7::Terminator::Cr.into();
    *terminator = er7::Terminator::CrLf;
    let inner_terminator: er7::Terminator = terminator.into();
    assert_eq!(inner_terminator, er7::Terminator::CrLf);

    let mut strict_message: Strict<Message> = Message::parse("MSH|^~\\&|LAB").unwrap().into();
    strict_message.segments.clear();
    let plain_message: Message = strict_message.into();
    assert_eq!(plain_message.segments, [] as [er7::Segment; 0]);

    let mut strict_segment: Strict<Segment> = Segment(inner_segment).into();
    strict_segment.name = "STRICT".into();
    let plain_segment: Segment = strict_segment.into();
    assert_eq!(plain_segment.name, "STRICT");

    let mut strict_separators: Strict<Separators> = Separators::default().into();
    strict_separators.escape = '!';
    let plain_separators: Separators = strict_separators.into();
    assert_eq!(plain_separators.escape, '!');
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
        [
            r#"serde = "1""#,
            r#"er7 = { path = "../er7", version = "0" }"#
        ]
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
    let covered = rule_ids(include_str!("../spec/07-testing-strategy/index.md"));

    assert_eq!(declared.len(), 13, "the rule index should hold S1–S13");
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
    // The section directories and the table of contents drift apart
    // silently: a new section nobody linked, or a link to a section that
    // was renamed. Each section is `<slug>/index.md`, so the slug is the
    // directory name and the link is the slug plus `/index.md`.
    let index = include_str!("../spec/index.md");
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("spec");

    let mut linked: Vec<String> = index
        .match_indices("](")
        .filter_map(|(at, _)| index[at + 2..].split_once(')').map(|(name, _)| name))
        .filter_map(|name| name.strip_suffix("/index.md"))
        .filter(|slug| slug.len() > 3 && slug.as_bytes()[2] == b'-')
        .map(str::to_string)
        .collect();
    linked.sort();
    linked.dedup();

    let mut on_disk: Vec<String> = std::fs::read_dir(&directory)
        .expect("spec directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|slug| slug.as_bytes()[0].is_ascii_digit())
        .collect();
    on_disk.sort();

    assert_eq!(linked, on_disk, "spec/index.md and spec/ disagree");
}
