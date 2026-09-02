# Contributing

Thanks for looking. This file is the workspace-wide version — enough to
file a good report, land a small change, or help in a way that involves no
code at all. The conventions for changing a particular crate are in that
crate's own `AGENTS.md`.

- [Never paste patient data](#never-paste-patient-data)
- [Ways to help that are not code](#ways-to-help-that-are-not-code)
- [Where to file](#where-to-file)
- [What a good report contains](#what-a-good-report-contains)
- [Before you open a pull request](#before-you-open-a-pull-request)
- [Conventions a reviewer will otherwise ask about](#conventions-a-reviewer-will-otherwise-ask-about)
- [The most useful contribution](#the-most-useful-contribution)
- [Performance changes](#performance-changes)
- [Fixing the website](#fixing-the-website)
- [Money](#money)
- [Licensing your contribution](#licensing-your-contribution)
- [Conduct](#conduct)

## Never paste patient data

HL7® v2 messages are clinical records, and an issue tracker is public and
permanent. A message pasted into one cannot be unpasted.

Redact the values and keep the structure:

```
MSH|^~\&|LAB|ACME|EHR|CLINIC|20260814080000||ORU^R01|MSG00042|P|2.5
PID|1||REDACTED^^^ACME&1.2.3.4&ISO^MR||REDACTED^REDACTED||REDACTED|F
OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F
```

Structure is what reproduces a parsing bug: the delimiters, the field
positions, the repetition separators, the component depth. Names,
identifiers, dates of birth, and addresses are not. Replace them and the
report still works.

This repository ships the tool for exactly this:

```sh
er7-redact message.er7            # a redacted message, same shape
er7-redact --report message.er7   # paths and actions, and no values at all
```

The `--report` output carries no values, so it is the safest thing to
attach. If a bug genuinely depends on a specific byte sequence in a value —
an unusual escape, a non-ASCII character set — describe the byte sequence
rather than the record it appeared in.

The rule that binds this project's own files is
[§1.4 of the family policy](spec/01-family-policy/index.md): every sample,
fixture, example, and benchmark input here is synthetic, and a redacted
real message is still a real message.

## Ways to help that are not code

Most of what this project needs is not a pull request. In rough order of
how much it would help:

| Contribution | What it looks like | Why it matters here |
| ------------ | ------------------ | ------------------- |
| **Tell us what real messages look like** | "Our lab sends `OBX-5` with a `~` in it and your outline shows…" | This is a one-maintainer project with no production traffic of its own. Real-world message shapes are the thing it cannot generate for itself. |
| **Answer the questions in [`RFC.md`](RFC.md)** | A paragraph in an issue, or an email | That file lists what the project actually does not know, and each answer changes a decision rather than filling a survey. |
| **Review a specification section** | "§8.3 says the path grammar accepts X, but Y is what senders do" | The spec is canonical; a correction there is worth more than a correction to the code. |
| **Try it against your interface and report back**, including "it worked" | An issue, or an email | A negative result is publishable here. Silence is not evidence of anything. |
| **Improve the documentation** | A tutorial that answers a question you had | The person who just learned something is the best person to write it down. |
| **Triage** | Reproduce someone else's report, or narrow it | With a bus factor of one, triage is the scarcest thing. |
| **Point out where the docs oversell** | "This claims X but §Y says Z" | Overclaiming in healthcare software is a defect. It is treated as one. |
| **Package it** | A distribution package, a container image | Nobody has done this, and `cargo install` is not how most hospitals install anything. |

None of these need permission, and none need Rust.

## Where to file

- **Issues and pull requests**: <https://github.com/er7-rust/er7-rust>.
- **GitLab and Codeberg carry mirrors**; issues live on GitHub.
- **Which crate?** If you are unsure, file it anyway and say what you were
  doing. Sorting it out is the maintainer's job, not yours.
  - Delimiters, escapes, path syntax, byte-for-byte rendering, the
    command-line outline → `er7`
  - Policies, actions, pseudonyms, the report → `er7-redact`
  - Serialize/Deserialize shapes → `serde-er7`
- **Anything security-sensitive**: email
  <joel@joelparkerhenderson.com> rather than opening an issue, and read
  [`SECURITY.md`](SECURITY.md) first — it says what counts as a
  vulnerability, what does not, and what you can honestly expect from a
  project with one maintainer.

## What a good report contains

1. **The crate and its version** — `cargo tree -p er7` if you are not sure
   what resolved.
2. **A minimal redacted message that reproduces it.** One segment is often
   enough.
3. **What you expected and what you got.** `er7 message.er7` on the message
   is usually the clearest way to show both, because it prints every value
   with the path that names it.
4. **The spec section, if you can find it.** Every rule carries a stable
   ID — `R<n>` for `er7`, `D<n>` for `er7-redact`, `S<n>` for `serde-er7` —
   and every section is numbered. "R13 says an unrecognised sequence stays
   literal, but…" turns a discussion into a fix.
5. **Your Rust version**, if it is a build failure.

The spec is the referee. If a crate's README, its rustdoc, or the website
disagrees with its `spec/index.md`, the spec is right and the other three
are the bug.

## Before you open a pull request

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc -p er7 --lib -- -W missing-docs
cargo +1.96 check --workspace --all-targets   # the MSRV floor
bin/check-trademarks                           # or: make check-trademarks
```

CI (`.github/workflows/ci.yml`) runs the four checks, the MSRV build, and
the trademark checker on every push and pull request — 39 hosted runs as of
2026-09-02, 38 green (`gh run list -R er7-rust/er7-rust --workflow=ci.yml
--limit 200 --json conclusion`), not a new, unproven workflow. Run the
checks on your machine first anyway: CI catches what you missed, it is not
a substitute for running them yourself before you push.

The MSRV floor is current stable minus two releases, so the exact
toolchain in that fifth line moves; the rule is
[§2](spec/rust-msrv-n-minus-2/index.md).

## Conventions a reviewer will otherwise ask about

- **Behaviour changes go in the spec first.** The spec is the source of
  truth, so a code change that contradicts it is either a bug fix or an
  unstated spec change. A change to observable behaviour that does not
  touch the matching `spec/` section is incomplete.
- **Every rule needs a test, and the coverage table is enforced by one.**
  A rule added without a test fails `cargo test`, rather than waiting for a
  careful reader.
- **Dependencies are the hard case.** The counts are zero, one, and two,
  each justified by name in
  [§1.1](spec/01-family-policy/index.md). A pull request that adds one has
  to argue against that table, in the crate's own spec, not in a commit
  message. `er7`'s empty table is enforced by
  `the_crate_has_no_runtime_dependencies`.
- **One `Cargo.lock`, at the workspace root.** Never one inside a member.
- **Raising the MSRV is a breaking change** and belongs in a release
  allowed to break, never in a patch.
- **A word mark's first use on a page carries `®`.** Sample messages and
  error strings never do. `bin/check-trademarks` will tell you; the
  reasoning is [§4](spec/hl7-trademarks-fair-use/index.md).
- **Serial commas** in prose, per [`spec/serial-comma/`](spec/serial-comma/index.md).
- **A declined idea gets written down**, in that crate's
  open-questions section, with the reasoning. A recorded "no, and here is
  why" saves the next person from re-litigating it.
  [`GOVERNANCE.md`](GOVERNANCE.md) explains the three groups most declines
  fall into, so you can tell in advance which one your idea meets.

## The most useful contribution

**A message shape this crate gets wrong.** Not a hypothetical one — one
that a real system actually sent you, redacted before you send it on.

The three crates are small and their behaviour is specified. What they have
never had is a corpus of what senders in the field actually emit: the
non-default delimiters, the character sets, the segments nobody documents,
the escape sequences that only one vendor produces. Every one of those is a
test this project cannot write for itself.

Second most useful: **a redaction policy rule you needed and had to add
yourself.** The built-in policy
([`er7-redact` §5.1](er7-redact/spec/05-built-in-policies/index.md)) is a
starting point and says so. What it is missing is knowable only from
someone who ran it against their own traffic.

## Performance changes

A change argued on performance needs a before-and-after from the benchmarks
in this repository:

```sh
git stash && cargo bench -p er7-bench -- --save-baseline before
git stash pop && cargo bench -p er7-bench -- --baseline before
```

Compare on the same machine; the figures in
[`BENCHMARKS.md`](BENCHMARKS.md) are from one laptop and are not a target.

Correctness wins over speed here every time. A faster parser that loses a
value, or that stops round-tripping byte for byte, is not faster; it is
broken.

## Fixing the website

The site is the [`er7-rust.github.io/`](er7-rust.github.io/) directory of
this workspace — edit it there, not in the published repository, which
`make publish` force-pushes over.

```sh
make site-dev     # http://localhost:5173
make site-check   # type-check, as the deploy does
```

Nothing on the site is normative: it explains the crates' READMEs and
specs. A correction there is a documentation fix; a correction to the
behaviour underneath belongs against the crate.

## Money

This project has no funding, no sponsor, and no legal entity behind it. It
is one person's unpaid work, and it will keep existing either way — so
treat everything in this section as optional.

If you want to send something anyway:

| Channel | Where |
| ------- | ----- |
| GitHub Sponsors | <https://github.com/sponsors/joelparkerhenderson> — verified live via GitHub's own API, one-off or recurring |
| PayPal | <https://www.paypal.com/paypalme/joelparkerhenderson> |
| Venmo | <https://www.venmo.com/joelparkerhenderson> |
| Bank transfer | ACH or international wire; email <joel@joelparkerhenderson.com> for details |
| Everything else | <https://linktr.ee/joelparkerhenderson> |

All but the bank transfer are also declared in
[`.github/FUNDING.yml`](.github/FUNDING.yml), which is what puts the
"Sponsor" button on the repository page.

**No Open Collective yet.** It was on the list; it is not set up. Checked
against Open Collective's own API rather than assumed: an account exists
at `joelparkerhenderson`, but as an `INDIVIDUAL` — a personal contributor
profile, not a fundable project collective — and no collective exists at
`er7-rust` at all. Creating a real one needs the maintainer's own sign-in
and a fiscal-host choice (Open Source Collective is the usual one for a
project like this), which is not something to do on someone else's
behalf. If that changes, it gets a row here and in
[`.github/FUNDING.yml`](.github/FUNDING.yml) the same day — not before.

**What money does not buy**, said plainly so nobody is disappointed:

- **Not support, and not a service level.** There is no contract, no queue
  position, and no response-time commitment. See
  [`MAINTAINERS.md`](MAINTAINERS.md).
- **Not a feature.** A paid feature request is still a feature request, and
  it still has to be right for the crates' scope. If a contribution came
  with an expectation attached, say so first and it will be declined
  before it is accepted.
- **Not influence over the specification.** The spec is decided on the
  merits.
- **Not a fix to the bus factor.** Money does not add a maintainer.
  [Becoming one](MAINTAINERS.md#becoming-a-maintainer) is a route that is
  open and free.

If your organisation depends on this in a clinical path and wants something
stronger than goodwill, the useful conversation is not a donation — it is
sponsoring a second maintainer, which would also close the "no second
security responder" gap [`MAINTAINERS.md`](MAINTAINERS.md) currently
lists. Email.

## Licensing your contribution

Everything here is offered under MIT, Apache-2.0, BSD-3-Clause,
GPL-2.0-only, or GPL-3.0-only, at the user's option
([`LICENSE.md`](LICENSE.md)). A contribution is offered on the same terms,
so that the choice stays available to everyone downstream. **There is no
CLA** and no copyright assignment.

If you used an AI tool, say so in the pull-request description: which tool,
and what it did. The project's own practice is disclosed in
[`AI_STATEMENT.md`](AI_STATEMENT.md) §10, and being the one who mentioned
it first is much better than being the one who was found out.

## Conduct

Be decent. Assume the person on the other end is working on a live clinical
interface and is short of time.

The full text is [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), at the
workspace root, and it applies across the whole workspace.

Contact: <joel@joelparkerhenderson.com>.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
