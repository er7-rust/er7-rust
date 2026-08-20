//! The `er7-redact` command: remove patient detail from HL7 v2 messages
//! in the ER7 encoding, and say what it removed.
//!
//! With no options it applies the curated policy of spec §5.1 and writes
//! the redacted messages. `--report` says what would change and writes
//! nothing else; `--show-policy` writes the policy itself, which is how a
//! caller turns the built-in default into a file to edit.
//!
//! This binary adds no behaviour of its own: everything it writes is the
//! library's output, formatted. It uses the published public API only, so
//! anything it needs, a downstream crate can have too.
//!
//! The input/output contract — options, formats, exit codes — is specified
//! by spec §10, and pinned by the `cli_*` tests in `tests/integration.rs`.

use std::fmt::Write as _;
use std::io::{Read, Write};
use std::process::ExitCode;

use er7::{RenderOptions, Terminator};
use er7_redact::{Policy, Redactor, Report, Rule};

const USAGE: &str = "\
Redact patient detail from HL7 v2 messages in the ER7 pipe-hat encoding.

Usage: er7-redact [OPTIONS] [FILE]

Arguments:
  [FILE]  Input holding one or more messages, or a batch file;
          \"-\" or omitted reads standard input

Options:
  -p, --policy <FILE>      Read rules from a policy file; may be repeated
  -r, --rule <RULE>        Add one rule, e.g. \"PID-5 replace REDACTED\";
                           may be repeated
  -a, --all                Redact everything except the MSH header
  -k, --key <KEY>          Pseudonym key, a number; default 0
  -m, --message <N>        Use only the Nth message of the input
  -t, --terminator <KIND>  Segment terminator to write: cr (default), lf, crlf
  -o, --output <FILE>      Write to FILE instead of standard output
      --report             Write what would change, and not the message
      --show-policy        Write the policy that would be applied, and exit
  -h, --help               Print help
  -V, --version            Print version

With no --policy, --rule, or --all, the built-in policy of the crate's
spec section 5.1 is applied: the patient identifiers in PID, NK1, PV1,
GT1, and IN1. It is a starting point, not a compliance certification.";

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
    let mut all = false;
    let mut key: u64 = 0;
    let mut report_only = false;
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
            "-a" | "--all" => all = true,
            "--report" => report_only = true,
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

    let policy = policy(all, &policies, &rules)?;
    if show_policy {
        // Deliberately before any input is read, so this works with no
        // FILE argument and never blocks on standard input (spec §10.3).
        return write_output(output.as_deref(), &policy.to_string());
    }

    let text = read_input(input.as_deref())?;
    let mut sources = er7::split_messages(&text);
    if sources.is_empty() {
        return fail("input contains no HL7 segments");
    }
    if let Some(n) = which {
        match sources.get(n - 1) {
            Some(&source) => sources = vec![source],
            None => {
                return fail(format!(
                    "--message {n}, but the input holds {}",
                    sources.len()
                ));
            }
        }
    }

    // Every message is parsed before anything is written, so a malformed
    // message late in a batch fails the run rather than producing a
    // half-redacted output (spec §10.1).
    let mut messages = Vec::with_capacity(sources.len());
    for (index, source) in sources.iter().enumerate() {
        match er7::parse(source) {
            Ok(message) => messages.push(message),
            Err(e) => return fail(format!("message {}: {e}", index + 1)),
        }
    }

    let redactor = Redactor::new(policy).with_key(key);
    let reports: Vec<Report> = messages
        .iter_mut()
        .map(|message| redactor.redact(message))
        .collect();

    let options = RenderOptions {
        terminator,
        trailing_terminator: true,
    };
    let rendered = if report_only {
        report(&reports)
    } else {
        messages.iter().map(|m| m.to_er7_with(options)).collect()
    };
    write_output(output.as_deref(), &rendered)
}

/// Assemble the policy the run applies (spec §10.2). The built-in default
/// is used only when nothing else is asked for, so that what runs can be
/// predicted from the arguments.
fn policy(all: bool, policies: &[String], rules: &[String]) -> Result<Policy, Exit> {
    let mut policy = match (all, policies.is_empty() && rules.is_empty()) {
        (true, _) => Policy::everything(),
        (false, true) => Policy::patient_identifiers(),
        (false, false) => Policy::new(),
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
fn report(reports: &[Report]) -> String {
    let mut out = String::new();
    for (index, report) in reports.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if reports.len() > 1 {
            let _ = writeln!(out, "# message {}", index + 1);
        }
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
