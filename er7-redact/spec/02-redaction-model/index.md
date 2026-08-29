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

## 2.9 What no rule names [D22]

`Redactor::uncovered` reports every leaf that carries text and is named by
no rule in the policy — the inverse of redaction: not what changed, but
what a rule never looked at.

```rust
pub fn uncovered(&self, message: &er7::Message) -> Vec<er7::Path>
```

It takes `&Message`, not `&mut Message` — a caller does not need to redact
anything, or even intend to, just to ask the question. Paths are fully
qualified and in message order, the same convention
[`Change::path`](../08-report/index.md) uses, so a row is a valid
`er7 --query` argument.

**Independent of posture.** This reports what no *rule* names, not what
the policy will eventually do to it. Under an accepting default
([§2.6](#26-the-two-postures-d9)), an uncovered position is left exactly
as it arrived when the message is redacted — this is the leak surface, and
what [§14.5](../14-roadmap/index.md) built this to show. Under a
rejecting default, an uncovered position is what the posture is about to
blank on the caller's behalf; reporting it there is not a defect, it is
telling the caller what the posture is silently doing.

**A leaf carries text** when it is neither empty nor the explicit null
(D3, D4) — matching exactly the test [§2.6](#26-the-two-postures-d9)'s
rejecting posture already applies before it acts. An empty or null leaf is
not a gap, because a rule would find nothing to redact there either way.

**Never mutates, and never sees redaction's own edits.** `uncovered`
computes which positions a rule would name by running the same matching
[§2.2](#22-what-a-rule-names) and [§2.3](#23-what-a-rule-reaches) logic
`redact` uses, against a disposable copy of the message, and discards the
copy. The positions it reports, and whether each one carries text, are
read from the message the caller passed in — untouched, whatever order
`uncovered` and `redact` are called in.

The default policy's own uncovered set is exactly the table in
[§5.4](../05-built-in-policies/index.md): free text, the sensitive but
non-identifying `PID` fields, organisational `MSH` fields, `Z` segments,
and set IDs. Running `uncovered` against a message under
`Policy::patient_identifiers()` reproduces that table from the code,
rather than asking a reader to trust that the two still agree.

## 2.10 Known values, wherever they appear [D23]

A positional policy has a structural blind spot: a patient name removed
from `PID-5` is still in the `NTE-3` that says "spoke with Mrs Everywoman
about the result" ([§14.2](../14-roadmap/index.md)). This is the second
pass that closes it, and it is on by default.

**What counts as a known value.** Every leaf a rule named, whose action
was not `keep`, and that carried text (D3, D4) — its **decoded** value,
before the rule's own action changed it. A rule's action is reused
exactly as found; `keep` never contributes one, because a `keep`'d
position is a declaration that the value there is not identifying at all
([§2.4](#24-rules-apply-in-order-d7-and-a-reject-beats-an-accept-d19)).
Collecting happens against the message as it arrived, so a `first 4` on
`PID-7` still contributes the full, undoctored date of birth as a known
value, not the four characters the rule kept.

Values shorter than three characters are never collected: a one- or
two-character value — a middle initial, a sex code — would match
constantly across ordinary clinical text, and the crate would spend its
credibility on it.

**Where the sweep looks.** Every leaf that carries text and that no rule
or posture already touched — the same set [`uncovered`](#29-what-no-rule-names-d22)
reports, computed the same way. A leaf a `keep` rule named is exempt from
the sweep for the same reason it is exempt from the posture: `keep` is
this policy's word for "not identifying," and the sweep does not
second-guess it.

**What a match does.** Case-insensitively, and only as a **whole word** —
bounded by a non-alphanumeric character or the edge of the leaf, so a
surname `Wood` matches standalone `Wood` and not the `Wood` inside
`Woodward`. Where a leaf's decoded text contains a known value, the
**whole leaf** gets that value's own action — the same one the named
position used, not a separate setting. This is the same granularity every
other action in the crate already works at
([§2.3](#23-what-a-rule-reaches)): nothing in this crate splices text out
of the middle of a value, so `NTE-3` loses the whole sentence, not just
the name inside it. A leaf that matches more than one known value takes
the action of whichever was collected first — rule order, then message
order, the same order [`Change`](../08-report/index.md) rows already
follow.

**One surprising consequence, written down rather than hidden:** when the
reused action is `pseudonym`, the swept leaf is hashed as its own whole
value, not as the matched substring. `PID-5`'s `SMITH` and `NTE-3`'s
"spoke with Mrs Smith about the result" do **not** get the same
pseudonym — the second input is the whole sentence, not the name inside
it. `Pseudonym`'s stability guarantee (D12) is about one value under one
key, not about text a sweep happened to find it inside.

**On by default, and a `Policy` field.** `Policy::search_known_values`
(`bool`) controls it, defaulting to `true` on every built-in policy and
every policy a file does not mention it in — the same "state it or get
the safer answer" convention [§2.8](#28-a-payload-that-is-not-er7-d21)
already uses for an unrecognised payload. `Policy::search_known_values`
is also the builder method that sets it. Appending one policy to another
can only turn it on, never off (D20's own logic extended to a third
field): `self.search_known_values || other.search_known_values`. The
policy file's `known-values` line
([§6.3](../06-policy-file-format/index.md)) is the only way to turn it
off for a whole policy.

**Reported like anything else.** A leaf the sweep redacts gets an
ordinary [`Change`](../08-report/index.md) row: its path, and the action
that ran. Nothing distinguishes a swept row from a named one, because the
report already carries no values (D13) — there is nothing about "how this
position was found" left to leak.

**Independent of, and invisible to, [`uncovered`](#29-what-no-rule-names-d22).**
`uncovered` reports what no *rule* names; the sweep redacts by *value*,
which `uncovered` has no way to anticipate — it would have to run the
sweep itself to know. A leaf the sweep will catch still appears in
`uncovered`'s output, and that is not a defect: `uncovered` is answering
"did a rule look here," and for that leaf the honest answer is still no.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
