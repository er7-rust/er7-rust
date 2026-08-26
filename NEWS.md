# News

Announcements, project status, where updates appear, and press contacts.

This is not the changelog. [`CHANGELOG.md`](CHANGELOG.md) records what
changed in the code, change by change; this file records what is worth
telling someone who is not reading diffs. The same material, written for
readers rather than for a repository, is at <https://er7-rust.github.io/news/>.

## Status at a glance

| | |
|---|---|
| First published | 2026-08-15 (`er7`); the workspace was assembled 2026-08-19 |
| Crates | Three, versioned and published independently |
| Maturity | `0.x`. New, and the API may still break in a minor bump. |
| Maintainers | One — [`MAINTAINERS.md`](MAINTAINERS.md) states the bus factor plainly |
| Scope | The ER7 pipe-hat *encoding*: parse, query, edit, write, redact, and Serde. Not validation, not a typed segment model, not a transport, not the HL7® FHIR® standard |
| Runtime dependencies | `er7` zero, `er7-redact` one, `serde-er7` two |
| Rust | Current stable minus three releases; today 1.95 |
| License | MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only |

## 2026-08-26 — `unsafe` is now a compile error

Every crate root in the workspace carries `#![forbid(unsafe_code)]`: the
three libraries, both binaries, every example, the benchmarks, and the
fuzz targets.

The choice of `forbid` over `deny` is the whole point. A `deny` can be
switched off by an `#[allow(unsafe_code)]` on the next function; a `forbid`
cannot be overridden anywhere below it. So this is a property of the build
rather than a convention somebody has to keep, and it is checkable by
anyone in one command.

It costs nothing here. Nothing in an ER7 encoder needs to reach past the
borrow checker — the whole workload is reading `&str`, walking delimiters,
and building `String`s.

## 2026-08-26 — A security policy, and who decides

[`SECURITY.md`](SECURITY.md) says where to send a vulnerability report,
what counts as one, and — the part most such files skip — what does not.
Six documented limitations are named as explicitly *not* vulnerabilities,
each linked to the specification section that argues for it, so that a
reporter can tell in advance whether they have found a bug or a decision.

It publishes six properties a reviewer can confirm without trusting anyone:
no `unsafe` in any published crate — now compiler-enforced by
`#![forbid(unsafe_code)]` on every crate root — no build scripts, no
filesystem, network, process, or environment access from library code, and
the dependency counts of zero, one, and two. Each row carries the command
that checks it.

It also discloses the awkward thing rather than burying it. Error values
here are deliberately narrow and none carries a field value from a message
body — but `er7::Error::MissingHeader` carries the leading alphanumeric run
of the input's first line, so feeding it something that is not ER7 at all
and then logging the whole error will log that. Better said than
discovered.

Disclosure is coordinated on a ninety-day deadline that applies whether or
not a fix exists. That is deliberate: a project with one maintainer is
exactly the kind that might go quiet, and nobody should be asked to sit on
a finding indefinitely on its behalf.

[`GOVERNANCE.md`](GOVERNANCE.md) arrived with it, and answers the question
an evaluator asks next: who decides. It names the model plainly — one
maintainer, no committee, no legal entity — and then sets out what
constrains that person. The strongest constraint is that the specification
is the authority: "R13 says an unrecognised sequence stays literal, and
this code does not" is a complete argument that anyone can make and that
the maintainer cannot wave away.

## 2026-08-26 — Asking for what this project cannot find out alone

[`RFC.md`](RFC.md) is an open request for comments, and it is unusually
specific about what it wants. Twelve questions, each already recorded as an
open question in a specification section, each with what a real answer
would change: whether the value tree matches the messages your senders
actually emit, what the built-in redaction policy misses, whether the
zero-dependency stance mattered in a real audit at your organisation or
bought nothing, and whether six months of Rust-version window is enough for
your toolchain approvals.

The reason it exists is stated in its first paragraph: this is a
one-maintainer project with no production traffic of its own, and no amount
of additional testing closes that gap. It closes when somebody who runs an
interface says what they saw.

[`CONTRIBUTING.md`](CONTRIBUTING.md) arrived alongside it, covering time,
code, and money — including a plain list of what a donation does not buy.

## 2026-08-26 — The evaluation documents are on the website

