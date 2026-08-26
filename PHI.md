# PHI, privacy, and what this software does with patient data

**Plain-language answers for a privacy officer, a security reviewer, or
anyone filling in a vendor questionnaire.** This page consolidates claims
that are made — and enforced — in the normative sources, and cites them
rather than restating them, so it cannot drift far from them: the
`er7-redact` specification (rules `D1`–`D21`, especially
[`§1.3`](er7-redact/spec/01-purpose-and-scope/index.md),
[`§5`](er7-redact/spec/05-built-in-policies/index.md), and
[`§7`](er7-redact/spec/07-pseudonyms/index.md)) and
[`SECURITY.md`](SECURITY.md)'s checkable-properties table.

**Status: no certification of any kind.** Nothing here is certified,
audited, or validated by HL7® International or anyone else, and these
crates are not a medical device.

## The short answers

| Question | Answer |
| --- | --- |
| Does this software send data anywhere? | **No.** Library code performs no I/O at all — no network, no filesystem, no environment reads. The CLIs read only the file you name (or stdin) and write only stdout or `-o`. [`SECURITY.md`](SECURITY.md) states this as a checkable property with the `grep` that checks it. |
| Does it phone home, or collect telemetry? | **No.** There is no such code in the repository. |
| Does it embed or call an AI model? | **No.** [`AI_STATEMENT.md`](AI_STATEMENT.md) §1: these crates ship no AI. AI is used to *build* the software. |
| Does it store PHI? | **No.** Nothing here persists anything. Messages exist in your process's memory for the duration of a call. |
| Can it remove patient detail from a message? | That is `er7-redact`'s purpose: remove or mask patient detail from HL7 v2 messages in the ER7 encoding without breaking the message ([`§1.1`](er7-redact/spec/01-purpose-and-scope/index.md)). Read the rest of this page before treating its output as de-identified. |
| Does its audit report leak what it redacted? | **No, by rule.** A report names paths and actions and never contains the text that was removed (`D13`). |
| Is redaction deterministic? | **Yes.** Same policy, key, and message produce the same output, byte for byte ([`§1.5`](er7-redact/spec/01-purpose-and-scope/index.md) priority 4) — usable as an audit trail. |
| Are the pseudonyms secure? | **No, and they do not claim to be.** FNV-1a, not a security primitive. See below. |
| Does redacting with the built-in policy make a message HIPAA-compliant? | **No.** See the Safe Harbor section below, which is most of why this page exists. |
| Is there real patient data in this repository? | **No, structurally.** Every sample is synthetic ([family policy §1.4](spec/01-family-policy/index.md)); a redacted real message still counts as real and is banned. |
| Who do I contact? | [`SECURITY.md`](SECURITY.md) — a redaction failure is treated as a vulnerability, not a bug. |

## The one framing that governs everything else

From [`er7-redact` §1.3](er7-redact/spec/01-purpose-and-scope/index.md),
and worth quoting because it is the sentence a privacy officer needs:

> whether what remains is de-identified under HIPAA, GDPR, or any other
> regime — that is a determination about a whole data set, made by a
> person who is accountable for it, not a property of one message

`er7-redact` is a **positional editor, not a compliance tool** (`D14`).
It removes the values you name, in the positions you name, and reports
what it did. The built-in policies are **a starting point, not a
compliance certification** (`D14`, [`§5.5`](er7-redact/spec/05-built-in-policies/index.md)) —
that framing appears in the spec, in [`SECURITY.md`](SECURITY.md), and
in the CLI's own `--help` text, deliberately.

## HIPAA Safe Harbor: an honest accounting

Safe Harbor (45 CFR §164.514(b)(2)) lists eighteen identifier categories
that must be removed, *and* requires no actual knowledge that what
remains could identify anyone. **Running `er7-redact` does not, by
itself, satisfy either half.** What follows maps the default policy,
[`Policy::patient_identifiers()`](er7-redact/spec/05-built-in-policies/index.md)
(§5.1), against those categories — honestly, which means the third
column matters more than the second.

**The structural limit first** ([`§5.4`](er7-redact/spec/05-built-in-policies/index.md),
[roadmap §14.2](er7-redact/spec/14-roadmap/index.md)): the policy is
positional. An identifier written into free text — `NTE-3`, `OBX-5`, any
narrative field — survives every positional rule, and the crate's own
roadmap calls free-text scanning "the largest real gap". Every row below
carries that asterisk: *covered at the named positions* never means
*covered wherever it appears*. The same applies to `Z` segments (local
extensions the crate cannot know) and to sender-specific field usage —
field usage varies by interface, and the spec tells you to outline a
real message before trusting any policy, including this one.

