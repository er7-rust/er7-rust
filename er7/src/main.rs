//! The `er7` command: read HL7® v2 messages in ER7 encoding and show what
//! is in them.
//!
//! The default output is an outline — one line per value, labelled with the
//! HL7 path that names it — because the hardest thing about a pipe-hat
//! message is working out which position a value is actually in. Every
//! label is a valid `--query` argument, so a path read off the outline can
//! be pasted straight back in.
//!
//! This binary adds no behaviour of its own: everything it prints is the
//! library's output, formatted. It uses the published public API only, so
//! anything it needs, a downstream crate can have too.
//!
//! The input/output contract — options, outline format, exit codes — is
//! specified by spec §12, and pinned by the `cli_*` tests in
//! `tests/integration.rs`.
#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::io::{Read, Write};
use std::process::ExitCode;

use er7::{Message, RenderOptions, Subcomponent, Terminator};

const USAGE: &str = "\
Read HL7® v2 messages in the ER7 pipe-hat encoding.

Usage: er7 [OPTIONS] [FILE]

Arguments:
  [FILE]  Input holding one or more messages, or a batch file;
          \"-\" or omitted reads standard input

Options:
  -q, --query <PATH>       Print the values at an HL7 path such as PID-5.1
                           or OBX[2]-5; may be given more than once
  -n, --normalize          Rewrite the input as canonical ER7
  -m, --message <N>        Use only the Nth message of the input
  -r, --raw                Show text as sent, without decoding escapes
  -t, --terminator <KIND>  Segment terminator to write: cr (default), lf, crlf
  -o, --output <FILE>      Write to FILE instead of standard output
  -h, --help               Print help
  -V, --version            Print version

With neither --query nor --normalize, print an outline of every value in
the input, labelled with the HL7 path that names it.

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.";

/// What the command was asked to produce.
enum Action {
    /// Label every value with its path.
    Outline,
    /// Print the values found at each of these paths.
    Query(Vec<String>),
    /// Write the messages back out as canonical ER7.
    Normalize,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Exit::Help) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Err(Exit::Version) => {
            println!("er7 {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Err(Exit::Failed(message)) => {
            eprintln!("er7: error: {message}");
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
    let mut queries: Vec<String> = Vec::new();
    let mut normalize = false;
    let mut raw = false;
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
            "-n" | "--normalize" => normalize = true,
            "-r" | "--raw" => raw = true,
            "-q" | "--query" => queries.push(value("--query")?),
            "-o" | "--output" => output = Some(value("--output")?),
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
            // Guarded like any other input argument: without the check, a
            // second input given as "-" would silently replace the first
            // and read standard input instead of the named file.
            "-" => {
                if input.is_some() {
                    return fail("more than one input file given");
                }
                input = Some("-".to_string());
            }
            _ if arg.starts_with('-') => return fail(format!("unknown option: {arg}")),
            _ if input.is_some() => return fail("more than one input file given"),
            _ => input = Some(arg),
        }
    }

    if normalize && !queries.is_empty() {
        return fail("--normalize and --query ask for different output; choose one");
    }
    let action = if normalize {
        Action::Normalize
    } else if queries.is_empty() {
        Action::Outline
    } else {
        Action::Query(queries)
    };

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

    let mut messages = Vec::with_capacity(sources.len());
    for (index, source) in sources.iter().enumerate() {
        match er7::parse(source) {
            Ok(message) => messages.push(message),
            Err(e) => return fail(format!("message {}: {e}", index + 1)),
        }
    }

    let options = RenderOptions {
        terminator,
        trailing_terminator: true,
    };
    let rendered = match action {
        Action::Normalize => messages.iter().map(|m| m.to_er7_with(options)).collect(),
        Action::Query(paths) => query(&messages, &paths, raw)?,
        Action::Outline => outline(&messages, raw),
    };

    write_output(output.as_deref(), &rendered)
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

/// Print every value found at each path, one per line, across every
/// message. A path that matches nothing contributes nothing.
fn query(messages: &[Message], paths: &[String], raw: bool) -> Result<String, Exit> {
    let mut out = String::new();
    for path in paths {
        let path = match path.parse::<er7::Path>() {
            Ok(path) => path,
            Err(e) => return fail(e.to_string()),
        };
        for message in messages {
            let values = if raw {
                message.query_path_raw(&path)
            } else {
                message.query_path(&path)
            };
            for value in values {
                out.push_str(&for_display(&value));
                out.push('\n');
            }
        }
    }
    Ok(out)
}

