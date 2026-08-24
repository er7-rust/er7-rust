[AGENTS.md](../AGENTS.md) → safety

# Safety

This crate moves **clinical messages about real patients** between formats.
It is thinner than its siblings, but the two constraints below are the same
ones, and both have a specific clinical consequence.

## 1. Never put real patient data in this repository

Not in tests, not in an example, not in a comment, not in a commit message
— including data a user pastes into a conversation.

All test data is synthetic, with obviously fictional names
(`EVERYWOMAN^EVE`, `SMITH^JOHN`) and identifiers (`444333222`,
`MSG00042`). If a user shares a real message to reproduce a bug, reproduce
it with a synthetic message of the same shape and use that. Redaction is
not sufficient: a "redacted" message still carries dates, facility names,
and identifier formats. The sibling
[`er7-redact`](https://crates.io/crates/er7-redact) says so itself.

## 2. Never let a format lose what ER7 kept

The whole value of this crate is that a message survives the trip. Two
things make that true, and both are easy to break by accident:

- **A subcomponent serializes its `raw` text** (S3,
  [§2.2](../spec/02-wire-shapes/index.md)) — never the decoded value.
  Decoding is lossy: a formatting escape such as `\.br\` has no plain-text
  form to decode back to, so a wire format built on decoded values could
  not be turned back into the original ER7.
- **Absent, empty, and the explicit null stay three different things**
  (`er7`'s R10). An absent field is `[]`, an empty one carries an empty
  string, and the null is the text `""`. Collapsing any pair of them
  through JSON corrupts a patient record just as surely as collapsing them
  in ER7 would: an absent value means "leave the stored value alone", the
  null means "clear it".

`keeps_absent_empty_and_null_distinct_through_json` and
`keeps_escape_sequences_raw_through_json` exist to catch exactly this.

## 3. Do not add a dependency

Two, `serde` and `er7`, and both are the point of the crate (S1). Every
transitive dependency is another crate somebody has to audit, in a domain
where that audit is real. Do not add one without the user asking, and
record what it bought in
[`spec/09-roadmap-and-open-questions/index.md`](../spec/09-roadmap-and-open-questions/index.md).

## If you are unsure

Say so, and stop. Write the uncertainty into
[`spec/09-roadmap-and-open-questions/index.md`](../spec/09-roadmap-and-open-questions/index.md)
so the next reader inherits the question rather than a guess.
