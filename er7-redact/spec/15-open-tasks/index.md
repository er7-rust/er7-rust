[`er7-redact` specification](../index.md) — section 15 of 17. Section
numbers (§15.x) are stable and cited from code, tests, and commit messages.

# 15. Open tasks

The backlog. Task IDs are stable and **never reused**, even after a task
is finished or withdrawn. When a task is finished, delete it from this
section in the same change that ships the work — the commit history is the
archive — and name the `T<n>` in the commit message.

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
