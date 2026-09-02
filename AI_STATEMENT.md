# AI statement

| | |
|---|---|
| Version | 1.8.0 |
| Effective date | 2026-09-02 |
| Status | Active |
| Author and owner | Joel Parker Henderson, maintainer |
| Canonical location | `AI_STATEMENT.md` at the workspace root |
| License | The same five-way choice as the rest of the project — [`LICENSE.md`](LICENSE.md) |
| Review | At every release that changes the practice described here, and on any trigger in §13 |

**Abstract.** This document discloses how artificial-intelligence tools are
used to develop ER7 Rust, an open-source Cargo workspace of Rust crates for
HL7® v2 messages in the ER7 pipe-hat encoding. It states what the tools do
and do not touch, who is accountable, which controls bound the work and how
each is enforced, the licensing and data posture, the rules for
contributors, the uses that are prohibited, and the limitations that
survive all of it. It is a self-declaration by the maintainer, written for
evaluators and regulated adopters performing supplier due diligence, and it
changes in the same commit that changes the practice it describes.

The key words **shall**, **should**, and **may** are used as ISO/IEC
Directives Part 2 defines them: requirement, recommendation, permission.

## 1. Scope

This document covers the use of AI tools in developing everything in this
workspace: the code of all three published crates, the benchmarks, the fuzz
targets, the tests, the website under `er7-rust.github.io/`, the
specifications under each crate's `spec/` and under the workspace
[`spec/`](spec/), and this document itself.

It does not cover an AI system in the product, because there is none:
**these crates ship no AI.** No model is trained, embedded, or called at
run time. Nothing here performs inference, and nothing in library code
reaches a network at all. AI is used to *build* the software, in the same
sense a compiler and a linter are used to build it.

## 2. Which frameworks apply, and which do not

Stated plainly, because borrowed authority is worse than none.

- **The EU AI Act imposes no obligation on this project.** The Act binds
  providers and deployers of AI *systems*. This workspace is not one: it
  ships no model and performs no inference. Content-marking duties bind an
  AI tool's provider, not the tool's user. This document is voluntary.
- **These crates are not a medical device.** They are parsing, editing, and
  encoding libraries with no clinical purpose and no clinical claim. A
  downstream integrator who gives *their* product a medical purpose may
  bring that product into scope; that classification is theirs to make, and
  this document exists partly so they can answer their own supplier
  questions.
- **ISO/IEC 42001 and the NIST AI Risk Management Framework are used for
  vocabulary only.** Neither is claimed as conformity.
- **No standard is claimed as conformity, and no certification exists.**
  No audit has occurred. The words "certified", "audited", and "validated"
  appear in this document only in this sentence, to say they do not apply.

## 3. Terms

This document reuses the W3C AI Content Disclosure vocabulary rather than
inventing one: **none** (entirely human-authored), **ai-assisted**
(human-authored; AI edited, refined, or filled in boilerplate),
**ai-generated** (AI-generated with human prompting and review),
**autonomous** (AI-generated without meaningful human oversight). An
**agentic tool** is one that plans and executes multi-step work — editing
files, running builds and tests — under a human's direction, as opposed to
inline completion.

## 4. Accountability

One named human — the maintainer, listed in
[`MAINTAINERS.md`](MAINTAINERS.md) — is the author of and accountable for
every change in this workspace, whatever tool produced the bytes. A tool
**shall not** be named as the author of, or a signer of, anything here,
because a tool cannot be responsible for accuracy, integrity, or
originality, and responsibility that cannot be borne cannot be assigned.
There is no AI-issued sign-off of any kind.

The commit trailers described in §10 record *participation*, not
authorship. The `Author:` field of every commit in this history is the
maintainer.

## 5. Where AI is used, and at what level

The tooling is agentic AI coding assistance — Claude Code, by Anthropic —
in sessions the maintainer directs and reviews. The repository carries
`AGENTS.md` at the workspace root and in each crate, with a `CLAUDE.md`
beside it pointing at it: those files are the standing instructions given
to the tools, they are committed, and they are readable by anyone
evaluating this claim.

Levels below use the §3 vocabulary. Deliberately, no percentage appears
anywhere in this document: no defensible method exists for measuring one.

