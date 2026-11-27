[er7-rust](../../index.md) → [docs](../index.md) → glossary

# Glossary

HL7® v2 terms as they appear in these crates, with the Rust type or call
that carries each one. The crates' specifications hold the normative
definitions. This page is the map between the vocabulary and the code.

| Term | Meaning | In Rust |
| ---- | ------- | ------- |
| **ER7** | The pipe-hat text encoding of HL7 v2 (`\|^~\&`), as opposed to its XML encoding | what `er7::parse` reads and `Message::to_er7` writes |
| **Message** | One header segment and the segments after it | `er7::Message` |
| **Segment** | One line, named by its first three characters (`PID`, `OBX`) | `er7::Segment`; `message.segment("PID")`, `segments_named("OBX")` |
| **Header** | The first segment, which declares the delimiters: `MSH`, or `FHS`/`BHS` in a batch | `message.header()`, `Segment::is_header()` |
| **Field** | A position in a segment, separated by `\|` | `er7::Field`; `segment.field(n)` |
| **Repetition** | One occurrence of a repeating field, separated by `~` | `er7::Repetition`; `field.repetition(n)`, `field.repetitions` |
| **Component** | A part of a repetition, separated by `^` | `er7::Component`; `repetition.component(n)` |
| **Subcomponent** | A part of a component, separated by `&`; the leaf that holds text | `er7::Subcomponent`; `raw`, `value()`, `set()` |
| **Delimiters**, **encoding characters** | The field separator (`MSH-1`) and the characters in `MSH-2` | `er7::Separators`; `message.separators` |
| **Truncation character** | The optional fifth `MSH-2` character (v2.7 and later) | `Separators::truncation: Option<char>` |
| **Escape sequence** | `\F\`, `\S\`, `\T\`, `\R\`, `\E\`, `\Xhh\`, and others, standing for a delimiter or other text | `er7::escape::{escape, unescape, escapes}` |
| **Raw** | Text as sent, escapes included | `Subcomponent::raw`; the `er7 --raw` flag |
| **Absent** | A position the message does not carry | `None` from an accessor or `query` |
| **Empty** | A position present with no text: `\|\|` | `is_empty()` |
| **Explicit null** | `""`: the sender is clearing the value | `is_null()`; the constant `er7::message::NULL` |
| **Path** | A position written as text: `PID-5.1`, `OBX[2]-5`, `PID-13[2]` | `er7::Path`; `message.query(path)`, `query_all(path)` |
| **Occurrence** | Which segment of a repeated name, 1-based: the `[2]` in `OBX[2]-5` | `Path::segment_occurrence`; `message.segment_at(name, n)` |
| **Batch** | Several messages in one file, optionally inside `FHS`/`BHS` envelopes | `er7::split_messages`, `er7::read_messages` |
| **Terminator** | What ends a segment: `\r` on the wire, often `\n` in files | `er7::Terminator`; `RenderOptions` |
| **MSH-9**, **MSH-10**, **MSH-12** | Message type, control ID, and version: the routing fields | `message_code()`, `trigger_event()`, `control_id()`, `version()` |
| **Policy** | What to redact, rule by rule, plus a default for everything else | `er7_redact::Policy` |
| **Rule** | One path and one action, such as `PID-5 replace REDACTED` | `er7_redact::Rule` |
| **Action** | What a rule does: `keep`, `clear`, `null`, `replace`, `mask`, `first`, `last`, `pseudonym` | `er7_redact::Action` |
| **Posture** | Whether a policy accepts or rejects a value no rule names | `er7_redact::Posture` |
| **Known-value sweep** | Redacting a value found at a named position wherever else it appears | `Policy::search_known_values` |
| **Pseudonym** | A stable stand-in for an identifier, keyed by a number | `er7_redact::pseudonym`; `Redactor::with_key` |
| **Report** | What a redaction changed: paths and actions, never values | `er7_redact::Report`, `Change` |
| **Wire shape** | How a tree serializes: a message is an object, a field an array of repetitions, a leaf a string | `serde_er7::Message` and the other wrappers |
| **Strict** | Deserialization that rejects an unknown key instead of ignoring it | `serde_er7::Strict<T>` |

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
