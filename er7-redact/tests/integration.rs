//! Black-box tests: the public API as a caller sees it, and the command's
//! input/output contract. Rules about one module's internals are tested
//! next to that module instead.

use std::process::{Command, Stdio};

use er7::Message;
use er7_redact::{Action, Policy, Redactor};

const ADT: &str = include_str!("../samples/adt_a08.er7");
const ORU: &str = include_str!("../samples/oru_r01.er7");
const POLICY: &str = include_str!("../samples/de-identify.policy");

fn parse(text: &str) -> Message {
    er7::parse(text).expect("sample parses")
}

/// Every count in the tree, so a test can assert the shape did not move.
fn shape(message: &Message) -> Vec<usize> {
    let mut counts = vec![message.segments.len()];
    for segment in &message.segments {
        counts.push(segment.fields.len());
        for field in &segment.fields {
            counts.push(field.repetitions.len());
            for repetition in &field.repetitions {
                counts.push(repetition.components.len());
                for component in &repetition.components {
                    counts.push(component.subcomponents.len());
                }
            }
        }
    }
    counts
}

/// The segment names and field counts: the numbering every path depends
/// on, and the part of the shape that survives being written out.
fn numbering(message: &Message) -> Vec<(String, usize)> {
    message
        .segments
        .iter()
        .map(|segment| (segment.name.clone(), segment.fields.len()))
        .collect()
}

#[test]
fn every_sample_keeps_its_shape() {
    // D1: whatever the policy, the redacted message has the same segments,
    // fields, repetitions, components, and subcomponents as the original
    // (spec §4.1).
    for sample in [ADT, ORU] {
        for policy in [
            Policy::patient_identifiers(),
            Policy::all_but_the_header(),
            Policy::reject_all(),
            Policy::parse(POLICY).expect("the sample policy parses"),
        ] {
            let mut message = parse(sample);
            let before = shape(&message);
            let numbers = numbering(&message);
            Redactor::new(policy).redact(&mut message);
            assert_eq!(shape(&message), before);

            // Written out and read back, the field numbering every path
            // depends on is unchanged too — and that is the guarantee that
            // matters downstream (spec §4.1).
            let text = message.to_er7();
            let again = parse(&text);
            assert_eq!(numbering(&again), numbers);
        }
    }
}

#[test]
fn a_cleared_field_reads_back_as_an_empty_one() {
    // The one place a written-out redaction is not shape-identical, and it
    // is HL7's doing rather than this crate's: a field whose only value was
    // cleared writes as empty, and an empty field has no repetitions at all
    // (`er7` R7). The field still occupies its position, so nothing shifts
    // (spec §4.1).
    let mut message = parse("MSH|^~\\&|LAB\rPID|1||9|X^Y");
    let policy = Policy::accept_all()
        .with("PID-3", Action::Clear)
        .unwrap()
        .with("PID-4", Action::Clear)
        .unwrap();
    Redactor::new(policy).redact(&mut message);
    assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|1|||^");

    let again = parse(&message.to_er7());
    assert_eq!(numbering(&again), numbering(&message));
    // The single-component field lost its repetition on the way back...
    assert_eq!(
        again
            .segment("PID")
            .unwrap()
            .field(3)
            .unwrap()
            .repetitions
            .len(),
        0
    );
    // ...while the two-component field kept both positions.
    assert_eq!(again.query("PID-4").unwrap().as_deref(), Some("^"));
}

#[test]
fn redacts_the_identifiers_a_sample_carries() {
    // The property every redaction test asserts (spec §11.5): the message
    // parses, the shape holds, and the value is gone from the *whole*
    // message rather than merely from the position that was named.
    let mut message = parse(ADT);
    let report = Redactor::default().redact(&mut message);
    let text = message.to_er7();

    for value in [
        "PATID1234",     // PID-3.1
        "JONES",         // PID-5, NK1-2, GT1-3, IN1-16
        "WILLIAM",       // as above
        "19610615",      // PID-7, GT1-8, IN1-18
        "GREENSBORO",    // PID-11, NK1-4, GT1-5, IN1-19
        "27401-1020",    // as above
        "(919)379-1212", // PID-13, NK1-5, GT1-6
        "444333222",     // PID-19
        "ATTEND",        // PV1-7
        "V00001",        // PV1-19.1
        "1122334",       // GT1-2.1
    ] {
        assert!(!text.contains(value), "{value} survived redaction");
    }

    // What is left is still a message, and still recognisable as the one
    // it came from.
    let message = parse(&text);
    assert_eq!(message.control_id().as_deref(), Some("MSG00001"));
    assert_eq!(message.query("PID-3.4").unwrap().as_deref(), Some("ADT1"));
    assert_eq!(message.query("PID-8").unwrap().as_deref(), Some("M"));
    assert_eq!(
        message.query("AL1-3.2").unwrap().as_deref(),
        Some("ACETAMINOPHEN")
    );
    assert!(!report.is_empty());
}

