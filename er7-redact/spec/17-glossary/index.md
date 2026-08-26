[`er7-redact` specification](../index.md) — section 17 of 17. Section
numbers (§17.x) are stable and cited from code, tests, and commit messages.

# 17. Glossary

Terms specific to this crate. For ER7's own vocabulary — segment, field,
repetition, component, subcomponent, escape sequence, explicit null — see
the `er7` spec §19, which this document does not restate.

| Term | Meaning |
| ---- | ------- |
| **action** | what to do to a selected value: one of the eight in [§3](../03-actions/index.md) |
| **rule** | one HL7® path and one action |
| **policy** | an ordered list of rules, plus a posture and what it does with an unrecognised payload |
| **posture** | which of the two things a policy does with a leaf no rule named: **accept** it, or **reject** it ([§2.6](../02-redaction-model/index.md)) |
| **accept by default** | the posture that leaves an unnamed leaf as it is — "redact these". Written `accept` |
| **reject by default** | the posture that applies an action to every unnamed leaf — "redact all but these". Written `reject ACTION` |
| **accept rule** | a rule whose action is `keep`: it exempts the position it names from the posture, and never restores a value ([§2.4](../02-redaction-model/index.md)) |
| **reject rule** | a rule with any other action. It beats an accept rule for the same leaf, whichever order they are in (D19) |
| **payload** | one chunk of input, as `er7::split_messages` returns it ([§2.8](../02-redaction-model/index.md)) |
| **unrecognised** | a payload that is not ER7, and so has no positions. A policy refuses it, passes it through, or acts on it whole ([§2.8](../02-redaction-model/index.md)) |
| **redactor** | a policy plus a pseudonym key; the only thing that edits a message |
| **report** | the list of positions a redaction changed, and what changed them ([§8](../08-report/index.md)) |
| **change** | one row of a report: a fully qualified path and an action |
| **leaf** | a subcomponent — the only place text lives, and so the only thing an action writes |
| **shape** | the counts at every level: how many segments, fields, repetitions, components, subcomponents. Preserved by D1 |
| **position** | one place in a message, named by a fully qualified path such as `PID[1]-5[1].1.1` |
| **de-identification** | removing enough that a person cannot reasonably be identified. A judgement about a data set, made by a person; not something this crate performs ([§1.3](../01-purpose-and-scope/index.md)) |
| **redaction** | removing named values from one message. What this crate does |
| **pseudonym** | a stable stand-in for an identifier, so that equal values stay equal across messages ([§7](../07-pseudonyms/index.md)) |
| **pseudonymisation** | replacing identifiers with pseudonyms. Reversible by whoever holds the key, and therefore **not** de-identification |
| **quasi-identifier** | a value that identifies nobody alone and identifies somebody in combination — a birth year, a postcode, a rare diagnosis ([§5.4](../05-built-in-policies/index.md)) |
| **key** | the `u64` a pseudonym is derived from. Not a secret in any managed sense ([§7.3](../07-pseudonyms/index.md)) |
| **linkage** | joining two records that concern the same patient. What pseudonyms preserve on purpose, and what an attacker uses |
| **free text** | a field holding prose rather than a coded value — `NTE-3`, `OBX-5` with a text value type. Where identifiers hide from positional rules |

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
