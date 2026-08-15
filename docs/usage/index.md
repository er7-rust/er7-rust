[er7](../../index.md) → [docs](../) → usage

# Tutorial

A walk-through of the `er7` crate, from a string to values, edits, and back.
Every snippet is a complete, runnable body; the same code as a program is in
[`examples/`](../../examples/README.md).

Reference material lives elsewhere: [paths](../paths/index.md),
[escape sequences](../escapes/index.md), [the API surface](../api/index.md),
and [the specification](../../spec/index.md) for exact behaviour.

## Contents

1. [Parse a message](#1-parse-a-message)
2. [Read a value by path](#2-read-a-value-by-path)
3. [Repeated segments and repeated fields](#3-repeated-segments-and-repeated-fields)
4. [Walk the tree instead](#4-walk-the-tree-instead)
5. [Absent, empty, and null](#5-absent-empty-and-null)
6. [Escape sequences](#6-escape-sequences)
7. [Edit a value](#7-edit-a-value)
8. [Write the message back](#8-write-the-message-back)
9. [Batch files](#9-batch-files)
10. [Unusual delimiters](#10-unusual-delimiters)
11. [Errors](#11-errors)

## 1. Parse a message

`er7::parse` takes ER7 text and returns a `Message`. The message's own
header supplies the delimiters, so nothing is assumed.

```rust
let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01|MSG00042|P|2.5\r\
            PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E||19620320|F\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";

let message = er7::parse(text)?;

assert_eq!(message.segments.len(), 3);
assert_eq!(message.separators, er7::Separators::default());
```

Segment terminators may be `\r`, `\n`, or `\r\n` — all three are accepted,
because messages taken off the wire and messages saved to a file differ
here.

The five routing fields every integration needs are one call away:

```rust
assert_eq!(message.message_code().as_deref(), Some("ORU"));
assert_eq!(message.trigger_event().as_deref(), Some("R01"));
assert_eq!(message.control_id().as_deref(), Some("MSG00042"));
assert_eq!(message.version().as_deref(), Some("2.5"));
```

## 2. Read a value by path

An HL7 path names a position: segment, field, component, subcomponent.

```rust
assert_eq!(message.query("PID-5")?.as_deref(), Some("EVERYWOMAN^EVE^E"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.query("PID-3.5")?.as_deref(), Some("MR"));
assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));
```

Notice `PID-5` returns the whole field with its `^` intact. A path that
stops above the leaf returns that subtree as written; only the leaf text is
decoded.

A position the message does not carry is `None`, not an error:

```rust
assert_eq!(message.query("PID-99")?, None);
assert_eq!(message.query("ZZZ-1")?, None);
```

The full notation is in [docs/paths](../paths/index.md).

## 3. Repeated segments and repeated fields

`query` gives the first match; `query_all` gives every one, in message
order.

```rust
let text = "MSH|^~\\&|LAB\r\
            PID|1||9|4|SMITH^JOHN|||||||555-1111~555-2222\r\
            OBX|1|NM|2093-3^Cholesterol^LN||187\r\
            OBX|2|NM|2571-8^Triglycerides^LN||102";
let message = er7::parse(text)?;

// Three OBX segments in, two values out — only two carried a fifth field.
assert_eq!(message.query_all("OBX-5")?, vec!["187", "102"]);

// Pin down which one with an occurrence index.
assert_eq!(message.query_all("OBX[2]-3.2")?, vec!["Triglycerides"]);

// A field that stops at the field level keeps its repetition separator...
assert_eq!(message.query("PID-11")?.as_deref(), Some("555-1111~555-2222"));

// ...but going deeper splits it.
assert_eq!(message.query_all("PID-11[2].1")?, vec!["555-2222"]);
```

## 4. Walk the tree instead

Paths are convenient; the tree is precise. Use it when you need the node
itself — to edit it, or to ask whether it is null.

```rust
let pid = message.segment("PID").expect("the message has a PID");
let name = pid.field(5).expect("PID-5 was sent");
let family = name.component(1).expect("PID-5.1 was sent");
let text = family.subcomponent(1).expect("PID-5.1.1 was sent");

assert_eq!(text.value(&message.separators), "SMITH");
assert_eq!(text.raw, "SMITH");
```

Every index is 1-based, matching HL7's own numbering, and every accessor
returns `Option` rather than panicking. `Segment::component` is a shortcut
for the common two-step:

```rust
assert_eq!(
    pid.component(5, 2).unwrap().to_text(&message.separators),
    "JOHN"
);
```

## 5. Absent, empty, and null

ER7 says three different things, and conflating them corrupts records.

```rust
let message = er7::parse("MSH|^~\\&|LAB\rPID|1||\"\"|X")?;
let pid = message.segment("PID").unwrap();

// Absent: never sent.
assert!(pid.field(9).is_none());

// Empty: sent as `||`, with no value.
assert!(pid.field(2).unwrap().is_empty());
assert!(!pid.field(2).unwrap().is_null());

// Null: sent as `""`, meaning "clear the stored value".
assert!(pid.field(3).unwrap().is_null());
assert!(!pid.field(3).unwrap().is_empty());
```

`is_empty` and `is_null` are never both true. A query decodes a null to the
empty string, since that is the value being conveyed — so when the
difference matters, ask the node:

```rust
assert_eq!(message.query("PID-3")?.as_deref(), Some(""));  // decoded
assert!(pid.field(3).unwrap().is_null());                  // the real answer
```

## 6. Escape sequences

A value that needs to contain a delimiter escapes it. Reading a value
decodes the sequences that stand for characters and leaves the rest alone.

```rust
let message = er7::parse(r"MSH|^~\&|LAB|Smith \T\ Jones|line\.br\next")?;

// `\T\` is the subcomponent separator as data, so it decodes.
assert_eq!(message.query("MSH-4")?.as_deref(), Some("Smith & Jones"));

// `\.br\` is a display command, so it is preserved as written.
assert_eq!(message.query("MSH-5")?.as_deref(), Some(r"line\.br\next"));
```

To see exactly what the sender wrote, ask for it raw:

```rust
let path = "MSH-4".parse::<er7::Path>()?;
assert_eq!(message.query_path_raw(&path), vec![r"Smith \T\ Jones"]);
```

Details, including the full sequence table, are in
[docs/escapes](../escapes/index.md).

## 7. Edit a value

Write through `set`, which encodes delimiters for you. Assigning `raw`
directly is possible but makes the escaping your problem.

```rust
let mut message = er7::parse("MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN")?;
let separators = message.separators;

message
    .segment_at_mut("PID", 1).unwrap()
    .field_mut(5).unwrap()
    .repetition_mut(1).unwrap()
    .component_mut(1).unwrap()
    .subcomponent_mut(1).unwrap()
    .set("O'BRIEN & SONS", &separators);

// The `&` was encoded, so the structure still holds.
assert!(message.to_er7().contains(r"O'BRIEN \T\ SONS^JOHN"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("O'BRIEN & SONS"));
```

Structural edits — a new repetition, a longer segment — go through the
public `Vec` fields:

```rust
use er7::{Component, Repetition, Subcomponent};

let phone = message.segment_at_mut("PID", 1).unwrap().field_mut(5).unwrap();
phone.repetitions.push(Repetition {
    components: vec![Component {
        subcomponents: vec![Subcomponent::new("ALIAS")],
    }],
});
assert!(message.to_er7().ends_with("~ALIAS"));
```

## 8. Write the message back

```rust
let text = "MSH|^~\\&|LAB\rPID|1||9|4|SMITH^JOHN";
assert_eq!(er7::parse(text)?.to_er7(), text);
```

That is the crate's central guarantee: canonical input comes back byte for
byte. "Canonical" means no blank lines and one consistent terminator —
parsing normalizes exactly those two things and nothing else.

To choose the terminator, or to end the last segment too:

```rust
use er7::{RenderOptions, Terminator};

let message = er7::parse("MSH|^~\\&|LAB\rPID|1")?;
let options = RenderOptions {
    terminator: Terminator::Lf,
    trailing_terminator: true,
};
assert_eq!(message.to_er7_with(options), "MSH|^~\\&|LAB\nPID|1\n");
```

Use `to_er7` for anything that will be sent, stored, or parsed again. Use
`to_text` — available at every level — for display, logging, and database
writes:

```rust
let message = er7::parse(r"MSH|^~\&|LAB|a\T\b")?;
let field = message.segment("MSH").unwrap().field(4).unwrap();

assert_eq!(field.to_er7(&message.separators), r"a\T\b");   // as sent
assert_eq!(field.to_text(&message.separators), "a&b");     // decoded
```

`to_text` output is not re-parseable: that decoded `&` would be read as a
subcomponent separator next time round.

## 9. Batch files

`split_messages` cuts a batch or a run of concatenated messages into
individual ones, dropping the `FHS`/`BHS`/`BTS`/`FTS` envelope. The results
borrow from the input, so there is no copy.

```rust
let batch = "FHS|^~\\&|SENDER\r\
             MSH|^~\\&|A||||1||ACK|B1|P|2.5\rMSA|AA|1\r\
             MSH|^~\\&|B||||2||ACK|B2|P|2.5\rMSA|AA|2\r\
             FTS|2";

for (index, source) in er7::split_messages(batch).iter().enumerate() {
    match er7::parse(source) {
        Ok(message) => println!("{}: {:?}", index + 1, message.control_id()),
        Err(e) => eprintln!("{}: skipping malformed message: {e}", index + 1),
    }
}
```

## 10. Unusual delimiters

Everything above works on a message that chose different delimiters,
because nothing is hardcoded.

```rust
let text = "MSH#*!?@#LAB#*ACME#SMITH*JOHN@JR";
let message = er7::parse(text)?;

assert_eq!(message.separators.field, '#');
assert_eq!(message.separators.subcomponent, '@');
assert_eq!(message.query("MSH-5.2.2")?.as_deref(), Some("JR"));
assert_eq!(message.to_er7(), text);
```

If you have a fragment with no header of its own — a segment lifted from a
log — supply the delimiters yourself:

```rust
let fragment = er7::parse_with("OBX|1|NM|2093-3^Cholesterol^LN||187",
                               er7::Separators::default());
assert_eq!(fragment.query("OBX-5")?.as_deref(), Some("187"));
```

`parse_with` cannot fail, so it returns `Message` rather than `Result`.

## 11. Errors

Only two situations produce an error: a message with no usable header, and
a path that is not a path.

```rust
use er7::Error;

assert!(matches!(er7::parse(""), Err(Error::Empty)));
assert!(matches!(er7::parse("PID|1"), Err(Error::MissingHeader(_))));
assert!(matches!(er7::parse("MSH"), Err(Error::BadHeader(_))));
assert!(matches!("PID-0".parse::<er7::Path>(), Err(Error::BadPath(_))));
```

Everything else is recovered rather than refused. Unknown segments, local
`Z` segments, ragged field counts, and undecodable escape sequences all
parse:

```rust
let odd = "MSH|^~\\&|LAB\rZPD|1|LOCAL^EXTENSION\rNTE|1||\\Qunknown\\Q";
let message = er7::parse(odd)?;
assert_eq!(message.query("ZPD-2.2")?.as_deref(), Some("EXTENSION"));
```

A receiver that rejects a message it could have read drops clinical
information, so the crate reads it as written.

## Next

- [Paths reference](../paths/index.md)
- [Escape sequences reference](../escapes/index.md)
- [API surface](../api/index.md)
- [FAQ](../faq/index.md)
- [Runnable examples](../../examples/README.md)
