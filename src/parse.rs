//! Reading ER7 text into the value tree.
//!
//! Parsing is deliberately forgiving below the header: unknown segments,
//! ragged field counts, and stray empty positions are all data, not errors.
//! The only thing that can fail is the header itself, because without its
//! delimiters there is no way to read anything that follows.

use crate::message::is_header_name;
use crate::{Component, Error, Field, Message, Repetition, Segment, Separators, Subcomponent};

/// The four segments that wrap a batch file rather than belong to any one
/// message: file and batch headers and trailers.
const BATCH_ENVELOPE: [&str; 4] = ["FHS", "BHS", "BTS", "FTS"];

/// Parse one ER7 message, taking its delimiters from its own header.
///
/// The first segment must be `MSH`, or the `FHS`/`BHS` header of a batch;
/// everything after it parses without ever failing.
///
/// ```
/// # fn main() -> Result<(), er7::Error> {
/// let message = er7::parse("MSH|^~\\&|LAB|ACME\rPID|1||9^^^ACME^MR")?;
/// assert_eq!(message.segments.len(), 2);
/// assert_eq!(message.query("PID-3.5")?.as_deref(), Some("MR"));
/// # Ok(())
/// # }
/// ```
pub fn parse(text: &str) -> Result<Message, Error> {
    let text = strip_bom(text);
    let lines = segment_lines(text);
    let (_, first) = *lines.first().ok_or(Error::Empty)?;
    let name = segment_name(first);
    if !is_header_name(name) {
        return Err(Error::MissingHeader(name.to_string()));
    }
    let separators = Separators::from_header(first)?;
    Ok(Message {
        separators,
        segments: lines
            .iter()
            .map(|(_, line)| parse_segment(line, &separators))
            .collect(),
    })
}

/// Parse ER7 text with delimiters supplied by the caller, for a fragment
/// that has no header of its own — a single segment pulled from a log, or
/// the body of a message whose `MSH` was read separately.
///
/// This cannot fail: whatever the text is, it becomes some tree.
///
/// ```
/// # use er7::Separators;
/// let separators = Separators::default();
/// let fragment = er7::parse_with("PID|1||9|4|SMITH^JOHN", separators);
/// assert_eq!(fragment.query("PID-5.1").unwrap().as_deref(), Some("SMITH"));
/// ```
pub fn parse_with(text: &str, separators: Separators) -> Message {
    Message {
        separators,
        segments: segment_lines(strip_bom(text))
            .iter()
            .map(|(_, line)| parse_segment(line, &separators))
            .collect(),
    }
}

/// Split input holding several messages, or a whole batch file, into the
/// individual messages — one per `MSH` segment.
///
/// The returned slices borrow from `text` and keep its original segment
/// terminators, so each one can be handed straight to [`parse`]. Batch
/// envelope segments (`FHS`, `BHS`, `BTS`, `FTS`) are left out: they
/// describe the file, not a message.
///
/// ```
/// let batch = "FHS|^~\\&|SENDER\rMSH|^~\\&|A\rMSA|AA|1\rMSH|^~\\&|B\rMSA|AA|2\rFTS|2";
/// let messages = er7::split_messages(batch);
/// assert_eq!(messages, vec!["MSH|^~\\&|A\rMSA|AA|1", "MSH|^~\\&|B\rMSA|AA|2"]);
/// ```
pub fn split_messages(text: &str) -> Vec<&str> {
    let text = strip_bom(text);
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut current: Option<(usize, usize)> = None;
    for (start, line) in segment_lines(text) {
        let end = start + line.len();
        // An envelope segment ends whatever came before it and starts
        // nothing, so the next MSH opens a fresh message.
        if is_batch_envelope(line) {
            spans.extend(current.take());
        } else if segment_name(line) == "MSH" || current.is_none() {
            // Start a message at each MSH, and at the first segment of the
            // input even when it is not an MSH — `parse` rejects that one
            // on its own, which is a better report than silently dropping
            // it here.
            spans.extend(current.replace((start, end)));
        } else if let Some(span) = current.as_mut() {
            span.1 = end;
        }
    }
    spans.extend(current);
    spans
        .into_iter()
        .map(|(start, end)| &text[start..end])
        .collect()
}

