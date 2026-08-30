# Security policy

How to report a vulnerability in these crates, what counts as one, what
this project can honestly promise about handling it, and the
security-relevant design decisions you should know about before deploying.

- [Reporting a vulnerability](#reporting-a-vulnerability)
- [What you can expect](#what-you-can-expect)
- [Supported versions](#supported-versions)
- [What is in scope](#what-is-in-scope)
- [What is not a vulnerability](#what-is-not-a-vulnerability)
- [Security-relevant design decisions](#security-relevant-design-decisions)
- [The attack surface, concretely](#the-attack-surface-concretely)
- [Never send a real message](#never-send-a-real-message)
- [Disclosure](#disclosure)
- [What this project does not have](#what-this-project-does-not-have)

## Reporting a vulnerability

**Email <joel@joelparkerhenderson.com>.** Put "security" in the subject.

Do **not** open a public issue for anything that could be exploited. There
is one maintainer and no embargo machinery, so a public report is a public
disclosure whether or not it was meant as one.

You may also use GitHub's private vulnerability reporting on
<https://github.com/er7-rust/er7-rust> if you prefer a tracked channel.
It is enabled as of 2026-08-26 — an earlier revision of this file offered
it while the repository setting was still off, which was exactly the kind
of unverified claim this project tries not to make. Enabled the same day,
and verifiable from the repository's settings: dependency alerts,
automated security fixes (Dependabot security PRs only —
`.github/dependabot.yml` keeps routine version bumps off, mirroring the
sibling repositories' security-only posture), and secret scanning — the
policy behind that posture is
[`spec/dependabot/index.md`](spec/dependabot/index.md).

A useful report contains:

1. **Which crate and version** — `er7`, `er7-redact`, or `serde-er7`.
2. **A synthetic input that reproduces it.** Redacted, never real. See
   [below](#never-send-a-real-message).
3. **What happens, and what should happen instead.**
4. **The impact you think it has**, in your own words. You do not need to
   be right; you need to be specific.

Encrypted mail is welcome; ask and a key will be provided. There is no
published PGP key today, and saying so is better than publishing one that
nobody is checking.

## What you can expect

Stated plainly rather than dressed up, because
[`MAINTAINERS.md`](MAINTAINERS.md) says the bus factor is one and this file
should not contradict it:

| | |
|---|---|
| **Acknowledgement** | Usually within a few days. Not guaranteed, and not contractual. |
| **Assessment** | An honest answer about whether it is a vulnerability, a bug, or intended behaviour — with reasoning. |
| **Fix** | For a confirmed vulnerability: a patch release, as fast as one person can produce one. |
| **Credit** | Your name in the release notes and [`CHANGELOG.md`](CHANGELOG.md), unless you ask otherwise. |
| **Payment** | None. There is no bug bounty and no budget. |
| **A service level** | None. There is no contract and no queue position. |

If a report goes unanswered for two weeks, assume the maintainer is
unavailable rather than ignoring you, and act accordingly — including
disclosing publicly if that is the right call for the people you are
protecting. [`MAINTAINERS.md`](MAINTAINERS.md) explains why no better
promise is available to a one-person project with no legal entity behind
it.

## Supported versions

| Version | Supported |
|---|---|
| The latest release of each crate | Yes |
| Anything older | No |

There is no long-term-support branch and no backporting. Every crate is
`0.x`; a fix ships in a new release, and the upgrade is the fix. Released
versions on crates.io are immutable, so a vulnerable version stays
downloadable — yanking it is the strongest available signal and will be
used when a version is genuinely dangerous.

## What is in scope

These are libraries and two command-line tools. They parse untrusted input
and, in one case, remove patient detail from it. That gives three real
categories:

| Category | Example | Why it matters |
|---|---|---|
| **Denial of service through parsing** | An input that makes `er7::parse` panic, hang, or allocate without bound | The parser is the untrusted-input surface, and R6 claims nothing below the header fails. A counterexample to R6 is a security bug, not just a spec bug. |
| **Redaction failure** | `er7-redact` leaves a value that its policy names for removal, or `--report` prints a value | This is the crate where being wrong leaks patient data. Treat any instance as a vulnerability. |
| **Silent corruption** | A message that round-trips to something other than what went in, without an error | A clinical record altered in transit is a patient-safety problem, and a silent one is worse than a refusal. |

Also in scope: anything that reads a file, opens a socket, spawns a
process, or writes a log from *library* code. None of those exist today —
see below — so any of them appearing would be a supply-chain or
tampering finding.

## What is not a vulnerability

Documented behaviour is not a vulnerability, even when it is a limitation.
Each of these is written down in a specification section, with reasoning:

- **`er7-redact` did not remove something no rule named.** A policy is a
  list; the accept-by-default posture only touches what it names. If you
  want the opposite, `--all-but-the-header` rejects every value no rule
  accepts. The built-in policy
  ([§5.1](er7-redact/spec/05-built-in-policies/index.md)) is a starting
  point and says so — **it is not a compliance certification**, and no
  configuration of it makes these crates HIPAA-, GDPR-, or
  Safe-Harbor-compliant. That obligation stays with the deployer.
- **A pseudonym is not cryptographically strong.** `Pseudonym` uses FNV-1a
  with a `u64` key. It is a stable stand-in so that redacted messages still
  join, and it is explicitly not a security primitive
  ([§7.2](er7-redact/spec/07-pseudonyms/index.md)). Reversing one is
  expected, not a finding. The reasoning, including why a stronger hash was
  declined, is
  [§16.3](er7-redact/spec/16-open-questions-and-declined-decisions/index.md).
- **Two spellings of the same patient identifier produce two pseudonyms.**
  `Pseudonym` maps a value, not a patient
  ([§16.6](er7-redact/spec/16-open-questions-and-declined-decisions/index.md)).
- **`er7` does not validate messages.** It parses structure and does not
  judge content. A malformed-but-parseable message being accepted is the
  design ([§11.2](er7/spec/11-error-handling/index.md)).
- **Escape sequences are decoded in fields where the standard does not
  scope escaping.** A recorded divergence, with mitigations, at
  [§18.2](er7/spec/18-open-questions-and-divergences/index.md).
- **A `#` cannot appear in a policy action's argument.** A parsing
  limitation of the policy file format, recorded at
  [§16.4](er7-redact/spec/16-open-questions-and-declined-decisions/index.md).

If you think one of these decisions is wrong, that is a
[request for comments](RFC.md), not a vulnerability report — and it is
genuinely welcome there.

## Security-relevant design decisions

Properties you can rely on, each checkable against the tree rather than
taken on trust:

| Property | How to confirm it |
|---|---|
| **No `unsafe` anywhere, and the compiler enforces it.** Every crate root carries `#![forbid(unsafe_code)]` — libraries, both binaries, every example, the benchmarks, and the fuzz targets. `forbid` cannot be locally overridden by an `allow`, so this is a build failure rather than a convention | `grep -rn 'unsafe' er7/src er7-redact/src serde-er7/src` returns only the `forbid` attributes themselves |
| **No runtime dependencies in `er7`**, one in `er7-redact` (`er7`), two in `serde-er7` (`serde`, `er7`) | Each `Cargo.toml`; `er7`'s empty table is enforced by the test `the_crate_has_no_runtime_dependencies` |
| **No build scripts.** Nothing runs at compile time | There is no `build.rs` in any crate |
| **Library code performs no I/O at all** — no filesystem, no network, no process spawning, no environment reads, no logging, no telemetry | `grep -rn "std::fs\|std::net\|std::env\|std::process" er7/src er7-redact/src serde-er7/src` matches only `main.rs` |
| **The command-line tools read only what you name** — a file argument or standard input — and write only to standard output or `-o` | `er7/src/main.rs`, `er7-redact/src/main.rs`. No configuration file is searched for and no environment variable is consulted. |
| **Fuzzed on the untrusted-input surface** | `er7/fuzz/`: `parse_roundtrip`, `parse_with_total`, `escape_roundtrip`, `query_paths` — four targets |
| **Every dependency's license and every advisory against it is checked**, on push and again every Monday whether or not anything pushed | `deny.toml`; run `cargo deny check` from the root and from `er7/fuzz/`, or read the `deny` job in `.github/workflows/ci.yml` and `.github/workflows/audit.yml` |
| **CI has a real hosted track record, not a single proof-of-concept run** | `gh run list -R er7-rust/er7-rust --workflow=ci.yml --limit 200 --json conclusion`: 34 hosted runs of `.github/workflows/ci.yml` as of 2026-08-30, 33 green. The one failure (`61eb30d`) was a flaky CLI-test race — a run that fails while parsing its arguments exits before reading standard input, closing a pipe the test helper was still writing to — root-caused and fixed the same day (`574daaf`), not swept aside |

**The one thing worth knowing before you log an error.** Error values are
deliberately narrow, and none of them carries a field value from a message
body. One variant does carry text taken from the input:
`er7::Error::MissingHeader` carries the leading ASCII-alphanumeric run of
the first line, so that the message can say what it found instead of a
header. Feed a file that is not ER7 at all and that run is that file's
content. If you log the whole error, you log it.

Everything else is bounded: `BadHeader` names delimiter characters from the
header, `BadPath` quotes the path *you* supplied, and
`er7_redact::Error::BadPolicy` names a line of *your own* policy file. The
`--report` output carries paths and actions and no values by design
([§8](er7-redact/spec/08-report/index.md)), which is what makes it safe to
attach to a ticket.

## The attack surface, concretely

If you are threat-modelling a deployment, this is the whole of it:

1. **The bytes you hand `er7::parse`.** Untrusted. This is the surface the
   fuzz targets exercise.
2. **The policy file you hand `er7-redact`.** Trusted — it is operator
   configuration, in your control, and it can only remove or replace
   values, never execute anything.
3. **The Serde format you route a message through** with `serde-er7`. That
   format crate is yours to choose and yours to audit; this crate is
   deliberately format-agnostic and depends on no format crate at all.

There is no fourth. Nothing here listens, connects, or persists.

## Never send a real message

An HL7® v2 message is a clinical record. A vulnerability report is still a
report, and a real message in one is a data breach regardless of intent.

Redact before you send — the tool is in this repository:

```sh
er7-redact message.er7            # a redacted message, same shape
er7-redact --report message.er7   # paths and actions, and no values at all
```

Structure is what reproduces a parsing bug: delimiters, field positions,
repetition separators, component depth. Names, identifiers, dates of birth,
and addresses are not. If a finding genuinely depends on a specific byte
sequence in a value, describe the byte sequence rather than the record it
appeared in.

A real message sent to this project will be deleted rather than used, and
the report it arrived with may have to be deleted with it. That helps
nobody, so redact first.

## Disclosure

Coordinated, with a deadline that favours users over tidiness:

1. You report privately.
2. The maintainer confirms or explains why it is not a vulnerability.
3. A fix is released, and the advisory is published on GitHub with credit.
4. **90 days after your report, you should disclose publicly whether or not
   a fix exists.** That is not a request for permission — it is this
   project stating in advance that it will not ask you to sit on a finding
   indefinitely, because a one-maintainer project is exactly the kind that
   might go quiet.

If a vulnerability is already being exploited, or is already public,
everything above compresses: report it, disclose it, and expect the fix to
follow rather than precede.

## What this project does not have

Named rather than quietly omitted, because their absence is information for
a security review — and the same list appears in
[`MAINTAINERS.md`](MAINTAINERS.md), where it belongs to the continuity
question:

- **No key escrow for the code-signing key.** A successor generates their
  own. Commits and tags are signed and GitHub-verified as of 2026-08-27;
  see [`MAINTAINERS.md`](MAINTAINERS.md) for the key and the verification
  evidence.
- **No Trusted Publishing.** Every crate still publishes with a long-lived
  crates.io API token sitting on the maintainer's own machine, not a
  short-lived OIDC credential issued per CI run. The intent to adopt it is
  recorded, not silently deferred — see
  [`spec/trusted-publishing/index.md`](spec/trusted-publishing/index.md)
  and [`MAINTAINERS.md`](MAINTAINERS.md) for why it waits on every forge
  this repository publishes to, not just the first one that supports it.
- **No bug bounty, and no security budget.**
- **No second responder.** One email address, one person.
- **No third-party audit.** Nobody has reviewed this code but its author
  and whoever reads it next.
- **No certification of any kind.** These crates are not certified,
  accredited, validated, or assessed by HL7 International or anyone else,
  and they are not a medical device.

If your organisation needs stronger assurances than that list allows, the
honest advice is in [`MAINTAINERS.md`](MAINTAINERS.md): pin a version, keep
a fork you can build, and budget for maintaining it. The crates are small,
dependency-light, and specified precisely so that this is cheap.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
