[`er7-redact` specification](../index.md) — section 16 of 17. Section
numbers (§16.x) are stable and cited from code, tests, and commit messages.

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
structural fidelity ([§1.5](../01-purpose-and-scope/index.md)).

`Action::Null` collapses, because that is what an HL7® null means
([§3.4](../03-actions/index.md)), and it is the documented exception rather
than the rule.

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
else they appear — has none of these problems, and is shipped: D23,
[§2.10](../02-redaction-model/index.md).

## 16.3 Declined: a cryptographic pseudonym, for now

**Considered.** HMAC-SHA-256 or keyed BLAKE3 instead of FNV-1a
([§7.4](../07-pseudonyms/index.md)).

**Declined** because it needs a dependency (D16), because hand-rolling a
primitive would be worse than either alternative, and because it would
strengthen the *claim* more than the *deployment*: the key is still a `u64`
in a config file next to the data it protects. The honest posture is
[§7.3](../07-pseudonyms/index.md) — say what leaks, and say when not to use
it.

**Revisit when** the key handling is solved, or a caller-supplied action
([T2](../15-open-tasks/index.md)) makes it the caller's decision, which is
where it belongs.

## 16.4 Open: an action's argument cannot contain `#`

A policy line's `#` starts a comment, everywhere, including inside an
action's argument ([§6.1](../06-policy-file-format/index.md)). So
`PID-5 replace PATIENT #1` writes `PATIENT`, and `PID-13 mask #` is read
as a bare `mask`, which masks with `*`.

Quoting would fix it, and would add an escaping story to a format whose
whole appeal is that there is nothing to learn. Left as it is until
somebody wants a `#`; a policy built in Rust has no such limit.

## 16.5 Open: whether `Keep` should be able to undo

`Keep` exempts a position from the posture; it does not undo an earlier
rule (D7, [§2.4](../02-redaction-model/index.md)), because rules apply in
order to the message as it stands and there is nothing to restore from.
That is also what makes a reject beat an accept whichever order the two are
in (D19), which is the behaviour the model is now kept for.

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
([§5.3](../05-built-in-policies/index.md)). Recorded so that nobody
concludes from [§7.1](../07-pseudonyms/index.md) that linkage is
guaranteed: it is guaranteed for identical text, and only that.

## 16.7 Open: the report is per-run, not per-message-and-run

`Redactor::redact` takes one message and returns one report. A caller
redacting a batch gets a report per message and has to keep the pairing
straight themselves; the CLI does this with `# message N` headings
([§10.3](../10-command-line-interface/index.md)).

A `redact_all` over a slice would carry the pairing in the type. It has
not been added because the loop is two lines and the borrow is clearer
when the caller writes it. Revisit if the CLI's bookkeeping grows.

## 16.8 Declined: `allow` and `deny` for the two postures

The postures are **accept** and **reject**
([§2.6](../02-redaction-model/index.md)), not allow and deny.

Allow/deny is the older and more widely recognised pair, and the one an
access-control reader arrives with. It was declined because that is the
problem: allow/deny describes *who may reach a thing*, and this crate
decides *whether a value survives into the output*. A reader who maps
`deny` onto "blocked, and therefore safe" has the guarantee backwards —
a rejected value is rewritten, not withheld, and everything around it is
still emitted.

Accept/reject also matches what the actions do. A rejected leaf is not
refused; it is replaced, cleared, masked, or pseudonymised, and the report
says which ([§8](../08-report/index.md)).

The prose does not use allow/deny as synonyms either, including in
examples, so that a search for one word finds every place the concept
appears.

## 16.9 Open: an accept rule that could narrow a reject

D19 makes a reject rule win over an accept rule for the same leaf, at any
depth ([§2.4](../02-redaction-model/index.md)). There is no way to say
"reject the whole of `PID`, except `PID-5`" with rules alone: the accept
loses.

The way to express it today is the posture — reject by default, and accept
the positions a test needs — which is the same statement from the other
side, and is what [§5.2](../05-built-in-policies/index.md) does with `MSH`.

A specificity rule (the narrowest path wins) would make both spellings
work. It was not adopted with D19 because "narrowest wins" and "safest
wins" disagree exactly where a policy is already wrong, and the ordering in
[§1.5](../01-purpose-and-scope/index.md) says which of the two to prefer
when they do. Revisit if real policies turn out to need the carve-out more
often than they need the guard.

## 16.10 Open: a swept `pseudonym` hashes the whole leaf it lands on

D23's sweep ([§2.10](../02-redaction-model/index.md)) reuses the named
position's own action, whole-leaf, on whatever leaf it finds a known
value in. For every action except `pseudonym` this is harmless: `replace`
writes the same text everywhere, `clear` empties everywhere. `pseudonym`
is different, because what it hashes is *its own leaf's value*, not the
value that made the sweep fire. `PID-5`'s `SMITH` and an `NTE-3` reading
"spoke with Mrs Smith" produce two different pseudonyms, because the
second input to the hash is the whole sentence, not the name inside it —
`Pseudonym`'s stability guarantee (D12) was never a promise about text a
sweep happened to find a match inside.

The alternative — hash only the matched substring, and splice it back
into the leaf — is the same "surgical, not whole-leaf" shape declined for
every other action in [§2.3](../02-redaction-model/index.md) and D23
itself, for the same reason: it needs new text-splicing logic, and it
stops being true for `first`/`last`, which have no meaning applied to a
substring in the middle of a sentence. Left as it is, and written down,
so nobody reads two different pseudonyms for the same value and concludes
the crate has a bug rather than a documented boundary.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