/// True for a batch envelope segment.
fn is_batch_envelope(line: &str) -> bool {
    BATCH_ENVELOPE.contains(&segment_name(line))
}

/// Remove a leading byte-order mark, which text editors add to files and
/// which is not part of the message.
fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{feff}').unwrap_or(text)
}

/// The segment name of a line, read without knowing the delimiters yet: the
/// leading run of letters and digits.
///
/// This is exact rather than a guess, because a field separator is never
/// alphanumeric — so the run ends precisely where the name does. It also
/// keeps a local segment such as `BTSX` from being read as the batch
/// trailer `BTS`.
fn segment_name(line: &str) -> &str {
    let end = line
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(line.len());
    &line[..end]
}

/// Every non-blank line of `text`, as (byte offset, line).
///
/// Lines end at `\r`, `\n`, or `\r\n`, which covers messages taken off the
/// wire and messages saved to a file. Blank lines are dropped; nothing else
/// is trimmed, so a value that really did have a trailing space keeps it.
fn segment_lines(text: &str) -> Vec<(usize, &str)> {
    let mut lines = Vec::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    // `\r` and `\n` cannot occur inside a multi-byte UTF-8 sequence, so
    // scanning bytes here never splits a character.
    for end in 0..=bytes.len() {
        if end < bytes.len() && bytes[end] != b'\r' && bytes[end] != b'\n' {
            continue;
        }
        let line = &text[start..end];
        if !line.trim().is_empty() {
            lines.push((start, line));
        }
        start = end + 1;
    }
    lines
}

fn parse_segment(line: &str, separators: &Separators) -> Segment {
    let mut tokens = line.split(separators.field);
    let name = tokens.next().unwrap_or("").to_string();
    let mut fields = Vec::new();
    if is_header_name(&name) {
        // Field 1 of a header is the field separator itself, and field 2 is
        // the encoding characters. Splitting or unescaping them would be
        // circular, so they are kept whole.
        fields.push(literal_field(separators.field.to_string()));
        if let Some(encoding) = tokens.next() {
            fields.push(literal_field(encoding.to_string()));
        }
    }
    fields.extend(tokens.map(|raw| parse_field(raw, separators)));
    Segment { name, fields }
}

/// A field holding one exact string, used for the delimiter fields of a
/// header segment.
fn literal_field(raw: String) -> Field {
    Field {
        repetitions: vec![Repetition {
            components: vec![Component {
                subcomponents: vec![Subcomponent::new(raw)],
            }],
        }],
    }
}

fn parse_field(raw: &str, separators: &Separators) -> Field {
    // A field the sender left out has no repetitions at all, which is what
    // distinguishes it from a repetition that is present but blank.
    if raw.is_empty() {
        return Field::default();
    }
    Field {
        repetitions: raw
            .split(separators.repetition)
            .map(|repetition| parse_repetition(repetition, separators))
            .collect(),
    }
}

fn parse_repetition(raw: &str, separators: &Separators) -> Repetition {
    Repetition {
        components: raw
            .split(separators.component)
            .map(|component| parse_component(component, separators))
            .collect(),
    }
}

