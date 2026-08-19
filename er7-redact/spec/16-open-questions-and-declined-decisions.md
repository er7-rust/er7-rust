[`er7-redact` specification](index.md) — section 16 of 17. Section numbers (§16.x) are stable and cited from code, tests, and commit messages.

# 16. Open questions and declined decisions

A recorded "no, and here is why" saves the next reader from
re-litigating it. A recorded question saves them from inheriting a silent
assumption.

## 16.1 Declined: collapsing a field to a single value

**Considered.** `PID-5 replace REDACTED` on `SMITH^JOHN^Q` producing
`PID|…|REDACTED` — one component, not three.

**Declined** because it changes the shape (D1). A test harness asserting
on `PID-5.2` finds nothing afterwards; a viewer lines the message up
differently; and a receiver that indexes components sees a different
message, not a redacted one. Repeating the placeholder per component is
uglier to read and correct to process, and this crate's second priority is
structural fidelity ([§1.5](01-purpose-and-scope.md)).

`Action::Null` collapses, because that is what an HL7 null means
([§3.4](03-actions.md)), and it is the documented exception rather than
the rule.

## 16.2 Declined: pattern matching

**Considered.** Recognising the shape of an SSN, a phone number, or a date
anywhere in the message, and redacting whatever matched.

**Declined for now** on two grounds. It needs either a dependency (D16) or
a hand-rolled matcher, and — the real objection — it false-positives on
clinical data. `123-45-6789` is an SSN; `1.23-45.6789` is a reference
range; an accession number is nine digits and so is a phone number without
its punctuation. A redactor that quietly destroys lab values is worse than
one that misses an identifier, because the damage is invisible until
somebody trusts the numbers.

The narrower version — remove values the policy *already found* wherever
else they appear — has none of these problems and is [T1](15-open-tasks.md).

## 16.3 Declined: a cryptographic pseudonym, for now

**Considered.** HMAC-SHA-256 or keyed BLAKE3 instead of FNV-1a
([§7.4](07-pseudonyms.md)).

**Declined** because it needs a dependency (D16), because hand-rolling a
primitive would be worse than either alternative, and because it would
strengthen the *claim* more than the *deployment*: the key is still a
`u64` in a config file next to the data it protects. The honest posture is
[§7.3](07-pseudonyms.md) — say what leaks, and say when not to use it.

**Revisit when** the key handling is solved, or a caller-supplied action
([T2](15-open-tasks.md)) makes it the caller's decision, which is where it
belongs.

## 16.4 Open: an action's argument cannot contain `#`

A policy line's `#` starts a comment, everywhere, including inside an
action's argument ([§6.1](06-policy-file-format.md)). So
`PID-5 replace PATIENT #1` writes `PATIENT`, and `PID-13 mask #` is read
as a bare `mask`, which masks with `*`.

Quoting would fix it, and would add an escaping story to a format whose
whole appeal is that there is nothing to learn. Left as it is until
somebody wants a `#`; a policy built in Rust has no such limit.

## 16.5 Open: whether `Keep` should be able to undo

`Keep` exempts a position from the fallback; it does not undo an earlier
rule (D7, [§2.4](02-redaction-model.md)), because rules apply in order to
the message as it stands and there is nothing to restore from.

An alternative model — decide every leaf's final action first, then apply
once — would make `Keep` undo, and would make a later `mask` apply to the
original rather than to the replacement. It is arguably more intuitive and
is certainly harder to explain when `Null` collapses a subtree that a
later rule names.

Left as it is, and written down, because the current model is the one a
reader gets right by reading the file top to bottom.

## 16.6 Open: what "the same patient" means across senders

`Pseudonym` maps a value, not a patient. Two systems that write the same
patient's identifier differently — with and without a leading zero, with a
different assigning authority in `PID-3.4` — produce two pseudonyms, and
nothing in the crate notices.

Normalising would require knowing which identifier namespace a value
belongs to, which is dictionary knowledge and version-specific
([§5.3](05-built-in-policies.md)). Recorded so that nobody concludes from
[§7.1](07-pseudonyms.md) that linkage is guaranteed: it is guaranteed for
identical text, and only that.

## 16.7 Open: the report is per-run, not per-message-and-run

`Redactor::redact` takes one message and returns one report. A caller
redacting a batch gets a report per message and has to keep the pairing
straight themselves; the CLI does this with `# message N` headings
([§10.3](10-command-line-interface.md)).

A `redact_all` over a slice would carry the pairing in the type. It has
not been added because the loop is two lines and the borrow is clearer
when the caller writes it. Revisit if the CLI's bookkeeping grows.
