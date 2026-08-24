[er7-redact](../../index.md) → docs → policies

# Policies

A reference for the policy file format and the four built-in policies. The
normative text is [spec §5](../../spec/05-built-in-policies/index.md) and
[§6](../../spec/06-policy-file-format/index.md); this page is the working
copy.

## The file format

One rule per line: a path, whitespace, an action.

```
# de-identify.policy

PID-3.1  pseudonym          # keep linkage, lose the record number
PID-5    replace REDACTED
PID-7    first 4            # a birth date reduced to its year
PID-11   clear
PID-19   null               # tell the receiver to clear its copy
NTE-3    clear
```

- Blank lines are ignored.
- `#` starts a comment, on its own line or after a rule. It also ends an
  action's argument, so replacement text cannot contain a `#`.
- Rules apply **in order**, and order is significant.
- Three reserved first words — `accept`, `reject`, and `unrecognised` —
  set what the policy does by default rather than naming a position.

## The actions

| Written | `PATID1234` becomes | Use for |
| ------- | ------------------- | ------- |
| `keep` | `PATID1234` | **accepting** a position, so a rejecting posture leaves it alone |
| `clear` | | an address, a phone number, anything a placeholder would not help |
| `null` | `""` | telling a receiver to **clear its stored value** |
| `replace` | `REDACTED` | shorthand for `replace REDACTED` |
| `replace NOT ON FILE` | `NOT ON FILE` | a name, where a placeholder reads better than a blank |
| `mask` | `*********` | shorthand for `mask *` |
| `mask #` | `#########` | a value whose length is wanted and whose content is not |
| `first 4` | `PATI` | a birth date reduced to its year |
| `last 4` | `1234` | an account number reduced to the digits a human matches on |
| `pseudonym` | `1f0b7a6d5c4e3b2a` | an identifier that must stay linkable |

Action names are case-insensitive; replacement text is taken as written.
`first 0` and `last 0` are legal, and mean the same as `clear`.

### `clear` versus `null`

| Action | Writes | A receiver reads it as |
| ------ | ------ | ---------------------- |
| `clear` | nothing | "the sender said nothing about this" — leave the stored value alone |
| `null` | `""` | "the sender is clearing this" — **delete** the stored value |

`clear` is almost always what redaction means. Use `null` only when the
policy really is meant to say "this system must not hold this value".

## Accept by default versus reject by default

Every policy is one of the two, and says which on its last lines.

**Accept by default** — redact what is listed, leave everything else:

```
PID-5  replace REDACTED
PID-7  first 4

accept
unrecognised  refuse
```

**Reject by default** — redact everything, and `keep` becomes the
interesting rule:

```
MSH    keep
OBX-2  keep
OBX-3  keep
OBX-5  keep

reject        replace REDACTED
unrecognised  mask *
```

The posture always runs last, wherever its line appears in the file. A
file that states no posture accepts by default; `reject keep` is legal and
means `accept`.

### A reject beats an accept

A `keep` rule **accepts** the position it names; a rule with any other
action **rejects** it. Where a position is named by both, the rejecting
rule wins — **whichever order they are in**, and at whatever depth:

```
PID    replace REDACTED   # rejecting the segment...
PID-5  keep               # ...does not carve the name back out
```

A field in both lists is a policy somebody got wrong, and redacting it is
the direction that fails safely: a value redacted by mistake costs a
policy edit, and a value left behind by mistake cannot be recalled.

Note that `keep` **exempts a position from the posture**; it does not undo
an earlier rule. Rules run in order against the message as it stands, so
once a value has been replaced there is nothing to restore it from.

A `keep` naming a whole segment is not narrowed by the posture: `MSH keep`
accepts every leaf of the header, including ones the policy's author never
saw. Only a reject rule reaches back into it.

### A payload that is not ER7

Input that does not parse has no positions in it, so no rule and no
posture can speak to it. The `unrecognised` line says what happens
instead:

| Written | Effect |
| ------- | ------ |
| `unrecognised refuse` | the run fails and writes nothing — the fail-closed default for a policy file, and for the CLI |
| `unrecognised pass` | the payload is written out byte for byte |
| `unrecognised mask *` | the payload is masked whole; any action works here |

Only the first payload of an input can be unrecognised: messages are split
at their headers, so junk after a message arrives as a segment of it.

## The built-in policies

### `patient_identifiers` — the default

About forty positions across five segments. The full table with a reason
per action is [spec §5.1](../../spec/05-built-in-policies/index.md); in
summary:

| Segment | Covers |
| ------- | ------ |
| `PID` | patient and account identifiers, names, birth and death dates, address, county, phones, SSN, driver's licence, mother's identifier, birth place |
| `NK1` | next of kin name, address, and phones |
| `PV1` | preadmit and visit numbers, attending, referring, consulting, and admitting doctors |
| `GT1` | guarantor number, names, address, phones, birth date, SSN |
| `IN1` | insured's name, birth date, address, policy number, identifier |

Identifier rules name the field's **first component** — `PID-3.1`, not
`PID-3` — so that the assigning authority and identifier type beside it
survive, and the message still looks like the interface it came from.

Write it out to a file to see it all, and to pin it:

```sh
er7-redact --show-policy > de-identify.policy
```

### `all_but_the_header` — the other posture

```
MSH  keep

reject        replace REDACTED
unrecognised  refuse
```

Everything below the header becomes `REDACTED`. Add `keep` rules for what
a test actually needs. This is the only thing that covers a local `Z`
segment, since no curated list can know what is in one.

### `accept_all` and `reject_all` — the bare postures

No rules, and no knowledge of HL7 at all. `accept_all` redacts nothing and
passes a payload it cannot parse through unchanged; `reject_all` redacts
everything it can reach — the header included, so the message stops being
routable — and masks such a payload whole.

They are the starting points, not the answers: `accept_all` is what you
build a policy on top of, and `reject_all` is `all_but_the_header` without
the one rule that keeps a message usable.

## What the default deliberately leaves alone

| Position | Why |
| -------- | --- |
| `NTE-3`, `OBX-5`, other free text | identifiers hide there constantly, and redacting them wholesale destroys the clinical content that makes a message worth sharing. Name them explicitly, or reject by default |
| `PID-8`, `PID-10`, `PID-15`–`PID-17`, `PID-22` | sensitive, but not identifiers, and usually the point of the test. Quasi-identifiers in combination: in a small population, redact them too |
| `MSH-3`–`MSH-6` | organisational rather than personal, and often what makes a message reproducible |
| `Z` segments | local, so no position means anything a list could know |
| set IDs (`PID-1`, `OBX-1`) | ordinal numbers, not data |

## The honest summary

A policy is a list of positions somebody wrote down. It removes the values
you name, in the positions you name, and reports what it did. Whether what
remains is safe to share is a judgement about your data set and its
recipients, and no library can make it for you.

## See also

- [`docs/usage/`](../usage/index.md) — the walk-through
- [`spec/05`](../../spec/05-built-in-policies/index.md),
  [`spec/06`](../../spec/06-policy-file-format/index.md) — the normative
  text
- [`samples/de-identify.policy`](../../samples/de-identify.policy) — a
  policy file that exercises every action
