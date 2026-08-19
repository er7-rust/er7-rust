[er7-redact](../../index.md) → docs → usage

# Usage

A walk-through, from a message you cannot share to one you can. The
normative rules are in [`spec/`](../../spec/index.md); this page explains
and illustrates them.

## 1. The shortest useful thing

```rust
use er7_redact::{Policy, Redactor};

let mut message = er7::parse(text)?;
let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);

println!("{}", message.to_er7());
println!("{report}");
```

Three lines, and the two things you get back are the redacted message and
a record of what changed. `Redactor::default()` is the same policy with the
default pseudonym key, for when there is nothing to configure.

On the command line, the same thing:

```sh
er7-redact message.er7 > redacted.er7
er7-redact --report message.er7        # what would change, and nothing else
```

## 2. What just happened

The [built-in policy](../policies/index.md#the-built-in-policies) named
about forty positions across `PID`, `NK1`, `PV1`, `GT1`, and `IN1`, and
applied an action to each:

| Went in | Came out | Action |
| ------- | -------- | ------ |
| `PID-3.1` `PATID1234` | `1f0b7a6d5c4e3b2a` | `pseudonym` |
| `PID-5` `EVERYWOMAN^EVE^E` | `REDACTED^REDACTED^REDACTED` | `replace` |
| `PID-7` `19610615` | `1961` | `first 4` |
| `PID-11` `12 ELM ST^^BOSTON^MA^02101` | `^^^^` | `clear` |

What did **not** happen matters as much:

- the message still parses, and every field still counts to the same
  place — `PID-3.4` is still the assigning authority;
- `PID-8` (sex) and the `OBX` results are untouched, because they are not
  identifiers and they are usually the reason the message is being shared;
- nothing was added: a rule for a field the message does not carry did
  nothing at all.

## 3. Saying what you want

A policy is an ordered list of rules. Build one in Rust:

```rust
use er7_redact::{Action, Policy};

let policy = Policy::new()
    .with("PID-3.1", Action::Pseudonym)?
    .with("PID-5", Action::redacted())?
    .with("PID-7", Action::First(4))?
    .with("NTE-3", Action::Clear)?;
```

…or read one from a file, which is what a team reviews in a pull request:

```
PID-3.1  pseudonym    # keep linkage, lose the record number
PID-5    replace REDACTED
PID-7    first 4
NTE-3    clear        # free text, where identifiers hide
```

```rust
let policy = Policy::parse(&std::fs::read_to_string("de-identify.policy")?)?;
```

The paths are `er7`'s, so everything that notation does works here: `OBX-5`
covers every `OBX` in the message, `PID-13[2]` names one repetition, and
`PID-5.1` reaches one component.

The [eight actions](../../spec/03-actions.md) are `keep`, `clear`, `null`,
`replace`, `mask`, `first`, `last`, and `pseudonym`.

## 4. Starting from the built-in

To keep the crate's list and add to it, write it out and edit the file:

```sh
er7-redact --show-policy > de-identify.policy
$EDITOR de-identify.policy
er7-redact --policy de-identify.policy message.er7
```

This is also how a policy gets pinned: the built-in list may **grow** in a
minor release ([spec §13.3](../../spec/13-compatibility-and-versioning.md)),
so a repository that needs the exact same redaction next year should check
the file in.

In Rust, the same thing without the file:

```rust
let policy = Policy::patient_identifiers()
    .with("NTE-3", Action::Clear)?
    .with("OBX-5", Action::Clear)?;
```

## 5. When you do not trust the list

If the message is unfamiliar, invert the model: redact everything, and
name what to keep.

```rust
let policy = Policy::everything()   // MSH keep, then a fallback over the rest
    .with("OBX-2", Action::Keep)?
    .with("OBX-3", Action::Keep)?
    .with("OBX-5", Action::Keep)?;
```

```sh
er7-redact --all message.er7
```

This is the only way to cover a `Z` segment nobody has documented, and the
only honest answer to "is there anything else in there?".

## 6. Keeping the message joinable

Clearing an identifier destroys the message as test data: nothing ties the
patient here to the same patient in the next message. `pseudonym` replaces
the identifier with a stable stand-in instead:

```rust
let redactor = Redactor::new(Policy::patient_identifiers()).with_key(20260815);
```

Every message redacted with that key maps `PATID1234` to the same token.
Different keys produce unrelated mappings, so two data sets redacted under
different keys cannot be joined.

The cost is real and is worth reading before relying on it
([spec §7.3](../../spec/07-pseudonyms.md)): a pseudonym preserves equality
on purpose, and anyone who has the key can invert the mapping by trying
every candidate identifier. Use it inside your own trust boundary; for data
leaving it, `clear` or `replace`.

## 7. Checking the work

```rust
for change in &report.changes {
    println!("{} {}", change.path, change.action);
}
// PID[1]-3[1].1.1 pseudonym
// PID[1]-5[1].1.1 replace REDACTED
// PID[1]-7[1].1.1 first 4
```

Every path is fully qualified, so a row can be pasted straight into
`er7 --query` to see what is there now. And a report holds **no values** —
neither the old text nor the new — so it is safe to paste into a ticket.

An empty report means nothing was found to redact. That is either good news
or a wrong policy, and the crate does not presume to say which; read it
before sharing the message.

## 8. What this does not do

- It does not find an identifier in free text. `NTE-3` saying "spoke with
  Mrs Everywoman" survives every positional policy, and the only current
  answers are naming that position or using `--all`.
- It does not tell you the result is de-identified. That is a judgement
  about a whole data set, made by a person who is accountable for it.
- It does not go back. There is no mapping table and no undo.

## See also

- [`docs/policies/`](../policies/index.md) — the file format and the
  built-in tables in full
- [`docs/api/`](../api/index.md) — every public item
- [`docs/faq/`](../faq/index.md) — the questions this page raises
- [`examples/`](../../examples/README.md) — the same ground as runnable
  programs
