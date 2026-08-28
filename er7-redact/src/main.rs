//! The `er7-redact` command: remove patient detail from HL7® v2 messages
//! in the ER7 encoding, and say what it removed.
//!
//! With no options it applies the curated policy of spec §5.1 and writes
//! the redacted messages. `--report` says what would change and writes
//! nothing else; `--uncovered` says what no rule named and writes nothing
//! else either; `--show-policy` writes the policy itself, which is how a
//! caller turns the built-in default into a file to edit.
//!
//! This binary adds no behaviour of its own: everything it writes is the
//! library's output, formatted. It uses the published public API only, so
//! anything it needs, a downstream crate can have too.
//!
//! The input/output contract — options, formats, exit codes — is specified
//! by spec §10, and pinned by the `cli_*` tests in `tests/integration.rs`.
#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::io::{Read, Write};
use std::process::ExitCode;

use er7::{Message, Path, RenderOptions, Terminator};
use er7_redact::{Policy, Posture, Redactor, Report, Rule, Unrecognised};

const USAGE: &str = "\
Redact patient detail from HL7® v2 messages in the ER7 pipe-hat encoding.

Usage: er7-redact [OPTIONS] [FILE]

Arguments:
  [FILE]  Input holding one or more messages, or a batch file;
          \"-\" or omitted reads standard input

Options:
  -p, --policy <FILE>      Read rules from a policy file; may be repeated
  -r, --rule <RULE>        Add one rule, e.g. \"PID-5 replace REDACTED\";
                           may be repeated
      --accept-all         Accept every value no rule names; applied last,
                           this switches off a policy file's \"reject\"
      --reject-all         Reject every value no rule names, the MSH
                           header included
      --all-but-the-header Reject every value no rule names, but keep the
                           MSH header so the message stays routable
  -k, --key <KEY>          Pseudonym key, a number; default 0
  -m, --message <N>        Use only the Nth message of the input
  -t, --terminator <KIND>  Segment terminator to write: cr (default), lf, crlf
  -o, --output <FILE>      Write to FILE instead of standard output
      --report             Write what would change, and not the message
      --uncovered          Write the positions no rule names, and not the
                           message
      --show-policy        Write the policy that would be applied, and exit
  -h, --help               Print help
  -V, --version            Print version

With no --policy, --rule, or posture flag, the built-in policy of the
crate's spec section 5.1 is applied: the patient identifiers in PID, NK1,
PV1, GT1, and IN1. It is a starting point, not a compliance certification.

A payload that is not ER7 fails the run, unless a policy file says to pass
it through or to mask it whole, or --reject-all masks it.

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Exit::Help) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Err(Exit::Version) => {
            println!("er7-redact {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Err(Exit::Failed(message)) => {
            eprintln!("er7-redact: error: {message}");
            ExitCode::FAILURE
        }
    }
}

enum Exit {
    Help,
    Version,
    Failed(String),
}

fn fail<T>(message: impl Into<String>) -> Result<T, Exit> {
    Err(Exit::Failed(message.into()))
}

fn run() -> Result<(), Exit> {
    let mut policies: Vec<String> = Vec::new();
    let mut rules: Vec<String> = Vec::new();
    let mut start: Option<Start> = None;
    let mut accept_all = false;
    let mut key: u64 = 0;
    let mut report_only = false;
    let mut uncovered_only = false;
    let mut show_policy = false;
    let mut which: Option<usize> = None;
    let mut terminator = Terminator::Cr;
    let mut input: Option<String> = None;
    let mut output: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |name: &str| match args.next() {
            Some(value) => Ok(value),
            None => fail(format!("missing value for {name}")),
        };
        match arg.as_str() {
            "-h" | "--help" => return Err(Exit::Help),
            "-V" | "--version" => return Err(Exit::Version),
            // Removed in 0.2, and refused by name rather than as an
            // unknown option: `-a` meant *reject* everything, so a script
            // that kept running under a renamed flag could silently have
            // switched posture (spec §10.2).
            "-a" | "--all" => {
                return fail(
                    "--all is now --all-but-the-header \
                     (or --reject-all, which redacts the header too)",
                );
            }
            "--accept-all" => {
                accept_all = true;
                start = Some(Start::Neutral);
            }
            "--reject-all" => start = Some(Start::RejectAll),
            "--all-but-the-header" => start = Some(Start::AllButTheHeader),
            "--report" => report_only = true,
            "--uncovered" => uncovered_only = true,
            "--show-policy" => show_policy = true,
            "-p" | "--policy" => policies.push(value("--policy")?),
            "-r" | "--rule" => rules.push(value("--rule")?),
            "-o" | "--output" => output = Some(value("--output")?),
            "-k" | "--key" => {
                let text = value("--key")?;
                match text.parse::<u64>() {
                    Ok(n) => key = n,
                    Err(_) => return fail(format!("--key wants a number, not {text:?}")),
                }
            }
            "-m" | "--message" => {
                let text = value("--message")?;
                match text.parse::<usize>() {
                    Ok(n) if n >= 1 => which = Some(n),
                    _ => return fail(format!("--message wants a number from 1, not {text:?}")),
                }
            }
            "-t" | "--terminator" => {
                let text = value("--terminator")?;
                terminator = match text.as_str() {
                    "cr" => Terminator::Cr,
                    "lf" => Terminator::Lf,
                    "crlf" => Terminator::CrLf,
                    _ => return fail(format!("--terminator wants cr, lf, or crlf, not {text:?}")),
                }
            }
            // Guarded like any other input: without the check, a second
            // "-" would silently replace the first input with stdin.
            "-" if input.is_some() => return fail("more than one input file given"),
            "-" => input = Some("-".to_string()),
            _ if arg.starts_with('-') => return fail(format!("unknown option: {arg}")),
            _ if input.is_some() => return fail("more than one input file given"),
            _ => input = Some(arg),
        }
    }

    let policy = policy(start, accept_all, &policies, &rules)?;
    if show_policy {
        // Deliberately before any input is read, so this works with no
        // FILE argument and never blocks on standard input (spec §10.3).
        return write_output(output.as_deref(), &policy.to_string());
    }

    let text = read_input(input.as_deref())?;
    let sources = split_input(&text, which)?;

    let redactor = Redactor::new(policy).with_key(key);
    let payloads = redact_payloads(&redactor, &sources)?;

    let options = RenderOptions {
        terminator,
        trailing_terminator: true,
    };
    // --uncovered wins if both are given, the same tolerant precedence
    // the starting-posture flags already use: whichever is checked first.
    let rendered = if uncovered_only {
        uncovered_report(&payloads, redactor.policy())
    } else if report_only {
        report(&payloads, redactor.policy())
    } else {
        payloads
            .iter()
            .map(|payload| payload.to_er7_with(options))
            .collect()
    };
    write_output(output.as_deref(), &rendered)
}

