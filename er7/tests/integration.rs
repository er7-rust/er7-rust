//! Black-box tests: the public API as a caller sees it, and the command's
//! input/output contract. Rules about one module's internals are tested
//! next to that module instead.

use std::process::{Command, Stdio};

use er7::{Message, RenderOptions, Separators, Terminator};

const ORU: &str = include_str!("../samples/oru_r01.er7");
const ADT: &str = include_str!("../samples/adt_a08.er7");
const BATCH: &str = include_str!("../samples/batch.er7");

fn parse(text: &str) -> Message {
    er7::parse(text).expect("sample parses")
}

#[test]
fn every_sample_round_trips_byte_for_byte() {
    for sample in [ORU, ADT] {
        let options = RenderOptions {
            terminator: Terminator::Cr,
            trailing_terminator: true,
        };
        assert_eq!(parse(sample).to_er7_with(options), sample);
    }
}

#[test]
fn reads_values_by_path() {
    let message = parse(ORU);
    assert_eq!(
        message.query("PID-5.1").unwrap().as_deref(),
        Some("EVERYWOMAN")
    );
    assert_eq!(
        message.query("PID-3.4.2").unwrap().as_deref(),
        Some("1.2.840.114398.1.100")
    );
    assert_eq!(
        message.query("OBR-4.2").unwrap().as_deref(),
        Some("Lipid Panel")
    );
    assert_eq!(
        message.query_all("OBX-3.2").unwrap(),
        vec!["Cholesterol", "Triglycerides", "Comment"]
    );
    assert_eq!(message.query_all("OBX[2]-5").unwrap(), vec!["102"]);
    assert_eq!(
        message.query_all("PID-13").unwrap(),
        vec!["555-555-1111~555-555-2222"]
    );
    assert_eq!(
        message.query_all("PID-13[2]").unwrap(),
        vec!["555-555-2222"]
    );
}

#[test]
fn decodes_escape_sequences_only_on_request() {
    let message = parse(ORU);
    assert_eq!(
        message.query("OBX[3]-5").unwrap().as_deref(),
        Some("Fasting sample & hemolysed")
    );
    let path = "OBX[3]-5".parse::<er7::Path>().unwrap();
    assert_eq!(
        message.query_path_raw(&path),
        vec![r"Fasting sample \T\ hemolysed"]
    );
}

#[test]
fn reads_the_msh_routing_fields() {
    let message = parse(ADT);
    assert_eq!(message.message_code().as_deref(), Some("ADT"));
    assert_eq!(message.trigger_event().as_deref(), Some("A08"));
    assert_eq!(message.message_structure().as_deref(), Some("ADT_A01"));
    assert_eq!(message.control_id().as_deref(), Some("MSG00001"));
    assert_eq!(message.version().as_deref(), Some("2.5"));
}

#[test]
fn treats_a_local_segment_like_any_other() {
    // A Z-segment has no definition anywhere, which is exactly why an
    // encoding-level crate has to carry it through unchanged.
    let message = parse(ADT);
    assert_eq!(
        message.query("ZPD-2.2").unwrap().as_deref(),
        Some("EXTENSION")
    );
    assert!(message.to_er7().contains("ZPD|1|LOCAL^EXTENSION^SEGMENT"));
}

#[test]
fn splits_a_batch_into_messages_that_parse() {
    let messages = er7::split_messages(BATCH);
    assert_eq!(messages.len(), 2);
    let ids: Vec<Option<String>> = messages.iter().map(|m| parse(m).control_id()).collect();
    assert_eq!(ids, vec![Some("B1".into()), Some("B2".into())]);
    let second = parse(messages[1]);
    assert_eq!(second.query("MSA-1").unwrap().as_deref(), Some("AE"));
    assert_eq!(second.query("ERR-2.1").unwrap().as_deref(), Some("PID"));
}

#[test]
fn edits_a_value_and_writes_the_message_back() {
    let mut message = parse(ORU);
    let separators = message.separators;
    message
        .segment_at_mut("PID", 1)
        .and_then(|pid| pid.field_mut(5))
        .and_then(|field| field.repetition_mut(1))
        .and_then(|repetition| repetition.component_mut(1))
        .and_then(|component| component.subcomponent_mut(1))
        .expect("PID-5.1 exists")
        .set("O'BRIEN & SONS", &separators);
    // The value is encoded on the way in, so the structure still holds.
    assert!(message.to_er7().contains(r"O'BRIEN \T\ SONS^EVE^E^^MS"));
    // And reading it back gives what was written.
    assert_eq!(
        message.query("PID-5.1").unwrap().as_deref(),
        Some("O'BRIEN & SONS")
    );
    assert_eq!(er7::parse(&message.to_er7()).unwrap(), message);
}

