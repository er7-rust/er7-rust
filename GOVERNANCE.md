# Governance

Who decides what happens to this project, how those decisions get made and
recorded, and how someone else can come to share them.

This is a small project with one maintainer. A governance document for a
project this size is usually either absent or aspirational — a description
of committees that do not exist. This one describes what actually happens,
because an accurate small answer is more useful to an evaluator than an
impressive invented one.

- [Who decides](#who-decides)
- [The specification is the authority](#the-specification-is-the-authority)
- [How a change lands](#how-a-change-lands)
- [How a decision gets recorded](#how-a-decision-gets-recorded)
- [What gets declined, and why](#what-gets-declined-and-why)
- [Release authority](#release-authority)
- [Scope, and who owns which layer](#scope-and-who-owns-which-layer)
- [Becoming a maintainer](#becoming-a-maintainer)
- [Disagreement](#disagreement)
- [Conduct enforcement](#conduct-enforcement)
- [Forking](#forking)
- [Changing this document](#changing-this-document)

## Who decides

**One person: the maintainer**, listed in
[`MAINTAINERS.md`](MAINTAINERS.md). There is no steering committee, no
technical board, no voting, and no legal entity. The GitHub organisation
`er7-rust` is an organisation in the GitHub sense only — it exists because
an organisation Pages site must be served from an org-owned repository.

This is a benevolent-dictator model, and naming it is more honest than
implying a process. The dictator part is real: one person can accept a pull
request, publish a release, or change a repository setting, and nobody can
overrule them. The benevolent part is not a promise — it is a set of
constraints this document and the specifications impose, which anyone can
check:

| Constraint | Where it binds |
| ---------- | -------------- |
| A decision about behaviour is written down before it is implemented | [§1.3](spec/01-family-policy/index.md) |
| A declined idea is recorded with its reasoning, where it can be argued with | Each crate's open-questions section |
| A rule without a test fails the build | Each crate's testing-strategy section |
| Nothing is merged automatically, and no tool has authority | [`AI_STATEMENT.md`](AI_STATEMENT.md) §4, §11 |
| The project says what it does not have, rather than omitting it | [`MAINTAINERS.md`](MAINTAINERS.md), [`SECURITY.md`](SECURITY.md) |

Those constraints are what make the bus factor of one survivable. They do
not make it acceptable — see [Forking](#forking).

## The specification is the authority

This is the one governance rule that matters more than who holds the
credentials.

**Each crate's `spec/` directory is canonical.** When the spec and the code
disagree, the spec is right and the code is a bug — or the spec is right
and needs changing *first*. Every behavioural guarantee carries a stable
rule ID (`R<n>` for `er7`, `D<n>` for `er7-redact`, `S<n>` for
`serde-er7`), every section is numbered, and a coverage table maps each
rule to the test that enforces it. The table is itself checked by
`cargo test`, so a rule added without a test fails the build rather than
waiting for a careful reader.

The practical effect is that arguments here are about text, not about
authority. "R13 says an unrecognised sequence stays literal, and this code
does not" is a complete argument that anyone can make and that the
maintainer cannot wave away. That is a deliberate transfer of power away
from the person holding the merge button, and it is the main thing this
project offers in place of a committee.

The workspace-level policy shared by all three crates lives in
[`spec/`](spec/index.md) at the root: dependency minimalism, the four
checks, the spec-driven discipline itself, the synthetic-data rule, the
minimum Rust version window, and the trademark rules.

## How a change lands

The same path for everyone, maintainer included:

1. **The specification section changes first.** A change to observable
   behaviour that does not touch the matching `spec/` section is
   incomplete, not merely undocumented.
2. **A test expresses the rule.** The test is the executable form of the
   spec clause.
3. **The code changes.**
4. **The documentation follows** — `index.md`, `docs/`, `examples/`, and
   the website are *derived*; they explain the spec and do not define it.
5. **The checks pass** — on somebody's machine first, and in CI
   (`.github/workflows/ci.yml`, still awaiting its first hosted run):

   ```sh
   cargo test --workspace
   cargo clippy --workspace --all-targets -- -D warnings
   cargo fmt --check
   cargo rustdoc -p er7 --lib -- -W missing-docs
   cargo +1.95 check --workspace --all-targets
   bin/check-trademarks
   ```

6. **The maintainer reviews and merges.** Nothing merges automatically. No
   tool merges anything, ever
   ([`AI_STATEMENT.md`](AI_STATEMENT.md) §11).

[`CONTRIBUTING.md`](CONTRIBUTING.md) is the practical version of this list.

## How a decision gets recorded

Three places, and which one a decision goes to is itself a decision:

| The decision | Where it goes |
| ------------ | ------------- |
| What a crate does | That crate's numbered spec section, with a rule ID |
| What was considered and declined | That crate's open-questions-and-declined-decisions section, with the reasoning |
| What is scheduled | That crate's roadmap section |
| What is wanted but unscheduled | That crate's open-tasks section |
| Policy shared by all three crates | The workspace [`spec/`](spec/index.md) |
| What the project wants to learn | [`RFC.md`](RFC.md) |

There is no separate `plan.md`, no `decisions/` directory, and no ADR
folder. A roadmap item and the rule it will change sit next to each other
on purpose.

## What gets declined, and why

Most declines here fall into three groups, and knowing which one applies
saves everybody time:

1. **It crosses a layer boundary.** `er7` is the ER7 *encoding*. Validation,
   typed segment models, code tables, and transports belong to the layer
   above, which is a different project. This is recorded at
   [`er7` §18.1](er7/spec/18-open-questions-and-divergences/index.md), and
   it is the most common reason a reasonable request is declined.
2. **It would add a dependency.** The counts are zero, one, and two, each
   justified by name in [§1.1](spec/01-family-policy/index.md). This is not
   a preference: healthcare integration code gets audited, and these crates
   are meant to sit at the bottom of somebody else's stack. A pull request
   that adds a dependency has to argue against that table, in the crate's
   own spec.
3. **It would make a claim the project cannot support.** No conformance
   certification, no comparative performance claim without a fair
   benchmark, no compliance statement. Overclaiming in healthcare software
   is treated as a defect.

A decline is written down with its reasoning, in the spec, where you can
argue with it. **A decision being recorded does not make it right; it makes
it arguable.**

## Release authority

The maintainer alone can publish. Each crate versions and releases
independently — there is no workspace version — and every crate follows
[Semantic Versioning](https://semver.org/).

| Rule | Effect |
| ---- | ------ |
| While a crate is `0.x`, the minor bump is the one allowed to break | A `0.1.2 → 0.2.0` may change an API; a patch never does |
| Raising the minimum Rust version is a breaking change | It never lands in a patch ([§2](spec/rust-msrv-n-minus-3/index.md)) |
| A release is a decision, not an automation | No workflow publishes; one person runs `cargo publish` |
| Yanking is reserved for genuinely dangerous versions | See [`SECURITY.md`](SECURITY.md) |

Every release is recorded in [`CHANGELOG.md`](CHANGELOG.md), and the ones
worth a sentence to a non-developer are in [`NEWS.md`](NEWS.md).

## Scope, and who owns which layer

Governance here includes deciding what is *not* this project's problem.

| Layer | Owner |
| ----- | ----- |
| The ER7 pipe-hat encoding — delimiters, the value tree, escapes, paths, batch input | This project |
| Tools that need no HL7® version: redaction, Serde | This project |
| HL7® v2 semantics — segments, data types, message structures, dictionaries per release, MLLP and SOAP transports | [`hl7-rust`](https://github.com/hl7-rust/hl7-rust), a separate project by the same maintainer |
| The HL7® FHIR® standard | Neither. Different standard. |

The split is the point, and it is argued for at
[`er7` §18.1](er7/spec/18-open-questions-and-divergences/index.md) and on
the [ecosystem page](https://er7-rust.github.io/ecosystem/). A request to
add dictionary knowledge to `er7` will be declined here and is welcome
there.

Shared maintainership does not mean shared decisions: each project's spec
governs it, and a decision in one does not bind the other.

## Becoming a maintainer

The route is open, deliberately unglamorous, and has no gatekeeping beyond
demonstrated judgement:

1. **Contribute.** A bug report carrying a synthetic message that
   reproduces the problem is as welcome as code. So is answering a question
   in [`RFC.md`](RFC.md).
2. **Show sustained judgement about the specification**, not just the code.
   In this project the spec is canonical, so someone who can be trusted
   with it can be trusted with the rest. The signal is not volume of
   commits; it is being right about what a rule should say, more than once,
   including when it means arguing with the maintainer.
3. **Ask.** Email <joel@joelparkerhenderson.com>. Nobody is expected to
   wait to be noticed.

Adding a maintainer means updating three things in one change: the roster
in [`MAINTAINERS.md`](MAINTAINERS.md), [`CODEOWNERS`](CODEOWNERS), and the
publishing-identity table in `MAINTAINERS.md`. A maintainer who is not in
all three is not really a maintainer.

A second maintainer changes this document too: the sections above describe
a single decider, and they would need to say how two people resolve a
disagreement. That rewrite is a welcome problem to have.

## Disagreement

If you think a decision is wrong:

1. **Say so, in the open**, in an issue or against the spec section. Cite
   the rule ID or section number — that is what they are numbered for.
2. **Expect a reasoned answer**, or a recorded decline with reasoning. "No"
   without a reason is a bug in this process, and pointing that out is
   fair.
3. **If you still disagree**, that is a legitimate place to end up. The
   maintainer decides, you are on record, and the record is public and
   permanent. [Forking](#forking) is always available and is not treated as
   hostile.

There is no appeal body, because inventing one would be fiction.

## Conduct enforcement

[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) — the Contributor
Covenant — applies across this workspace.

Enforcement is the maintainer's, which means a report about the maintainer
goes to the maintainer. That is a real structural weakness of a one-person
project and there is no honest way to paper over it. If that is not
acceptable to you, GitHub's own reporting channels sit above this project,
and the public record of an interaction is available to anyone.

## Forking

**Forking is legitimate continuity, not a hostile act**, and this project
is arranged to make it cheap:

- Five licences, at your option, including permissive and copyleft ones, so
  a fork can be relicensed to suit whoever maintains it.
- Zero, one, and two runtime dependencies, so a fork inherits an auditable
  artefact rather than a tree.
- A specification with numbered rules bound to tests, so a fork inherits
  the *reasoning*, not just the code.
- No CLA and no copyright assignment, so contributors keep their rights.

If the maintainer becomes unavailable, a fork is the intended answer and
[`MAINTAINERS.md`](MAINTAINERS.md) says so. If you fork because you
disagree, that is also fine; a note in an issue saying where it lives is
appreciated, and will be linked rather than ignored.

## Changing this document

This file changes the way anything else here does: in a commit, with
reasoning, by the maintainer. It has no special status and no amendment
procedure, because a procedure that one person can change unilaterally is
not a procedure.

The change to watch for is a second name in
[`MAINTAINERS.md`](MAINTAINERS.md). Everything above is written for a
project with one, and it should be rewritten the day that stops being true.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
