# Comparisons

Interface engines, the mature libraries on other platforms, the other Rust
crates, the sibling projects in this family, and the pipe-splitting you
were about to write yourself. When each is the right answer, and when this
project is the wrong one.

The reader-friendly version is <https://er7-rust.github.io/comparison/>.

**No performance comparison is claimed here.** Nothing in this workspace
has been benchmarked against another library. Comparing fairly means
matching what each one actually does, and a parser that only splits on
pipes is not doing the same work as one that preserves escape sequences and
round-trips byte for byte. So this document compares *capability and
shape*, which is checkable, rather than speed, which would not be. Our own
measured figures, with their method and their machine, are in
[`BENCHMARKS.md`](BENCHMARKS.md).

## First, which kind of thing do you need?

Most comparisons in this space go wrong by putting products from three
different categories into one table. The honest first question is what you
are building.

| If you need to… | You want | This workspace |
|---|---|---|
| *Run* interfaces: routes, retries, queues, monitoring, on-call | An interface engine | Not that. Useful alongside one. |
| Write an application that happens to speak HL7® v2 | A library | Yes — `er7`, `er7-redact`, `serde-er7` |
| Do a one-off transformation at a shell prompt | A command-line tool | Yes — two binaries, no Rust knowledge required |
| Validate a message against the standard's tables | A conformance tool | **No.** Deliberately out of scope; see below |
| Speak the HL7® FHIR® standard | An HL7® FHIR® library | No. Different standard entirely. |

## Interface engines

