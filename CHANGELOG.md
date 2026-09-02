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
([`spec/rust-msrv-n-minus-2/index.md`](spec/rust-msrv-n-minus-2/index.md)).

## 2026-09-02 — `serde-er7` 0.3.0

### Added

- **`Strict<T>` (`T` in `Message`, `Segment`, `Separators`): opt-in
  `deny_unknown_fields`-style deserialization.** The plain types keep
  rule S8's tolerance unconditionally — deserializing `Message`, `Segment`,
  or `Separators` directly still ignores a key it does not recognize, the
  same as before this release. `Strict<T>` is a separate, additive entry
  point for a caller who wants the opposite for one particular call, such
  as validating a hand-written JSON fixture: it reports an unrecognized
  key with `serde::de::Error::unknown_field`, naming the key and what it
  could have meant, instead of ignoring it. Strictness nests —
  `Strict<Message>` also catches a typo inside a segment or the separators
  object it contains, not only at the top level. Two distinct things
  improve over the plain type: a typo on a *required* key now names the
  actual mistake instead of a generic "missing field" that never mentions
  it, and a typo on the one *optional* key this crate has
  (`Separators::truncation`) is caught at all, where the plain type
  silently defaults it to `None`. Follows the same `Deref`/`DerefMut`/
  `From`-both-ways convention every other wrapper type in this crate
  carries, plus a delegating `Serialize`. S13,
  [`serde-er7` §11](serde-er7/spec/11-strict-mode/index.md). New example:
  `examples/catch_a_typo_with_strict.rs`.

## 2026-08-30 — `er7-redact` 0.4.0

### Changed

- **Breaking: `Action` gained a ninth variant, `Custom(CustomAction)`.**
  `Action` is not `#[non_exhaustive]` (spec §13.1 lists adding a variant as
  breaking), so a caller matching it exhaustively needs a new arm.
  `Action::custom(f)` is the usual way to reach it: the same signature as
  `Action::apply` itself (`&str, u64) -> Option<String>`), because it runs
  through that same call — a rule naming a position with a custom action
  runs at exactly the point a built-in one would. `CustomAction` is a
  newtype around `Arc<dyn Fn(&str, u64) -> Option<String> + Send + Sync>`,
  with hand-written `Debug` (a fixed placeholder — nothing truthful to
  print about a closure), `Clone` (an `Arc` clone), and `PartialEq`/`Eq`
  (`Arc::ptr_eq` — identity, not behavior, since there is no general way to
  compare closures). `Display` writes the fixed placeholder `<custom>`,
  which is not a §6.2 keyword and does not re-parse to an equal policy — a
  real, permanent exception to that guarantee, alongside the pre-existing
  empty-`Replace`/clear one, since a closure has no text to spell.
  Idempotence (D10) is not claimed for it either: provable for the
  built-in eight because this crate wrote all eight; a caller's own
  closure is the caller's property to prove. D24,
  [`er7-redact` §3.8](er7-redact/spec/03-actions/index.md). Closes T2.

### Added

- **`er7-redact`: `examples/date_shift_with_a_custom_action.rs`.** T5
  asked for a ninth built-in action — parse an HL7® timestamp, shift it by
  a per-patient offset derived from the pseudonym key, write it back at
  the same precision — and was declined as a built-in on the same grounds
  as pattern matching ([§16.2](er7-redact/spec/16-open-questions-and-declined-decisions/index.md)):
  it would be the first action whose correctness depends on timestamp
  *format* knowledge (leap years, variable precision) rather than
  redaction logic. `Action::custom` (D24, shipped alongside it) answers
  T5's three open questions for free — where the per-patient key comes
  from, what happens to an unparseable timestamp, whether parsing a
  timestamp crosses the "no dictionary knowledge" line — so the example
  builds the whole thing on published API: a from-scratch
  proleptic-Gregorian calendar, verified round-trip correct for 146,000
  consecutive days, and a per-patient offset from this crate's own
  `pseudonym()`. Documentation only; no API surface changed beyond D24
  itself.
  [`er7-redact` §14.4](er7-redact/spec/14-roadmap/index.md).

## 2026-08-29 — `er7` 0.2.1

Patch: purely additive, no existing signature changed.

### Added