| # | Safe Harbor category | What `patient_identifiers()` does | What it does not do |
| --- | --- | --- | --- |
| 1 | Names | Replaces patient, alias, mother's maiden, next-of-kin, guarantor, insured, and clinician names at `PID-5/6/9`, `NK1-2`, `PV1-7/8/9/17`, `GT1-3/4`, `IN1-16` | Names in free text or any unnamed position stay |
| 2 | Geographic subdivisions smaller than a state | Clears addresses, county, birth place at `PID-11/12/23`, `NK1-4`, `GT1-5`, `IN1-19` | Facility names (`MSH-3`–`MSH-6`) are deliberately kept and can be quasi-identifiers |
| 3 | Dates (all elements except year) related to the individual | Truncates birth and death dates to the year (`first 4` on `PID-7`, `PID-29`, `GT1-8`, `IN1-18`) | **Admission, discharge, and event timestamps are not touched** (`PV1`, `MSH-7`, `EVN`, `OBX-14`, …); the ≥90 age aggregation rule is not implemented |
| 4 | Telephone numbers | Clears `PID-13/14`, `NK1-5/6`, `GT1-6/7` | Phone numbers anywhere else stay |
| 5 | Fax numbers | Only insofar as they sit in the cleared phone fields | No fax-specific rule exists |
| 6 | Email addresses | Only insofar as they sit inside the cleared `PID-13`/`PID-14` fields | No email-specific rule exists |
| 7 | Social security numbers | Clears `PID-19`, `GT1-12` | SSNs elsewhere stay |
| 8 | Medical record numbers | **Pseudonymizes** `PID-2.1/3.1/4.1` | A pseudonym is a *replacement*, not a removal — see the caveat below the table |
| 9 | Health plan beneficiary numbers | Pseudonymizes `IN1-36`, `IN1-49.1` | Same caveat |
| 10 | Account numbers | Pseudonymizes `PID-18.1`, `GT1-2.1`; also visit/preadmit numbers `PV1-5.1/19.1` | Same caveat |
| 11 | Certificate / license numbers | Clears the driver's licence at `PID-20` | No other certificate or license position is named |
| 12 | Vehicle identifiers | **Nothing** — no positional home in the curated table | |
| 13 | Device identifiers and serial numbers | **Nothing** | |
| 14 | URLs | **Nothing** | |
| 15 | IP addresses | **Nothing** | |
| 16 | Biometric identifiers | **Nothing** — these travel in `OBX` payloads the policy deliberately leaves | |
| 17 | Full-face photographs and comparable images | **Nothing** — as above | |
| 18 | Any other unique identifying number, characteristic, or code | **Structurally out of reach for a positional policy** — this category is open-ended, and rare diagnoses, timestamps, and small populations re-identify without any listed identifier ([`§5.5`](er7-redact/spec/05-built-in-policies/index.md)) | |

**The pseudonym caveat (categories 8–10).** Safe Harbor permits a
re-identification code only if it is *not derived from* information about
the individual (§164.514(c)). An `er7-redact` pseudonym **is** derived
from the identifier — FNV-1a over a `u64` key and the value — so it does
not meet that condition. For data leaving your trust boundary, the spec
itself says to use `Clear` or `Replace` instead ([`§7.3`](er7-redact/spec/07-pseudonyms/index.md)).

**The defensible summary**: with the default policy, categories 12–18
are untouched, category 3 is materially incomplete, and every category
is uncovered inside free text. The strict alternative,
[`Policy::all_but_the_header()`](er7-redact/spec/05-built-in-policies/index.md)
(§5.2), inverts the risk: reject by default, so nothing below `MSH`
survives unless you name it — at the cost of clinical meaning. Either
way, a Safe Harbor determination remains a judgement about your whole
data set, made by an accountable person, with this tool as one input.

## Pseudonyms, stated plainly

From [`§7`](er7-redact/spec/07-pseudonyms/index.md), because this is the
one place a caller can reasonably believe they have more protection than
they do (`D12`):

- The construction is **FNV-1a** over the key bytes then the value
  bytes. It is a hash, **not a MAC and not a security primitive**: not
  collision-resistant against an adversary, not slow enough to resist a
  dictionary attack, not a secret-keeping mechanism.
- A pseudonym leaks **equality** by design (that is its purpose), which
  enables frequency analysis; and medical record numbers come from small
  spaces, so anyone holding the key can enumerate candidates and invert
  the mapping completely. The key is a `u64` in a config file, not a
  managed secret.
- Therefore: pseudonyms are for data that **stays inside your own trust
  boundary** — test environments, internal reproductions, CI fixtures.
  For data leaving it, use `Clear` or `Replace`.
- Two spellings of the same identifier produce two pseudonyms: the
  function maps a value, not a patient.

## What to rely on, and where it is enforced

Every load-bearing claim above has a rule ID in
[`er7-redact`'s spec](er7-redact/spec/01-purpose-and-scope/index.md),
and §11.1 there maps each rule to the test that enforces it: shape
preservation (`D1`, `D2`), never inventing a value (`D3`, `D4`),
reports without redacted text (`D13`), reject-beats-accept (`D19`),
appending never weakens a posture (`D20`). A redacted message that still
contains a value its policy named for removal is a **vulnerability** under
[`SECURITY.md`](SECURITY.md), with a 90-day coordinated-disclosure
deadline — not a routine bug.

If a question is not answered here, in those specs, or in
[`SECURITY.md`](SECURITY.md), ask — an unanswered question is more
useful to this project than a guessed one.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
