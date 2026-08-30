---
name: er7-skill
description: Parse, query, edit, write, and redact HL7® v2 messages in the ER7 pipe-hat encoding using the er7-rust family of Rust crates (er7, serde-er7, er7-redact). Use when a task involves reading or building an HL7 v2 message in Rust, an ER7 or MLLP-stripped batch file, an HL7 path such as PID-5.1 or OBX[2]-5, giving a message tree Serde support, or removing patient detail from a message before it is logged, shared, or committed.
---

# er7-skill — work with ER7-encoded HL7® v2 messages in Rust

HL7® v2 messages travel as **ER7**: pipe-and-hat delimited text, six levels
deep (message, segment, field, repetition, component, subcomponent). This
skill is how to read, build, edit, and redact that text correctly with the
`er7-rust` crate family, rather than hand-rolling a `split('|')` that
quietly breaks on the first escaped delimiter or repeated field.

This skill is for **using** ER7 and this crate family, in any project.
It teaches the domain — what an ER7 message actually looks like, the
terms an HL7 message uses for its own parts, and where the sharp edges
are — with worked examples. If the task is instead *changing* the
`er7-rust` repository itself (its code, spec, or release process), that is
a different skill: `er7-rust-maintainer-skill`.

## Concepts and terminology

An ER7 message is one **message**, made of **segments** (one per line,
each starting with a three-letter name like `MSH` or `PID`), each holding
**fields** separated by `|`. A field can repeat (`~`), and each repetition
can hold **components** (`^`) and, below that, **subcomponents** (`&`). A
value lives only at the bottom of that tree, in a subcomponent; everything
above it is structure.

```
PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M
```

This is one segment, `PID`. Field 3 is `12345^^^ACME^MR` — one field, four
components (`^`-separated: the ID, then three left blank, then the
assigning authority). Field 5 is `SMITH^JOHN^Q` — the patient's name, as
components family/given/middle. Nothing here repeats (`~`), so there is
only one occurrence of each field.

A few terms carry real weight once you start editing a message rather
than just reading it:

| Term | Means |
| ---- | ----- |
| **Delimiters** | Which characters mean field/repetition/component/subcomponent/escape. Declared by the message itself, in `MSH-1` and `MSH-2` — never assume `\|^~\&`, even though it is the HL7®-recommended default. |
| **Path** | The address of one value, e.g. `PID-5.1` (segment `PID`, field 5, component 1) or `OBX[2]-5` (the *second* `OBX` segment). |
| **Absent vs. empty vs. null** | Three different facts a field can carry — see the rules below. Getting this wrong is the single most common way integration code corrupts a clinical record. |
| **Escape sequence** | A `\F\`-style token inside a value that stands for a delimiter character, so the value can contain one without breaking the structure. |
| **Batch file** | Several messages concatenated, optionally wrapped in `FHS`/`BHS` (file/batch header) and `BTS`/`FTS` (trailer) segments that describe the *file*, not any message in it. |
| **Redaction** | Removing patient detail from a message *without changing its shape* — same segments, same fields, same delimiters, so downstream tooling that expects that shape still works. |

## The three crates, and which one you want

| Crate | Adds | Reach for it when |
| ----- | ---- | ------------------ |
| [`er7`](https://docs.rs/er7/) | Parse, query, edit, and write ER7 text. Zero dependencies. | You need to read a value out of a message, build one, or write one back out — this is almost always the crate you want. |
| [`er7-redact`](https://docs.rs/er7-redact/) | Removes patient detail from a message without moving its shape, by a policy of HL7 paths and actions. | The message (or anything derived from it — a log line, a test fixture, a bug report) is about to leave the system it arrived in. |
| [`serde-er7`](https://docs.rs/serde-er7/) | `Serialize`/`Deserialize` for every `er7` type. | A message tree needs to cross a Serde boundary — JSON over HTTP, a config file, anything besides ER7 text itself. |

```toml
[dependencies]
er7 = "0.2"
# er7-redact = "0.3"   # only if you are redacting
# serde-er7 = "0.2"    # only if you need Serde
```

None of these three know what a field *means* — no dictionary of segment
or data-type definitions, no message-structure grammars, no validation.
That is deliberate: the encoding is stable across every HL7 v2 release,
2.1 through 2.9, and a dictionary is version-specific. If a task needs
"what should `OBX-5` contain for this message type," that is a layer
above this skill, not a gap to patch here.

## The five rules that keep a message intact

Get these wrong and the code will compile, run, and quietly corrupt a
clinical record. Read them before writing anything that touches a
message's contents.

1. **The round trip is byte-for-byte.** `er7::parse(text)?.to_er7() ==
   text` for any canonical input. Never trim, normalize, case-fold, or
   "clean up" a value on the way in or out — a value that looks like
   whitespace might be data this crate cannot tell apart from noise.
2. **Absent, empty, and explicit null are three different things**, not
   one. A field that was never sent (`field.repetitions.is_empty()`), a
   field sent blank (`||`, present with nothing in it), and a field the
   sender explicitly nulled (`""`, meaning *clear this value*) must never
   collapse into each other. Treating a null as empty leaves a withdrawn
   allergy on the record; treating an empty as absent erases a value that
   was actually sent. Check `is_null()` before writing to a record —
   never infer it from the decoded value, since a null's decoded value is
   also `""`.
3. **Edit through `Subcomponent::set`, not by assigning `raw` directly.**
   `set` encodes delimiters on the way in; a raw `&` or `^` written
   straight into `raw` silently splits the value the next time the
   message is parsed, shifting every value after it.
4. **Nothing below the header can fail.** Unknown segments, ragged field
   counts, an odd number of components — all data, not errors. The only
   things `parse` rejects are a missing or unusable header. If you find
   yourself wanting to add validation on top of this crate, that
   validation is a separate, deliberate layer — not something to smuggle
   into a parse step.
5. **A message is a clinical record about a real patient.** Never put a
   real message in a test fixture, a log statement, a bug report, or a
   commit — reproduce the shape with synthetic data instead
   (`SMITH^JOHN`, `MSG00042`, `444333222`). If a message must leave the
   system it arrived in for any reason, redact it first with
   `er7-redact` — and even then, remember pseudonyms are stable
   identifiers for linking records, not a security primitive.

## Quick recipes

**Parse and query:**

```rust
let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
            PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";

