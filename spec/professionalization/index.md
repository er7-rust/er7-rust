[er7-rust](../../index.md) → [spec](../index.md) → professionalization

# §5 Professionalization

This specification defines what "professional" means for this repository and
binds the maintainer as much as any contributor. The audience is healthcare
professionals and the engineers who serve them, worldwide, in production use;
the standing constraint is that a wrong claim in this domain has clinical
cost. Rationale and current execution state live in [`plan.md`](../../plan.md)
and [`tasks.md`](../../tasks.md); this file holds the rules.

## Rules

1. **Plans are files, and a checked box is a verified fact.** `plan.md` and
   `tasks.md` exist at the repository root. A `[x]` means the work was done
   and verified, with the evidence named — never that it is intended,
   assumed, or inherited from a sibling repository.
2. **The special files exist and stay accurate.** The canonical list is
   [`spec/special-files-for-public-repos/`](../special-files-for-public-repos/index.md).
   Every countable claim in those files (crate counts, test counts, coverage
   lists, "X is enabled/disabled") is measured before it is written and
   re-verified when cited.
3. **Self-declared gaps are promises.** A gap named in SECURITY.md,
   MAINTAINERS.md, or AI_STATEMENT.md ("no CI", "unsigned commits") is either
   closed or consciously accepted in `tasks.md` — and the declaring document
   is updated in the same change that closes it.
4. **CI enforces what documents claim.** Every check a document says this
   repository runs (tests, clippy, fmt, MSRV, trademark rules, doc gates)
   runs in CI on every push. A laptop-only check is a claim, not a guarantee.
5. **Trademark discipline.** The word marks this repository's pages use are
   owned by Health Level Seven International — HL7® above all; the full
   mark list, the quoted fair-use terms, and rules **T1–T6** (registration
   mark on first use per page, the verbatim disclaimer, the standard-name
   phrasing, no mark in this project's own names, no implied endorsement,
   `TRADEMARKS.md` as the canonical notice) are
   [`spec/hl7-trademarks-fair-use/`](../hl7-trademarks-fair-use/index.md).
   `bin/check-trademarks` enforces T1, T2, and T3 mechanically over every
   Markdown page, website route, Rust doc-comment, crate description, and
   CLI usage text; it runs locally as `make check-trademarks` and in CI as
   the `trademarks` job of
   [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml).
6. **Patient data is addressed in plain language.** `PHI.md` at the root
   states what the software does and does not do with patient data, for a
   reader who is a privacy officer, not a Rust programmer. It never claims
   compliance or certification.
7. **Conduct has a document and a path.** `CODE_OF_CONDUCT.md` at the root
   (Contributor Covenant 2.1 plus this family's claim-accuracy clause:
   overstating what the software does is a conduct matter, not only a bug).
8. **Harmonization runs through the family.** The sibling repositories
   (`hl7-rust`, `er7-rust`, `fhir-rust`, `snomed-rust`, `openehr-rust`)
   share these rules, the special-files list, and the six workstreams
   (governance; compliance — licensing and trademarks; security and supply
   chain; privacy and patient data; outreach; audit and harmonization).
   Conventions sync from the repository that owns the canonical copy rather
   than drifting independently.
9. **Outreach is gated.** No promotion while a rule above is unmet for the
   surface being promoted; `help/outreach/index.md` names the prerequisites.

## Status in this repository

Assessed 2026-08-26, updated 2026-08-27 and 2026-08-30, by reading the
tree rather than remembering it.

