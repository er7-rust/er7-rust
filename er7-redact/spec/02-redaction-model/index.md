[`er7-redact` specification](../index.md) — section 2 of 17. Section
numbers (§2.x) are stable and cited from code, tests, and commit messages.

# 2. The redaction model

Implemented in `src/redact.rs` and `src/policy.rs`.

## 2.1 Five nouns

| Noun | Is | Type |
| ---- | -- | ---- |
| **action** | what to do to a value | `Action` ([§3](../03-actions/index.md)) |
| **rule** | one HL7® path and one action | `Rule` |
| **policy** | an ordered list of rules, plus what it does by default | `Policy` |
| **posture** | what a policy does with a leaf no rule named: accept it or reject it | `Posture` ([§2.6](#26-the-two-postures-d9)) |
| **redactor** | a policy, plus the key pseudonyms are derived from | `Redactor` |

A `Redactor` is the only thing that touches a message:

```rust
let mut message = er7::parse(text)?;
let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);
```

It edits in place and returns a [`Report`](../08-report/index.md) of what
changed. Nothing else in the crate mutates a message.

## 2.2 What a rule names

A rule's target is an [`er7::Path`](https://docs.rs/er7/) — the same
notation the `er7` crate reads, specified by the `er7` spec §8.1, e.g.
`PID-5`, `PID-13[2]`, `OBX[2]-5.1`. This crate adds no notation of its own,
with the single exception of the `accept`, `reject`, and `unrecognised`
lines in a policy file, which set what the policy does by default rather
than naming a position ([§6.3](../06-policy-file-format/index.md)).

Path semantics come from `er7` unchanged, and two of them do most of the
work here:

- **An omitted occurrence index means every one** (`er7` R19). `OBX-5`
  names the fifth field of *every* `OBX` segment, and `PID-13` names
  *every* repetition of the field. This is what makes a short policy cover
  a long message.
- **A position the message does not have yields nothing** (`er7` R20). A
  rule for `PID-19` against a message with no `PID-19` does nothing, and
  that is not an error ([§2.5](#25-a-rule-that-matches-nothing-d8)).

## 2.3 What a rule reaches

A path that stops above a subcomponent names **every leaf beneath it**.
`PID-5` on `SMITH^JOHN^Q` names three leaves, and an action that replaces
text replaces all three:

```
PID|1||9|4|SMITH^JOHN^Q      →   PID|1||9|4|REDACTED^REDACTED^REDACTED
```

The alternative — collapsing the field to one value — was declined; see
[§16.1](../16-open-questions-and-declined-decisions/index.md). Replacing
every leaf is what keeps the component count intact, and the component
count is what downstream tools index by (D1).

`Action::Null` is the exception, because collapsing is exactly what an HL7
null *means*; see [§3.4](../03-actions/index.md) and D6.

## 2.4 Rules apply in order [D7], and a reject beats an accept [D19]

A policy's rules are applied first to last, each to the message as it
stands. Where two rules name the same position, both run, in order.

A rule whose action is `keep` **accepts** the position it names; a rule
with any other action **rejects** it. The two combine by one rule, and it
does not depend on the order they were written in:

**[D19] A reject rule beats an accept rule for the same leaf.** A leaf
named by both is a policy somebody got wrong — the same field accepted in
one place and rejected in another — and redacting it is the direction that
fails safely ([§1.5](../01-purpose-and-scope/index.md), priority 1). The
wrong answer is recoverable by editing the policy; the other one is not
recoverable at all.

```
PID-5  keep               # accept the name...
PID-5  replace REDACTED   # ...and reject it: REDACTED wins
```

```
PID-5  replace REDACTED   # reject the name...
PID-5  keep               # ...too late, and it would lose anyway
```

The same holds when the two rules are at different depths: a reject that
names a whole segment beats an accept that names one field inside it.

```
PID    replace REDACTED   # reject the segment...
PID-5  keep               # ...does not carve the name back out
```

This falls out of how the engine works rather than being enforced on top
of it, which is why it holds whatever the order: `keep` writes nothing, so
it can only ever *exempt a leaf from the posture* ([§2.6](#26-the-two-postures-d9)).
It never restores. Once a value has been replaced, there is nothing left to
restore it from.

An accept that names a whole segment is **not narrowed** by the posture:
`MSH keep` exempts every leaf of the header, including the ones the message
adds that the policy's author never saw. Only a reject rule (D19) reaches
into it.

## 2.5 A rule that matches nothing [D8]

Not an error, and not a warning. A policy is written against a family of
messages, not one message: a policy covering `GT1` applied to a message
with no guarantor segment must simply do nothing, or every policy would
need a variant per message type.

The [report](../08-report/index.md) is where this becomes visible: a rule
that matched nothing contributes no rows, so an empty report means nothing
was found to redact — which is either good news or a wrong policy, and the
crate does not presume to say which.

## 2.6 The two postures [D9]

Every policy has a **posture**: what it does with a leaf that no rule
named.

| Posture | A leaf no rule named | Written | Rust |
| ------- | -------------------- | ------- | ---- |
| **accept by default** | is left exactly as it is | `accept` | `Posture::Accept` |
| **reject by default** | gets this action | `reject clear` | `Posture::Reject(Action::Clear)` |

Accepting by default is "redact what is listed". Rejecting by default is
"redact everything except what is listed", which is the safer posture when
the message is unfamiliar — it is the only one that covers a `Z` segment
nobody has documented, or a field an interface started sending last week:

```rust
let policy = Policy::reject_all().with("OBX-2", Action::Keep)?;
```

There is no third posture and no way to leave it unstated. A policy is
built by naming one ([§5](../05-built-in-policies/index.md)), a policy file
writes one out ([§6.5](../06-policy-file-format/index.md)), and
`--show-policy` prints it, so "which of the two is this?" is always
answerable without reading code.

Rules and the posture compose in one pass over the message:

1. Every rule runs, in order, recording each leaf position it named.
2. Where the posture rejects, its action then runs over every leaf that
   appears in no rule's target — including leaves named by a `keep` rule,
   which is what `keep` is for.

Rejecting by default never reaches the header's delimiter fields (D5,
[§4.4](../04-what-redaction-preserves/index.md)), because a message whose
`MSH-1` reads `REDACTED` is not a message.

A posture of `reject null` writes the explicit null into each leaf it
covers, rather than collapsing anything: the position a posture names *is*
the leaf, so there is nothing above it to collapse to
([§3.4](../03-actions/index.md)).

### Appending never weakens the posture [D20]

Policies concatenate: several `--policy` files and `--rule` arguments make
one policy ([§10.2](../10-command-line-interface/index.md)), and
`Policy::append` is how. Rules are appended in order, and the posture is
combined by taking the **stricter** of the two:

| This policy | The appended one | Result |
| ----------- | ---------------- | ------ |
| accepts | accepts | accepts |
| accepts | rejects | rejects |
| rejects | accepts | **rejects** — the appended policy cannot turn it off |
| rejects | rejects | rejects, with the appended policy's action |

A policy file that says nothing about its posture accepts by default, and a
file of extra rules is exactly such a file. If appending adopted its
posture, adding a rule to a strict policy would silently switch redaction
off for everything the file did not name — a value that should have gone
and did not, which is the one failure
[§1.5](../01-purpose-and-scope/index.md) puts first. So it cannot happen by
accident: weakening a posture is done deliberately, with `Policy::posture`
or the CLI's `--accept-all`
([§10.2](../10-command-line-interface/index.md)).

What a policy does with an **unrecognised payload**
([§2.8](#28-a-payload-that-is-not-er7-d21)) is combined the other way: the
appended policy's disposition wins outright.

The difference is whether silence can be told from a decision. A file that
says nothing about its posture is identical to one that means "accept", so
its silence is not trusted. A file that says nothing about an unrecognised
payload is given `refuse` when it is read
([§6.1](../06-policy-file-format/index.md)) — the strictest disposition
there is — so every value one carries is somebody's decision, and a file
that went to the trouble of writing `unrecognised pass` is not overruled by
a default it never saw.

## 2.7 Determinism

Two runs of the same policy, with the same key, over the same message
produce byte-identical output. Nothing in the crate reads the clock, the
environment, or a random source, and `Pseudonym` is a pure function of the
key and the value ([§7](../07-pseudonyms/index.md)). This is priority 4 in
[§1.5](../01-purpose-and-scope/index.md), and it is what lets a redacted
message be committed to a repository and diffed.

## 2.8 A payload that is not ER7 [D21]

A **payload** is one chunk of input, as `er7::split_messages` returns it
(the `er7` spec §9). Usually it parses and is redacted. When it does not
parse — an MLLP frame, a JSON body, a truncated file, a stray log line in
a batch — there is no tree to walk, no position to name, and so nothing
the rules or the posture can say about it.

In practice only the **first** payload can be unrecognised.
`split_messages` cuts the input at each header, so anything following a
message belongs to that message: junk in the middle of a batch arrives as
a *segment* of the message before it, where the rules and the posture
reach it like anything else. This default covers input that never had a
header to begin with.

What happens to it is the policy's third default:

| Disposition | What is written | Rust |
| ----------- | --------------- | ---- |
| **refuse** | nothing: the caller is told the payload did not parse, and the run fails | `Unrecognised::Refuse` |
| **pass** | the payload, byte for byte | `Unrecognised::Pass` |
| **act on it whole** | the action applied to the payload as one value | `Unrecognised::Apply(Action::Mask('*'))` |

The default is the one that matches the policy's own claim about itself:

| Policy | Disposition | Why |
| ------ | ----------- | --- |
| `Policy::accept_all()` | pass | a policy named "accept all" that silently replaced a payload with `***` would be a surprise, and it redacts nothing else either |
| `Policy::reject_all()` | `mask *` | it rejects every value it can see; a payload it cannot see is not an exception, and masking it whole says "something was here" without saying what |
| `Policy::patient_identifiers()` | refuse | it is a *list of positions*, and it has no opinion about a payload with no positions in it. Refusing is fail-closed: nothing is written, and a person decides ([§10.4](../10-command-line-interface/index.md)) |
| `Policy::all_but_the_header()` | refuse | as above |

Any of them is overridden with `Policy::on_unrecognised`, or the
`unrecognised` line of a policy file
([§6.3](../06-policy-file-format/index.md)).

Refusing is not an error the *library* raises: `Redactor::redact` takes a
parsed message and cannot fail ([§9.2](../09-error-handling/index.md)), and
`Redactor::unrecognised` returns the text to write in the payload's place,
or `None` when the policy refuses it. Turning that `None` into a
diagnostic and a non-zero exit is the caller's job, and is what the CLI
does ([§10.4](../10-command-line-interface/index.md)).

Masking a payload whole preserves its length, and nothing else. That is the
same leak `Mask` always has ([§5.5](../05-built-in-policies/index.md)),
applied to a bigger value: use `clear` or `replace` where the size of what
was dropped is itself worth hiding.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