- **`Segment::first_value(field, component, &separators) -> Option<String>`
  (R26).** The decoded text of one field's first repetition and
  subcomponent, treating an empty result as absent — the same three things
  `Message::query` does, scoped to a segment already in hand rather than
  searched for by path. Raised by the `hl7-2-5-to-xml-using-rust` and
  `hl7-2-5-to-json-using-rust` port (T5, shipped in 0.1.0): both wrote the
  identical eight-line helper to read `OBX-2` while iterating a message's
  own `OBX` segments, evidence [`er7` §10.2](er7/spec/10-msh-conveniences/index.md)
  asks for before adding a convenience. [`er7` §5.4](er7/spec/05-value-tree/index.md).
  Closes T8.

## 2026-08-29 — `er7` 0.2.0, `serde-er7` 0.2.0, `er7-redact` 0.3.0

All three minor releases, for the same reason: the workspace MSRV moved,
which never lands in a patch. `er7-redact` carries two more breaking
changes of its own on top of that.

### Changed

- **Breaking, all three crates: the MSRV moved from N-3 to N-2 — 1.95 to
  1.96.** `rust-version` bumped in each `Cargo.toml`; nothing else in the
  workspace's own CI needed to change, since the `msrv` job already reads
  the pinned value from `er7/Cargo.toml` at run time and cross-checks the
  other two crates agree, rather than carrying a second hard-coded copy.
  Verified against the real `1.96` toolchain, not just declared:
  `cargo +1.96 check --workspace --all-targets` and
  `cargo +1.96 test --workspace` both clean.
  [`spec/rust-msrv-n-minus-2/index.md`](spec/rust-msrv-n-minus-2/index.md).
- **Breaking, `er7-redact` only: a value found at a named position is now
  redacted wherever else it appears, by default.** `Policy` gained a new
  public field, `search_known_values` (defaults to `true`), which breaks
  a struct literal built from `Policy`'s old field set — the same trap
  every other all-`pub`-fields struct in this family carries. More than
  the field: `Redactor::redact` on an existing policy can now change a
  leaf it left alone before, wherever that leaf repeats a value already
  found at a named position, case-insensitively and only as a whole word.
  The policy file format gained a fourth reserved word, `known-values
  on`/`known-values off`, to turn it off for a policy that should only
  ever redact by position. D23,
  [`er7-redact` §2.10](er7-redact/spec/02-redaction-model/index.md).
  Closes T1.

### Added

- **`er7-redact`: `Redactor::uncovered`, and the CLI's `--uncovered`
  flag.** Reports every leaf that carries text and is named by no rule —
  the set a rejecting posture already computes internally to decide what
  to blank, now exposed on its own, and independent of the policy's
  posture. Additive; does not change what an existing caller's `redact`
  does. D22, [`er7-redact` §2.9](er7-redact/spec/02-redaction-model/index.md).
  Closes T6.
- **`er7`: `examples/build_a_message.rs`**, showing why there is still no
  builder API — `parse_with` is the builder for anything already
  expressible as ER7 text, which an ACK almost always is. Documentation
  only; no API surface changed.
  [`er7` §5.5](er7/spec/05-value-tree/index.md). Closes T7.

## 2026-08-26 — `er7` 0.1.4, `serde-er7` 0.1.4, `er7-redact` 0.2.2

Metadata only. **No API changed**, no `Error` variant was added, and no
command-line option or outline label format moved, so all three are patch
releases under
[`er7` §14.2](er7/spec/14-compatibility-and-versioning/index.md).

### Changed

- **Every crate description now carries the verbatim HL7® trademark
  disclaimer**, followed by "This project is an independent work." — the
  three-part shape the family converged on, applied here by owner
  directive. A crates.io page has no shared footer, so rule T2 of
  [`spec/hl7-trademarks-fair-use/`](spec/hl7-trademarks-fair-use/index.md)
  needs the disclaimer in the description itself; `bin/check-trademarks`
  now enforces exactly that for every publishable crate (verified by
  breaking one description and watching the check fail).

### Fixed

- **A latent race in both CLI test helpers**, which CI caught on the 0.1.3
  release commit while ten local runs had passed. A run that fails while
  reading its arguments exits before it reads standard input at all, which
  closes the pipe the test is still writing to; the helper treated that
  `BrokenPipe` as fatal instead of as the command behaving correctly. Test
  code only — `tests/` does not ship in any published crate, so this rides
  along rather than motivating the release.

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