#[test]
fn only_a_missing_or_broken_header_is_an_error() {
    assert!(er7::parse("").is_err());
    assert!(er7::parse("PID|1").is_err());
    assert!(er7::parse("MSH").is_err());
    // Everything below the header parses, however odd it is.
    let odd = "MSH|^~\\&|LAB\rZZZ\rPID|||||||||||||||||||||||||||||||\rNTE|1||\r\r";
    assert_eq!(er7::parse(odd).unwrap().segments.len(), 4);
}

#[test]
fn honors_a_message_that_chooses_its_own_delimiters() {
    let text = "MSH#*!?@#LAB#*ACME#SMITH*JOHN@JR!DOE*JANE";
    let message = er7::parse(text).unwrap();
    assert_eq!(message.separators.field, '#');
    assert_eq!(message.separators.subcomponent, '@');
    assert_eq!(message.query("MSH-5.2.2").unwrap().as_deref(), Some("JR"));
    assert_eq!(message.query_all("MSH-5[2].1").unwrap(), vec!["DOE"]);
    assert_eq!(message.to_er7(), text);
}

#[test]
fn parses_a_fragment_that_has_no_header() {
    let fragment = er7::parse_with("OBX|1|NM|2093-3^Cholesterol^LN||187", Separators::default());
    assert_eq!(fragment.query("OBX-5").unwrap().as_deref(), Some("187"));
}

#[test]
fn the_crate_has_no_runtime_dependencies() {
    // R25: the `[dependencies]` table is empty, and stays empty. Healthcare
    // integration code gets audited, this crate is meant to sit at the
    // bottom of a stack of HL7 crates, and nothing in ER7 needs a
    // dependency. Asserting it here makes the rule fail loudly rather than
    // drift (spec §15.1).
    let manifest = include_str!("../Cargo.toml");

    // Collect the body of every dependency table: runtime, dev, and build.
    let mut in_dependencies = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_dependencies = line.contains("dependencies");
            continue;
        }
        if !in_dependencies || line.is_empty() || line.starts_with('#') {
            continue;
        }
        panic!("expected no dependencies, found {line:?} in Cargo.toml");
    }

    // The lock file is the second witness. er7 now shares a workspace lock
    // with its siblings, so the whole-file package count no longer proves
    // anything — instead, find er7's own [[package]] entry and check that
    // entry carries no `dependencies` key.
    let lock = include_str!("../../Cargo.lock");
    let start = lock
        .find("name = \"er7\"\n")
        .expect("expected Cargo.lock to contain an er7 package entry");
    let entry = &lock[start..];
    let entry = &entry[..entry.find("\n\n").unwrap_or(entry.len())];
    assert!(
        !entry.contains("dependencies"),
        "expected er7's Cargo.lock entry to have no dependencies, found: {entry:?}"
    );
}

/// A message exercising the corners: a v2.7 truncation character, an
/// explicit null, a formatting escape, a decoded delimiter, and hex data.
const EDGES: &str = "MSH|^~\\&#|LAB|ACME|EHR|CLINIC|20260815120000||ADT^A08^ADT_A01|N1|P|2.7\r\
                     PID|1||9^^^ACME^MR||\"\"||19800101|M\r\
                     NTE|1||Line one\\.br\\Line two \\S\\ more\\X0D\\tail#";

#[test]
fn carries_the_corners_through_unchanged() {
    let message = parse(EDGES);
    assert_eq!(message.separators.truncation, Some('#'));
    // The explicit null survives as itself, not as an empty field.
    let pid = message.segment("PID").unwrap();
    assert!(pid.field(5).unwrap().is_null());
    assert!(!pid.field(5).unwrap().is_empty());
    assert_eq!(message.query("PID-5").unwrap().as_deref(), Some(""));
    // Sequences that stand for characters decode; the rest stay as sent.
    assert_eq!(
        message.query("NTE-3").unwrap().as_deref(),
        Some("Line one\\.br\\Line two ^ more\rtail#")
    );
    assert_eq!(message.to_er7(), EDGES);
}