let message = er7::parse(text)?;
let last_name = message.query("PID-5.1")?;          // first match, decoded
let all_obx_values = message.query_all("OBX-5")?;    // every match
```

Already holding one `Segment` while iterating (say, every `OBX` in a
message)? `Segment::first_value` reads one field's first repetition and
subcomponent directly, treating an empty result as absent — no need to
re-query the whole message by path:

```rust
for obx in message.segments_named("OBX") {
    let observation_id = obx.first_value(2, 1, &message.separators); // Option<String>
}
```

**Edit a value and write it back:**

```rust
let separators = message.separators;
message
    .segment_at_mut("PID", 1).unwrap()
    .field_mut(5).unwrap()
    .repetition_mut(1).unwrap()
    .component_mut(1).unwrap()
    .subcomponent_mut(1).unwrap()
    .set("O'BRIEN & SONS", &separators);

let out = message.to_er7();
```

**Split a batch file** (an `FHS`/`BHS`/`BTS`/`FTS`-wrapped file, or several
messages concatenated):

```rust
for source in er7::split_messages(batch_text) {
    match er7::parse(source) {
        Ok(message) => { /* one message at a time */ }
        Err(e) => eprintln!("skipping malformed message: {e}"),
    }
}
```

Too large to load as one `String`? `er7::read_messages(reader)` is the
streaming counterpart, for any `BufRead`: same splitting rules, one owned
`String` per message, nothing held in memory but the reader's own buffer.

```rust
let mut messages = er7::read_messages(std::io::BufReader::new(file));
while let Some(source) = messages.next().transpose()? {
    let message = er7::parse(&source)?;
    // ...
}
```

**Redact before sharing:**

```sh
er7-redact message.er7            # a redacted copy, same shape
er7-redact --report message.er7   # what changed: paths and actions, no values
er7-redact --uncovered message.er7 # positions no rule names, and not the message
```

By default `Policy::search_known_values` is `true`: a value found at a
named position (say, the patient ID in `PID-3`) is also redacted wherever
else it repeats in the message, case-insensitively and as a whole word —
not just at the position a rule names. Turn it off with
`Policy::search_known_values(false)`, or `known-values off` in a policy
file, for a policy that should only ever redact by position.

`Redactor::uncovered(&message)` returns the paths no rule names at all
(text-carrying leaves only) — useful to check a policy's coverage
independent of whether its posture is accepting or rejecting.

## Where to look next

Each crate ships its own tutorial-style docs and a spec that is the
single source of truth for its behaviour — read the crate's own
documentation before guessing at an edge case:

- <https://docs.rs/er7/> — full API reference, generated from the source
- <https://er7-rust.github.io/> — worked examples for the format, paths,
  escapes, the CLI, and the ecosystem as a whole
- <https://github.com/er7-rust/er7-rust> — source, specs (`spec/` in each
  crate directory), and runnable `examples/`

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
