# The ER7 pipe-hat format

Background on the format itself, independent of this crate. What `er7`
does with it is in [`index.md`](index.md).

**ER7** — "Encoding Rules 7" — is the original text encoding for HL7
version 2 messages, and still the one nearly every production interface
speaks. It is defined in chapter 2 of every v2 release, from 2.1 in 1990
through 2.9. The nickname *pipe-hat* comes from its two most visible
delimiters, `|` and `^`.

An ER7 message is plain text, positional, and small:

```
MSH|^~\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5
PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F
```

That is a lab result for a patient: who sent it, who the patient is, and
what the cholesterol was. The same content in HL7's XML encoding runs to
several kilobytes.

## Hierarchy

Six levels, each with its own delimiter:

| Level        | Separated by | Example |
|--------------|--------------|---------|
| message      | —            | the whole text |
| segment      | carriage return | `PID\|1\|\|444333222...` |
| field        | `\|`          | `EVERYWOMAN^EVE^E` |
| repetition   | `~`          | `555-1111~555-2222` |
| component    | `^`          | `EVERYWOMAN` |
| subcomponent | `&`          | `1.2.840.114398.1.100` |

A **segment** is three characters of name followed by its fields: `MSH` is
the message header, `PID` patient identification, `OBX` an observation.
Names beginning with `Z` are local extensions, defined by whoever is at the
two ends and by nobody else.

Everything is **positional**. `PID-5.1` is a family name because it is
fifth and first, not because anything in the message says so. This is why
one misplaced `|` corrupts everything after it, and why a message needs a
dictionary — the HL7 standard for that version — before its values mean
anything.

## Delimiters

Only the segment terminator is fixed: a carriage return, `\r`, hex 0D. The
standard is explicit that implementers cannot change it. In practice many
systems store messages in files with `\n` or `\r\n` instead, so a tolerant
reader accepts all three.

The rest of the delimiters are declared by the message, in its first two
fields:

```
MSH|^~\&|
   ^^^^^
   |||||
   ||||+- subcomponent separator  (MSH-2 position 4)
   |||+-- escape character        (MSH-2 position 3)
   ||+--- repetition separator    (MSH-2 position 2)
   |+---- component separator     (MSH-2 position 1)
   +----- field separator         (MSH-1, the 4th character of the message)
```

MSH-1 is a field whose value is the field separator, and MSH-2 is a field
whose value is the encoding characters. This is circular by design, and it
means those two fields can never be parsed or escaped like ordinary ones.
It also means a reader learns the delimiters from bytes 4–8 of the message
and must not assume `|^~\&`, however universal that choice is in practice.

HL7 v2.7 added a fifth encoding character, the **truncation character**
(recommended `#`), marking a value the sender cut short to fit a length
limit. Most messages omit it.

The batch envelope segments `FHS` and `BHS` declare delimiters the same
way, since a batch file may begin with either.

## Empty, and the explicit null

Three states, easily confused, and the difference is clinical:

| On the wire | Means |
|-------------|-------|
| the field was never sent | no information; leave any existing value alone |
| `\|\|` | present but no value; likewise leave it alone |
| `\|""\|` | the **explicit null** — the sender is clearing this value |

Trailing fields a sender has nothing for may simply be dropped, so a `PID`
ending after field 8 is normal and says nothing about fields 9 onward. The
two-character `""` is the only way to say "delete what you have".

## Escape sequences

A value that needs to contain a delimiter escapes it. A sequence is the
escape character, a body, and the escape character again.

| Sequence | Meaning |
|----------|---------|
| `\F\` | the field separator as data |
| `\S\` | the component separator as data |
| `\T\` | the subcomponent separator as data |
| `\R\` | the repetition separator as data |
| `\E\` | the escape character as data |
| `\H\` | start highlighting |
| `\N\` | normal text, ending highlighting |
| `\Xdd..\` | hexadecimal data; pairs of digits, each pair one byte |
| `\Zdd..\` | locally defined, meaning agreed between the two ends |
| `\Cxxyy\` | switch to a single-byte character set |
| `\Mxxyyzz\` | switch to a multi-byte character set; `zz` optional |
| `\.cmd\` | a formatted-text display command, listed below |

The standard scopes escaping to `ST`, `TX`, and `FT` fields and to the
fourth component of `ED` — but a receiver has no way to know a field's data
type without the dictionary, so in practice sequences are decoded wherever
they appear.

The display commands, used inside `FT` fields:

| Command | Meaning |
|---------|---------|
| `.sp <n>` | end the line and skip `n` vertical spaces |
| `.br` | begin a new output line |
| `.fi` | begin word wrap (the default) |
| `.nf` | begin no-wrap |
| `.in <n>` | indent by `n` spaces |
| `.ti <n>` | temporarily indent `n` spaces |
| `.sk <n>` | skip `n` spaces to the right |
| `.ce` | centre the next line |

Because a carriage return ends a segment, a value that genuinely contains
one must send `\X0D\`. Nothing else will survive.

## Batch files

Several messages can share a file, wrapped in an envelope:

```
FHS   file header
  BHS   batch header
    MSH   message ...
    MSH   message ...
  BTS   batch trailer
FTS   file trailer
```

The envelope segments describe the file, not any message in it. A reader
that wants the messages drops them and starts a new message at each `MSH`.

## On the wire

ER7 messages are usually carried by **MLLP** (Minimal Lower Layer
Protocol): each message is wrapped in a start byte (0x0B) and an end
sequence (0x1C 0x0D) over a TCP connection, and the receiver answers with
an `ACK` message quoting the original's MSH-10 control ID. The framing is a
separate concern from the encoding, which is why this crate handles only
the latter.

## Why it persists

The tradeoffs are stark, and they have kept ER7 in place for thirty-five
years.

In its favour: messages are tiny, so an interface engine can move millions
a day; the format is trivially streamable; and it is embedded in hundreds
of thousands of production interfaces, most of which will never be
rewritten.

Against it: it is positional, so it is brittle and unreadable without
tooling; there is no schema in the message itself; and the same field
number means different things in different versions. HL7 published an XML
encoding in v2.3.1 and FHIR later, but neither displaced ER7 in the
installed base.

## References

- [HL7 v2.5 chapter 2, control](https://www.hl7.eu/HL7v2x/v25/std25/ch02.html)
- [HL7 v2.8 chapter 2, control](https://www.hl7.eu/HL7v2x/v28/std28/ch02.html)
- [HL7 v2+ XML encoding syntax](http://v2plus.hl7.org/2021Jan/xml-encoding-rules.html)
- [Caristix: HL7 ER7 encoding](https://caristix.com/help-center/v3/test/task/hl7-er7-encoding/)
- [Rhapsody: HL7 escape sequences](https://rhapsody.health/resources/hl7-escape-sequences/)
- [Saga IT: HL7 v2 encoding and delimiters](https://saga-it.com/docs/hl7/reference/encoding)
- [InterSystems: HL7 escape sequences](https://docs.intersystems.com/latest/csp/docbook/DocBook.UI.Page.cls?KEY=EHL72_ESCAPE_SEQUENCES)
- [ETLworks: HL7 2.x formats](https://support.etlworks.com/hc/en-us/articles/360014078373-HL7-2-x-Formats)
- [hl7apy](https://crs4.github.io/hl7apy/) — a Python library, useful as a second reading of the same rules