fn parse_component(raw: &str, separators: &Separators) -> Component {
    Component {
        subcomponents: raw
            .split(separators.subcomponent)
            .map(Subcomponent::new)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_hierarchy() {
        let message = parse("MSH|^~\\&|LAB\rPID|1||A^B&C~D").unwrap();
        assert_eq!(message.separators, Separators::default());
        let pid = message.segment("PID").unwrap();
        assert_eq!(pid.fields.len(), 3);
        let field = pid.field(3).unwrap();
        assert_eq!(field.repetitions.len(), 2);
        assert_eq!(field.repetition(1).unwrap().components.len(), 2);
        assert_eq!(
            field
                .repetition(1)
                .unwrap()
                .component(2)
                .unwrap()
                .subcomponents
                .len(),
            2
        );
        assert_eq!(
            field
                .repetition(2)
                .unwrap()
                .component(1)
                .unwrap()
                .subcomponent(1)
                .unwrap()
                .raw,
            "D"
        );
    }

    #[test]
    fn numbers_header_fields_the_way_hl7_does() {
        let message = parse("MSH|^~\\&|LAB|ACME").unwrap();
        let msh = message.header().unwrap();
        assert_eq!(msh.field(1).unwrap().to_er7(&message.separators), "|");
        assert_eq!(msh.field(2).unwrap().to_er7(&message.separators), "^~\\&");
        assert_eq!(msh.field(3).unwrap().to_er7(&message.separators), "LAB");
        assert_eq!(msh.field(4).unwrap().to_er7(&message.separators), "ACME");
    }

    #[test]
    fn accepts_every_terminator_and_drops_blank_lines() {
        for text in [
            "MSH|^~\\&|LAB\rPID|1",
            "MSH|^~\\&|LAB\nPID|1",
            "MSH|^~\\&|LAB\r\nPID|1",
            "MSH|^~\\&|LAB\r\n\r\nPID|1\r\n",
            "\u{feff}MSH|^~\\&|LAB\rPID|1",
        ] {
            let message = parse(text).unwrap();
            assert_eq!(message.segments.len(), 2, "for {text:?}");
            assert_eq!(message.segments[1].name, "PID");
        }
    }

    #[test]
    fn rejects_input_without_a_header() {
        assert!(matches!(parse(""), Err(Error::Empty)));
        assert!(matches!(parse("  \r\n  "), Err(Error::Empty)));
        match parse("PID|1") {
            Err(Error::MissingHeader(name)) => assert_eq!(name, "PID"),
            other => panic!("expected a missing-header error, got {other:?}"),
        }
        assert!(matches!(parse("MSH"), Err(Error::BadHeader(_))));
    }

    #[test]
    fn parses_a_fragment_with_given_delimiters() {
        let fragment = parse_with("PID|1||A^B\rNTE|1||note", Separators::default());
        assert_eq!(fragment.segments.len(), 2);
        assert_eq!(fragment.query("PID-3.2").unwrap().as_deref(), Some("B"));
    }

    #[test]
    fn splits_a_batch_file() {
        let batch = "FHS|^~\\&|F\rBHS|^~\\&|B\r\
                     MSH|^~\\&|A\rMSA|AA|1\r\
                     MSH|^~\\&|B\rMSA|AA|2\r\
                     BTS|2\rFTS|1";
        assert_eq!(
            split_messages(batch),
            vec!["MSH|^~\\&|A\rMSA|AA|1", "MSH|^~\\&|B\rMSA|AA|2"]
        );
    }

    #[test]
    fn splits_plain_concatenated_messages() {
        let text = "MSH|^~\\&|A\rMSH|^~\\&|B\n";
        assert_eq!(split_messages(text), vec!["MSH|^~\\&|A", "MSH|^~\\&|B"]);
        assert_eq!(split_messages(""), Vec::<&str>::new());
    }

    #[test]
    fn keeps_a_headerless_first_message_for_parse_to_reject() {
        let text = "PID|1\rMSH|^~\\&|A";
        assert_eq!(split_messages(text), vec!["PID|1", "MSH|^~\\&|A"]);
        assert!(parse(split_messages(text)[0]).is_err());
    }

    #[test]
    fn does_not_mistake_a_local_segment_for_an_envelope() {
        let text = "MSH|^~\\&|A\rBTSX|1";
        assert_eq!(split_messages(text), vec![text]);
    }
}
