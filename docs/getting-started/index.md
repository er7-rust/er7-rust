[er7-rust](../../index.md) → [docs](../index.md) → getting started

# Your first message in 5 minutes

From an empty directory to a parsed, queried, edited, and rewritten HL7®
v2 message. Every output below was captured from a real run of the code
shown. The message is synthetic, like every message in this repository.

## 1. Make a project (30 seconds)

```sh
cargo new hello-er7
cd hello-er7
cargo add er7
```

`er7` has no dependencies of its own, so this builds in seconds. It needs
Rust 1.98 or later.

## 2. Save a message (30 seconds)

Save this as `message.er7`. Each segment goes on its own line. `er7`
accepts `\r`, `\n`, or `\r\n` between segments.

```text
MSH|^~\&|LAB|ACME|EHR|CLINIC|20261126091500||ORU^R01|MSG00042|P|2.5
PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE||19620320|F|||||555-0100~555-0199
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F
OBX|2|NM|2571-8^Triglycerides^LN||""|mg/dL|||||F
```

`MSH` is the header, `PID` the patient, and each `OBX` one result.
`|` separates fields, `^` components, and `~` repetitions. The two
quote characters in the second `OBX` are the explicit null: "clear this
value".

## 3. Read, edit, and write it (3 minutes)

Replace `src/main.rs` with:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse. The header's own delimiters are used, never assumed.
    let text = std::fs::read_to_string("message.er7")?;
    let message = er7::parse(&text)?;
    println!("{} segments, a {:?}", message.segments.len(), message.message_code());

    // 2. Read by HL7 path. A missing position is None, not an error.
    println!("family name: {:?}", message.query("PID-5.1")?);
    println!("PID-99:      {:?}", message.query("PID-99")?);

    // 3. Repeating fields and repeated segments.
    println!("phones:      {:?}", message.query_all("PID-13.1")?);
    for obx in message.segments_named("OBX") {
        let test = obx.first_value(3, 2, &message.separators);
        let null = obx.field(5).is_some_and(er7::Field::is_null);
        println!("OBX:         {test:?}, value is the explicit null: {null}");
    }

    // 4. Edit a leaf. `set` escapes delimiters, so the message cannot break.
    let mut message = message;
    let separators = message.separators;
    let family = message
        .segment_at_mut("PID", 1)
        .and_then(|pid| pid.field_mut(5))
        .and_then(|field| field.repetition_mut(1))
        .and_then(|repetition| repetition.component_mut(1))
        .and_then(|component| component.subcomponent_mut(1))
        .ok_or("PID-5.1 is not in the message")?;
    family.set("SMITH & JONES", &separators);

    // 5. Write it back. Untouched text is reproduced byte for byte.
    println!("{}", message.to_er7().replace('\r', "\n"));
    Ok(())
}
```

`cargo run` prints:

```text
4 segments, a Some("ORU")
family name: Some("EVERYWOMAN")
PID-99:      None
phones:      ["555-0100", "555-0199"]
OBX:         Some("Cholesterol"), value is the explicit null: false
OBX:         Some("Triglycerides"), value is the explicit null: true
MSH|^~\&|LAB|ACME|EHR|CLINIC|20261126091500||ORU^R01|MSG00042|P|2.5
PID|1||444333222^^^ACME^MR||SMITH \T\ JONES^EVE||19620320|F|||||555-0100~555-0199
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F
OBX|2|NM|2571-8^Triglycerides^LN||""|mg/dL|||||F
```

Four things to notice:

- **Paths are 1-based**, as in every HL7 document. `PID-5.1` is the first
  component of the fifth field.
- **A missing position is `None`**, never a panic or an error. Only a
  message without a usable header fails to parse.
- **The explicit null is not empty.** `is_null` tells "clear this value"
  apart from "nothing sent", and a database write needs to know which.
- **`set` takes text, not ER7.** The `&` became `\T\` on the wire, so
  it cannot split the component.

## 4. The same from the command line (1 minute)

```sh
cargo install er7
er7 message.er7
```

prints every value with the path that names it:

```text
MSH-1       |
MSH-2       ^~\&
MSH-3       LAB
MSH-4       ACME
MSH-5       EHR
MSH-6       CLINIC
MSH-7       20261126091500
MSH-9.1     ORU
MSH-9.2     R01
MSH-10      MSG00042
MSH-11      P
MSH-12      2.5
PID-1       1
PID-3.1     444333222
PID-3.4     ACME
PID-3.5     MR
PID-5.1     EVERYWOMAN
PID-5.2     EVE
PID-7       19620320
PID-8       F
PID-13[1]   555-0100
PID-13[2]   555-0199
OBX[1]-1    1
OBX[1]-2    NM
OBX[1]-3.1  2093-3
OBX[1]-3.2  Cholesterol
OBX[1]-3.3  LN
OBX[1]-5    187
OBX[1]-6    mg/dL
OBX[1]-11   F
OBX[2]-1    2
OBX[2]-2    NM
OBX[2]-3.1  2571-8
OBX[2]-3.2  Triglycerides
OBX[2]-3.3  LN
OBX[2]-5    ""
OBX[2]-6    mg/dL
OBX[2]-11   F
```

`er7 -q PID-5.1 -q PID-13 message.er7` prints just those:

```text
EVERYWOMAN
555-0100~555-0199
```

## 5. Share it without the patient (bonus)

```sh
cargo install er7-redact
er7-redact message.er7
```

```text
MSH|^~\&|LAB|ACME|EHR|CLINIC|20261126091500||ORU^R01|MSG00042|P|2.5
PID|1||73f419a72093eda0^^^ACME^MR||REDACTED^REDACTED||1962|F|||||~
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F
OBX|2|NM|2571-8^Triglycerides^LN||""|mg/dL|||||F
```

`er7-redact --report message.er7` says what changed, and never the values:

```text
PID[1]-3[1].1.1   pseudonym
PID[1]-5[1].1.1   replace REDACTED
PID[1]-5[1].2.1   replace REDACTED
PID[1]-7[1].1.1   first 4
PID[1]-13[1].1.1  clear
PID[1]-13[2].1.1  clear
```

The built-in policy is a starting point, not a compliance certification.
It changes only the positions it names, so read
[the redaction guide](../../er7-redact/docs/usage/index.md) before sharing
a real message.

## Next

- [Which crate do I need?](../which-crate/index.md)
- [HL7 v2 for Rust developers](../hl7-v2-for-rust-developers/index.md)
- [The full `er7` tutorial](../../er7/docs/usage/index.md)

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