/// Lay out every value in every message next to the HL7 path that names it.
fn outline(messages: &[Message], raw: bool) -> String {
    let mut out = String::new();
    for (index, message) in messages.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if messages.len() > 1 {
            let _ = writeln!(out, "# message {}{}", index + 1, describe(message));
        }
        let rows = rows(message, raw);
        let width = rows
            .iter()
            .map(|(label, _)| label.len())
            .max()
            .unwrap_or(0)
            .clamp(8, 28);
        for (label, value) in rows {
            let _ = writeln!(out, "{label:<width$}  {value}");
        }
    }
    out
}

/// A short summary of a message for the outline's heading, e.g.
/// ": ORU^R01 MSG9 (HL7 2.5)". Every part is optional, because a message
/// that omits them still has to be readable.
fn describe(message: &Message) -> String {
    let mut parts = Vec::new();
    if let Some(code) = message.message_code() {
        match message.trigger_event() {
            Some(trigger) => parts.push(format!("{code}^{trigger}")),
            None => parts.push(code),
        }
    }
    parts.extend(message.control_id());
    parts.extend(message.version().map(|v| format!("(HL7 {v})")));
    if parts.is_empty() {
        return String::new();
    }
    format!(": {}", parts.join(" "))
}

/// One (path, value) row per value the message carries.
fn rows(message: &Message, raw: bool) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    for (index, segment) in message.segments.iter().enumerate() {
        // Label a repeated segment with its occurrence, so that the paths
        // in the outline can be pasted straight back into --query.
        let total = message
            .segments
            .iter()
            .filter(|s| s.name == segment.name)
            .count();
        let occurrence = message.segments[..index]
            .iter()
            .filter(|s| s.name == segment.name)
            .count()
            + 1;
        let prefix = if total > 1 {
            format!("{}[{occurrence}]", segment.name)
        } else {
            segment.name.clone()
        };
        for (index, field) in segment.fields.iter().enumerate() {
            let number = index + 1;
            let label = format!("{prefix}-{number}");
            // A header's first two fields are the delimiters themselves.
            if segment.is_header() && number <= 2 {
                rows.push((label, for_display(&field.to_er7(&message.separators))));
                continue;
            }
            if field.is_empty() && !field.is_null() {
                continue;
            }
            match field.repetitions.as_slice() {
                [only] => push_repetition(&mut rows, label, only, message, raw),
                many => {
                    for (index, repetition) in many.iter().enumerate() {
                        if repetition.is_empty() && !repetition.is_null() {
                            continue;
                        }
                        let label = format!("{label}[{}]", index + 1);
                        push_repetition(&mut rows, label, repetition, message, raw);
                    }
                }
            }
        }
    }
    rows
}

fn push_repetition(
    rows: &mut Vec<(String, String)>,
    label: String,
    repetition: &er7::Repetition,
    message: &Message,
    raw: bool,
) {
    match repetition.components.as_slice() {
        [only] => push_component(rows, label, only, message, raw),
        many => {
            for (index, component) in many.iter().enumerate() {
                if component.is_empty() && !component.is_null() {
                    continue;
                }
                let label = format!("{label}.{}", index + 1);
                push_component(rows, label, component, message, raw);
            }
        }
    }
}

fn push_component(
    rows: &mut Vec<(String, String)>,
    label: String,
    component: &er7::Component,
    message: &Message,
    raw: bool,
) {
    match component.subcomponents.as_slice() {
        [only] => rows.push((label, leaf(only, message, raw))),
        many => {
            for (index, subcomponent) in many.iter().enumerate() {
                if subcomponent.is_empty() {
                    continue;
                }
                let label = format!("{label}.{}", index + 1);
                rows.push((label, leaf(subcomponent, message, raw)));
            }
        }
    }
}

fn leaf(subcomponent: &Subcomponent, message: &Message, raw: bool) -> String {
    // Show the explicit null as it was sent. It decodes to the empty
    // string, and printing nothing here would hide the very thing that
    // makes it different from a field the sender left out.
    if subcomponent.is_null() {
        return subcomponent.raw.clone();
    }
    let value = if raw {
        subcomponent.raw.clone()
    } else {
        subcomponent.value(&message.separators).into_owned()
    };
    for_display(&value)
}

/// Make a value safe to print on one line: a decoded `\X0D\` really is a
/// carriage return, and printing it would break the layout.
fn for_display(value: &str) -> String {
    if !value.contains(['\r', '\n', '\t']) {
        return value.to_string();
    }
    value
        .replace('\r', "\\r")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
