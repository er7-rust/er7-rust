[`er7-redact` specification](index.md) — section 2 of 17. Section numbers (§2.x) are stable and cited from code, tests, and commit messages.

# 2. The redaction model

Implemented in `src/redact.rs` and `src/policy.rs`.

## 2.1 Four nouns

| Noun | Is | Type |
| ---- | -- | ---- |
| **action** | what to do to a value | `Action` ([§3](03-actions.md)) |
| **rule** | one HL7 path and one action | `Rule` |
| **policy** | an ordered list of rules, plus an optional fallback action | `Policy` |
| **redactor** | a policy, plus the key pseudonyms are derived from | `Redactor` |

A `Redactor` is the only thing that touches a message:

```rust
let mut message = er7::parse(text)?;
let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);
```

It edits in place and returns a [`Report`](08-report.md) of what changed.
Nothing else in the crate mutates a message.

## 2.2 What a rule names

A rule's target is an [`er7::Path`](https://docs.rs/er7/) — the same
notation the `er7` crate reads, specified by the `er7` spec §8.1, e.g.
`PID-5`, `PID-13[2]`, `OBX[2]-5.1`. This crate adds no notation of its own,
with the single exception of `*` in a policy file, which sets the fallback
([§6.3](06-policy-file-format.md)).

Path semantics come from `er7` unchanged, and two of them do most of the
work here:

- **An omitted occurrence index means every one** (`er7` R19). `OBX-5`
  names the fifth field of *every* `OBX` segment, and `PID-13` names
  *every* repetition of the field. This is what makes a short policy cover
  a long message.
- **A position the message does not have yields nothing** (`er7` R20). A
  rule for `PID-19` against a message with no `PID-19` does nothing, and
  that is not an error ([§2.5](#25-a-rule-that-matches-nothing)).

## 2.3 What a rule reaches

A path that stops above a subcomponent names **every leaf beneath it**.
`PID-5` on `SMITH^JOHN^Q` names three leaves, and an action that replaces
text replaces all three:

```
PID|1||9|4|SMITH^JOHN^Q      →   PID|1||9|4|REDACTED^REDACTED^REDACTED
```

The alternative — collapsing the field to one value — was declined; see
[§16.1](16-open-questions-and-declined-decisions.md). Replacing every leaf
is what keeps the component count intact, and the component count is what
downstream tools index by (D1).

`Action::Null` is the exception, because collapsing is exactly what an HL7
null *means*; see [§3.4](03-actions.md) and D6.

## 2.4 Rules apply in order [D7]

A policy's rules are applied first to last, each to the message as it
stands. Where two rules name the same position, both run, in order.

This is worth stating precisely because it has one consequence that
surprises people: `Action::Keep` **exempts a position from the fallback**
([§2.6](#26-the-fallback)); it does not undo an earlier rule. Once a value
has been replaced, it is gone — there is nothing to restore it from.

```
PID-5  replace REDACTED
PID-5  keep            # too late: PID-5 already reads REDACTED
```

The order that matters for the reverse case works as expected:

```
*      replace REDACTED   # fallback: everything not named below
PID-5  keep               # ...except the name, which is left alone
```

The fallback always runs last regardless of where the `*` line appears.

## 2.5 A rule that matches nothing [D8]

Not an error, and not a warning. A policy is written against a family of
messages, not one message: a policy covering `GT1` applied to a message
with no guarantor segment must simply do nothing, or every policy would
need a variant per message type.

The [report](08-report.md) is where this becomes visible: a rule that
matched nothing contributes no rows, so an empty report means nothing was
found to redact — which is either good news or a wrong policy, and the
crate does not presume to say which.

## 2.6 The fallback [D9]

A policy may carry a **fallback action**, which is applied to every leaf
that no rule named. This inverts the model from "redact what is listed" to
"redact everything except what is listed", which is the safer posture when
the message is unfamiliar:

```rust
let policy = Policy::new()
    .fallback(Action::redacted())
    .with("MSH", Action::Keep)?;
```

Rules and the fallback compose in one pass over the message:

1. Every rule runs, in order, recording each leaf position it named.
2. The fallback then runs over every leaf that appears in no rule's target
   — including leaves named by a `Keep` rule, which is what `Keep` is for.

The fallback never reaches the header's delimiter fields (D5,
[§4.4](04-what-redaction-preserves.md)), because a message whose `MSH-1`
reads `REDACTED` is not a message.

A fallback of `null` writes the explicit null into each leaf it covers,
rather than collapsing anything: the position a fallback names *is* the
leaf, so there is nothing above it to collapse to
([§3.4](03-actions.md)).

## 2.7 Determinism

Two runs of the same policy, with the same key, over the same message
produce byte-identical output. Nothing in the crate reads the clock, the
environment, or a random source, and `Pseudonym` is a pure function of the
key and the value ([§7](07-pseudonyms.md)). This is priority 4 in
[§1.5](01-purpose-and-scope.md), and it is what lets a redacted message be
committed to a repository and diffed.
