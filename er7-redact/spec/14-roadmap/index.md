[`er7-redact` specification](../index.md) — section 14 of 17. Section
numbers (§14.x) are stable and cited from code, tests, and commit messages.

# 14. Roadmap

In priority order. Each item says what it buys and what it would cost;
items with a "done when" clause and an ID live in
[§15](../15-open-tasks/index.md).

## 14.1 Shipped — 0.2

The shape described by §1–§13 as of the 0.2 release: policies, eight
actions, the two postures and the four built-ins they come in, the file
format, pseudonyms, the report, and the CLI. Complete — and, as of 0.4.0,
one action further: D24 added a ninth, caller-supplied one
([§3.8](../03-actions/index.md)), which is why §14.2–§14.4 below each
say "shipped" or "resolved" rather than "now."

0.2 made the posture explicit ([§2.6](../02-redaction-model/index.md)).
Before it, a policy carried an optional *fallback* action, and "accept by
default" was the absence of one — a distinction a reader had to infer from
a missing line. Now a policy states which of the two it takes, cannot be
built without saying, and writes it out. Three rules came with it: a reject
rule beats an accept rule for the same leaf (D19), appending never weakens
a posture (D20), and a payload that is not ER7 is refused, passed, or acted
on whole (D21, [§2.8](../02-redaction-model/index.md)).

What it cost: `Policy::new`, `Policy::fallback`, `Policy::everything`, the
`*` line in a policy file, and the CLI's `--all`
([§13](../13-compatibility-and-versioning/index.md)). Each is refused by
name rather than left to fail quietly, because the one outcome worth ruling
out is a policy that still runs and has changed posture.

## 14.2 Shipped — free-text scanning

A rule for `PID-5` can remove a name there while leaving a repeat in
`NTE-3`. This motivated the known-value sweep ([§5.4](../05-built-in-policies/index.md)).

The second pass compares text from positions named by rules with other
leaves. A whole-word match can remove an entire note, including unrelated
text in that note. The [policy guide](../../docs/policies/index.md#values-found-elsewhere)
shows the verified Unicode boundaries (T7). Pattern matching — recognising the *shape*
of a phone number, an SSN, a date — remains declined
([§16.2](../16-open-questions-and-declined-decisions/index.md)).

Shipped 2026-08-29 as `Policy::search_known_values` (on by default) and
the sweep it drives (D23, [§2.10](../02-redaction-model/index.md)),
reusing `uncovered`'s own notion of "a leaf no rule or posture already
touched" rather than a third way of walking the tree. Closed
[T1](../15-open-tasks/index.md).

## 14.3 Shipped — a caller-supplied action

`Action` was a closed enum, which meant a caller who needed a real MAC, a
date shift, or a lookup table could not express it
([§7.4](../07-pseudonyms/index.md)). Shipped 2026-08-30 as
`Action::Custom(CustomAction)` (D24, [§3.8](../03-actions/index.md)),
admitted by [§16.11](../16-open-questions-and-declined-decisions/index.md)
per §3.1's own rule that a ninth action needs a section saying why the
eight were not enough.

The cost turned out smaller than expected: `Action` kept its ordinary
`Clone`, `PartialEq`, and `Eq` unchanged — `CustomAction`, a newtype
around the closure, carries the hand-written impls instead (`Clone` is an
`Arc` clone; equality is identity, `Arc::ptr_eq`). What did not survive is
`Display` round-tripping a policy that holds one through a file — it
never could, since a closure has no text to spell — and that boundary is
exactly what [§6.5](../06-policy-file-format/index.md) now states rather
than leaves implicit. Closed [T2](../15-open-tasks/index.md).

## 14.4 Resolved — date shifting, without a built-in

Shifting every date in a message by a per-patient offset preserves
intervals — the thing `first 4` destroys and the thing longitudinal
analysis needs. It requires parsing HL7® timestamps, which is dictionary
knowledge of exactly the kind [§5.3](../05-built-in-policies/index.md) is
careful about, and it needs the per-patient offset to come from somewhere.

Resolved 2026-08-30, once `Action::custom` (D24,
[§3.8](../03-actions/index.md)) existed to resolve it into: not a ninth
built-in, all three of T5's open questions answered by the same
mechanism, and a full worked example
(`examples/date_shift_with_a_custom_action.rs`) rather than a promise this
crate would parse a timestamp correctly forever.
[§16.12](../16-open-questions-and-declined-decisions/index.md) is the
decision; closed [T5](../15-open-tasks/index.md).

## 14.5 Shipped — a redaction check

The inverse tool: given a message and a policy, report the positions that
carry text and are *not* named by any rule. This is how a caller finds out
what a policy is missing. Shipped 2026-08-28 as `Redactor::uncovered`
(D22, [§2.9](../02-redaction-model/index.md)) and the CLI's `--uncovered`
([§10](../10-command-line-interface/index.md)). Coverage now inspects
the original tree directly, including leaves that a null action would
collapse ([§16.13](../16-open-questions-and-declined-decisions/index.md)).
Closed [T6](../15-open-tasks/index.md).

## 14.6 Not planned

- **Re-identification.** No mapping table, no key escrow, no undo. A crate
  that can put the names back is a crate that has to be secured, and the
  whole value of a redacted export is that it does not.
- **A validator.** `er7`'s R24 applies here too: this crate knows which
  positions to edit, not whether a message is correct.
- **Transport.** Nothing reads a socket.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