#[test]
fn an_untouched_message_round_trips() {
    // D17: a policy that names nothing the message carries leaves the tree
    // exactly as it was, so `er7`'s round-trip guarantee carries through
    // (spec §4.5).
    let policy = Policy::accept_all()
        .with("ZZZ-1", Action::redacted())
        .unwrap()
        .with("PID-99", Action::Clear)
        .unwrap();
    for sample in [ADT, ORU] {
        let mut message = parse(sample);
        let report = Redactor::new(policy.clone()).redact(&mut message);
        assert!(report.is_empty());
        assert_eq!(message.to_er7(), sample);
    }

    // And a leaf no action touched keeps the sender's own spelling, escape
    // sequences included, rather than being decoded and re-encoded.
    let text = "MSH|^~\\&|LAB\rOBX|3|ST|X||Sample hemolysed \\T\\ redrawn";
    let mut message = parse(text);
    Redactor::new(Policy::patient_identifiers()).redact(&mut message);
    assert_eq!(message.to_er7(), text);
}

#[test]
fn replacement_text_cannot_break_the_message() {
    // D11: replacement text goes in through `Subcomponent::set`, so a
    // delimiter inside it is escaped rather than splitting the message
    // (spec §3.5).
    let mut message = parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN");
    let policy = Policy::accept_all()
        .with("PID-5", Action::Replace("A|B^C~D&E".to_string()))
        .unwrap();
    Redactor::new(policy).redact(&mut message);

    let text = message.to_er7();
    assert_eq!(
        text,
        "MSH|^~\\&|LAB\rPID|1||9||A\\F\\B\\S\\C\\R\\D\\T\\E^A\\F\\B\\S\\C\\R\\D\\T\\E"
    );

    // It reads back as what was written, in the position it was written.
    let message = parse(&text);
    assert_eq!(
        shape(&message),
        shape(&parse("MSH|^~\\&|LAB\rPID|1||9||SMITH^JOHN"))
    );
    assert_eq!(
        message.query("PID-5.1").unwrap().as_deref(),
        Some("A|B^C~D&E")
    );

    // The same holds for a mask character that is itself a delimiter.
    let mut message = parse("MSH|^~\\&|LAB\rPID|AB");
    let policy = Policy::accept_all()
        .with("PID-1", Action::Mask('|'))
        .unwrap();
    Redactor::new(policy).redact(&mut message);
    assert_eq!(message.to_er7(), "MSH|^~\\&|LAB\rPID|\\F\\\\F\\");
    assert_eq!(
        parse(&message.to_er7()).query("PID-1").unwrap().as_deref(),
        Some("||")
    );
}

#[test]
fn pseudonyms_link_across_messages() {
    // The point of a pseudonym: the same identifier maps the same way in
    // every message redacted with the same key, and differently under a
    // different key (spec §7.1).
    let redactor = Redactor::new(Policy::patient_identifiers()).with_key(42);
    let one = {
        let mut message = parse("MSH|^~\\&|LAB\rPID|1||PATID1234^^^ACME^MR");
        redactor.redact(&mut message);
        message.query("PID-3.1").unwrap().expect("a value")
    };
    let two = {
        // Sixteen empty fields between PV1-2 and the visit number, so that
        // the identifier really is in field 19.
        let text = format!("MSH|^~\\&|LAB\rPV1|1|I{}|PATID1234", "|".repeat(16));
        let mut message = parse(&text);
        Redactor::new(
            Policy::accept_all()
                .with("PV1-19.1", Action::Pseudonym)
                .unwrap(),
        )
        .with_key(42)
        .redact(&mut message);
        message.query("PV1-19.1").unwrap().expect("a value")
    };
    assert_eq!(one, two);

    let other = {
        let mut message = parse("MSH|^~\\&|LAB\rPID|1||PATID1234^^^ACME^MR");
        Redactor::new(Policy::patient_identifiers())
            .with_key(43)
            .redact(&mut message);
        message.query("PID-3.1").unwrap().expect("a value")
    };
    assert_ne!(one, other);
}

