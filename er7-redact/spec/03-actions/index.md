[`er7-redact` specification](../index.md) — section 3 of 17. Section
numbers (§3.x) are stable and cited from code, tests, and commit messages.

# 3. Actions

Implemented in `src/action.rs`.

An action is what happens to a value the policy has selected. There are
eight built in, and no more are added to that closed set without a section
in [§16](../16-open-questions-and-declined-decisions/index.md) saying why
the existing eight were not enough. A ninth, [§3.8](#38-a-caller-supplied-action-d24),
is not part of that set — it is the caller's own code, not a new built-in
— and [§16.11](../16-open-questions-and-declined-decisions/index.md) is
the section that admitted it.

## 3.1 The eight built-in actions

| Action | Spelled | `PATID1234` becomes | Use for |
| ------ | ------- | ------------------- | ------- |
| `Keep` | `keep` | `PATID1234` | accepting a position, so the posture does not reject it ([§2.6](../02-redaction-model/index.md)) |
| `Clear` | `clear` | (empty) | anything whose presence is not wanted, e.g. an address |
| `Null` | `null` | `""` | telling a receiver to *clear its stored value*, not merely that none was sent |
| `Replace(text)` | `replace REDACTED` | `REDACTED` | a name, where an obvious placeholder reads better than a blank |
| `Mask(char)` | `mask *` | `*********` | a value whose length is wanted and whose content is not |
| `First(n)` | `first 4` | `PATI` | a birth date reduced to its year (`19610615` → `1961`) |
| `Last(n)` | `last 4` | `1234` | an account number reduced to the digits a human matches on |
| `Pseudonym` | `pseudonym` | `1f0b7a6d5c4e3b2a` | an identifier that must stay *linkable* across messages ([§7](../07-pseudonyms/index.md)) |

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

The distinction is the sharpest edge in HL7® (the `er7` spec §5.3), and
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
[§4.1](../04-what-redaction-preserves/index.md)).

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

This is a guarantee about the eight built-ins, provable because this crate
wrote all eight. It is not extended to
[§3.8](#38-a-caller-supplied-action-d24): a caller's own closure can do
anything, idempotent or not, and that is the caller's decision to make and
the caller's property to prove.

## 3.7 Zero counts

`first 0` and `last 0` are legal and equivalent to `clear`. They are not
rejected: a policy computed from a configuration table should not have to
special-case the boundary, and the result — no characters kept — is
unambiguous.

## 3.8 A caller-supplied action [D24]

`Action::Custom(CustomAction)` runs the caller's own function instead of
one of the eight built-ins — a real MAC, a lookup table keyed on
something outside the message, a per-patient date shift
([T5](../15-open-tasks/index.md)). [§16.11](../16-open-questions-and-declined-decisions/index.md)
is why it exists at all; this section is what it is.

```rust
pub struct CustomAction(/* private */);

impl CustomAction {
    pub fn new(f: impl Fn(&str, u64) -> Option<String> + Send + Sync + 'static) -> CustomAction;
}
```

`Action::custom(f)` is the usual way to reach it —
`Action::custom(|value, _key| Some(value.to_uppercase()))` — and is
shorthand for `Action::Custom(CustomAction::new(f))`.

**Same signature as [`Action::apply`](#32-actions-read-and-write-decoded-values),
because it runs through the same call.** The closure receives the leaf's
*decoded* text and the redactor's pseudonym key, and returns the
replacement the same way every other action does: `Some(text)` to write
`text`, `None` to leave the leaf as it is. Everything [§3.2](#32-actions-read-and-write-decoded-values)
says about decoded text and [D11](#35-replacement-text-cannot-corrupt-the-message-d11)'s
encoding-on-the-way-in applies unchanged — a custom action cannot break
the message any more than a built-in one can.

**`Action` keeps its ordinary `Debug`, `Clone`, `PartialEq`, and `Eq`.**
None of the four is meaningful for a bare closure, so `CustomAction`
carries hand-written versions instead of deriving them:

- `Debug` prints a fixed placeholder. There is nothing truthful to say
  about an opaque function.
- `Clone` clones the `Arc` — cheap, and every clone still runs the same
  closure.
- `PartialEq`/`Eq` compare **identity**, via `Arc::ptr_eq`: two
  `CustomAction`s are equal exactly when they wrap the same closure, never
  merely because two different closures happen to compute the same
  values. There is no general way to compare closures for behavioral
  equality, so identity is the only comparison that is not lying.

**No policy-file spelling, on purpose.** `Display` writes `<custom>` — not
a keyword in [§6.2](../06-policy-file-format/index.md)'s grammar, and not
readable back by `Action::parse`, which never produces a `Custom` action
under any input. A policy file is reviewed as text, and a caller-supplied
closure has no text to review; making `Custom` invisible to `parse` is
what keeps that promise rather than silently breaking it.
[§6.5](../06-policy-file-format/index.md) says what this means for a
policy that holds one.

**A worked example: date shifting.** `first 4` on a birth date keeps the
year and destroys the interval to the next event, which longitudinal test
data needs intact. `Action::custom` shifts every date in a message by a
per-patient offset instead, computed from the message's own patient
identifier before the policy for that message is built — a full,
independently tested worked example is
[`examples/date_shift_with_a_custom_action.rs`](../../examples/date_shift_with_a_custom_action.rs).
It is not a ninth built-in, and
[§16.12](../16-open-questions-and-declined-decisions/index.md) says why.

[`er7::Subcomponent::value`]: https://docs.rs/er7/latest/er7/message/struct.Subcomponent.html#method.value
[`er7::Subcomponent::set`]: https://docs.rs/er7/latest/er7/message/struct.Subcomponent.html#method.set

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
