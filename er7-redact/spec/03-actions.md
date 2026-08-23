[`er7-redact` specification](index.md) — section 3 of 17. Section numbers (§3.x) are stable and cited from code, tests, and commit messages.

# 3. Actions

Implemented in `src/action.rs`.

An action is what happens to a value the policy has selected. There are
eight, and no more are added without a section in
[§16](16-open-questions-and-declined-decisions.md) saying why the existing
eight were not enough.

## 3.1 The eight actions

| Action | Spelled | `PATID1234` becomes | Use for |
| ------ | ------- | ------------------- | ------- |
| `Keep` | `keep` | `PATID1234` | accepting a position, so the posture does not reject it ([§2.6](02-redaction-model.md)) |
| `Clear` | `clear` | (empty) | anything whose presence is not wanted, e.g. an address |
| `Null` | `null` | `""` | telling a receiver to *clear its stored value*, not merely that none was sent |
| `Replace(text)` | `replace REDACTED` | `REDACTED` | a name, where an obvious placeholder reads better than a blank |
| `Mask(char)` | `mask *` | `*********` | a value whose length is wanted and whose content is not |
| `First(n)` | `first 4` | `PATI` | a birth date reduced to its year (`19610615` → `1961`) |
| `Last(n)` | `last 4` | `1234` | an account number reduced to the digits a human matches on |
| `Pseudonym` | `pseudonym` | `1f0b7a6d5c4e3b2a` | an identifier that must stay *linkable* across messages ([§7](07-pseudonyms.md)) |

`Action::redacted()` is the shorthand for `Replace("REDACTED")`, and the
placeholder every built-in policy uses.

## 3.2 Actions read and write decoded values

An action operates on the leaf's **decoded** text — what the value means,
not how it was spelled ([`er7::Subcomponent::value`]). It is written back
through [`er7::Subcomponent::set`], which encodes anything that needs it.

Two consequences worth stating:

- `Mask` and `First`/`Last` count **characters** — Unicode scalar values —
  as a reader would. A name written `O\T\BRIEN` on the wire is nine
  characters (`O'BRIEN` is seven; the escape is one `&`), not eleven, so
  `first 3` gives `O'B` and not `O\T`.
- `Pseudonym` hashes the decoded value, so two senders that spell the same
  identifier differently — one escaping a character the other does not —
  still map to the same pseudonym.

## 3.3 `Clear` versus `Null`

The distinction is the sharpest edge in HL7 (the `er7` spec §5.3), and
choosing wrongly here is a patient-safety bug rather than a privacy one:

| Action | Writes | A receiver reads it as |
| ------ | ------ | ---------------------- |
| `Clear` | nothing | "the sender said nothing about this" — leave the stored value alone |
| `Null` | `""` | "the sender is clearing this" — **delete** the stored value |

`Clear` is almost always what redaction means: the redacted message is a
copy for testing, and it should not instruct anything to delete a record.
`Null` exists because a policy sometimes *is* meant to say "this system
must not hold this value", and saying it in HL7 requires the explicit null.

## 3.4 `Null` collapses [D6]

`Null` is the one action that changes the shape of a message, and it is
allowed to because the shape *is* the meaning: an HL7 null is a single
`""`, not a `""` in each component.

```
PID|1||9|4|SMITH^JOHN^Q      →   PID|1||9|4|""
```

The position the path names — a field, a repetition, a component, or a
subcomponent — is replaced by exactly one subcomponent whose text is `""`.
Everything beneath it is discarded. A path that names a whole segment
nulls each of that segment's fields.

Every other action leaves the shape untouched (D1,
[§4.1](04-what-redaction-preserves.md)).

## 3.5 Replacement text cannot corrupt the message [D11]

Whatever text an action writes goes through `er7::Subcomponent::set`, so a
delimiter inside it is escaped rather than written literally. A policy of
`PID-5 replace A|B^C` produces `A\F\B\S\C` on the wire — one value, which
reads back as `A|B^C` — and not a field break that would shift every value
after it.

This holds for `Mask` too: `mask |` is legal, and writes `\F\` per
character.

## 3.6 Idempotence [D10]

Applying a policy twice produces the same message as applying it once, for
every action except `Pseudonym`:

- `Keep`, `Clear`, `Null`, `Replace` — trivially, the second pass writes
  what is already there.
- `First(n)`, `Last(n)` — a value already `n` characters or shorter is
  unchanged.
- `Mask(c)` — a run of `c` masks to a run of the same length.
- `Pseudonym` — **not idempotent.** A pseudonym is a value like any other,
  so a second pass hashes the first pass's output and produces a different
  one. Redact once; if a message must be redacted again, redact the
  original.

## 3.7 Zero counts

`first 0` and `last 0` are legal and equivalent to `clear`. They are not
rejected: a policy computed from a configuration table should not have to
special-case the boundary, and the result — no characters kept — is
unambiguous.

[`er7::Subcomponent::value`]: https://docs.rs/er7/latest/er7/message/struct.Subcomponent.html#method.value
[`er7::Subcomponent::set`]: https://docs.rs/er7/latest/er7/message/struct.Subcomponent.html#method.set
