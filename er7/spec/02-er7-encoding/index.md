[`er7` specification](../index.md) — section 2 of 19. Section numbers
(§2.x) are stable and cited from code, tests, and commit messages.

# 2. The ER7 encoding

Background on the format itself, independent of this crate. What the crate
does with it starts at [§3](../03-delimiters/index.md). Sources are listed
in [§2.9](#29-sources).

## 2.1 What ER7 is

**ER7** — "Encoding Rules 7" — is the original text encoding for HL7®
version 2 messages, and still the one nearly every production interface
speaks. The nickname *pipe-hat* comes from its two most visible
delimiters, `|` and `^`.

An ER7 message is plain text, positional, and small:

```
MSH|^~\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5
PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F
```

That is a lab result: who sent it, who the patient is, and what the
cholesterol was. The same content in HL7's XML encoding runs to several
kilobytes.

## 2.2 Hierarchy

Six levels, each with its own delimiter:

| Level        | Separated by    | Example |
|--------------|-----------------|---------|
| message      | —               | the whole text |
| segment      | carriage return | `PID\|1\|\|444333222...` |
| field        | `\|`            | `EVERYWOMAN^EVE^E` |
| repetition   | `~`             | `555-1111~555-2222` |
| component    | `^`             | `EVERYWOMAN` |
| subcomponent | `&`             | `1.2.840.114398.1.100` |

A **segment** is three characters of name followed by its fields: `MSH` is
the message header, `PID` patient identification, `OBX` an observation.
Names beginning with `Z` are local extensions, defined by whoever is at the
two ends and by nobody else.

Everything is **positional**. `PID-5.1` is a family name because it is
fifth and first, not because anything in the message says so. This is why
one misplaced `|` corrupts everything after it, and why a message needs a
dictionary — the HL7 standard for that version — before its values mean
anything. It is also why this crate stops at the encoding
([§1.3](../01-purpose-and-scope/index.md)).

## 2.3 Delimiters

Only the segment terminator is fixed: a carriage return, `\r`, hex 0D. The
standard is explicit that implementers cannot change it. In practice many
systems store messages in files with `\n` or `\r\n` instead, so a tolerant
reader accepts all three (R4).

The rest are declared by the message, in its first two fields:

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

MSH-1 is a field whose value *is* the field separator, and MSH-2 is a field
whose value *is* the encoding characters. This is circular by design, and
it is why those two fields can never be split or escaped like ordinary ones
(R8). It also means a reader learns the delimiters from bytes 4–8 of the
message and must not assume `|^~\&`, however universal that choice is in
practice (R1).

HL7 v2.7 added a fifth encoding character, the **truncation character**
(recommended `#`), marking a value the sender cut short to fit a length
limit. Most messages omit it.

The batch envelope segments `FHS` and `BHS` declare delimiters the same
way, since a batch file may begin with either.

## 2.4 Empty, and the explicit null

Three states, easily confused, and the difference is clinical:

| On the wire | Means |
|-------------|-------|
| the field was never sent | no information; leave any existing value alone |
| `\|\|` | present but no value; likewise leave it alone |
| `\|""\|` | the **explicit null** — the sender is clearing this value |

Trailing fields a sender has nothing for may simply be dropped, so a `PID`
ending after field 8 is normal and says nothing about fields 9 onward. The
two-character `""` is the only way to say "delete what you have". This
crate keeps all three apart (R10, R11).

## 2.5 Escape sequences

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

### 2.5.1 The escaping scope, and why this crate ignores it

The standard scopes escaping to `ST`, `TX`, and `FT` fields and to the
fourth component of the `ED` data type. A receiver cannot apply that rule
without knowing each field's data type — which requires the dictionary this
crate does not have ([§1.3](../01-purpose-and-scope/index.md)). So
sequences are decoded wherever they appear.

The risk is a false positive: a value that legitimately contains a
backslash, in a field where escaping does not apply, being read as a
sequence. The mitigation is R13 — unrecognized sequences stay literal — and
`Subcomponent::raw`, which always holds exactly what arrived. This is
recorded as a known divergence in
[§18.2](../18-open-questions-and-divergences/index.md).

## 2.6 Batch files

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
that wants the messages drops them and starts a new message at each `MSH`
(R21).

## 2.7 On the wire

ER7 messages are usually carried by **MLLP** (Minimal Lower Layer
Protocol): each message is wrapped in a start byte (0x0B) and an end
sequence (0x1C 0x0D) over a TCP connection, and the receiver answers with
an `ACK` message quoting the original's MSH-10 control ID. The framing is a
separate concern from the encoding, which is why this crate handles only
the latter (R24).

## 2.8 Why ER7 persists

The tradeoffs are stark, and they have kept ER7 in place for thirty-six
years (2.1 shipped 1990, per [§1.1](../01-purpose-and-scope/index.md)).

In its favour: messages are tiny, so an interface engine can move millions
a day; the format is trivially streamable; and it is embedded in hundreds
of thousands of production interfaces, most of which will never be
rewritten.

Against it: it is positional, so it is brittle and unreadable without
tooling; there is no schema in the message itself; and the same field
number means different things in different versions. HL7 published an XML
encoding in v2.3.1 and the HL7® FHIR® standard later, but neither
displaced ER7 in the installed base.

## 2.9 Sources

- [HL7 v2.5 chapter 2, control](https://www.hl7.eu/HL7v2x/v25/std25/ch02.html) — encoding rules, delimiter table, escape sequences
- [HL7 v2.8 chapter 2, control](https://www.hl7.eu/HL7v2x/v28/std28/ch02.html) — the same, with the truncation character
- [HL7 v2+ XML encoding syntax](http://v2plus.hl7.org/2021Jan/xml-encoding-rules.html) — the alternative encoding ER7 is contrasted with
- [Caristix: HL7 ER7 encoding](https://caristix.com/help-center/v3/test/task/hl7-er7-encoding/)
- [Rhapsody: HL7 escape sequences](https://rhapsody.health/resources/hl7-escape-sequences/)
- [Saga IT: HL7 v2 encoding and delimiters](https://saga-it.com/docs/hl7/reference/encoding)
- [InterSystems: HL7 escape sequences](https://docs.intersystems.com/latest/csp/docbook/DocBook.UI.Page.cls?KEY=EHL72_ESCAPE_SEQUENCES)
- [ETLworks: HL7 2.x formats](https://support.etlworks.com/hc/en-us/articles/360014078373-HL7-2-x-Formats)
- [hl7apy](https://crs4.github.io/hl7apy/) — a Python library, useful as a second reading of the same rules

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
