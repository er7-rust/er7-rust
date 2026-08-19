[`er7-redact` specification](index.md) — section 5 of 17. Section numbers (§5.x) are stable and cited from code, tests, and commit messages.

# 5. Built-in policies

Implemented in `src/policy.rs`.

There are exactly two. **Both are a starting point, not a compliance
certification [D14]** — read [§5.5](#55-what-these-policies-do-not-do)
before relying on either.

## 5.1 `Policy::patient_identifiers()`

What the CLI uses when no policy is given, and what
`Redactor::default()` applies. It names the positions that carry a
**patient identifier** in the five segments that carry them in every HL7
v2 release from 2.3 to 2.9.

An identifier rule names the field's **first component** — `PID-3.1`, not
`PID-3` — because that component is the ID number itself. The assigning
authority and identifier type beside it (`^^^ADT1^MR`) describe the
interface rather than the patient, and a message that keeps them still
looks like the one it came from, which is most of what makes it useful as
test data. Everything else names the whole field.

There is deliberately no `Policy::default()`. An empty default would
silently redact nothing and a curated one would silently redact forty
positions; a redaction crate can afford neither surprise, so a caller
names the policy they mean.

### PID — patient identification

| Path | Field | Action | Why this action |
| ---- | ----- | ------ | --------------- |
| `PID-2.1` | Patient ID | `pseudonym` | an identifier; linkage across messages is usually wanted |
| `PID-3.1` | Patient identifier list | `pseudonym` | the identifier everything else joins on |
| `PID-4.1` | Alternate patient ID | `pseudonym` | as `PID-3.1` |
| `PID-5` | Patient name | `replace REDACTED` | a blank name reads as a parsing bug; a placeholder does not |
| `PID-6` | Mother's maiden name | `replace REDACTED` | a classic knowledge-based authenticator |
| `PID-7` | Date of birth | `first 4` | the year is usually what a test needs; the day is what identifies |
| `PID-9` | Patient alias | `replace REDACTED` | as `PID-5` |
| `PID-11` | Patient address | `clear` | no placeholder is meaningful for an address |
| `PID-12` | County code | `clear` | geography narrows a population fast |
| `PID-13` | Home phone | `clear` | directly contactable |
| `PID-14` | Business phone | `clear` | as `PID-13` |
| `PID-18.1` | Patient account number | `pseudonym` | an identifier, and often a join key |
| `PID-19` | Social security number | `clear` | the single most re-identifying value in the message |
| `PID-20` | Driver's licence number | `clear` | as `PID-19` |
| `PID-21.1` | Mother's identifier | `pseudonym` | identifies a second person |
| `PID-23` | Birth place | `clear` | as `PID-12` |
| `PID-29` | Death date and time | `first 4` | as `PID-7` |

### NK1 — next of kin

| Path | Field | Action |
| ---- | ----- | ------ |
| `NK1-2` | Name | `replace REDACTED` |
| `NK1-4` | Address | `clear` |
| `NK1-5` | Phone number | `clear` |
| `NK1-6` | Business phone number | `clear` |

### PV1 — patient visit

| Path | Field | Action |
| ---- | ----- | ------ |
| `PV1-5.1` | Preadmit number | `pseudonym` |
| `PV1-7` | Attending doctor | `replace REDACTED` |
| `PV1-8` | Referring doctor | `replace REDACTED` |
| `PV1-9` | Consulting doctor | `replace REDACTED` |
| `PV1-17` | Admitting doctor | `replace REDACTED` |
| `PV1-19.1` | Visit number | `pseudonym` |

Clinician names are not patient identifiers, but a small enough care team
identifies a patient as surely as a name does, and they are the values
most often objected to in a shared message.

### GT1 — guarantor

| Path | Field | Action |
| ---- | ----- | ------ |
| `GT1-2.1` | Guarantor number | `pseudonym` |
| `GT1-3` | Guarantor name | `replace REDACTED` |
| `GT1-4` | Guarantor spouse name | `replace REDACTED` |
| `GT1-5` | Guarantor address | `clear` |
| `GT1-6` | Guarantor home phone | `clear` |
| `GT1-7` | Guarantor business phone | `clear` |
| `GT1-8` | Guarantor date of birth | `first 4` |
| `GT1-12` | Guarantor SSN | `clear` |

### IN1 — insurance

| Path | Field | Action |
| ---- | ----- | ------ |
| `IN1-16` | Name of insured | `replace REDACTED` |
| `IN1-18` | Insured's date of birth | `first 4` |
| `IN1-19` | Insured's address | `clear` |
| `IN1-36` | Policy number | `pseudonym` |
| `IN1-49.1` | Insured's ID number | `pseudonym` |

## 5.2 `Policy::everything()`

The other posture: a fallback of `replace REDACTED` over every leaf, with
one `keep` rule for the whole `MSH` segment so the message stays routable
and identifiable.

```
MSH  keep
*    replace REDACTED
```

Use it when the message is unfamiliar, when it is full of `Z` segments
nobody has documented, or when the answer to "is there anything else in
here?" has to be "no" rather than "not that I listed". The cost is that
the result is no longer clinically meaningful: every code, every value,
and every timestamp below `MSH` reads `REDACTED`.

Add `keep` rules for the positions a test actually needs:

```rust
let policy = Policy::everything()
    .with("OBX-2", Action::Keep)?   // value type
    .with("OBX-3", Action::Keep)?;  // observation identifier
```

## 5.3 Why this crate may hold a field table at all

`er7` refuses to know what `PID-5` means (its R24), and that refusal is
the reason it is trustworthy at the bottom of a stack. This crate is a
layer above, and knowing which positions carry patient detail is precisely
the capability it exists to add. The table is:

- **explicit** — written out above, not inferred from a data dictionary;
- **narrow** — five segments, chosen because their identifier fields have
  not moved since v2.3;
- **overridable** — every built-in policy is an ordinary `Policy` value
  that a caller can extend, edit, or ignore;
- **not authoritative** — see below.

## 5.4 What the default policy deliberately does not touch

| Position | Why not |
| -------- | ------- |
| `NTE-3`, `OBX-5` and other free text | identifiers hide in free text constantly, and no positional rule can find them. Redacting them wholesale would destroy the clinical content that makes a test message useful. Redact these explicitly, or use `Policy::everything()` |
| `PID-8` (sex), `PID-10` (race), `PID-15` (language), `PID-16` (marital status), `PID-17` (religion), `PID-22` (ethnic group) | sensitive, but not identifiers, and the values a clinical test is usually about. They are also quasi-identifiers in combination: in a small population, redact them too |
| `MSH-3` to `MSH-6` (sending and receiving application and facility) | organisational, not personal — and a facility name is often what makes the message reproducible. It is also, in a small enough system, a quasi-identifier |
| `Z` segments | local extensions, so no position means anything the crate could know. `Policy::everything()` is the answer here |
| `PID-1`, `OBX-1` and other set IDs | ordinal numbers, not data |

## 5.5 What these policies do not do [D14]

A built-in policy is a list of positions someone wrote down. It is not:

- **a compliance determination.** Whether a data set is de-identified is a
  judgement about the whole set, its recipients, and what else they hold —
  not a property of one message, and not something a library can assert;
- **a match for your senders.** Field usage varies by interface. Run the
  `er7` CLI's outline over a real message and read what is actually in it
  before trusting any policy, including this one;
- **proof against inference.** A rare diagnosis, a timestamp, and a
  facility identify a patient with no name at all;
- **leak-free by construction.** `Mask` preserves length. `First(4)` on a
  birth date preserves the year. `Pseudonym` preserves *equality*, which
  is the whole point of it and is also what makes a frequency analysis
  possible ([§7.3](07-pseudonyms.md)).

The honest summary: this crate removes the values you name, in the
positions you name, and reports what it did. Everything else is yours.