/// Cut the input into payloads, and keep the one `--message` asked for
/// (spec §10.1).
fn split_input(text: &str, which: Option<usize>) -> Result<Vec<&str>, Exit> {
    let sources = er7::split_messages(text);
    if sources.is_empty() {
        return fail("input contains no HL7 segments");
    }
    let Some(n) = which else { return Ok(sources) };
    match sources.get(n - 1) {
        Some(&source) => Ok(vec![source]),
        None => fail(format!(
            "--message {n}, but the input holds {}",
            sources.len()
        )),
    }
}

/// Redact every payload of the input (spec §10.1).
///
/// All of them are parsed before any is written, so one the policy refuses
/// fails the run rather than producing a half-redacted output, however
/// late in a batch it is.
fn redact_payloads(redactor: &Redactor, sources: &[&str]) -> Result<Vec<Payload>, Exit> {
    let mut payloads = Vec::with_capacity(sources.len());
    for (index, source) in sources.iter().enumerate() {
        match er7::parse(source) {
            Ok(mut message) => {
                // Computed before redact() mutates the message, so a
                // rejecting posture blanking a position first can never
                // make it look covered (spec §2.9): --uncovered would
                // otherwise report fewer gaps than a caller who checked
                // before running the redaction that answered them.
                let uncovered = redactor.uncovered(&message);
                let report = redactor.redact(&mut message);
                payloads.push(Payload::Message(Box::new(message), report, uncovered));
            }
            // D21: what a payload that is not ER7 gets is the policy's to
            // say, and only refusing it fails the run (spec §2.8).
            Err(e) => match redactor.unrecognised(source) {
                Some(text) => payloads.push(Payload::Unrecognised(text)),
                None => return fail(format!("message {}: {e}", index + 1)),
            },
        }
    }
    Ok(payloads)
}

/// Which built-in policy the run starts from (spec §10.2).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Start {
    /// An empty accepting policy that still refuses a payload it cannot
    /// read: what `--policy` and `--rule` are applied on top of, and what
    /// `--accept-all` starts from. The CLI keeps the fail-closed
    /// disposition here even though `Policy::accept_all` passes, because
    /// a run that was asked for no policy at all has been told nothing
    /// about what an unreadable payload is worth (spec §10.2).
    Neutral,
    /// `--reject-all`.
    RejectAll,
    /// `--all-but-the-header`.
    AllButTheHeader,
}

/// One payload of the input, after the policy has had its say.
enum Payload {
    /// It parsed: the redacted message, what changed in it, and the
    /// positions no rule named, computed before redaction changed
    /// anything (spec §2.9).
    Message(Box<Message>, Report, Vec<Path>),
    /// It did not parse and the policy did not refuse it: the text to
    /// write in its place (spec §2.8).
    Unrecognised(String),
}

impl Payload {
    /// What to write for this payload, terminated so that the next one
    /// starts on its own segment.
    fn to_er7_with(&self, options: RenderOptions) -> String {
        match self {
            Payload::Message(message, ..) => message.to_er7_with(options),
            Payload::Unrecognised(text) => {
                let mut text = text.clone();
                // A passed-through payload usually carries its own line
                // ending; a masked one has had every character of it
                // replaced, and without this would run into the payload
                // after it.
                if !text.ends_with(['\r', '\n']) {
                    text.push_str(match options.terminator {
                        Terminator::Cr => "\r",
                        Terminator::Lf => "\n",
                        Terminator::CrLf => "\r\n",
                    });
                }
                text
            }
        }
    }
}

