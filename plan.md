# Plan — `er7` Rust workspace

Goal: a production-grade, spec-driven Rust workspace for the ER7 encoding of
HL7® v2 — the `er7` parser (zero runtime dependencies, enforced by a test),
`serde-er7`, the `er7-redact` tool, and the isolated `er7-bench` —
professionalized for its real audience: healthcare professionals and the
engineers who serve them, worldwide, in settings where a wrong claim has
clinical cost.

Method: **specification-driven development.** Behavior lives in per-crate
`spec/` directories before it is implemented; cross-cutting policy lives in
the workspace `spec/` (family policy, trademark fair use, MSRV N−3). Per-crate
roadmaps and open tasks stay where they are
(`er7/spec/16-roadmap/`, `er7/spec/17-open-tasks/`,
`er7-redact/spec/14-roadmap/`, `15-open-tasks/`,
`serde-er7/spec/09-roadmap-and-open-questions/`) — this file holds only what
none of them can: the workspace-level professionalization work. Execution
items live in [`tasks.md`](tasks.md), where a `[x]` means verified, not
intended.

## Where the workspace stands (verified 2026-08-26)

Three crates released independently on crates.io (er7 0.1.2, serde-er7 0.1.2,
er7-redact 0.2.0, per `CHANGELOG.md`). The root document set is nearly
complete and unusually substantive — SECURITY.md documents checkable
properties with the commands that check them, TRADEMARKS.md has a mark-by-mark
table, and `bin/check-trademarks` enforces the fair-use rules mechanically.
The trademark-compliance sweep — 100+ files extending compliance into
crate doc-comments, manifests, and the site copy — landed 2026-08-26.

The honest part of the document set is that it names its own gaps —
no CI, unsigned commits, no second security responder — rather than implying
they are covered. This plan closes those gaps deliberately instead of leaving
them as standing confessions.

## Workstreams — professionalization (2026-08 onward)

Six workstreams, shared with the sibling repositories (`hl7-rust`,
`fhir-rust`, `snomed-rust`, `openehr-rust`) so the family converges on one
posture. Open items for each are in `tasks.md`.

1. **Governance.** GOVERNANCE.md, MAINTAINERS.md, and RFC.md exist and are
   candid about the single-maintainer model. The placement gap is closed
   (2026-08-26): the code of conduct now lives at `/CODE_OF_CONDUCT.md`,
   where GitHub's community-health detection reads it, with a
   claim-accuracy clause added and the three linking documents repointed.

2. **Compliance — licensing and trademarks.** The strongest of the five
   repositories here: TRADEMARKS.md, a checker, and an in-flight sweep
   extending coverage to rustdoc and crate metadata. The checker now runs
   in CI (`.github/workflows/ci.yml`) as well as from `make
   check-trademarks`, and the sweep landed 2026-08-26. The `LICENSES/`
   directory with the five full license texts landed 2026-08-26 too
   (REUSE convention, copied from `fhir-rust/LICENSES/`).

3. **Security and supply chain.** SECURITY.md is the family's best (in-scope
   categories, documented non-vulnerabilities with spec citations, a
   disclosed leak path, a 90-day disclosure deadline). CI now exists
   (`.github/workflows/ci.yml`, 2026-08-26): the four checks in
   `spec/01-family-policy/index.md` §1.2, an MSRV build, and the trademark
   checker — first hosted run still pending, and the fuzz targets are still
   laptop-only. Plus unsigned commits and tags, no
   `cargo deny`/`cargo audit`, no SBOM.

4. **Privacy and patient data.** `er7-redact` is a PHI tool, and its claims
   are scoped correctly — "a starting point, not a compliance certification"
   appears in the spec, SECURITY.md, and the CLI usage text. As of
   2026-08-26 the dispersed claims are consolidated in a root `PHI.md` a
   privacy officer can read, including the explicit HIPAA Safe Harbor
   18-identifier coverage/non-coverage table (with categories 12–18 and
   free text honestly marked as uncovered).

5. **Outreach.** The site (`er7-rust.github.io/`, in-repo) publishes
   trademarks, news, and comparison pages — but none of the governance
   surface (security, governance, maintainers, RFC, AI statement), so the
   professionalization work is invisible to anyone who does not open the
   repository. CONTRIBUTING.md's "Money" section's donation routes are
   surfaced by `.github/FUNDING.yml` as of 2026-08-26.

6. **Audit and harmonization.** No findings register and no plan/tasks
   history — this file and `tasks.md` are the start. Family conventions to
   converge on: the canonical special-files list (the local
   `spec/special-files-for-public-repos/index.md` omits four entries the
   `fhir-rust` version carries), and per-crate document parity (`serde-er7/`
   has a CITATION.cff as of 2026-08-26, but still lacks the
   conduct/contributing files its siblings have).

## Open decisions (awaiting a call, not code)

- **Free-text scanning in `er7-redact`** — its own roadmap
  (`er7-redact/spec/14-roadmap/index.md` §14.2) calls this "the largest real
  gap". A capability decision, not professionalization; it stays in the
  crate's roadmap, and [`RFC.md`](RFC.md) Q13 now asks operators the
  question that would settle the design.
- **CI hosting shape** — decided by doing, 2026-08-26: one root workflow
  (`.github/workflows/ci.yml`, no containers), per-crate lanes declined.
  Fuzz smoke is not in it yet; `tasks.md` tracks that separately.

## Non-goals (for now)

- No new crates until the professionalization workstreams are closed.
- No interface-engine ambitions; parsing, mapping, and redaction only.

## Risks & watch items

- The trademark sweep is committed, but the *published* crates predate it:
  the compliance posture in crate doc-comments and manifests reaches
  crates.io only with each crate's next release.
- SECURITY.md's "checkable properties" now have a CI workflow behind them,
  but it has not yet had a hosted run; until one goes green they remain
  claims, not guarantees.
- Pseudonyms in `er7-redact` are FNV-1a and documented as not a security
  primitive — any outreach that touches de-identification must repeat that
  framing, not soften it.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
