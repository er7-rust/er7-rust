[`er7-redact` specification](index.md) — section 4 of 17. Section numbers (§4.x) are stable and cited from code, tests, and commit messages.

# 4. What redaction preserves

Implemented in `src/redact.rs`.

This is the section that makes the crate usable rather than merely
destructive. A redacted message has to survive everything that was going
to be done to the original.

## 4.1 The shape is preserved [D1]

Redaction rewrites **leaf text only**. After a redaction:

- the same segments are present, with the same names, in the same order;
- each segment has the same number of fields;
- each field has the same number of repetitions;
- each repetition has the same number of components;
- each component has the same number of subcomponents;
- the delimiter set is unchanged.

So every field number still counts to the same place. A test harness that
asserts on `PID-13[2]` finds that position afterwards; a message viewer
lines up the same way.

`Action::Null` is the sole exception in the tree, and it collapses on
purpose (D6, [§3.4](03-actions.md)).

### The one thing writing out changes

Written back to text and read again, a redacted message has the same
segments and the same field numbering — always. Below the field, there is
one unavoidable difference, and it is HL7's rather than this crate's:

```
PID|1||9|X^Y          →   PID|1|||^          (PID-3 and PID-4 cleared)
```

`PID-4` kept both component positions, because the `^` between them is
still written. `PID-3` had one value, so clearing it leaves an empty
field — and an empty field, re-read, has no repetitions at all (`er7` R7).

The field still occupies its position, so nothing shifts and no path
resolves to the wrong place. And it is what a cleared value has to look
like: a redacted field that could still be told apart from a field the
sender left empty would be announcing that something had been removed.

## 4.2 No position is created [D2]

Redaction never lengthens a segment, a field, a repetition, or a
component. A rule for `PID-19` against a message whose `PID` stops at
field 18 does nothing at all — it does not pad the segment with empty
fields in order to have something to redact.

Two reasons. Padding changes what the message says: HL7 distinguishes a
field that was never sent from one sent empty, and inventing the latter is
a claim the sender never made. And padding is visible: a message that grew
eleven trailing pipes announces that something was redacted there, which
is the opposite of the point.

## 4.3 Empty stays empty, null stays null [D3, D4]

Before any action runs, two kinds of leaf are skipped:

| Leaf | Why it is skipped |
| ---- | ----------------- |
| empty (`\|\|`) | there is nothing to redact. Writing `REDACTED` into it would **invent** a value — and announce that a value used to be there, which is a disclosure in itself |
| explicit null (`\|""\|`) | the null is an instruction to the receiver, not patient data. Overwriting it turns "clear this value" into a value, which corrupts a record (the `er7` spec §5.3, R10) |

The skip is unconditional: it applies to rules and to the fallback alike,
and to every action. An explicitly written `PID-5 replace REDACTED` still
leaves an empty `PID-5` empty.

To *make* a position null, use `Action::Null`, which is the only way to
write one ([§3.3](03-actions.md)).

## 4.4 The delimiters are untouchable [D5]

Fields 1 and 2 of a header segment — `MSH`, `FHS`, `BHS` — are the field
separator and the encoding characters (`er7` R8). They are structure, not
data, and no rule and no fallback ever changes them. A rule naming them is
accepted, applied to nothing, and reported as nothing.

A message whose `MSH-1` had been redacted would not parse; a message whose
`MSH-2` had been redacted would parse into different values. Both are
worse than the disclosure they would prevent, and neither field carries
patient detail in the first place.

## 4.5 An untouched message round-trips [D17]

If no rule and no fallback names anything the message carries, then
`redact` leaves the tree exactly as it was, and writing it back reproduces
the input byte for byte — the `er7` round-trip guarantee (R16) carries
through unchanged.

More narrowly, and just as important: **a leaf no action touches is never
rewritten.** Its raw text is left as the sender wrote it rather than
decoded and re-encoded, so an escape sequence the crate does not
understand, or one spelled unusually, survives redaction intact.

## 4.6 What is not preserved

Stated plainly, so nobody is surprised:

- **The values.** That is the point.
- **Byte length.** `Replace` and `Pseudonym` change how long a value is.
  `Mask` does not, which is exactly why `Mask` leaks a little
  ([§5.5](05-built-in-policies.md)).
- **Cross-field consistency.** If a name appears in both `PID-5` and a
  free-text `NTE-3`, redacting `PID-5` does not touch the copy. Nothing
  positional can.
- **Any way back.** There is no mapping table and no undo
  ([§1.3](01-purpose-and-scope.md)).