#[test]
fn redaction_is_reproducible() {
    // Spec §2.7: nothing reads the clock, the environment, or a random
    // source, so the same policy over the same message is byte-identical
    // — which is what makes a redacted message diffable in a repository.
    let run = || {
        let mut message = parse(ADT);
        let report = Redactor::default().redact(&mut message);
        (message.to_er7(), report.to_string())
    };
    assert_eq!(run(), run());
}

#[test]
fn the_sample_policy_exercises_every_action() {
    // The policy file in `samples/` is the readable form of spec §6, and
    // reading it must not drift from writing it (D18).
    let policy = Policy::parse(POLICY).expect("the sample policy parses");
    assert_eq!(Policy::parse(&policy.to_string()).unwrap(), policy);

    let mut message = parse(ORU);
    Redactor::new(policy).redact(&mut message);

    assert_eq!(
        message.query("PID-5.1").unwrap().as_deref(),
        Some("REDACTED")
    );
    assert_eq!(message.query("PID-7").unwrap().as_deref(), Some("1961"));
    assert_eq!(message.query("PID-11.1").unwrap().as_deref(), Some(""));
    assert_eq!(
        message.query("PID-13[1]").unwrap().as_deref(),
        Some("************")
    );
    assert_eq!(message.query("NTE-3").unwrap().as_deref(), Some(""));
    // `null` says "clear your stored value", which is not the same as
    // saying nothing (spec §3.3); this message has no PID-19, so the
    // rule applies to nothing at all (D2).
    assert!(message.segment("PID").unwrap().field(19).is_none());
    // And the timestamp a `keep` rule exempted is still there.
    assert_eq!(
        message.query("MSH-7").unwrap().as_deref(),
        Some("20260815120000")
    );
}

#[test]
fn the_crate_has_one_runtime_dependency() {
    // D16: `er7` and nothing else, in a domain where every transitive
    // dependency is another crate somebody has to audit (spec §12.1).
    let manifest = include_str!("../Cargo.toml");
    let mut in_dependencies = false;
    let mut dependencies = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_dependencies = line.contains("dependencies");
            continue;
        }
        if !in_dependencies || line.is_empty() || line.starts_with('#') {
            continue;
        }
        dependencies.push(line.to_string());
    }
    assert_eq!(
        dependencies,
        [r#"er7 = { path = "../er7", version = "0" }"#]
    );
}