/// Run the command with `args`, feeding it `stdin`, and return
/// (exit status success, stdout, stderr).
fn cli(args: &[&str], stdin: &str) -> (bool, String, String) {
    use std::io::Write;
    let mut child = Command::new(env!("CARGO_BIN_EXE_er7"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command builds");
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(stdin.as_bytes())
        .expect("writes to stdin");
    let output = child.wait_with_output().expect("the command finishes");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn cli_outlines_stdin_by_default() {
    let (ok, stdout, _) = cli(&[], ORU);
    assert!(ok);
    assert!(stdout.contains("PID-5.1     EVERYWOMAN"), "{stdout}");
    assert!(stdout.contains("MSH-2       ^~\\&"), "{stdout}");
    // A repeated segment is labelled so its paths can be reused verbatim.
    assert!(stdout.contains("OBX[2]-3.2  Triglycerides"), "{stdout}");
}

#[test]
fn cli_shows_nulls_and_control_characters_legibly() {
    let (ok, stdout, _) = cli(&[], EDGES);
    assert!(ok);
    assert!(stdout.contains("PID-5     \"\"\n"), "{stdout}");
    assert!(
        stdout.contains("NTE-3     Line one\\.br\\Line two ^ more\\rtail#\n"),
        "{stdout}"
    );
    // --raw shows the same value exactly as the sender wrote it.
    let (ok, stdout, _) = cli(&["--raw", "-q", "NTE-3"], EDGES);
    assert!(ok);
    assert_eq!(stdout, "Line one\\.br\\Line two \\S\\ more\\X0D\\tail#\n");
}

#[test]
fn cli_queries_paths_in_order() {
    let (ok, stdout, _) = cli(&["-q", "PID-5.1", "-q", "OBX-3.2"], ORU);
    assert!(ok);
    assert_eq!(stdout, "EVERYWOMAN\nCholesterol\nTriglycerides\nComment\n");
}

#[test]
fn cli_normalizes_to_the_chosen_terminator() {
    let (ok, stdout, _) = cli(
        &["--normalize", "--terminator", "lf"],
        "MSH|^~\\&|A\r\n\r\nPID|1",
    );
    assert!(ok);
    assert_eq!(stdout, "MSH|^~\\&|A\nPID|1\n");
}

#[test]
fn cli_selects_one_message_of_a_batch() {
    let (ok, stdout, _) = cli(&["--message", "2", "-q", "MSH-10"], BATCH);
    assert!(ok);
    assert_eq!(stdout, "B2\n");
    let (ok, _, stderr) = cli(&["--message", "9"], BATCH);
    assert!(!ok);
    assert!(stderr.contains("the input holds 2"), "{stderr}");
}

#[test]
fn cli_reports_errors_on_stderr() {
    for (args, input, expected) in [
        (vec![], "PID|1", "not the MSH, FHS, or BHS header"),
        (vec![], "", "no HL7 segments"),
        (vec!["-q", "PID-0"], ORU, "invalid HL7 path"),
        (vec!["--nope"], ORU, "unknown option"),
        (vec!["-q", "PID-5", "-n"], ORU, "choose one"),
    ] {
        let (ok, stdout, stderr) = cli(&args, input);
        assert!(!ok, "expected {args:?} to fail");
        assert!(stdout.is_empty(), "{stdout}");
        assert!(stderr.contains(expected), "{args:?}: {stderr}");
        assert!(stderr.starts_with("er7: error: "), "{stderr}");
    }
}

#[test]
fn cli_prints_help_and_version() {
    for flag in ["-h", "--help"] {
        let (ok, stdout, _) = cli(&[flag], "");
        assert!(ok);
        assert!(stdout.contains("Usage: er7 [OPTIONS] [FILE]"), "{stdout}");
    }
    let (ok, stdout, _) = cli(&["--version"], "");
    assert!(ok);
    assert!(stdout.starts_with("er7 "), "{stdout}");
}

/// Every `| R<n> |` cell at the start of a table row in `text`.
fn rule_ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("| R"))
        .filter_map(|rest| rest.split_once(" |"))
        .map(|(number, _)| format!("R{number}"))
        .filter(|id| id[1..].chars().all(|c| c.is_ascii_digit()))
        .collect()
}

#[test]
fn every_rule_has_a_coverage_row() {
    // Spec-driven development only works if the spec is the single source
    // of truth. A rule in the §1.4 index with no row in the §13.1 coverage
    // table is a rule nobody agreed to test, and a row in §13.1 for a rule
    // that no longer exists is a table nobody re-read. Both used to be
    // caught by review; this catches them by `cargo test`
    // (AGENTS/spec-driven-development.md).
    let declared = rule_ids(include_str!("../spec/01-purpose-and-scope/index.md"));
    let covered = rule_ids(include_str!("../spec/13-testing-strategy/index.md"));

    assert_eq!(declared.len(), 25, "§1.4 should index R1–R25");
    let missing: Vec<&String> = declared.iter().filter(|r| !covered.contains(r)).collect();
    assert!(missing.is_empty(), "no row in §13.1 for {missing:?}");
    let orphan: Vec<&String> = covered.iter().filter(|r| !declared.contains(r)).collect();
    assert!(
        orphan.is_empty(),
        "§13.1 covers {orphan:?}, which §1.4 does not declare"
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

    // And every section names itself, so a section that was copied keeps
    // its own number rather than its neighbour's.
    for slug in &on_disk {
        let number = &slug[..2];
        let text = std::fs::read_to_string(directory.join(slug).join("index.md")).expect("section");
        let heading = format!("# {}. ", number.trim_start_matches('0'));
        assert!(
            text.contains(&heading),
            "{slug} has no `{heading}` heading of its own"
        );
    }
}