| Rule | State | Evidence |
| ---- | ----- | -------- |
| 1 | **Met** | `plan.md` and `tasks.md` at the root, committed 2026-08-26; every `[x]` added since names its evidence in the line |
| 2 | **Met** | The special files exist at the root (SECURITY.md, MAINTAINERS.md, GOVERNANCE.md, CONTRIBUTING.md, AI_STATEMENT.md, PHI.md, CODE_OF_CONDUCT.md, TRADEMARKS.md, RFC.md, CODEOWNERS, …) — every entry checked against the filesystem 2026-08-27, not assumed from the list. The local [`spec/special-files-for-public-repos/index.md`](../special-files-for-public-repos/index.md) was diffed line by line against the canonical `fhir-rust` copy the same day: one wording gap (a missing "2.1" on the CODE_OF_CONDUCT.md entry), fixed, and one deliberate local addition (TRADEMARKS.md) that the file itself already disclosed. Not "four entries missing" — that description was itself stale by the time this row was re-checked |
| 3 | **Met as practice** | The "no CI" gap declared in MAINTAINERS.md, AI_STATEMENT.md §7/§12, SECURITY.md, and CONTRIBUTING.md was closed 2026-08-26 with all four declaring documents updated in the same change; dependency auditing closed 2026-08-27 without ever having been declared as a gap in one of those three documents in the first place — `plan.md` tracked it instead, which this rule does not cover, and SECURITY.md's checkable-properties table gained the row rather than losing a declared one. The "unsigned commits" gap closed 2026-08-27 — signing is on, verified locally, and GitHub-verified with a live "Verified" badge once the maintainer registered the (dedicated, passphrase-protected) code-signing key's public half himself; MAINTAINERS.md and SECURITY.md were both corrected in the same change. Still open: no second responder, which has its own `tasks.md` item |
| 4 | **Met** | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) runs the four checks, an MSRV build (1.96 as of 2026-08-29, read dynamically from `er7/Cargo.toml` rather than a second hard-coded pin — checked against the manifests 2026-08-30, not assumed unchanged since the last pass), a fuzz-target build-and-smoke job, `bin/check-trademarks`, and (since 2026-08-27) `cargo deny` — on every push and pull request. Checked against `gh run list` 2026-08-27 rather than the earlier "not yet proven" wording: 13 hosted runs of this workflow, 12 green; the one failure (`61eb30d`) was a flaky CLI-test race, root-caused and fixed the same day (`574daaf`). A real track record now, not a single proof-of-concept run |
| 5 | **Met** | `bin/check-trademarks` passes across the tree (run 2026-08-26); the 100+-file sweep extending compliance into crate doc-comments, manifests, and site copy landed 2026-08-26 — what remains unpublished is only what waits for each crate's next crates.io release |
| 6 | **Met** | [`PHI.md`](../../PHI.md), added 2026-08-26: privacy-officer Q&A, the Safe Harbor coverage/non-coverage table, no compliance claim anywhere in it |
| 7 | **Met** | [`CODE_OF_CONDUCT.md`](../../CODE_OF_CONDUCT.md) at the root since 2026-08-26 with the claim-accuracy clause and the maintainer's address as the reporting path; upgraded to Contributor Covenant **2.1** 2026-08-27 — the only substantive change between the two versions is "caste, color," added to the Pledge's protected-characteristics list, applied verbatim against the official 2.1 text rather than paraphrased |
| 8 | **Met** | The rules, workstreams, and file set are shared with the family; the special-files list is back in sync (rule 2), and per-crate document parity was resolved 2026-08-26 — the root CONTRIBUTING.md/CODE_OF_CONDUCT.md/SECURITY.md cover all three crates, and each crate's README says so, rather than three drifting copies |
| 9 | **Met** | [`help/outreach/index.md`](../../help/outreach/index.md), added 2026-08-27: nine prerequisites checked against the tree that day, every one met or not applicable — the trademark row in particular, closed without ever needing the written-clearance step the sibling `hl7-rust` repository is still waiting on, since no name in this workspace uses a word mark. `spec/promote/` keeps the channel research; this file keeps only the gate. No promotion has occurred yet — the gate is open, not pulled |

The related list of *which* files this all applies to is
[`spec/special-files-for-public-repos/`](../special-files-for-public-repos/index.md);
this section owns the rules about how they are kept true.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
