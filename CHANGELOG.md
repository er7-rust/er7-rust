# Changelog

Notable changes to this workspace, newest first.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
loosely, with one deliberate departure: **this workspace has no single
version number.** Three crates version independently on crates.io, so each
entry below is dated and names the crate versions that carried it. `cargo`
resolves per crate; a date here is what a person needs to line up "the
release where X changed" with what is in their `Cargo.lock`.

Every crate follows [Semantic Versioning](https://semver.org/). While a
crate is `0.x`, a minor bump is the one allowed to break — including a
raised minimum supported Rust version, which is always a breaking change
and never lands in a patch
([`spec/rust-msrv-n-minus-3/index.md`](spec/rust-msrv-n-minus-3/index.md)).

## Unreleased

### Fixed

- **A latent race in both CLI test helpers**, which CI caught on the 0.1.3
  release commit while ten local runs had passed. A run that fails while
  reading its arguments exits before it reads standard input at all, which
  closes the pipe the test is still writing to; the helper treated that
  `BrokenPipe` as fatal instead of as the command behaving correctly. Test
  code only — `tests/` does not ship in any published crate, so no release
  is needed.

## 2026-08-26 — `er7` 0.1.3, `serde-er7` 0.1.3, `er7-redact` 0.2.1

Documentation, policy, and build posture. **No API changed**, no `Error`
variant was added, and no command-line option or outline label format
moved, so all three are patch releases under
[`er7` §14.2](er7/spec/14-compatibility-and-versioning/index.md).

What reaches a user of the published crates: the `®` on the first use of a
word mark in every doc comment and both `--help` outputs, a trademark
notice in each crate's rendered documentation and README, the corrected
crate descriptions on crates.io, and `#![forbid(unsafe_code)]` on every
crate root.

### Added

- **`#![forbid(unsafe_code)]` on every crate root** — the three libraries,
  both binaries, all 17 examples, the benchmark crate and its bench target,
  and all three fuzz targets. `forbid` rather than `deny`, so an
  `#[allow(unsafe_code)]` further down cannot reopen it: `unsafe` is now a
  compile error rather than a review comment. Verified by inserting one and
  watching the build fail. Policy and reasoning in
  [§1.2](spec/01-family-policy/index.md), with each crate's own build
  section pointing at it.
- **[`SECURITY.md`](SECURITY.md)**: how to report a vulnerability, and what
  is honestly promised about handling one. It names three in-scope
  categories — denial of service through parsing, redaction failure, and
  silent corruption — and lists six documented limitations that are
  explicitly *not* vulnerabilities, each pointing at the spec section that
  records the reasoning. It publishes six checkable properties (no
  `unsafe`, no build scripts, no I/O from library code, and the dependency
  counts) with the command that confirms each, and discloses the one place
  message-derived bytes reach an error string:
  `er7::Error::MissingHeader` carries the leading alphanumeric run of the
  first line, so logging that error logs it. Disclosure is coordinated with
  a ninety-day deadline that applies **whether or not a fix exists**,
  because a one-maintainer project is exactly the kind that might go quiet.
- **[`GOVERNANCE.md`](GOVERNANCE.md)**: who decides, and what constrains
  them. It names the model — a single maintainer — rather than describing
  committees that do not exist, and then sets out the five checkable
  constraints that make that survivable, the strongest being that the
  specification is the authority: an argument citing a rule ID is one the
  person holding the merge button cannot wave away. Also the three groups
  most declines fall into, release authority, the layer boundary with the
  sibling `hl7-rust` project, and why forking is treated as legitimate
  continuity rather than a hostile act.
- **[`CONTRIBUTING.md`](CONTRIBUTING.md)** at the workspace root: how to
  help with time, with code, or with money. It leads with never pasting
  patient data — and points at `er7-redact --report`, which prints paths
  and actions with no values, as the thing to attach instead. It has a
  table of eight ways to help that need no Rust, the checks to run before a
  pull request, and a **Money** section that names the donation channels
  and then says what money does *not* buy: no support, no service level, no
  feature, no influence over the specification, and no fix for a bus factor
  of one.

  The near-empty per-crate `CONTRIBUTING.md` files now defer to it rather
  than restating it, and [`MAINTAINERS.md`](MAINTAINERS.md) and
  [`AI_STATEMENT.md`](AI_STATEMENT.md) point at the root file.
- **[`RFC.md`](RFC.md)**: twelve questions this project genuinely cannot
  answer for itself, because it is one maintainer with no production
  traffic. Each question names the specification section that records it
  and says what a real answer would change — from "does the value tree
  match the messages you actually receive" to "is the zero-dependency
  stance load-bearing, or is it theatre". It also lists the feedback that
  cannot be acted on, and the decisions already made with reasoning
  attached, so that a "no" can be argued with rather than repeated.
- **Five new website pages**, so the documents an evaluation asks for are
  readable without cloning the repository:
  [`/install/`](https://er7-rust.github.io/install/),
  [`/comparison/`](https://er7-rust.github.io/comparison/),
  [`/benchmarks/`](https://er7-rust.github.io/benchmarks/),
  [`/news/`](https://er7-rust.github.io/news/), and
  [`/trademarks/`](https://er7-rust.github.io/trademarks/). Install,
  Comparison, and Benchmarks joined the header navigation; News and
  Trademarks are in the footer, which also now carries the trademark
  disclaimer on every page of the site. `sitemap.xml` lists all fifteen
  routes.
- **A trademark policy, applied across the whole tree**
  ([`spec/hl7-trademarks-fair-use/index.md`](spec/hl7-trademarks-fair-use/index.md)).
  HL7® is someone else's word mark, and HL7 International's fair-use terms
  ask for the ® on a mark's first use per page and for a disclaimer
  wherever the marks appear. Rules T1–T6 state what that means here, §4.3
  defines what a "page" is in a repository, and `bin/check-trademarks` —
  also `make check-trademarks` — enforces T1, T2, and T3 so the rule
  cannot drift.

  Applied to every Markdown file, every website route, every Rust doc
  comment, the three crate descriptions, and both `--help` outputs.
  Deliberately *not* applied to sample messages, error strings, citation
  blocks, code identifiers, or crates.io keywords; §4.3 and §4.5 say why.
  [`TRADEMARKS.md`](TRADEMARKS.md) is the canonical notice.
- **[`spec/promote/index.md`](spec/promote/index.md)**: researched
  channels for reaching HL7 and Rust professionals — the HL7 Zulip, the
  Mirth and InterSystems communities, This Week in Rust, trade press,
  conferences — with a ninety-day sequence, message templates, and the
  etiquette rules that keep the project welcome in professional
  communities.
- **[`COMPARISONS.md`](COMPARISONS.md)**: interface engines, HAPI and the
  mature libraries, the other Rust HL7 crates with their crates.io figures,
  the `hl7-rust` sibling family, and hand-rolled pipe splitting — including
  four cases where you should choose something else.
- **[`BENCHMARKS.md`](BENCHMARKS.md)**: measured figures with confidence
  intervals, on a named machine and toolchain, from benchmarks in this
  repository that anyone can run — plus the optimisation history and four
  rules for how not to read the numbers.
- **[`INSTALL.md`](INSTALL.md)**, **[`NEWS.md`](NEWS.md)**,
  **[`MAINTAINERS.md`](MAINTAINERS.md)**,
  **[`AI_STATEMENT.md`](AI_STATEMENT.md)**,
  **[`LICENSE.md`](LICENSE.md)** at the workspace root,
  **`CITATION.cff`** (with ORCID), **`CODEOWNERS`**, and this file.

### Changed

- [`SECURITY.md`](SECURITY.md)'s "no `unsafe`" row is now a compiler
  guarantee rather than an observation, and its verification command
  changed accordingly — `grep -rn unsafe` now matches the `forbid`
  attributes themselves, so the row says to expect exactly those.
- Three broken in-page anchors fixed, found while link-checking the new
  files: `er7` §2 pointed at §2.8 for its sources when they are §2.9, and
  two `er7-redact` anchors omitted the trailing rule ID that GitHub keeps
  in a heading's slug.
- [`MAINTAINERS.md`](MAINTAINERS.md) no longer says there is no published
  security policy, because there now is one. The gap it discloses in that
  place is narrower and truer: no *second* security responder.
- The root [`index.md`](index.md) now indexes the project-level documents
  above, so a reader landing on the repository finds them without hunting.
- The site's `version` constant was stale at 0.1.1 and now reads 0.1.2, and
  the about page's stated MSRV was stale at 1.85 and now reads 1.95 — the
  N-3 policy's actual floor. Its test count moved 126 → 127.
- The three crate `description` fields now read "HL7® v2". This is
  published metadata, so it reaches crates.io on the next release of each
  crate rather than immediately.

## 2026-08-25

Documentation only.

- The website's source lives in this repository, at
  `er7-rust.github.io/`, and the root documentation now says so rather
  than implying a sibling repository.
- Every specification section moved into a directory of its own, so a
  section can grow an example or a diagram without becoming a second file
  in a flat list.

## 2026-08-24

### Added

- **The website's history merged into this monorepo.** A change to a
  crate's public surface and the page that teaches it can now land in one
  commit, so a page that still documents a removed API is a broken change
  rather than a follow-up.
- **`make publish`**: one command that pushes the monorepo and then splits
  the site subdirectory out to the repository GitHub Pages deploys from.
  Forgetting the second push was the easy mistake, and it is now guarded:
  a dirty tree or a branch other than `main` stops the whole thing before
  the first push happens.

### Changed

- The site's deploy moved off Node 20.
- The accept-by-default and reject-by-default postures of a redaction
  policy are now documented on the website, not only in the spec.

## 2026-08-23 — `er7-redact` 0.2.0

### Changed

- **Breaking.** A policy now states its posture explicitly: accept by
  default, or reject by default. `Policy::accept_all()` runs its rules over
  a message and leaves everything else alone; `Policy::all_but_the_header()`
  rejects every value that no rule accepts, keeping `MSH` intact. The
  second is the one to reach for when the message is unfamiliar, because
  the failure mode of a missed rule is a redacted value rather than a
  leaked one.

  This is why the release is `0.2.0` and not `0.1.3`: existing policies
  must now say which posture they mean.

## 2026-08-21 — `serde-er7` 0.1.2

### Changed

- The minimum supported Rust version is pinned at 1.95 in the manifest,
  matching the other two crates.

## 2026-08-20 — `er7` 0.1.2, `er7-redact` 0.1.2

### Added

- **Fuzz targets** for `er7`: `parse_roundtrip`, `escape_roundtrip`, and
  `query_paths`, under [`er7/fuzz/`](er7/fuzz/).
- **Criterion benchmarks**, in the unpublished workspace member
  [`er7-bench/`](er7-bench/) — deliberately not in `er7` itself, so that
  `er7` can keep both `[dependencies]` and `[dev-dependencies]` empty.
  Figures in [`BENCHMARKS.md`](BENCHMARKS.md).

### Changed

- **`query` no longer walks the whole message to return one value.** It
  returns on first match. Measured against a 402-segment message, this cut
  `query/field` by 85% and `query/subcomponent` by 92%: a single lookup is
  now effectively independent of message length.
- The minimum supported Rust version policy — current stable minus three
  releases — is pinned across the workspace as `rust-version = "1.95"`.

### Fixed

- A second input file on the command line is now reported as an error
  rather than silently ignored while the tool read standard input instead.

## 2026-08-19 — The workspace

### Added

- **Three separate repositories became one Cargo workspace**, with their
  histories preserved and still walkable under their own directories.
  `er7-redact` and `serde-er7` now reach `er7` through a path dependency,
  so a change to `er7` is picked up by its siblings immediately, without
  publishing.
- **A workspace-level specification** at [`spec/`](spec/): the dependency,
  testing, safety, and MSRV policy that all three crates genuinely share,
  stated once so it cannot drift across three restatements.

### Fixed

- Source links, example filenames, and `CITATION.cff` targets that the
  merge had made stale or ambiguous.

## 2026-08-18 — `er7` 0.1.1, `er7-redact` 0.1.1, `serde-er7` 0.1.1

### Changed

- **Clippy's pedantic lints are on across all three crates**, and the
  checks run with `-D warnings`, so a pedantic finding fails the build like
  any other. Turning them on found 48, 34, and 5 issues respectively; all
  are fixed.
- **Every specification rule is now bound to the test that enforces it**,
  and the coverage table that states so is itself enforced by a test.

## 2026-08-17 — `er7-redact` 0.1.0

First release. Remove patient detail from an ER7 message without changing
its shape: same segments, same fields, same components, same delimiters, so
every path that resolved to a value still resolves to one.

## 2026-08-16 — `serde-er7` 0.1.0

First release. Serialize and deserialize `er7` message trees through Serde,
so a parsed message can flow through JSON, YAML, or any other Serde data
format and come back out unchanged.

## 2026-08-15 — `er7` 0.1.0

First release. Parse, query, edit, and write HL7 v2 messages in the ER7
pipe-hat encoding, with zero dependencies.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