Everything a reader needs before adopting this is now a page rather than a
file in a repository: [install](https://er7-rust.github.io/install/),
[comparison](https://er7-rust.github.io/comparison/),
[benchmarks](https://er7-rust.github.io/benchmarks/),
[news](https://er7-rust.github.io/news/), and
[trademarks](https://er7-rust.github.io/trademarks/). The Markdown files in
the repository stay canonical; the pages are the same material written for
readers.

## 2026-08-26 — The HL7® marks, marked

HL7 International lets anyone use its word marks descriptively, and asks
three things in return: the ® on a mark's first use on each page, a
disclaimer wherever the marks appear, and the Fast Healthcare
Interoperability Resources called the "HL7® FHIR® standard". This project
now does all three, everywhere — every document, every website page, every
Rust doc comment, every crate description, and both `--help` outputs.

Two decisions are worth stating, because they are visible. Sample messages,
error strings, citation blocks, and code identifiers are **not** marked: a
`®` inside `MSH|^~\&|…` corrupts the sample, and `no HL7 segments` is a
diagnostic that gets grepped and asserted, not prose that gets read. And
none of this is enforced by good intentions: `bin/check-trademarks` fails
the build if a page drifts.

The policy is [`spec/hl7-trademarks-fair-use/index.md`](spec/hl7-trademarks-fair-use/index.md)
and the notice is [`TRADEMARKS.md`](TRADEMARKS.md).

## 2026-08-26 — What this project claims, and how to check it

A set of documents that answer what an evaluation actually asks, rather
than what a README usually says.

**Benchmarks with their method.** [`BENCHMARKS.md`](BENCHMARKS.md)
publishes measured figures with confidence intervals, on a named machine
and toolchain, from benchmarks in the repository that anyone can run. Three
worth carrying away: a 21 KiB, 402-segment lab result parses in about
260 µs; writing a message back out is roughly twelve times cheaper than
parsing it; and a single `query` is effectively independent of message
length, because it returns on first match rather than walking everything.

**A comparison that says when to choose something else.**
[`COMPARISONS.md`](COMPARISONS.md) puts interface engines, HAPI and the
mature libraries, the other Rust HL7® crates, the sibling `hl7-rust` family,
and hand-rolled pipe splitting in their actual categories — and names four
cases where one of those is the better answer. It claims no performance
comparison, because none has been measured.

**A statement of who is behind this, and what happens if he isn't.**
[`MAINTAINERS.md`](MAINTAINERS.md) inventories every publishing identity,
says the bus factor is one, and lists what is missing — no CI, no signed
commits, no security policy file — rather than omitting it.

**A disclosure of how the software is built.**
[`AI_STATEMENT.md`](AI_STATEMENT.md) states where AI tooling is used, at
what level, under which controls, and with which limitations still
standing.

Also added: [`INSTALL.md`](INSTALL.md), [`CHANGELOG.md`](CHANGELOG.md),
[`TRADEMARKS.md`](TRADEMARKS.md),
[`LICENSE.md`](LICENSE.md) at the workspace root, `CITATION.cff` with an
ORCID, `CODEOWNERS`, [`spec/promote/index.md`](spec/promote/index.md), and
this file.

## 2026-08-25 — The website lives in this repository

The source of <https://er7-rust.github.io> moved into this monorepo, so a
change to a crate's public surface and the page that teaches it land
together. A page that still documents a removed API is now a broken change
rather than a follow-up.

## 2026-08-23 — `er7-redact` 0.2.0: a policy states its posture

A redaction policy now says explicitly whether it accepts or rejects by
default. `Policy::all_but_the_header()` rejects every value no rule
accepts, keeping `MSH` intact — the posture to choose when the message is
unfamiliar, because the failure mode of a missed rule is a redacted value
rather than a leaked one. **This is a breaking change**: existing policies
must say which posture they mean.

## 2026-08-20 — Single-value queries stopped walking the whole message

`query` now returns on first match. Against a 402-segment message that is
85% off a field lookup and 92% off a subcomponent lookup. Fuzz targets and
Criterion benchmarks landed in the same release, and the MSRV policy —
current stable minus three — was pinned across the workspace.

## 2026-08-19 — Three repositories became one workspace

`er7`, `er7-redact`, and `serde-er7` were assembled into one Cargo
workspace with their histories preserved. They still version and publish
independently; what they now share is one `Cargo.lock`, one set of build
checks, and one workspace-level [`spec/`](spec/) holding the policy that is
genuinely common to all three.

## Where updates appear

| Channel | What arrives there |
|---|---|
| <https://er7-rust.github.io> | Documentation for all three crates, and the ecosystem map |
| [crates.io](https://crates.io/crates/er7) | Every release, per crate; the authoritative version list |
| [GitHub](https://github.com/er7-rust/er7-rust) | Commits, issues, and releases. *Watch → Releases only* is the low-volume subscription. |
| [`CHANGELOG.md`](CHANGELOG.md) | Change-by-change detail |

There is no mailing list and no social account. If one is added, it will be
announced here first. The plan for how this project reaches people, and the
etiquette that governs it, is [`spec/promote/index.md`](spec/promote/index.md).

## Press and media

**Contact:** Joel Parker Henderson, <joel@joelparkerhenderson.com>. Sole
maintainer, and the only person who can speak for the project. Please say
what you are writing and by when; a same-day reply is likely but not
promised, and [`MAINTAINERS.md`](MAINTAINERS.md) explains why nothing here
is promised.

**Available on request:** background on HL7 v2 and why a pipe-delimited
encoding designed in the 1980s is still the backbone of hospital
integration; why de-identifying a message without changing its shape is
harder than it looks, and why the shape matters; commentary on memory
safety and dependency auditing in health IT; the reasoning behind any
design decision in the project, all of which is written down in the specs.

### Boilerplate, ready to quote

> ER7 Rust is an open-source Cargo workspace of three Rust crates for HL7
> v2 messages in the ER7 pipe-hat encoding: `er7` parses, queries, edits,
> and writes them with zero runtime dependencies; `er7-redact` removes
> patient detail without changing the shape of the message; and `serde-er7`
> carries a message tree through JSON, YAML, or any other Serde data
> format. It is multi-licensed under MIT, Apache-2.0, BSD-3-Clause,
> GPL-2.0-only, or GPL-3.0-only, at the user's option, and is maintained by
> Joel Parker Henderson. <https://er7-rust.github.io>

### Facts a story might need, all checkable

- **Three crates**, published on crates.io from 2026-08-15.
- **Zero runtime dependencies in `er7`**, enforced by a test that fails if
  one is added. Criterion lives in a separate unpublished workspace member
  so that even the development dependencies stay empty.
- **A byte-for-byte round trip** is a tested guarantee: a message parsed and
  not modified renders back identically.
- **Redaction preserves structure.** Same segments, fields, components,
  delimiters, and escape sequences, so every path that resolved to a value
  still resolves to one — which is what lets a redacted message still
  reproduce the bug the original caused.
- **`er7-redact --report` prints paths and actions with no values**, so the
  report itself is safe to paste into a ticket.
- **Five licenses at the user's option**, chosen so that a proprietary
  vendor and a public-sector project can both adopt it without asking.
- **A minimum supported Rust version of current stable minus three**,
  chosen because hospital toolchains are approved on a cycle measured in
  quarters.
- **Benchmarks and their method are published**, on a named machine, with
  confidence intervals and the reasons not to over-read them.

### What this project will not say

Stated so nobody has to ask twice, and so no quote implies otherwise:

- **It is not certified or accredited by anyone.** HL7 International has
  not assessed it. No conformance testing body has assessed it.
- **It is not a medical device**, and it makes no clinical claim. See
  [`AI_STATEMENT.md`](AI_STATEMENT.md) §2.
- **It does not validate messages** against the standard's tables, and
  never has. A story describing it as a validator would be wrong.
- **It has no production track record to cite.** It was first published in
  2026. Any story implying hospital deployments would be inventing them.
- **No benchmark comparison against another library exists**, so no
  "faster than X" claim can be sourced to this project.
- **No adoption or download figure will be offered as a success metric.**
  The numbers are public on crates.io and are small, as you would expect of
  a project this age.
- **No example, sample, or screenshot anywhere in this project contains
  real patient data**, and none ever will. Every one is synthetic, by
  policy ([§1.4](spec/01-family-policy/index.md)).

### Trademark

HL7® and FHIR® are registered trademarks of Health Level Seven
International. This project implements a published encoding and is
**independent of, not affiliated with, and not endorsed by** HL7
International. Please carry that qualifier in any coverage; a story that
implies otherwise would be wrong in a way that matters to a standards body.

The full notice is [`TRADEMARKS.md`](TRADEMARKS.md), and the fair-use
policy this project holds itself to — including the rule that puts the ®
on this page — is
[`spec/hl7-trademarks-fair-use/index.md`](spec/hl7-trademarks-fair-use/index.md).

## Corrections

If something on this page — or anywhere in this repository — stops being
true, that is a defect and worth reporting the same way any other defect
is. Everything here is written so it can be checked rather than believed,
which only works if people check.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
