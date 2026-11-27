[er7-rust](../../index.md) → [docs](../index.md) → HL7® v2 for Rust developers

# HL7® v2 for Rust developers

You know Rust. This page is how an HL7 v2 message maps onto Rust types in
`er7`, and the handful of places where HL7's conventions will surprise you.
For the format itself (delimiters, escapes, batches, the wire), read the
site's [format page](https://er7-rust.github.io/format/).

## The tree is nested `Vec`s

An ER7 message is six levels deep, and `er7` models every level as a plain
struct with `pub` fields:

```text
Message      { separators: Separators, segments: Vec<Segment> }
Segment      { name: String,           fields: Vec<Field> }
Field        { repetitions: Vec<Repetition> }       // split on ~
Repetition   { components: Vec<Component> }         // split on ^
Component    { subcomponents: Vec<Subcomponent> }   // split on &
Subcomponent { raw: String }                        // the text, escapes and all
```

Nothing is cached, so mutating any `Vec` directly is safe. Every node is
`Clone`, so moving a whole field from one message to another is a clone and
an assignment.

## Two numbering systems

HL7 numbers from 1, and so do the accessors: `segment.field(5)`,
`field.repetition(1)`, `repetition.component(2)`. Each returns an
`Option`, and index `0` is `None` rather than a panic. The `Vec`s
underneath number from 0, as Rust's always do. `segment.fields[4]` is
`PID-5`.

One HL7 quirk sits on top: **in `MSH`, field 1 is the field separator
itself.** `MSH-1` is `|`, `MSH-2` is `^~\&`, and `MSH-3` is the first
field after them. `er7` follows the standard's numbering, so `MSH-9` in a
document is `message.query("MSH-9")` in code. If you index
`msh.fields` directly, the header's `fields[0]` holds `MSH-1`.

## Absent, empty, and null are three different things

| On the wire | Meaning | In `er7` |
| ----------- | ------- | -------- |
| the field is not there | the sender said nothing | the accessor or `query` returns `None` |
| `\|\|` | the sender sent nothing | `is_empty()` is true |
| `\|""\|` | **clear the stored value** | `is_null()` is true |

`Subcomponent::value` reads the null as `""`, because that is the value
being conveyed. Ask `is_null()` whenever you write to a database. Treating a
null as empty leaves a withdrawn allergy on a patient's record.

## `raw` and `value`

`Subcomponent::raw` is the text exactly as sent, escape sequences
included. `value(&separators)` decodes it and returns a `Cow<str>`, which
borrows when there is nothing to decode. Keep `raw` when you pass a message
on, and read `value` when you need the meaning. Write with
`set(text, &separators)`, which escapes any delimiter in your text.

Every function that touches text takes `&Separators`. Pass
`&message.separators`: a message declares its own delimiters in
`MSH-1`/`MSH-2`, and `|^~\&` is only the convention.

## Errors happen once, at the header

`er7::parse` fails only when there is no usable header: no `MSH`,
`FHS`, or `BHS`, or unusable delimiters. Below the header nothing fails.
Unknown segments, ragged fields, and escape sequences `er7` cannot decode
are all kept as data. So once you have a `Message`, every read is an
`Option` and every write succeeds.

`message.to_er7()` reproduces canonical input byte for byte, so parsing
and rewriting a message you did not edit changes nothing.

## No dictionary

`er7` does not know that `PID-5` is a patient name, that `OBX-5`'s type
depends on `OBX-2`, or what an `ADT^A08` must contain. That knowledge
differs by version, site, and vendor, and the crate deliberately leaves it
to you. The usual pattern is a typed view of your own that names each
position once:

```rust
use std::error::Error;

/// Your own typed view: er7 has no dictionary, so the meaning of each
/// position lives in your code, in one place.
#[derive(Debug)]
struct Patient {
    id: Option<String>,
    family: Option<String>,
    given: Option<String>,
    birth_date: Option<String>,
}

impl Patient {
    fn from_message(message: &er7::Message) -> Option<Patient> {
        let pid = message.segment("PID")?;
        let separators = &message.separators;
        Some(Patient {
            id: pid.first_value(3, 1, separators),
            family: pid.first_value(5, 1, separators),
            given: pid.first_value(5, 2, separators),
            birth_date: pid.first_value(7, 1, separators),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = std::fs::read_to_string("message.er7")?;
    let mut message = er7::parse(&text)?;
    println!("{:?}", Patient::from_message(&message));

    // Add a third phone number: a repetition is a Vec element like any other.
    let pid = message.segment_at_mut("PID", 1).ok_or("no PID")?;
    let phones = pid.field_mut(13).ok_or("no PID-13")?;
    let mut extra = er7::Repetition::default();
    extra.components.push(er7::Component::default());
    extra.components[0].subcomponents.push(er7::Subcomponent::new("555-0142"));
    phones.repetitions.push(extra);
    println!("{:?}", message.query_all("PID-13.1")?);
    Ok(())
}
```

which prints, for the message in [the 5-minute guide](../getting-started/index.md):

```text
Some(Patient { id: Some("444333222"), family: Some("EVERYWOMAN"), given: Some("EVE"), birth_date: Some("19620320") })
["555-0100", "555-0199", "555-0142"]
```

## Batches and streams

A file can hold many messages, either concatenated or wrapped in
`FHS`/`BHS` batch envelopes. `er7::split_messages(&text)` splits a string
you already hold. `er7::read_messages(reader)` reads them one at a time from
any `BufRead`, so a large file never has to fit in memory.

## Where next

- [Glossary](../glossary/index.md): every term above, with its Rust type.
- [The `er7` tutorial](../../er7/docs/usage/index.md) and
  [path notation](../../er7/docs/paths/index.md).
- [Which crate do I need?](../which-crate/index.md)

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
