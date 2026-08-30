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

**Partly resolved by [§16.11](#1611-added-a-ninth-action-caller-supplied-d24):**
a caller who wants HMAC-SHA-256, keyed BLAKE3, or anything else can supply
it with `Action::custom` instead of waiting for this crate to pick one.
`Pseudonym` itself is unchanged, and still FNV-1a — that question, and the
key-in-a-config-file honesty problem above, are about what the *built-in*
should be, and `Action::custom` does not answer either. **Revisit the
built-in** when key handling is solved; the caller-side gap this section
first named is closed.

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

## 16.11 Added: a ninth action, caller-supplied [D24]

**Considered.** [§3.1](../03-actions/index.md) states the eight built-in
actions are closed, and names its own exception clause: a ninth is added
only with a section here saying why the eight were not enough. This is
that section.

**Why the eight were not enough.** A real MAC, a lookup table keyed on
something outside the message, and a date shift with a per-patient offset
([T5](../15-open-tasks/index.md)) all need to run caller code against the
decoded value — nothing built in can express "redact this the way *my*
function says to." [T2](../15-open-tasks/index.md)'s own "why" named all
three; none is buildable as a closed-enum variant.

**What it cost, and what it did not.** `Action` derives `Debug`, `Clone`,
`PartialEq`, and `Eq`, none of which a bare closure supports. The
new variant does not hold one directly — it holds `CustomAction`, a
newtype around `Arc<dyn Fn(&str, u64) -> Option<String> + Send + Sync>`
with its own hand-written `Debug` (a fixed placeholder; there is nothing
truthful to print about an opaque closure), `Clone` (an `Arc` clone, cheap,
already how [`Pseudonym`'s key](../07-pseudonyms/index.md) is threaded
through unrelated call sites), and `PartialEq`/`Eq` (identity — two
`CustomAction`s are equal exactly when they share the same `Arc`, never
when two closures merely compute the same thing, because there is no
general way to compare closures for behavioral equality and pretending
otherwise would be the wrong kind of convenient). `Action` itself keeps
its ordinary derives unchanged; only `CustomAction` carries the manual
impls. So the pessimistic framing this task was raised under — "the cost
is that `Action` stops being `Clone`, `PartialEq`, and `Display`-able in
the ordinary way" — did not hold: it stayed all three, at the cost of one
small wrapper type instead.

**`Display` and the policy file are a real, permanent boundary, not a
temporary gap.** A closure has no textual spelling, so `CustomAction`'s
`Display` writes a fixed placeholder (`<custom>`) that is not a `#[6.2](../06-policy-file-format/index.md)`
keyword and cannot be read back — `Action::parse` never produces a
`Custom`, on purpose, because policy files are reviewed as text and a line
that cannot be read is not a value a reviewer approved. A rule holding a
`Custom` action is Rust-API-only; [§6.5](../06-policy-file-format/index.md)
says what that means for round-tripping.

**Idempotence (D10) is not claimed for it.** The built-in eight are
idempotent except `Pseudonym`, and that exception is provable because this
crate wrote all eight. A caller's closure can do anything, including
something order-dependent; D10 stays a guarantee about the built-ins only.

## 16.12 Declined: a built-in date-shift action

**Considered.** [T5](../15-open-tasks/index.md) asked for a ninth action
that parses an HL7® timestamp, adds a per-patient offset derived from the
pseudonym key, and writes the result back at the same precision it was
given — because `first 4` on a birth date keeps the year and destroys
every interval to the next event, which is what longitudinal test data
needs.

**Declined as a built-in, resolved by [§3.8](../03-actions/index.md)
instead.** T5's own open questions turn out to be the same question asked
three ways, and `Action::custom` (D24) answers all three at once, for
free:

- *"Whether parsing an HL7 timestamp crosses the line §5.3 draws."*
  [§5.3](../05-built-in-policies/index.md) allows a table that is
  explicit and not inferred from a data dictionary; parsing timestamp
  *syntax* is closer to that than to knowing what a code *means*, but the
  question does not have to be settled, because the caller's own closure
  does the parsing — this crate never touches a timestamp either way.
- *"What happens to a timestamp the action cannot parse."* The caller's
  closure decides, the same way it decides everything else `Action::custom`
  runs: `None` to leave it, `Some(String::new())` to clear it.
- *"Where the per-patient key comes from when the policy is applied to
  one message at a time."* From the message's own patient identifier,
  read before the policy for *that* message is built, the same way a
  caller already reads it to pick a pseudonym key per run. There is
  nothing this crate could add here: it never sees "the patient" as a
  concept spanning positions, only one leaf and one key per call.

**What a built-in would have cost, and why that tips it.** Unlike D24
itself — additive, and the eight built-ins keep meaning exactly what they
meant — a date-shift built-in would be this crate's first `Action` whose
correctness depends on HL7 timestamp *format* knowledge (leap years,
variable precision, an optional degree-of-precision indicator on the wire
in later releases). That is exactly the kind of scope creep
[§16.2](#162-declined-pattern-matching) already declined pattern matching
over, for the same reason: the crate would own a piece of parsing logic
that has nothing to do with redaction and everything to do with getting
subtly wrong on a message shaped slightly differently than the one it was
tested against.

**Done when, reframed.** T5's own bar — "a message's dates shift
consistently, an unparseable timestamp has a documented outcome" — is met
by [`examples/date_shift_with_a_custom_action.rs`](../../examples/date_shift_with_a_custom_action.rs):
a full, tested worked example, including a from-scratch proleptic-Gregorian
calendar implementation (correct across two centuries, checked day by
day) and the explicit "same offset for the same patient, independently of
message order" property. It lives as an example, not in `src/`, precisely
because it is a demonstration of what `Action::custom` can do, not a
built-in this crate maintains.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
