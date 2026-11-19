[`er7-redact` specification](../index.md) — section 15 of 17. Section
numbers (§15.x) are stable and cited from code, tests, and commit messages.

# 15. Open tasks

The backlog. Task IDs are stable and **never reused**, even after a task
is finished or withdrawn. When a task is finished, delete it from this
section in the same change that ships the work — the commit history is the
archive — and name the `T<n>` in the commit message.

## T7. Characterize multilingual known-value matching

The [D23 sweep](../02-redaction-model/index.md) lowercases text and
checks adjacent characters for alphanumeric boundaries. Its tests use
ASCII names. A deployment needs to know which spelling differences
or adjacent characters can leave text in a note.

**Done when** synthetic cases cover composed and decomposed accents,
case mappings that expand to multiple scalars, a short CJK value, and
a value next to another letter without a space. Check both the edited
field and the note. Document each observed boundary in the policy guide.
Review any proposal to broaden matching separately, with examples of
the additional text it would remove.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
