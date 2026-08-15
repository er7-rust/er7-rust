# er7

Parse, query, edit, and write HL7 v2 messages in the **ER7** pipe-hat
encoding — as a Rust library and a command-line tool, with zero
dependencies.

```
MSH|^~\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5
PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F
```

ER7 is compact, ubiquitous, and unforgiving: a value's meaning comes
entirely from its position, so one misplaced `|` silently shifts everything
after it. This crate makes that structure explicit and keeps it intact.

This README is a tour. [`spec/index.md`](spec/index.md) is the normative
specification of every rule — the single source of truth this crate
implements against. [`spec/er7-format.md`](spec/er7-format.md) describes
the format itself, independent of any implementation.

## Command line

```sh
# Show every value with the HL7 path that names it
er7 samples/oru_r01.er7
```

```
MSH-1       |
MSH-2       ^~\&
MSH-3       LAB
MSH-9.1     ORU
MSH-9.2     R01
MSH-9.3     ORU_R01
PID-3.1     444333222
PID-3.4.1   ACME
PID-3.4.2   1.2.840.114398.1.100
PID-5.1     EVERYWOMAN
PID-5.2     EVE
PID-13[1]   555-555-1111
PID-13[2]   555-555-2222
OBX[1]-3.2  Cholesterol
OBX[1]-5    187
OBX[2]-3.2  Triglycerides
```

Every path in that outline can be pasted straight back in:

```sh
# Pull out specific values
er7 --query PID-5.1 --query OBX-5 samples/oru_r01.er7

# Rewrite as canonical ER7, with line feeds so a terminal can show it
er7 --normalize --terminator lf samples/oru_r01.er7

# Read a batch file, take its second message
cat samples/batch.er7 | er7 --message 2
```

`er7 --help` lists the rest.

## Library

```rust
let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01|MSG00042|P|2.5\r\
            PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";

let message = er7::parse(text)?;

assert_eq!(message.control_id().as_deref(), Some("MSG00042"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));

// What went in comes back out, byte for byte.
assert_eq!(message.to_er7(), text);
```

`query` returns the first match and `query_all` returns every one, so
`OBX-5` across a result with three observations gives three values.

Editing goes through `set`, which encodes delimiters on the way in so the
structure cannot be broken by a value:

```rust
let separators = message.separators;
message
    .segment_at_mut("PID", 1).unwrap()
    .field_mut(5).unwrap()
    .repetition_mut(1).unwrap()
    .component_mut(1).unwrap()
    .subcomponent_mut(1).unwrap()
    .set("O'BRIEN & SONS", &separators);

assert!(message.to_er7().contains(r"O'BRIEN \T\ SONS"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("O'BRIEN & SONS"));
```

The whole escape-sequence vocabulary is available directly when you need
it: `er7::escape::escapes` tokenizes and classifies every sequence, and
`unescape` / `escape` are the two passes built on it.

## What it does

- **Full hierarchy**: message, segment, field, repetition (`~`), component
  (`^`), subcomponent (`&`).
- **Delimiters from the message**: MSH-1 and MSH-2 are read, never assumed,
  including the truncation character HL7 v2.7 added. A message that uses
  `#*!?@` parses as happily as one that uses `|^~\&`.
- **Round trip, byte for byte**: text is stored exactly as it arrived and
  decoded only when you ask for a value, so a message survives a trip
  through this crate unchanged — unusual delimiters, unknown segments,
  empty positions, escape sequences and all.
- **Escape sequences**: the whole vocabulary — `\F\ \S\ \T\ \R\ \E\`,
  `\Xdd..\`, `\H\`, `\N\`, `\Zdd..\`, `\Cxxyy\`, `\Mxxyyzz\`, `\.br\` — is
  tokenized and classified. The ones that stand for characters decode; the
  ones that describe presentation are preserved as written.
- **HL7 paths**: `PID-5.1`, `OBX[2]-5`, `PID-13[2].1`, in either the
  `PID-5.1` or `PID.5.1` spelling.
- **Absent, empty, and null** are three different answers, not one — the
  explicit `""` means *clear this value*, and losing that distinction
  corrupts patient records.
- **Batch files**: `FHS`/`BHS`/`BTS`/`FTS` envelopes are recognized and
  messages come out one at a time.
- **Nothing fails except a missing header**: unknown segments, local `Z`
  segments, ragged field counts, and stray positions are data, not errors.

## What it deliberately does not do

This crate is an encoding, not a dictionary. It does not know which fields
a segment has, what data type each carries, which message structures exist,
or what any code table means — all of that is version-specific and belongs
in a layer above. It performs no validation and no transport (MLLP framing
is a separate concern).

The one exception is a handful of `MSH` accessors — `message_code`,
`trigger_event`, `message_structure`, `control_id`, `version` — because
routing a message requires reading them and those positions have never
moved.

For the v2.5 dictionary layer, see the sibling crate
[hl7-2-5-to-xml-using-rust](https://github.com/joelparkerhenderson/hl7-2-5-to-xml-using-rust),
which converts ER7 to the official v2.xml representation.

## Documentation

- [`spec/index.md`](spec/index.md) — the normative specification (source of
  truth for behavior)
- [`spec/er7-format.md`](spec/er7-format.md) — the ER7 format itself:
  delimiters, escape sequences, batch files, and why it persists
- `cargo doc --no-deps --open` — the library API
- [`AGENTS.md`](AGENTS.md) — conventions and required checks for anyone,
  human or agent, changing this code
- [`samples/`](samples/) — example messages: a lab result, an admission
  update with a `Z` segment, and a batch file

## Development

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lint-clean
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
cargo run -- samples/oru_r01.er7
```

## License

MIT OR Apache-2.0, at your option.