| Activity | Level | Notes |
|---|---|---|
| Crate code | ai-generated | Written in directed sessions against the HL7 v2 standard and each crate's own spec; reviewed and committed by the maintainer |
| Tests, fuzz targets, benchmarks | ai-generated | Held to the same authority as the code they exercise; §7 governs what happens when one fails |
| The `spec/` documents | ai-generated | The normative layer. A rule in a spec is only there because a test backs it |
| Documentation, the website, and this statement | ai-generated | Held to the repository's own prose rules |
| What the crates do and do not do — scope, non-goals, what a release claims | none | Decided by the maintainer. This is the one release-related row that stayed `none` through both changes below: an agent scopes nothing on its own |
| Whether a specific, already-scoped release is ready to publish | ai-assisted | As of 2026-09-02, an agent may judge this against the project's own stated readiness criteria — the four checks, spec/code/test agreement, correct SemVer classification, a clean `cargo package --list` — once the maintainer has scoped and named the release. See [`GOVERNANCE.md`'s Release authority](GOVERNANCE.md#release-authority) |
| Running `cargo publish` for a release already decided ready | ai-assisted | As of 2026-09-02, an agent may run the command itself. Same section |
| Accepting a contribution from someone else | none | Prohibited use; see §11 |

**autonomous** appears in no row, and that is the point of the next
section.

## 6. Human oversight

The maintainer directs the work, reads the result, and commits every
change; nothing lands on its own authority, and no commit or release is
automated. Where the tools run multi-step sessions, the decisions with
consequences — what a specification rule means, whether an incompleteness
is acceptable, what a released version claims — are the maintainer's. A
decision that exists only inside a tool session is not a decision this
project made.

**Publishing has three parts: scoping a release, judging it ready, and
running the command — and only the first is unconditionally the
maintainer's.** Before 2026-09-02, all three were his alone. That day, the
keystroke moved: he could direct an agent to run `cargo publish` for a
release he had already decided on and named, but the readiness judgment
stayed his. Later the same day, the readiness judgment moved too: given a
release the maintainer has already scoped and named — what changes, what
version — an agent may work through this project's own stated readiness
criteria (the four checks, spec/code/test agreement, correct SemVer
classification, a clean `cargo package --list`) and decide the release
meets them, then run `cargo publish` for it, without a further per-release
checkpoint from the maintainer.

What did not move, either time: an agent never decides that a crate
should release *at all* — scoping and naming a release is the maintainer's
alone, unconditionally — never decides what a released version *claims*,
and never publishes as a standing, unattended job or on its own
initiative outside a session the maintainer is directing. See the
prohibition in §11 and [`GOVERNANCE.md`'s Release authority](GOVERNANCE.md#release-authority)
for the rule itself.

## 7. Quality controls, and what each one proves

AI-produced work is not a shortcut around engineering process. Every
change, whoever or whatever wrote it, passes the same gates
([§1.2 of the family policy](spec/01-family-policy/index.md)):

```sh
cargo test --workspace                            # unit, integration, and doc tests
cargo clippy --workspace --all-targets -- -D warnings   # pedantic lints, denied
cargo fmt --check                                 # formatting
cargo rustdoc -p er7 --lib -- -W missing-docs     # every public item documented
cargo +1.96 check --workspace --all-targets       # the MSRV floor
```

- **Spec authority.** Each crate's `spec/index.md` is the single source of
  truth for its behaviour, numbered so it can be cited (`R<n>` for `er7`,
  `D<n>` for `er7-redact`, `S<n>` for `serde-er7`). Every rule is backed by
  a test, and the coverage table that says so is itself enforced by a test.
  A change to behaviour goes in the spec first, and a code change that
  contradicts the spec is a bug in one of the two. This is the control that
  catches a plausible-but-wrong implementation regardless of who wrote it,
  because a tool cannot quietly redefine what correct means.
- **The round-trip assertion.** A message parsed and not modified must
  render back byte for byte. It is the single property that catches most of
  what a confident-looking parser gets wrong, and it is a test rather than
  an assumption.
- **The empty dependency table, enforced by a test.**
  `the_crate_has_no_runtime_dependencies` fails if anything is added to
  `er7`'s `[dependencies]`. A supply-chain rule that only lives in prose is
  a rule a future session can talk itself out of.
- **Fuzz targets** — `parse_roundtrip`, `escape_roundtrip`, `query_paths`
  in [`er7/fuzz/`](er7/fuzz/) — on the surface that takes untrusted input.
- **Benchmarks with a published method and machine**
  ([`BENCHMARKS.md`](BENCHMARKS.md)), so a performance claim is a
  measurement rather than an impression.
- **Tests and expectations shall not be weakened to make a build pass.**
  That is a standing hard rule, for humans and tools alike.

What these controls do **not** prove is §12. A workflow
([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs these gates
on every push and pull request; its first hosted run went green
2026-08-26, and as of 2026-09-02 it has run 44 times, 43 green
(`gh run list -R er7-rust/er7-rust --workflow=ci.yml --limit 200 --json
conclusion`) — a real, growing track record, not a single proof-of-concept
run. The one failure (`61eb30d`) was a flaky CLI-test race, root-caused and
fixed the same day (`574daaf`), not swept aside. [`SECURITY.md`](SECURITY.md#security-relevant-design-decisions)
carries the same figure in its checkable-properties table — corrected here
to a cross-reference that actually holds it, in place of a prior version's
"`MAINTAINERS.md` says the same thing," which by the time this was
rechecked, `MAINTAINERS.md` no longer did.

## 8. Licensing and provenance of AI output

The project is multi-licensed — MIT, Apache-2.0, BSD-3-Clause,
GPL-2.0-only, or GPL-3.0-only, at the user's option. The position taken
here follows the Apache Software Foundation's and LLVM's published
reasoning rather than wishful shortcuts: an AI tool's output does not
launder anyone's copyright, the full provenance of generated text is
generally not knowable, and prompting alone is not treated as authorship.

In practice: contributions of substantially copied third-party material are
refused however they were produced; generated code is held to the same
originality expectations as human code, under the same review; and if
identifiable third-party material is found in the tree, it is removed or
licensed properly, exactly as it would be for a human-introduced copy. The
tools are used under terms that do not restrict the output's use under
these licenses.

The HL7 standards themselves are not this project's to license, and are not
reproduced here: the crates implement an encoding described by the
standard, and [`LICENSE.md`](LICENSE.md) covers this project's own work
only.

## 9. Data

**No patient data, no personally identifiable health information, and no
customer data exists anywhere in this project** — not in the repository,
not in test fixtures, not in `samples/`, not in benchmark inputs, not in
telemetry (there is none), and therefore not in any prompt. Every sample
and fixture is synthetic. This is family policy
([§1.4](spec/01-family-policy/index.md)), which binds future changes too,
and it is a structural property a reader can check against the tree rather
than a promise about tool behaviour.

One consequence worth stating for a project that includes a redaction tool:
a *redacted real message is still a real message*, and is equally excluded.

Vendor-side data handling is governed by the tool vendor's terms; this
document deliberately makes no claim on the vendor's behalf, because such
claims go stale silently.

## 10. Rules for contributors

Contributors **may** use AI tools. A contribution with **ai-generated**
content per §3 **should** say so in the pull-request description: which
tool, and what it did.

**This project records tool participation in commit trailers**, in the form
`Co-Authored-By: Claude <model> <noreply@anthropic.com>`, and the history
carries them. That is a deliberate choice and worth naming, because it is
not universal — some projects require such trailers and others forbid them,
and there is no ecosystem-wide agreement. The reasoning here is that the
trailer is a per-commit fact, cheap to record and impossible to reconstruct
later, while this document is the standing disclosure that explains what
the trailer means. §4 governs how to read it: the trailer records
participation, and the `Author:` field records accountability.

A contributor remains responsible for their submission in full, under the
same [`CONTRIBUTING.md`](CONTRIBUTING.md) bar as any other work:
understood, explained on request, tested, and honest.

## 11. Prohibited uses

In this project, AI **shall not**: commit or merge anything on its own
authority; decide whether to accept a contribution from someone else; sign
anything; decide what the HL7 v2 standard means where it is silent, or what
a release claims; decide that a crate should release *at all*, or publish
as a standing job or on its own initiative outside a session the
maintainer is directing; or weaken a test, an expectation, a spec rule, or
a gate to make something pass.

**Permitted, not prohibited, once the maintainer has scoped and named a
release:** judging whether it meets this project's own stated readiness
criteria, and running `cargo publish` for it — per [§6](#6-human-oversight),
as of 2026-09-02. Stated here so the boundary reads as one list, not two
documents that have to be reconciled by a reader.

Two more, specific to this domain:

- AI **shall not** be used to add a dependency to any of the three crates.
  The counts are zero, one, and two, each justified by name in
  [§1.1](spec/01-family-policy/index.md); changing one is a decision, not
  a convenience.
- AI **shall not** generate a sample, fixture, or example by paraphrasing a
  real message. Synthetic means constructed, not laundered.

## 12. Limitations and residual risks

This section exists because a disclosure without one is marketing.

- **The gates prove what they test, not correctness.** The test suite
  demonstrates the behaviours it covers. Coverage is real and ratchets
  upward, and it is still a boundary.
- **The machine enforcement of the gates has a young but real history.** A
  CI workflow exists as of 2026-08-26 (`.github/workflows/ci.yml`); as of
  2026-09-02 it has 44 hosted runs, 43 green, and the one failure was
  root-caused and fixed the same day it happened rather than ignored. A
  gate that depends on one person remembering to run it is weaker than
  one a robot refuses to skip; this project now has the stronger kind,
  and it has started accumulating a history rather than resting on a
  single proof-of-concept run.
- **Review depth is one person's.** [`MAINTAINERS.md`](MAINTAINERS.md) says
  the bus factor is one. "The maintainer understands and can explain every
  committed change" is the honest claim; "every line was independently
  re-derived" would not be.
- **Scope is narrow by design and easily overread.** These crates parse an
  encoding. They do not validate against the standard's tables, carry a
  typed segment model, or speak any transport. AI-assisted development
  makes it easy to produce more surface than one person can verify, and the
  narrow scope is part of the counterweight.
- **Retroactivity.** Commits predating this statement carry the trailers
  described in §10 but no other disclosure marker. This document describes
  the practice, not a per-commit audit trail, and no such trail is claimed.
- **Provenance uncertainty survives.** Whether any generated fragment
  echoes unlicensed training material is not fully knowable with current
  tools. §8 states the handling, not a guarantee.
- **The legal ground is unsettled.** Copyright in AI output is an open
  question in most jurisdictions. This document records positions, and
  positions may have to change.
- **This is a self-declaration.** No third party has audited it. The
  checkable artefacts — the specs, the tests, the trailers, the published
  benchmark method — are the counterweight: they can disagree with this
  document, and if they do, the document is wrong.

## 13. Review and change

This statement is reviewed at every release that changes the practice
described here, and revised off-cycle when any of these fires: the tooling
changes materially, a tool vendor's terms change in a way §8 or §9 relies
on, a binding rule emerges that touches this use, or a claim in this
document stops being true. The change lands as a commit like everything
else, and the version and the change log in Annex A update in the same
commit.

## 14. Reporting

A suspected provenance, licensing, or quality problem in this repository —
including a claim in this document that does not survive checking — is a
report this project wants. Open an issue and cite this file; for anything
security-sensitive, email <joel@joelparkerhenderson.com>, and read
[`MAINTAINERS.md`](MAINTAINERS.md) first for what is and is not promised
about response. The handling commitment is the same as for any defect:
answered, and never silently absorbed.

## 15. References

**Normative for this project** — the documents that bind the practice
described here: [`LICENSE.md`](LICENSE.md);
[`spec/01-family-policy/index.md`](spec/01-family-policy/index.md);
[`spec/rust-msrv-n-minus-2/index.md`](spec/rust-msrv-n-minus-2/index.md);
each crate's own `spec/index.md`; the workspace and per-crate `AGENTS.md`;
[`CONTRIBUTING.md`](CONTRIBUTING.md);
[`MAINTAINERS.md`](MAINTAINERS.md);
[`GOVERNANCE.md`](GOVERNANCE.md), whose Release authority section §§5–6
now cite directly.

**Informative** — the sources this document's structure and positions draw
on: the W3C AI Content Disclosure vocabulary; the ISO/IEC Directives Part 2
verbal forms; the Apache Software Foundation's and LLVM's generative-tooling
positions; the Linux Foundation's generative-AI policy; the practice of the
FerroEHR project, whose AI statement is the structural model for this one.

## Annex A. Change log

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-08-26 | First issue. |
| 1.1.0 | 2026-08-26 | §7 and §12 updated: a CI workflow now exists (`.github/workflows/ci.yml`); its first hosted run is still pending. |
| 1.2.0 | 2026-08-26 | §7 and §12 updated again: the first hosted run went green (all four jobs, including the fuzz smoke run added the same day). |
| 1.3.0 | 2026-08-27 | §7 and §12 updated a third time: a claim in this document stopped being true, per §13's own trigger — "one run proves the gate executes" had become 13 runs, 12 green, and the document still said one. Corrected to the real figure, with the one failure named and its fix cited rather than omitted. |
| 1.4.0 | 2026-08-30 | §7 and §12 updated a fourth time, same §13 trigger: "13 runs, 12 green" had become 34 runs, 33 green (`gh run list -R er7-rust/er7-rust --workflow=ci.yml --limit 200 --json conclusion`), and the document still said 13. Corrected; the one failure and its same-day fix are still named rather than dropped now that the run count has grown past it. |
| 1.5.0 | 2026-09-02 | §7 and §12 updated a fifth time, same §13 trigger: "34 runs, 33 green" had become 39 runs, 38 green, and the document still said 34. Corrected; same one failure, still named. |
| 1.6.0 | 2026-09-02 | §13's "the tooling changes materially" trigger, not the run-count one: `GOVERNANCE.md`'s Release authority changed the same day — the maintainer may now direct an agent to run `cargo publish` for a release he has decided on, rather than only ever typing it himself. §5 gained a table row, §6 gained a paragraph, and §11's prohibited-uses list now names publishing on an agent's own authority alongside committing on its own authority, which it already prohibited. §7 and §12's run count corrected again in the same change, the usual §13 run-count trigger, since it had moved again by the time this version landed. |
| 1.7.0 | 2026-09-02 | §7 and §12 corrected a sixth time, same run-count trigger, same day: "43 runs, 42 green" (1.6.0, a few hours earlier) had become 44 runs, 43 green — this session's own commits, including 1.6.0's own publish, each push a new run. Corrected once more, together with `SECURITY.md`, `CONTRIBUTING.md`, `plan.md`, `spec/professionalization/index.md`, and `help/outreach/index.md` in the same pass, so all six agree. Noted here rather than left implicit: this number will be stale again by the time this very commit's own CI run concludes, and that is not a defect in the correction — see the note added to `plan.md` and `spec/professionalization/index.md` the same day. This is the last correction to this specific figure for one session; re-check on the next trigger, not on a timer. |
| 1.8.0 | 2026-09-02 | Explicit maintainer instruction, §13's "tooling changes materially" trigger: an agent may now decide that a specific, already-scoped release is ready to publish, not only run the command for one the maintainer judged ready himself (1.6.0). §5 split the single "whether and when to publish" row into three — scope/claims (still `none`), readiness (now `ai-assisted`), and running the command (already `ai-assisted`, unchanged) — so the level column keeps meaning one thing per row. §6 rewritten around three parts, not two: scoping (unconditionally the maintainer's), readiness, and the keystroke. §11's prohibited-uses list narrowed from "decide to publish a release" (now too broad — readiness is permitted) to "decide that a crate should release at all" (still prohibited — scoping is not), with an explicit permitted-uses note added in the same place so the boundary reads as one list. Annex B gained `agent-may-decide-release-readiness: true`, distinct from `agent-may-run-cargo-publish` (1.6.0) and `release-decisions: none` (unchanged, unaffected by both). |

## Annex B. Machine-readable summary

Levels per the W3C AI Content Disclosure vocabulary (§3); the prose above
is authoritative where the two could ever disagree.

```yaml
ai-statement:
  version: 1.8.0
  last-updated: 2026-09-02
  vocabulary: w3c-ai-content-disclosure
  disclosure-default: ai-generated
  tools:
    - name: Claude Code
      provider: Anthropic
  processes:
    design: ai-assisted
    implementation: ai-generated
    testing: ai-generated
    documentation: ai-generated
    review: none
    standards-adjudication: none
    # Whether a crate releases at all, and what a released version
    # claims — unaffected by either field below.
    release-decisions: none
  commit-trailers: true
  # cargo publish itself, not the decision to release (release-decisions
  # above, unchanged at none): as of 2026-09-02 an agent may run it, but
  # only when the maintainer has already decided on and named the
  # release. See GOVERNANCE.md's Release authority.
  agent-may-run-cargo-publish: true
  # Judging whether an already-scoped, already-named release meets this
  # project's own stated readiness criteria — the four checks, spec/code/
  # test agreement, correct SemVer classification. Also as of 2026-09-02,
  # added later the same day. Still requires the maintainer to have
  # scoped and named the release first; see release-decisions above.
  agent-may-decide-release-readiness: true
  ships-ai-system: false
  autonomous-use: none
```

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
