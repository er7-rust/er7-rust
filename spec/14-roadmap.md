[`er7-redact` specification](index.md) — section 14 of 17. Section numbers (§14.x) are stable and cited from code, tests, and commit messages.

# 14. Roadmap

In priority order. Each item says what it buys and what it would cost;
items with a "done when" clause and an ID live in
[§15](15-open-tasks.md).

## 14.1 Now — 0.1

The shape described by §1–§13: policies, eight actions, two built-ins, the
file format, pseudonyms, the report, and the CLI. Complete.

## 14.2 Next — free-text scanning

The largest real gap ([§5.4](05-built-in-policies.md)). A name in
`NTE-3` survives every positional policy, and everybody who has redacted a
message by hand has found one there.

The design question is what the crate is allowed to know. Two candidates:

- **Redact-what-you-know**: take the values already found at identifier
  positions and remove those strings wherever else they appear in the
  message. No pattern matching, no false positives on clinical text, and
  it composes with the existing model — the values come from the same
  policy. It misses anything not present in a named position.
- **Pattern matching**: recognise the shape of a phone number, an SSN, a
  date. Catches more, invents a dependency or a hand-rolled matcher, and
  false-positives on lab values, which is worse than it sounds because it
  destroys the clinical content.

The first is the one to build; it is [T1](15-open-tasks.md). The second is
declined for now ([§16.2](16-open-questions-and-declined-decisions.md)).

## 14.3 Next — a caller-supplied action

`Action` is a closed enum, which means a caller who needs a real MAC, a
date shift, or a lookup table cannot express it
([§7.4](07-pseudonyms.md)). A variant holding a function would open it.
The cost is that `Action` stops being `Clone`, `PartialEq`, and
`Display`-able in the ordinary way, and a policy stops round-tripping
through a file — so the design has to keep the two kinds of policy
separable. [T2](15-open-tasks.md).

## 14.4 Later — date shifting

Shifting every date in a message by a per-patient offset preserves
intervals — the thing `first 4` destroys and the thing longitudinal
analysis needs. It requires parsing HL7 timestamps, which is dictionary
knowledge of exactly the kind [§5.3](05-built-in-policies.md) is careful
about, and it needs the per-patient offset to come from somewhere. Worth
doing, not worth guessing at. [T5](15-open-tasks.md).

## 14.5 Later — a redaction check

The inverse tool: given a message and a policy, report the positions that
carry text and are *not* named by any rule. This is how a caller finds out
what a policy is missing, and it is a small amount of code on top of the
existing walk — the fallback already computes exactly this set.
[T6](15-open-tasks.md).

## 14.6 Not planned

- **Re-identification.** No mapping table, no key escrow, no undo. A crate
  that can put the names back is a crate that has to be secured, and the
  whole value of a redacted export is that it does not.
- **A validator.** `er7`'s R24 applies here too: this crate knows which
  positions to edit, not whether a message is correct.
- **Transport.** Nothing reads a socket.