/// Run the command with `args`, feeding it `stdin`, and return
/// (exit status success, stdout, stderr).
fn cli(args: &[&str], stdin: &str) -> (bool, String, String) {
    use std::io::Write;
    let mut child = Command::new(env!("CARGO_BIN_EXE_er7-redact"))
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
fn cli_redacts_stdin_with_the_built_in_policy() {
    let (ok, stdout, _) = cli(&[], ADT);
    assert!(ok);
    assert!(!stdout.contains("JONES"), "{stdout}");
    assert!(!stdout.contains("PATID1234"), "{stdout}");
    // The header is untouched, and every segment is terminated.
    assert!(stdout.starts_with("MSH|^~\\&|ADT1|MCM|"), "{stdout}");
    assert!(stdout.ends_with('\r'), "{stdout:?}");
    assert!(er7::parse(&stdout).is_ok());
}

#[test]
fn cli_takes_rules_instead_of_the_built_in_policy() {
    // Spec §10.2: naming rules replaces the default rather than adding to
    // it, so what runs can be predicted from the arguments.
    let (ok, stdout, _) = cli(&["-r", "PID-5 replace X"], ADT);
    assert!(ok);
    assert!(stdout.contains("|X^X^X^X|"), "{stdout}");
    // The default policy was not applied, so the birth date is intact.
    assert!(stdout.contains("|19610615|"), "{stdout}");
}

#[test]
fn cli_rejects_everything_but_the_header() {
    // Spec §10.2: the two rejecting flags differ by exactly one rule —
    // whether the header survives to make the message routable.
    let (ok, stdout, _) = cli(&["--all-but-the-header"], ADT);
    assert!(ok);
    assert!(stdout.starts_with("MSH|^~\\&|ADT1|MCM|"), "{stdout}");
    assert!(stdout.contains("EVN|REDACTED|REDACTED"), "{stdout}");
    assert!(
        stdout.contains("ZPD|REDACTED|REDACTED^REDACTED^REDACTED"),
        "{stdout}"
    );
    assert!(er7::parse(&stdout).is_ok());

    let (ok, stdout, _) = cli(&["--reject-all"], ADT);
    assert!(ok);
    // The delimiters survive, because they are the delimiters (D5); the
    // rest of the header does not.
    assert!(
        stdout.starts_with("MSH|^~\\&|REDACTED|REDACTED|"),
        "{stdout}"
    );
    assert!(!stdout.contains("ADT1"), "{stdout}");
    assert!(er7::parse(&stdout).is_ok());

    // `--all` was removed, and says so rather than reading as unknown.
    let (ok, _, stderr) = cli(&["--all"], ADT);
    assert!(!ok);
    assert!(
        stderr.contains("--all is now --all-but-the-header"),
        "{stderr}"
    );
}

#[test]
fn cli_accept_all_switches_off_a_policy_files_reject() {
    // Spec §10.2: appending never weakens (D20), so `--accept-all` is
    // applied last and is the one thing that can.
    let policy = "OBX-3 clear\nreject replace REDACTED\n";
    let path = std::env::temp_dir().join("er7-redact-accept-all.policy");
    std::fs::write(&path, policy).expect("writes the policy");
    let path = path.to_string_lossy().into_owned();

    let (ok, rejecting, _) = cli(&["-p", &path], ORU);
    assert!(ok);
    assert!(rejecting.contains("REDACTED"), "{rejecting}");

    let (ok, accepting, _) = cli(&["--accept-all", "-p", &path], ORU);
    assert!(ok);
    assert!(!accepting.contains("REDACTED"), "{accepting}");
    // The file's own rule still ran; only its posture was overridden.
    assert!(!accepting.contains("2093-3"), "{accepting}");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn cli_masks_an_unrecognised_payload() {
    // D21: what a payload that is not ER7 gets is the policy's to say,
    // and only refusing it fails the run (spec §2.8, §10.4).
    //
    // The junk comes first because `er7::split_messages` cuts at headers:
    // anything after a message belongs to it, so the first payload is the
    // only one that can be unrecognised (spec §2.8).
    let batch = format!("{{\"patient\": \"EVERYWOMAN\"}}\r{ADT}");

    // The built-in default refuses, which is what it did before 0.2.
    let (ok, stdout, stderr) = cli(&[], &batch);
    assert!(!ok, "{stdout}");
    assert!(stdout.is_empty(), "{stdout}");
    assert!(stderr.contains("message 1"), "{stderr}");

    // `--reject-all` masks it whole, and nothing of it survives.
    let (ok, stdout, _) = cli(&["--reject-all"], &batch);
    assert!(ok);
    assert!(!stdout.contains("EVERYWOMAN"), "{stdout}");
    assert!(stdout.starts_with("****"), "{stdout}");
    // The message after it was still redacted.
    assert!(!stdout.contains("JONES"), "{stdout}");
    // What comes out is not itself a message, and cannot be: a masked
    // payload is a wall of asterisks where a header would have to be.
    assert!(er7::parse(&stdout).is_err());

    // A policy file can ask for either, and the report says which.
    let path = std::env::temp_dir().join("er7-redact-unrecognised.policy");
    std::fs::write(&path, "PID-5 clear\nunrecognised pass\n").expect("writes the policy");
    let path = path.to_string_lossy().into_owned();

    let (ok, stdout, _) = cli(&["-p", &path], &batch);
    assert!(ok);
    assert!(stdout.contains("{\"patient\": \"EVERYWOMAN\"}"), "{stdout}");

    let (ok, stdout, _) = cli(&["-p", &path, "--report"], &batch);
    assert!(ok);
    assert!(
        stdout.contains("# message 1: unrecognised payload, passed through"),
        "{stdout}"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn cli_writes_a_report_instead_of_the_message() {
    let (ok, stdout, _) = cli(&["--report"], ADT);
    assert!(ok);
    // Paths are padded to a common width, then two spaces, then the
    // action (spec §10.3).
    assert!(
        stdout.contains("PID[1]-5[1].1.1   replace REDACTED\n"),
        "{stdout}"
    );
    assert!(stdout.contains("PID[1]-11[1].1.1  clear\n"), "{stdout}");
    // D13: a report holds paths and actions, and no values.
    for value in ["JONES", "PATID1234", "19610615"] {
        assert!(!stdout.contains(value), "the report leaked {value}");
    }
    // A dry run: the message itself is not written.
    assert!(!stdout.contains("MSH|"), "{stdout}");
}

#[test]
fn cli_shows_the_policy_without_reading_input() {
    // No FILE, no stdin: `--show-policy` must not block (spec §10.3).
    let (ok, stdout, _) = cli(&["--show-policy"], "");
    assert!(ok);
    assert!(stdout.starts_with("PID-2.1   pseudonym\n"), "{stdout}");
    // Both defaults are stated, so a reader never has to know which one
    // was the quiet one (spec §6.5).
    assert!(
        stdout.ends_with("\naccept\nunrecognised  refuse\n"),
        "{stdout}"
    );
    // What it writes is a policy file that reads back.
    let policy = Policy::parse(&stdout).expect("the shown policy parses");
    assert_eq!(policy, Policy::patient_identifiers());

    let (ok, stdout, _) = cli(&["--show-policy", "--all-but-the-header"], "");
    assert!(ok);
    assert_eq!(
        stdout,
        "MSH  keep\n\nreject        replace REDACTED\nunrecognised  refuse\n"
    );
}

#[test]
fn cli_selects_one_message_and_a_terminator() {
    let batch = format!("{ADT}\r{ORU}");
    let (ok, stdout, _) = cli(&["--message", "2", "--terminator", "lf"], &batch);
    assert!(ok);
    assert!(stdout.starts_with("MSH|^~\\&|LAB|ACME|"), "{stdout}");
    assert!(!stdout.contains('\r'), "{stdout:?}");
    assert_eq!(stdout.lines().count(), 7);

    let (ok, _, stderr) = cli(&["--message", "9"], &batch);
    assert!(!ok);
    assert!(stderr.contains("the input holds 2"), "{stderr}");
}

#[test]
fn cli_keys_its_pseudonyms() {
    let (ok, one, _) = cli(&["-r", "PID-3.1 pseudonym", "-k", "1"], ADT);
    let (_, two, _) = cli(&["-r", "PID-3.1 pseudonym", "-k", "2"], ADT);
    let (_, again, _) = cli(&["-r", "PID-3.1 pseudonym", "-k", "1"], ADT);
    assert!(ok);
    assert_ne!(one, two);
    assert_eq!(one, again);
}

#[test]
fn cli_reports_errors_on_stderr() {
    // D15: the two situations that can fail, plus the argument errors the
    // command adds (spec §10.4).
    for (args, input, expected) in [
        (vec![], "PID|1", "not the MSH, FHS, or BHS header"),
        (vec![], "", "no HL7 segments"),
        (vec!["-r", "PID-5 obfuscate"], ADT, "unknown action"),
        (vec!["-r", "PID-0 clear"], ADT, "invalid HL7 path"),
        (vec!["-p", "no/such/policy"], ADT, "reading no/such/policy"),
        (vec!["--key", "many"], ADT, "--key wants a number"),
        (vec!["--nope"], ADT, "unknown option"),
    ] {
        let (ok, stdout, stderr) = cli(&args, input);
        assert!(!ok, "expected {args:?} to fail");
        assert!(stdout.is_empty(), "{stdout}");
        assert!(stderr.contains(expected), "{args:?}: {stderr}");
        assert!(stderr.starts_with("er7-redact: error: "), "{stderr}");
    }
}

#[test]
fn cli_a_policy_that_changes_nothing_is_not_an_error() {
    // Spec §10.4: the message simply carried none of the positions the
    // policy names, which is not a failure.
    let (ok, stdout, stderr) = cli(&["-r", "ZZZ-1 clear"], ORU);
    assert!(ok);
    assert!(stderr.is_empty(), "{stderr}");
    assert_eq!(stdout, format!("{ORU}\r"));

    let (ok, stdout, _) = cli(&["-r", "ZZZ-1 clear", "--report"], ORU);
    assert!(ok);
    assert!(stdout.is_empty(), "{stdout}");
}

#[test]
fn cli_prints_help_and_version() {
    for flag in ["-h", "--help"] {
        let (ok, stdout, _) = cli(&[flag], "");
        assert!(ok);
        assert!(
            stdout.contains("Usage: er7-redact [OPTIONS] [FILE]"),
            "{stdout}"
        );
    }
    for flag in ["-V", "--version"] {
        let (ok, stdout, _) = cli(&[flag], "");
        assert!(ok);
        assert_eq!(
            stdout,
            format!("er7-redact {}\n", env!("CARGO_PKG_VERSION"))
        );
    }
}

/// Whether `name` is a Markdown file, by extension.
fn is_markdown(name: &str) -> bool {
    std::path::Path::new(name)
        .extension()
        .is_some_and(|extension| extension == "md")
}

/// Every `| D<n> |` cell at the start of a table row in `text`.
fn rule_ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("| D"))
        .filter_map(|rest| rest.split_once(" |"))
        .map(|(number, _)| format!("D{number}"))
        .filter(|id| id[1..].chars().all(|c| c.is_ascii_digit()))
        .collect()
}

#[test]
fn every_rule_has_a_coverage_row() {
    // Spec-driven development only works if the spec is the single source
    // of truth. A rule in the §1.4 index with no row in the §11.1 coverage
    // table is a rule nobody agreed to test, and a row in §11.1 for a rule
    // that no longer exists is a table nobody re-read
    // (AGENTS/spec-driven-development.md).
    let declared = rule_ids(include_str!("../spec/01-purpose-and-scope.md"));
    let covered = rule_ids(include_str!("../spec/11-testing-strategy.md"));

    assert_eq!(declared.len(), 21, "§1.4 should index D1–D21");
    let missing: Vec<&String> = declared.iter().filter(|d| !covered.contains(d)).collect();
    assert!(missing.is_empty(), "no row in §11.1 for {missing:?}");
    let orphan: Vec<&String> = covered.iter().filter(|d| !declared.contains(d)).collect();
    assert!(
        orphan.is_empty(),
        "§11.1 covers {orphan:?}, which §1.4 does not declare"
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
        .filter(|name| name.len() > 3 && name.as_bytes()[2] == b'-' && is_markdown(name))
        .map(str::to_string)
        .collect();
    linked.sort();
    linked.dedup();

    let mut on_disk: Vec<String> = std::fs::read_dir(&directory)
        .expect("spec directory")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| is_markdown(name) && name.as_bytes()[0].is_ascii_digit())
        .collect();
    on_disk.sort();

    assert_eq!(linked, on_disk, "spec/index.md and spec/ disagree");

    // And every section names itself, so a file that was copied keeps its
    // own number rather than its neighbour's.
    for name in &on_disk {
        let number = name[..2].trim_start_matches('0');
        let heading = format!("# {number}. ");
        let text = std::fs::read_to_string(directory.join(name)).expect("section");
        assert!(
            text.contains(&heading),
            "{name} has no `{heading}` heading of its own"
        );
    }
}

#[test]
fn the_documented_positions_match_the_built_in_policy() {
    // D14: spec §5.1 is the normative table and `Policy::patient_identifiers`
    // is its executable form. They are changed together, so a path in one
    // and not the other is a bug in whichever was forgotten.
    let spec = include_str!("../spec/05-built-in-policies.md");
    // §5.1 only: §5.4 tabulates the positions the policy deliberately does
    // *not* touch, which are paths in the same shape.
    let spec = spec.split("## 5.2").next().expect("§5.1 comes first");
    let documented: Vec<String> = spec
        .lines()
        .filter_map(|line| line.strip_prefix("| `"))
        .filter_map(|rest| rest.split_once('`'))
        .map(|(path, _)| path.to_string())
        .filter(|path| path.contains('-') && path.as_bytes()[0].is_ascii_uppercase())
        .collect();

    let policy: Vec<String> = Policy::patient_identifiers()
        .rules
        .iter()
        .map(|rule| rule.path.to_string())
        .collect();

    assert_eq!(documented, policy, "spec §5.1 and Policy disagree");
}
