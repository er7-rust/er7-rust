[← docs](../../index.md#documentation)

# Tutorial

A walk-through from a parsed message to JSON and back. Every code block
here is drawn from a runnable program in [`examples/`](../../examples/README.md)
— run the named example to see it with real output.

## §1 Parse, then serialize

Start the way you would with plain `er7`: parse ER7 text into a message.
The only difference is which type you parse into.

```rust
use serde_er7::Message;

let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5\r\
            PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F";

let message = Message::parse(text)?;
```

`Message::parse` is a thin wrapper over `er7::parse` — it returns
`Result<Message, er7::Error>`, the same error type, so anything you already
know about handling `er7::Error` (spec §11 in the `er7` repository) applies
unchanged.

Now hand it to any Serde format. This crate never mentions JSON itself —
`serde_json` is just the format used here because it is easy to read on a
page:

```rust
let json = serde_json::to_string_pretty(&message)?;
println!("{json}");
```

Run this yourself with:

```sh
cargo run --example round_trip_via_json
```

## §2 Back the other way: JSON to ER7

The direction most tutorials skip: you have JSON — from a web form, a
document store, a test fixture — and you need ER7 text for a legacy
receiver.

```rust
let json = r#"{
  "separators": {"field":"|","component":"^","repetition":"~","escape":"\\","subcomponent":"&","truncation":null},
  "segments": [
    {"name": "MSH", "fields": [[["|"]],[["^~\\&"]],[["LAB"]],[["ACME"]],[],[],[["20260815090000"]],[],[[["ADT"],["A08"],["ADT_A01"]]],[["MSG00001"]],[["P"]],[["2.5"]]]},
    {"name": "PID", "fields": [[["1"]],[],[[["555-44-4444"]]],[],[[["SMITH"],["JOHN"]]]]}
  ]
}"#;

let message: serde_er7::Message = serde_json::from_str(json)?;

// `Deref` reaches straight through to `er7::Message`'s own API — no
// unwrapping needed.
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("SMITH"));
println!("{}", message.to_er7());
```

Run this yourself with:

```sh
cargo run --example build_message_from_json
```

Building this JSON by hand is the fiddly part — see
[§3](#3-the-shape-worked-through-by-hand) for the nesting rule that makes
it mechanical rather than guesswork.

## §3 The shape, worked through by hand

Every level below `Message`/`Segment` is a plain sequence, one array level
per level of the ER7 tree. Reading a value's nesting depth tells you how
many arrays deep it sits:

| ER7 | Meaning | JSON |
|-----|---------|------|
| `SMITH` | one subcomponent | `"SMITH"` |
| `SMITH&JONES` | two subcomponents (one component, `&`-separated) | `["SMITH", "JONES"]` |
| `SMITH^JOHN` | two components (one repetition, `^`-separated) | `[["SMITH"], ["JOHN"]]` |
| `555-1111~555-2222` | two repetitions (`~`-separated) | `[["555-1111"], ["555-2222"]]` |
| `` (absent field, `\|\|`) | no repetitions at all | `[]` |

Read the nesting from the outside in: a **field** is an array of
*repetitions*; each repetition is an array of *components*; each component
is an array of subcomponent strings. `SMITH^JOHN` has one repetition (no
`~`) holding two components, each a single subcomponent — so as a whole
field it is `[ [ ["SMITH"], ["JOHN"] ] ]`: one repetition, containing two
components, each a one-element array. The table above shows the
*repetition* and *component* levels in isolation to keep each row small;
building the full field means composing them, outside in, the same way you
compose the ER7 delimiters (`~` outside, `^` inside) to get there in the
first place. Run `cargo run --example inspect_a_segment_as_json` and read
the pretty-printed output for a real segment if the nesting feels abstract
on the page — seeing an actual `PID` segment laid out is usually faster
than composing it from the table by hand.

The full normative table, including `Message` and `Segment`'s object
shapes, is
[`spec/02-wire-shapes/index.md`](../../spec/02-wire-shapes/index.md); the
[`docs/api/`](../api/index.md) reference lists every type with a link back
to it.

## §4 What does and does not round-trip

```rust
let message = Message::parse(text)?;
let json = serde_json::to_string(&message)?;
let back: Message = serde_json::from_str(&json)?;
assert_eq!(back.to_er7(), text);
```

This works because every subcomponent serializes its *raw* text — escape
sequences intact — never the decoded form `er7::Subcomponent::value`
produces. If you need the decoded, human-readable text instead (for
logging or display, not for sending anywhere), read it from the `er7`
value after deserializing, the same way you would with plain `er7`:

```rust
let separators = back.separators;
let decoded = back.query("OBX-3.2")?; // already decoded — `query` always is
```

See
[`spec/04-round-trip-guarantee/index.md`](../../spec/04-round-trip-guarantee/index.md)
for exactly what this guarantee covers.

## §5 Working with one piece of a message

You do not need a whole `Message` to serialize something. Any wrapper type
serializes on its own:

```rust
use serde_er7::Segment;

let pid = message.segment("PID").unwrap().clone();
let json = serde_json::to_string_pretty(&Segment(pid))?;
```

Run this yourself with:

```sh
cargo run --example inspect_a_segment_as_json
```

## Next

- [`docs/api/`](../api/index.md) — every type, one line each, linking to
  its rustdoc.
- [`docs/faq/`](../faq/index.md) — answers to questions this tutorial
  doesn't cover.
- [`spec/`](../../spec/index.md) — the normative specification, if you are
  changing this crate rather than only using it.