/// Assemble the policy the run applies (spec §10.2). The built-in default
/// is used only when nothing else is asked for, so that what runs can be
/// predicted from the arguments.
fn policy(
    start: Option<Start>,
    accept_all: bool,
    policies: &[String],
    rules: &[String],
) -> Result<Policy, Exit> {
    // The CLI's own empty policy still refuses a payload it cannot read,
    // where `Policy::accept_all` passes one: a run that was handed rules
    // and nothing else has been told nothing about what an unreadable
    // payload is worth, and refusing is the answer that loses no value
    // quietly (spec §10.2).
    let neutral = || Policy::accept_all().on_unrecognised(Unrecognised::Refuse);
    let named_nothing = start.is_none() && policies.is_empty() && rules.is_empty();
    let mut policy = match start {
        // The built-in default is used only when nothing else was asked
        // for, so that what runs can be predicted from the arguments.
        _ if named_nothing => Policy::patient_identifiers(),
        Some(Start::RejectAll) => Policy::reject_all(),
        Some(Start::AllButTheHeader) => Policy::all_but_the_header(),
        Some(Start::Neutral) | None => neutral(),
    };
    for path in policies {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => return fail(format!("reading {path}: {e}")),
        };
        match Policy::parse(&text) {
            Ok(parsed) => policy.append(parsed),
            Err(e) => return fail(format!("{path}: {e}")),
        }
    }
    for text in rules {
        match Rule::parse(text) {
            Ok(rule) => policy.rules.push(rule),
            Err(e) => return fail(e.to_string()),
        }
    }
    // Last, and so it wins: appending never weakens a posture (D20), which
    // would leave no way at all to run a policy file's rules without its
    // `reject` line (spec §10.2).
    if accept_all {
        policy = policy.posture(Posture::Accept);
    }
    Ok(policy)
}

fn read_input(path: Option<&str>) -> Result<String, Exit> {
    match path {
        None | Some("-") => {
            let mut buffer = String::new();
            match std::io::stdin().read_to_string(&mut buffer) {
                Ok(_) => Ok(buffer),
                Err(e) => fail(format!("reading standard input: {e}")),
            }
        }
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => Ok(text),
            Err(e) => fail(format!("reading {path}: {e}")),
        },
    }
}

fn write_output(path: Option<&str>, text: &str) -> Result<(), Exit> {
    match path {
        Some(path) => match std::fs::write(path, text) {
            Ok(()) => Ok(()),
            Err(e) => fail(format!("writing {path}: {e}")),
        },
        None => match std::io::stdout().write_all(text.as_bytes()) {
            Ok(()) => Ok(()),
            // A closed pipe is how `head` says it has seen enough.
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
            Err(e) => fail(format!("writing to standard output: {e}")),
        },
    }
}

/// Lay every change out next to the path that names it (spec §10.3).
fn report(payloads: &[Payload], policy: &Policy) -> String {
    let mut out = String::new();
    for (index, payload) in payloads.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if payloads.len() > 1 {
            let _ = writeln!(out, "# message {}", index + 1);
        }
        let report = match payload {
            Payload::Message(_, report, _) => report,
            // A payload with no positions in it has no rows to write. It
            // gets a comment instead, which no reader can mistake for a
            // change (spec §10.3).
            Payload::Unrecognised(_) => {
                let what = match &policy.unrecognised {
                    Unrecognised::Pass => "passed through".to_string(),
                    Unrecognised::Apply(action) => action.to_string(),
                    Unrecognised::Refuse => unreachable!("a refused payload failed the run"),
                };
                let _ = writeln!(out, "# message {}: unrecognised payload, {what}", index + 1);
                continue;
            }
        };
        let width = report
            .changes
            .iter()
            .map(|change| change.path.to_string().len())
            .max()
            .unwrap_or(0)
            .clamp(8, 28);
        for change in &report.changes {
            let path = change.path.to_string();
            let _ = writeln!(out, "{path:<width$}  {}", change.action);
        }
    }
    out
}

/// List every position no rule names, one per line (spec §10.3, §2.9).
fn uncovered_report(payloads: &[Payload], policy: &Policy) -> String {
    let mut out = String::new();
    for (index, payload) in payloads.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if payloads.len() > 1 {
            let _ = writeln!(out, "# message {}", index + 1);
        }
        let gaps = match payload {
            Payload::Message(_, _, gaps) => gaps,
            // Same reasoning as `report`: nothing parsed, so there are no
            // positions to name, and a comment says so instead.
            Payload::Unrecognised(_) => {
                let what = match &policy.unrecognised {
                    Unrecognised::Pass => "passed through".to_string(),
                    Unrecognised::Apply(action) => action.to_string(),
                    Unrecognised::Refuse => unreachable!("a refused payload failed the run"),
                };
                let _ = writeln!(out, "# message {}: unrecognised payload, {what}", index + 1);
                continue;
            }
        };
        for path in gaps {
            let _ = writeln!(out, "{path}");
        }
    }
    out
}