[Open Integration Engine](https://github.com/OpenIntegrationEngine) — the
community fork of Mirth Connect, made after Mirth moved to a
commercial-only license at version 4.6 — and its commercial siblings
(Rhapsody, InterSystems, Cloverleaf) are a different category of thing
entirely. An engine gives you channels, routing, a management UI,
JavaScript transformers, persistence, retry, alerting, and an operations
story. It is a system you deploy and run.

A library gives you a function call. If your problem is "forty interfaces,
three hospitals, and someone has to be paged when one stops", an engine is
the right answer and no amount of crate substitutes for it.

These crates are useful *alongside* an engine rather than instead of it:

- The service at the end of a channel, where you would otherwise be writing
  the v2 parsing again in whatever language that service is in.
- A shell-level check, redaction, or normalisation, using the command-line
  tools, without standing anything up.
- Getting a message into a bug report. `er7-redact --report` prints the
  paths and actions with **no values in the output**, which is the shape a
  ticket can carry.

## HAPI, and the mature libraries

[HAPI HL7v2](https://hapifhir.github.io/hapi-hl7v2/) is the reference
open-source HL7 v2 library, in Java, dual licensed under MPL 1.1 and
GPL 2.0. It has been maintained for two decades, ships a generated typed
model for every segment and message of every release, and has seen far more
real traffic than anything here. [NHapi](https://github.com/nHapiNET/nHapi)
is its .NET port; [hl7apy](https://github.com/crs4/hl7apy) is the
best-established Python option, covering v2.1 through v2.8.2.

**If your platform is the JVM, use HAPI.** That is not modesty: a
twenty-year-old library with complete release coverage and a large user
base is the lower-risk choice, and reimplementing it in a language you were
not otherwise using is a bad trade.

Where this workspace differs, stated as trade-offs rather than wins:

| | The mature libraries | Here |
|---|---|---|
| Typed model | Complete generated model: every segment, every release | **None.** `er7` is a structural tree; a value's meaning comes from its path, not from a generated class |
| Validation | Conformance checking against the standard's tables | **None**, on purpose. `er7` parses structure and does not judge content |
| Runtime | A JVM, a CLR, or a Python interpreter | A static binary: no runtime, no GC |
| Dependency tree | Substantial, and audited as such | Zero, one, and two crates respectively |
| Round trip | Varies by parser and options | Byte for byte, as a tested guarantee |
| Track record | Two decades of production traffic | First published 2026-08-15. New. |
| License | MPL 1.1 or GPL 2.0 for HAPI | MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only, at your option |

The license row decides some evaluations outright. A permissive option
matters if you are linking into a closed-source product; a copyleft option
matters if your organisation prefers one. Offering five is how this project
avoids having that conversation with anyone.

## The other Rust crates

A small field, and cooperation beats competition in it. Download figures
below are from the crates.io API on **2026-08-30** (re-checked live; the
previous pass was 2026-08-26) and are recorded as context, not as a ranking.

| Crate | Latest | Last published | Downloads | Shape |
|---|---|---|---|---|
| [`hl7-parser`](https://github.com/hamaluik/hl7-parser) | 0.3.0 | 2025-02 | 17,104 | The most-used and most actively developed alternative. Parses structure without validating; optional `serde`, and timestamp parsing into `chrono`, `time`, or `jiff`; message building; cursor-by-character-index; lenient separators. Apache-2.0 |
| [`hl7v2-parser`](https://github.com/EffortlessMetrics/hl7v2-rs) | 1.2.0 | 2026-03 | 213 | Newer. "Zero-allocation where possible"; a companion `hl7v2_stream` gives event-based streaming with bounded memory |
| [`rust-hl7`](https://github.com/wokket/rust-hl7) | 0.5.0 | 2021-09 | 14,782 | Buffer-copy-free indexing with HL7 notation; self-described as experimental; explicitly no plan for conformance checking. Last published 2021 |
| [`hl7-mllp-codec`](https://github.com/wokket/hl7-mllp-codec) | 0.4.0 | 2022-07 | 25,778 | Not a parser at all: a Tokio codec for MLLP framing. Complementary to any of the above, including these crates |
| `er7` | 0.2.1 | 2026-08 | 464 | This one |

**When to choose one of those instead.** These are real reasons, not
hedges:

- **You want timestamp types.** `hl7-parser` parses HL7 timestamps into
  `chrono`, `time`, or `jiff`. `er7` returns the text and leaves the
  interpretation to you, because doing otherwise means a dependency.
- **You want streaming with bounded memory over very large inputs.**
  `hl7v2_stream` is built for that; `er7` parses a message into a tree.
- **You want `serde` on the parser itself.** `hl7-parser` has a feature
  flag. Here it is a separate crate, `serde-er7`, precisely so that users
  who do not want `serde` do not pay for it.
- **You are already using one and it works.** Switching a working HL7
  parser is rarely the highest-value thing on anyone's list.

**Where these crates differ.** Also real:

- **Zero runtime dependencies in `er7`, enforced by a test** — the
  `[dependencies]` table is empty and
  `the_crate_has_no_runtime_dependencies` fails if that changes. Criterion
  lives in a separate unpublished workspace member so that even
  `[dev-dependencies]` stays empty.
- **A command-line tool, not only a library.** `er7 message.er7` prints
  every value with the HL7 path that names it, and every label in that
  output is a valid query you can paste back in.
- **Redaction as a first-class, separately published thing.**
  `er7-redact` is the crate that has no equivalent in the list above: it
  removes patient detail while keeping segments, fields, components,
  delimiters, and escape sequences intact, so every path that resolved to a
  value still resolves to one.
- **Specification-first development.** Every behaviour is a numbered rule
  in `spec/`, and each rule names the test that enforces it. That is a
  claim you can check by reading the tree rather than trusting a README.
- **Five licenses.** The alternatives above are Apache-2.0 or MIT.

## The sibling projects

The same author maintains two adjacent Rust projects, and the boundary
between them matters more than the overlap:

| Project | Layer | Relationship |
|---|---|---|
| [`hl7-rust`](https://github.com/hl7-rust/hl7-rust) | HL7 v2 *semantics*: segments, composite data types, message structures, dictionaries per release, MLLP and SOAP transports, conversions to and from v2.xml and JSON | Builds **on** `er7`. If you need to know that `PID-5` is a person name and validate it as one, that is `hl7-2`, not `er7` |
| This workspace | The ER7 *encoding*: text in, tree out, tree in, identical text out | The layer underneath. It knows delimiters, escaping, and paths; it does not know what a segment means |

`er7` is deliberately the smaller, dumber, lower thing. The split exists so
that the encoding layer can carry zero dependencies and be audited on its
own. The whole family and its boundaries are drawn at
<https://er7-rust.github.io/ecosystem/>.

## Splitting on pipes yourself

The honest comparison, because it is what most teams actually do and it is
sometimes right.

```python
fields = line.split("|")          # this works, until it doesn't
```

**When hand-rolling is the right answer:** you read one field, from one
sender, whose messages you control, in a script that will not outlive the
week.

**When it stops being the right answer**, in roughly the order teams
discover it:

1. **Escape sequences.** `\F\` is a literal `|` in a value. Splitting on
   `|` cuts a value in half, silently.
2. **The `MSH` segment is special.** `MSH-1` *is* the field separator and
   `MSH-2` *is* the encoding characters, so the segment does not index like
   any other, and off-by-one errors here are subtle and permanent.
3. **A sender who does not use the default delimiters.** They are declared
   in `MSH-2` and you are expected to read them, not assume them.
4. **Repeating fields.** `~` separates repetitions, so `PID-13` may be one
   phone number or four, and the code that assumed one is now wrong.
5. **Components and subcomponents.** `^` and `&` go two levels deeper.
6. **Writing.** Now you need to escape on the way out, and a value
   containing a delimiter that is not escaped breaks the message for
   everyone downstream.
7. **Round-tripping.** Someone asks why the message that came out is not
   byte-identical to the one that went in, and the answer is a week.

The pitch is not "your split is wrong." It is that the seven items above
are exactly what `er7` implements, in about the same runtime cost, with a
test for each and no dependency to audit.

## What none of these crates do

Stated here so no comparison implies otherwise:

- **No validation or conformance checking.** Structure is parsed; content
  is not judged.
- **No typed segment model.** No generated class per segment.
- **No transport.** No MLLP, no TCP, no SOAP — see `hl7-mllp-codec` or the
  `hl7-2-mllp` sibling.
- **No HL7® FHIR® standard support**, and no v2-to-FHIR® mapping.
- **No clinical claim.** These are encoding libraries, not medical devices.
  See [`AI_STATEMENT.md`](AI_STATEMENT.md) §2.

Each crate's own `spec/` states its scope and its non-goals precisely; that
is the authoritative version of this list.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
