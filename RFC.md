# Request for comments

This project is asking for specific feedback, and this file says what
kind — what it wants to learn, why each question is open, and what an
answer would change.

It exists because of an awkward fact: **this is a one-maintainer project
with no production traffic of its own.** The crates are specified, tested,
and benchmarked, and none of that tells anyone whether they are *right* for
the messages real systems send. That gap cannot be closed by writing more
tests. It closes when somebody who runs an interface says what they saw.

The questions below are the real ones. Each is already recorded in a
specification section, and each has a decision attached that a good answer
would change. Workspace-level decisions that are waiting on the
maintainer rather than on outside feedback live in
[`plan.md`](plan.md)'s "Open decisions" section, not here.

- [How to answer](#how-to-answer)
- [What kind of feedback helps most](#what-kind-of-feedback-helps-most)
- [The questions](#the-questions)
- [What happens to your answer](#what-happens-to-your-answer)
- [Feedback we cannot act on](#feedback-we-cannot-act-on)
- [Decisions already made, and why](#decisions-already-made-and-why)
- [Status](#status)

## How to answer

| Route | Good for |
| ----- | -------- |
| [A GitHub issue](https://github.com/er7-rust/er7-rust/issues) | Anything you are willing to have in public, which is most things |
| Email <joel@joelparkerhenderson.com> | Anything you would rather not put in an issue tracker, including anything about your own environment |
| A pull request against the relevant `spec/` section | If you already know what the text should say |

**Never send a real message.** Redact it first — `er7-redact message.er7`
does exactly this, and `er7-redact --report message.er7` prints paths and
actions with no values at all. [`CONTRIBUTING.md`](CONTRIBUTING.md) leads
with this, and it applies here too.

You do not have to answer all of it, or any of it in order. One paragraph
about one question is a real contribution.

## What kind of feedback helps most

**Most useful, in order:**

1. **"Here is a message shape you get wrong."** Redacted, minimal,
   reproducible. This is the single most valuable thing anyone can send.
2. **"We evaluated this and chose something else, because X."** A lost
   evaluation with a reason is worth more than a won one without.
3. **"Your reasoning in §N is wrong, and here is why."** The specs argue
   for their decisions in prose precisely so the argument can be attacked.
4. **"We ran it against our traffic and nothing broke."** Boring, and
   currently unobtainable any other way.
5. **"This claim oversells."** Overclaiming in healthcare software is a
   defect and is treated as one.

**Least useful:** a feature request with no message behind it. Not because
requests are unwelcome, but because this project's failure mode is adding
surface nobody can verify — see
[`AI_STATEMENT.md`](AI_STATEMENT.md) §12.

Disagreement is welcome and does not need softening. If a decision is
wrong, the useful message says so.

## The questions

### Q1. Does the value tree match the messages you actually receive?

`er7` parses into a fixed hierarchy — message, segment, field, repetition,
component, subcomponent — and keeps **absent**, **empty**, and the explicit
**null** distinct, because collapsing them changes clinical meaning.

**What we want to know:** does anything your senders emit not fit that
shape? Nested repetitions, unusual depth, a segment whose structure the
outline gets visibly wrong.

**What an answer changes:** the value tree is the foundation everything
else sits on, so a genuine mismatch here is the most consequential thing
anyone could report. Spec:
[`er7` §5](er7/spec/05-value-tree/index.md).

### Q2. Are escape sequences decoded in the right places?

The standard scopes escaping to certain field types. `er7` decodes
sequences **wherever they appear**, because applying the standard's scope
needs a dictionary this crate deliberately does not have. This is a
recorded divergence, not an oversight.

**What we want to know:** has this ever bitten you — a value that
legitimately contained a backslash, in a field where escaping does not
apply, read as a sequence?

**What an answer changes:** the current position is "rare in practice, and
`Subcomponent::raw` is always available as an override". One real example
would move that from an argument to a measurement. Spec:
[`er7` §18.2](er7/spec/18-open-questions-and-divergences/index.md).

### Q3. Is the path syntax the one you would have guessed?

`PID-5.1`, `OBX[2]-5`, `PID-3.4.2`. Every label the command-line tool
prints is a valid query, so output pastes back in as input.

**What we want to know:** what did you type that did not work, and what did
you expect it to mean? Especially if you came from HAPI, Mirth, or another
tool with its own notation.

**What an answer changes:** path syntax is user-facing and cheap to extend
before 1.0.0, and expensive after. Spec:
[`er7` §8](er7/spec/08-paths-and-queries/index.md).

### Q4. What does the built-in redaction policy miss?

`er7-redact`'s default policy covers the patient identifiers in `PID`,
`NK1`, `PV1`, `GT1`, and `IN1`. It is a starting point and says so; it is
not a compliance certification and never will be.

**What we want to know:** what did you have to add? What did it redact that
you needed to keep? Did `--all-but-the-header` turn out to be the posture
you actually wanted?

**What an answer changes:** this is the crate where being wrong has the
worst consequences, and the built-in policy is the part that most needs
outside eyes. Spec:
[`er7-redact` §5.1](er7-redact/spec/05-built-in-policies/index.md).

### Q5. Should a pseudonym be cryptographic?

`Pseudonym` uses FNV-1a with a `u64` key, so it is a stable stand-in and
**not** a security primitive. A stronger hash was considered and declined:
it would strengthen the claim more than the deployment, because the key is
still a number in a config file next to the data it protects.

**What we want to know:** does that reasoning survive contact with your
threat model, or is your objection to it a real one?

**What an answer changes:** the decision is explicitly marked "revisit when
the key handling is solved". Spec:
[`er7-redact` §16.3](er7-redact/spec/16-open-questions-and-declined-decisions/index.md).

### Q6. Does "the same patient" need to mean more than "the same text"?

`Pseudonym` maps a *value*, not a patient. Two systems that write the same
identifier differently — a leading zero, a different assigning authority —
produce two different pseudonyms, and nothing in the crate notices.

**What we want to know:** is that a problem you have, and what would
correct behaviour look like given that normalising needs dictionary
knowledge this layer does not have?

**What an answer changes:** currently this is recorded only so nobody
concludes that linkage is guaranteed. Spec:
[`er7-redact` §16.6](er7-redact/spec/16-open-questions-and-declined-decisions/index.md).

### Q7. Is the zero-dependency stance load-bearing, or is it theatre?

`er7` has an empty `[dependencies]` table, enforced by a test. The
justification is that healthcare integration code gets audited and this
crate is meant to sit at the bottom of somebody's stack.

**What we want to know:** did it actually matter in a real review at your
organisation? Or did your process not care, and the cost — no timestamp
types, Serde in a separate crate — buy nothing?

**What an answer changes:** this constraint shapes every other decision in
the family. If it buys nothing in practice, that is worth knowing before
1.0.0 freezes the API around it. Spec:
[§1.1](spec/01-family-policy/index.md).

### Q8. Is the minimum Rust version window right?

Current stable minus three releases — roughly six months. Chosen because
hospital toolchains are approved on a cycle measured in quarters.

**What we want to know:** is six months enough for your organisation? Would
twelve change whether you could adopt this?

**What an answer changes:** the window is a policy, not a technical limit,
and it is the kind of thing that is set once by guesswork and never
revisited unless somebody says something. Spec:
[§2](spec/rust-msrv-n-minus-3/index.md).

### Q9. Are five licences useful, or just confusing?

MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, and GPL-3.0-only, at your
option. The theory is that a proprietary vendor and a public-sector project
can both adopt it without asking.

**What we want to know:** did the choice help, or did your legal review
treat an unusual expression as a reason to look at something else instead?

**What an answer changes:** this is reversible and nobody has ever reported
either outcome. Spec: [`LICENSE.md`](LICENSE.md).

### Q10. Is performance anywhere near your bottleneck?

A 402-segment lab result parses in about 260 µs on a laptop.

**What we want to know:** is that irrelevant to you because I/O dominates,
or do you have a workload where it matters — and if so, what shape is it?
Batch files in the hundreds of megabytes are the case we know about and
have not solved.

**What an answer changes:** a streaming reader for large batch files is on
the roadmap and unscheduled. Real demand would schedule it. Spec:
[`er7` §16.1](er7/spec/16-roadmap/index.md), and
[`BENCHMARKS.md`](BENCHMARKS.md).

### Q11. What is missing that stops you adopting this at all?

The crates deliberately do not validate, do not carry a typed segment
model, and do not speak any transport. That is scope discipline, not an
oversight — the layer above is a different project.

**What we want to know:** did one of those absences actually stop you? Did
you want them *here*, or were you content to get them from another crate?

**What an answer changes:** the answer is unlikely to be "add it to `er7`",
because the boundary is deliberate. It is likely to change the
documentation, which is currently better at saying what is out of scope
than at saying where to get it. Spec:
[`er7` §18.1](er7/spec/18-open-questions-and-divergences/index.md).

### Q12. Does the documentation earn trust or spend it?

There is a lot of it: specifications with numbered rules, a comparison that
names cases where you should choose something else, benchmarks with their
caveats, a stated bus factor of one, an AI-use disclosure.

**What we want to know:** did that read as rigour, or as a project
protesting too much? Which document did you actually use, and which did you
skip?

**What an answer changes:** documentation nobody reads is a maintenance
cost with no benefit, and it is the kind of thing a maintainer cannot judge
from inside.

### Q13. Would "redact what you know" catch what you actually find in free text?

The largest real gap in `er7-redact` is free text: a name in `NTE-3` or
`OBX-5` survives every positional rule. The design on the table is
**redact-what-you-know** — take the values found at identifier positions
and remove those strings wherever else they appear in the message. No
pattern matching (that was declined: it false-positives on lab values),
so it misses anything not present at a named position.

**What we want to know:** when you have found an identifier in free text
by hand, was it a value that also appeared in `PID` or another named
position — or something else entirely (a nickname, a relative, a phone
number never sent structurally)? The answer decides whether
redact-what-you-know closes most of the gap or a sliver of it.

**What an answer changes:** whether that design gets built as specified,
gets built with additions, or is not worth its cost. Spec:
[`er7-redact` §14.2](er7-redact/spec/14-roadmap/index.md), tracked as
[T1](er7-redact/spec/15-open-tasks/index.md).

### Q14. What does a defensible Safe Harbor statement need from a tool like this?

[`PHI.md`](PHI.md) now maps the default policy against the eighteen HIPAA
Safe Harbor identifier categories, honestly: categories 12–18 untouched,
dates materially incomplete (admission, discharge, and event timestamps
are not in the curated table), every category uncovered inside free text,
and pseudonyms flagged as derived codes that do not meet §164.514(c). The
determination itself stays with an accountable person — the tool cannot
make it.

**What we want to know:** if you have taken a data set through a Safe
Harbor or expert-determination review, what did the reviewer actually
ask for? Is a per-identifier coverage table the right shape, or did you
need something else — a per-message inventory of positions that carry
text and are named by no rule (the "redaction check" on the roadmap), a
dates story (`PV1-44/45`, `EVN`, date shifting), or a `Clear`-by-default
variant of the identifier policy for data leaving a trust boundary?

**What an answer changes:** whether the curated policy grows the missing
date rules, whether the
[redaction check (§14.5)](er7-redact/spec/14-roadmap/index.md) gets
scheduled, and what PHI.md's accounting should look like for a reviewer
rather than for a maintainer.

## What happens to your answer

1. **It gets recorded, whatever it is.** A question answered "no, and here
   is why" goes into that crate's open-questions section with the
   reasoning, so the next person finds it rather than re-asking.
2. **A change to behaviour starts in the spec.** Not in the code. If your
   report changes what a crate does, the specification section changes
   first, then the test, then the code.
3. **You get credit unless you ask not to.** Say if you would rather not be
   named, or would rather your employer not be.
4. **A "no" comes with reasoning.** If a suggestion is declined it is
   declined in writing, in the spec, where you can argue with it. How that
   decision gets made, and the three groups most declines fall into, is
   [`GOVERNANCE.md`](GOVERNANCE.md).

## Feedback we cannot act on

Said in advance so nobody wastes their time:

- **A real patient message.** It will be deleted, not used, and the report
  it came with may have to be deleted too. Redact first.
- **"Add HL7® FHIR® standard support."** Different standard, different
  project. See [`COMPARISONS.md`](COMPARISONS.md).
- **"Make it validate messages."** Deliberately out of scope at this layer;
  validation needs a version-specific dictionary, which is what the layer
  above owns.
- **"It should be faster than X."** Not without a benchmark that runs both
  fairly on the same machine. This project publishes no comparative
  figures for exactly that reason.
- **A vulnerability report, in public.** Email it instead, and read
  [`SECURITY.md`](SECURITY.md) first.
- **Anything that requires a response-time commitment.** There is one
  maintainer and no service level; [`MAINTAINERS.md`](MAINTAINERS.md) is
  blunt about what that means.

## Decisions already made, and why

Before proposing one of these, read the reasoning — it is written down
precisely so it can be argued with rather than repeated:

| Decision | Recorded at |
| -------- | ----------- |
| The crate stops at the encoding: no dictionary, no validation, no transport | [`er7` §18.1](er7/spec/18-open-questions-and-divergences/index.md) |
| Escape sequences are decoded in every field | [`er7` §18.2](er7/spec/18-open-questions-and-divergences/index.md) |
| Redaction never collapses a field to a single value | [`er7-redact` §16.1](er7-redact/spec/16-open-questions-and-declined-decisions/index.md) |
| No pattern matching for identifiers — it false-positives on lab values | [`er7-redact` §16.2](er7-redact/spec/16-open-questions-and-declined-decisions/index.md) |
| The postures are **accept** and **reject**, not allow and deny | [`er7-redact` §16.8](er7-redact/spec/16-open-questions-and-declined-decisions/index.md) |
| Serde lives in its own crate rather than behind a feature flag | [`serde-er7` §3](serde-er7/spec/03-dependencies-and-format-agnosticism/index.md) |
| No attribute-driven mapping from arbitrary structs to ER7 | [`serde-er7` §9.1](serde-er7/spec/09-roadmap-and-open-questions/index.md) |

A decision being recorded does not make it right. It makes it arguable.

## Status

| | |
|---|---|
| Version | 1.1.0 |
| Opened | 2026-08-26 |
| Revised | 2026-08-26 — Q13 and Q14 added, out of the free-text decision in [`plan.md`](plan.md) and the Safe Harbor accounting in [`PHI.md`](PHI.md). Question numbers are stable and never reused. |
| Closes | It does not. Questions are removed as they are answered, and added as they arise. |
| Answered so far | Nothing yet. This file is new. |

When a question here gets a real answer, it moves out of this file and into
the spec section it changed, and [`NEWS.md`](NEWS.md) says so.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
