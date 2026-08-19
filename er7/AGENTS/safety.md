[AGENTS.md](../AGENTS.md) → safety

# Safety

This crate handles **clinical messages about real patients**. Read this
before writing code that touches behaviour.

Nothing here is theoretical. Every constraint below exists because
violating it has a specific, plausible clinical consequence.

## 1. Never put real patient data in this repository

Not in tests, not in samples, not in an example, not in a comment, not in a
commit message — including data a user pastes into a conversation.

- All test data is **synthetic**. Names are obviously fictional
  (`EVERYWOMAN^EVE`, `SMITH^JOHN`, `JONES^WILLIAM`); identifiers are
  obviously fake (`444333222`, `MSG00042`, `PATID1234`).
- If a user shares a real message to reproduce a bug, **reproduce it with a
  synthetic message that has the same shape** and use that. Do not commit
  theirs.
- Redaction is not sufficient. A "redacted" message still carries dates,
  facility names, and identifier formats.

## 2. Never collapse absent, empty, and null (R10, R11)

This is the sharpest edge in the whole format
([§5.3](../spec/05-value-tree.md)).

| On the wire | Means | A receiver must |
| ----------- | ----- | --------------- |
| field absent | no information | leave the stored value alone |
| `\|\|` | present, no value | leave the stored value alone |
| `\|""\|` | explicit null | **clear** the stored value |

Treating a null as empty means an allergy that was withdrawn stays on the
record. Treating an empty as a null means a value that was never sent
erases one that was.

Concretely, in this codebase:

- `is_empty` and `is_null` must never both be true for the same node.
- An accessor must return `None` for absent, not a default.
- `Subcomponent::value` reports a null as `""`. That is correct — it is the
  value being conveyed — which is exactly why any code path that writes to
  a record must ask `is_null()` first and must not infer it from `value()`.
- The CLI shows a null as `""` rather than as blank
  ([§12.3](../spec/12-command-line-interface.md)) for the same reason.

## 3. Never corrupt a message you are writing back (R16)

A message that arrives correct and leaves altered is worse than one that
fails to parse: the failure is visible, the alteration is not.

- Do not trim, normalize, case-fold, reorder, or decode at parse time.
- Do not "fix" a message on the way out — not a missing field, not an odd
  delimiter, not a malformed escape.
- When writing a value into a message, use
  `Subcomponent::set` ([§5.5](../spec/05-value-tree.md)), which encodes
  delimiters. Assigning `raw` directly with an unescaped `&` in it silently
  splits the component the next time the message is parsed, shifting every
  value after it.
- A carriage return in a value must be written as `\X0D\`. A literal one
  ends the segment and truncates the message
  ([§6.3](../spec/06-escape-sequences.md)).

## 4. Never claim more than the crate knows (R24)

The crate is an encoding, not a dictionary
([§1.3](../spec/01-purpose-and-scope.md)). It does not know what a value
*means*.

- Do not add a validator. A partial validator is worse than none: it
  implies the messages it passes are correct.
- Do not derive a message structure from the code and trigger event; that
  mapping is version-specific and a wrong answer routes a message to the
  wrong handler ([§10.3](../spec/10-msh-conveniences.md)).
- Do not guess a data type in order to decide whether to decode an escape
  ([§18.2](../spec/18-open-questions-and-divergences.md)).
- Do not add a field-name lookup, however small. `PID-5` is a patient name
  in one version and something else in another.

If a task seems to need any of these, the answer is a layer above, not a
patch here. Say so.

## 5. Never fail where you could carry on (R6)

A receiver that rejects a message it could have read drops clinical
information. Below the header, everything is data
([§11.2](../spec/11-error-handling.md)).

- Unknown segments, local `Z` segments, ragged field counts, unexpected
  component counts: parse them.
- An undecodable `\X..\`, an unknown `\..\`, an unterminated escape: keep
  them literally.
- Do not add an `Error` variant for something recoverable.

The one place strictness is correct is the delimiter set
([§3.3](../spec/03-delimiters.md)): a message that reuses one character for
two roles cannot be read back as the sender meant, so guessing would
produce confidently wrong values.

## 6. Dependencies are an audit surface (R25)

Healthcare integration code gets audited. Every transitive dependency is
another thing to review and another supply-chain risk, and this crate is
meant to sit at the bottom of a stack
([§15.1](../spec/15-dependencies-and-build.md)).

Do not add one without the user asking for it.

## If you are unsure

Say so, and stop. Write the uncertainty into
[`spec/18-open-questions-and-divergences.md`](../spec/18-open-questions-and-divergences.md)
so the next reader inherits the question rather than a guess. A recorded
open question is a good outcome; a silent assumption in a clinical
codebase is not.
