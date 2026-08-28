[`er7-redact` specification](../index.md) — section 15 of 17. Section
numbers (§15.x) are stable and cited from code, tests, and commit messages.

# 15. Open tasks

The backlog. Task IDs are stable and **never reused**, even after a task
is finished or withdrawn. When a task is finished, delete it from this
section in the same change that ships the work — the commit history is the
archive — and name the `T<n>` in the commit message.

---

## T1 — Redact known values wherever they appear

**Why.** A patient name removed from `PID-5` is still in the `NTE-3` that
says "spoke with Mrs Everywoman about the result". This is the single
biggest gap in a positional redactor ([§14.2](../14-roadmap/index.md),
[§5.4](../05-built-in-policies/index.md)).

**Shape.** A second pass. Collect the decoded values the policy found at
its named positions, then search every other leaf for those strings and
apply an action where one is found. No patterns, so no false positives on
clinical text; the crate only removes strings it already knows are
identifiers.

**Open questions.** Case sensitivity. Substring versus whole-word (a
surname `Wood` inside `Woodward`). Minimum length, so a one-character
initial does not blank the message. Whether the action is the same one the
position's rule used, or a separate setting.

**Done when.** A message where the name appears in both `PID-5` and
`NTE-3` comes back with neither, the report names both positions, a
clinical word that merely resembles an identifier is untouched, and §5.4
is rewritten to describe what is now covered.

---

## T2 — A caller-supplied action

**Why.** `Action` is closed, so a real MAC
([§7.4](../07-pseudonyms/index.md)), a lookup table, or a date shift cannot
be expressed at all.

**Shape.** A variant holding a boxed function from the decoded value to a
replacement, plus a decision about what happens to `Clone`, `PartialEq`,
`Display`, and the policy file format when one is present.

**Done when.** A policy mixing built-in and caller-supplied actions
redacts correctly, a policy holding one reports rather than panics when
asked to write itself to a file, and §3 and §6 say what the boundary is.

---

## T5 — Date shifting

**Why.** `first 4` on a birth date destroys the interval between events,
which is what longitudinal test data is for
([§14.4](../14-roadmap/index.md)).

**Shape.** An action that parses an HL7® timestamp, adds a per-patient
offset derived from the pseudonym key, and writes it back at the same
precision it was given.

**Open questions.** Where the per-patient key comes from when the policy
is applied to one message at a time. What happens to a timestamp the
action cannot parse — leave it, or clear it. Whether parsing an HL7
timestamp crosses the line §5.3 draws.

**Done when.** A message's dates shift consistently, an unparseable
timestamp has a documented outcome, and §3 gains the action with a
worked example.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
