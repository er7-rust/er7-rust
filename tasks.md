# Tasks

Execution checklist; rationale and workstreams live in [`plan.md`](plan.md).
A `[x]` here means the work is **verified done**, not intended — check items
off in the same change that completes them, with the evidence named.

Per-crate engineering tasks stay in the crate specs
(`er7/spec/17-open-tasks/`, `er7-redact/spec/15-open-tasks/`,
`serde-er7/spec/09-roadmap-and-open-questions/`); this file holds the
workspace-level professionalization work only.

## Done (verified 2026-08-26, the state this file starts from)

- [x] Three crates released independently on crates.io (er7 0.1.2,
      serde-er7 0.1.2, er7-redact 0.2.0), recorded in `CHANGELOG.md`.
- [x] Root document set exists and is substantive: GOVERNANCE.md, SECURITY.md,
      LICENSE.md, CONTRIBUTING.md, MAINTAINERS.md, AI_STATEMENT.md,
      TRADEMARKS.md, RFC.md, CODEOWNERS, CITATION.cff, NEWS.md,
      COMPARISONS.md, BENCHMARKS.md, INSTALL.md, CHANGELOG.md.
- [x] Trademark fair use is specified (`spec/hl7-trademarks-fair-use/`) and
      mechanically checked (`bin/check-trademarks`, rules T1/T2/T3), with a
      mark-by-mark TRADEMARKS.md.
- [x] PHI claims in `er7-redact` are scoped honestly and consistently:
      "a starting point, not a compliance certification" in the spec,
      SECURITY.md, and the CLI usage text; pseudonyms documented as not a
      security primitive.
- [x] SECURITY.md documents checkable properties with the commands that check
      them, a disclosed leak path (`er7::Error::MissingHeader`), and a 90-day
      coordinated-disclosure deadline.

## Next up

Grouped by `plan.md` workstream. Order within a group is priority order.

### In flight — land it first

- [x] **Commit the trademark-compliance sweep** — landed 2026-08-26
      (® on first prose use, disclaimer in pages, doc-comments, manifests,
      CLI usage; anchor fixes; site copy; 104 modified files, plus the
      `#![forbid(unsafe_code)]` posture and its family-policy section
      that travelled with it). `bin/check-trademarks` and the four checks
      all ran green immediately before the commit; the `# Trademarks`
      rustdoc sections were moved to sit right after each crate's opening
      summary, and the root `index.md` carries the notice near the top.

### Security and supply chain

- [x] **Stand up CI at the repository root** — done 2026-08-26:
      `.github/workflows/ci.yml` runs the four checks from
      `spec/01-family-policy/index.md` §1.2, an MSRV 1.95 build, and
      `bin/check-trademarks`; MAINTAINERS.md "What is not here yet" and
      AI_STATEMENT.md §7/§12 updated in the same change. YAML validated
      locally. Fuzz smoke was deliberately left out (next item, since
      done). **The first hosted run went green 2026-08-26** (run
      32987307797, head `77ad805`, all four jobs — the four checks, MSRV,
      fuzz smoke, trademarks — after GitHub Actions' outage that day
      delayed the push event by half an hour); the "not yet proven"
      wording in MAINTAINERS.md, SECURITY.md, GOVERNANCE.md,
      AI_STATEMENT.md, and plan.md was retired in the same change that
      records this.
- [x] **Add a fuzz smoke run to CI** — done 2026-08-26: a `fuzz` job in
      `.github/workflows/ci.yml` (nightly toolchain with the `rustfmt,
      clippy` components named explicitly — the `snomed-rust` job broke
      without them — then fmt, clippy, `cargo fuzz build`, and a 20-second
      seed run of each of the three targets). Verified locally first:
      fmt required reformatting `query_paths.rs` (done in the same
      change), clippy was clean, and all three targets built and ran a
      5-second smoke on nightly-aarch64-apple-darwin without findings.
- [x] **Turn on the repository-side security settings** — done 2026-08-26
      (not previously listed here, surfaced by SECURITY.md itself: it
      offered GitHub private vulnerability reporting while the setting
      was off). Enabled and verified by GET: private vulnerability
      reporting, dependency alerts, automated security fixes, and secret
      scanning; `.github/dependabot.yml` added with
      `open-pull-requests-limit: 0` on the cargo entries so dependabot
      opens security PRs only (the `fhir-rust` posture — its first hour
      with default limits opened 47 version-bump PRs). SECURITY.md
      records the correction in the same change.
- [ ] Tag releases and sign commits/tags going forward; record the posture
      change in MAINTAINERS.md.
- [x] **Add dependency auditing (`cargo deny`: advisories, licenses, bans,
      sources) on push plus a weekly cron** — done 2026-08-27: `deny.toml`
      at the root (found by cargo-deny from either workspace, since it
      walks up), allow-listing exactly the licenses this dependency tree
      carries — the workspace's own five, plus Unicode-3.0
      (`unicode-ident`, reached through criterion's `serde_derive`) and
      NCSA (`libfuzzer-sys`, in `er7/fuzz`'s separate workspace) — with
      `sources.unknown-registry`/`unknown-git` set to `deny` rather than
      the generated template's `warn`. `ci.yml` gained a `deny` job (two
      `EmbarkStudios/cargo-deny-action@v2` steps, one per manifest) on the
      existing push/PR trigger; a new `.github/workflows/audit.yml` reruns
      the same check on a Monday cron plus `workflow_dispatch`, matching
      `dependabot.yml`'s day. Along the way, fixed a real finding: the
      fuzz workspace's `er7 = { path = ".." }` had no version bound, so
      `bans.wildcards = "deny"` (raised from the template's `allow`) failed
      it — every other workspace member already pins `version = "0"`
      alongside its path dependency, and `er7/fuzz/Cargo.toml` now does
      too. `er7/fuzz/Cargo.toml` also gained the same `description` and
      five-license `license` field every other crate in the family
      carries, which `licenses.private.ignore = false` now verifies
      instead of skipping. All four categories pass, in both workspaces,
      confirmed by running `cargo deny check` directly (not just planned
      through CI) before committing. `plan.md` workstream 3 and
      `SECURITY.md`'s checkable-properties table updated in the same
      change; "no `cargo deny`/`cargo audit`" dropped from the still-open
      list, SBOM stays open.
- [x] **Add `.github/ISSUE_TEMPLATE/` and a stated issue-response
      expectation** — done 2026-08-27: `bug_report.md` and
      `wrong_claim.md`, plus `config.yml` routing security reports to
      GitHub's private advisory form (verified enabled by
      `gh api repos/er7-rust/er7-rust/private-vulnerability-reporting`
      before linking it) and everything else to
      [`MAINTAINERS.md`](MAINTAINERS.md). Adapted from the sibling
      `hl7-rust` repository's three-file set rather than invented
      independently, per rule 8's "sync from the repository that owns the
      canonical copy" — trimmed for this repo's shape (three crates, no
      transports, so the environment question drops the
      transport-specific parenthetical `hl7-rust` carries). The response
      expectation itself is new prose, not copied: a "What to expect from
      an issue or pull request" section in `MAINTAINERS.md`, stating the
      same one-week target `hl7-rust`'s `config.yml` already implied but
      had not written down as prose anywhere in that repository either —
      worth carrying back there.

### Governance

- [x] **Put the code of conduct at the root** — done 2026-08-26:
      `git mv er7/CODE_OF_CONDUCT.md CODE_OF_CONDUCT.md`, the three links
      in GOVERNANCE.md, CONTRIBUTING.md, and MAINTAINERS.md repointed
      (verified by `grep -rn er7/CODE_OF_CONDUCT`: zero hits outside the
      site copy, which keeps its own), and the claim-accuracy clause
      adapted from the `fhir-rust` version ("One Addition Specific to
      This Project").
- [x] **Upgrade the root CODE_OF_CONDUCT.md from Contributor Covenant 2.0
      to 2.1** — done 2026-08-27, resolving the decision this item
      previously deferred: `spec/professionalization/index.md` rule 7
      names 2.1 as the family's own requirement, not an externally imposed
      one, and nothing in this repository's history recorded 2.0 as a
      deliberate choice — it was a plain gap, not a declined decision. The
      official 2.1 text was fetched and diffed against 2.0 rather than
      paraphrased from memory: the only substantive change is "caste,
      color," added to the Pledge's list of protected characteristics;
      everything else, including this project's own claim-accuracy clause
      and the maintainer-email reporting path, is unchanged. The
      Attribution section's version number and URL updated to match.
      `spec/professionalization/index.md` rule 7's status row and
      `spec/special-files-for-public-repos/index.md`'s note both updated
      in the same change.

### Compliance — licensing and trademarks

- [x] **Add `LICENSES/` with the full text of all five licenses** — done
      2026-08-26: the five texts copied from `fhir-rust/LICENSES/` (REUSE
      convention, one file per SPDX identifier), and `LICENSE.md`'s table
      now links each local text ahead of the URL, with a note on why a URL
      alone was not sufficient (MIT, Apache-2.0, and BSD-3-Clause require
      the text to travel with the software).
- [x] **Fix the `CITATION.cff` license fields and add the missing file** —
      done 2026-08-26: `license` is now the five-identifier CFF list (the
      `fhir-rust`/`snomed-rust` convention) in the root file *and* in
      `er7/` and `er7-redact/`, all of which said "See license file";
      `version`/`date-released` added everywhere (the root names all three
      crate versions, since the workspace has no single number);
      `serde-er7/CITATION.cff` created, crate-scoped. The two pre-existing
      per-crate files were also invalid YAML (unquoted `title:` scalars
      containing ": ") — fixed; all four now parse (`python3 -c
      "yaml.safe_load"` on each).

### Privacy and patient data

- [x] **Add root `PHI.md`** — done 2026-08-26: privacy-officer Q&A built
      from the spec's own claims (§1.3's whole-data-set framing, D14's
      "starting point, not a compliance certification", §7's FNV-1a
      pseudonym caveats), with the 18-identifier Safe Harbor table drawn
      row by row from `er7-redact/spec/05-built-in-policies/index.md`
      §5.1/§5.4 — categories 12–18 honestly marked untouched, dates
      marked materially incomplete, free text named as the structural
      gap per §14.2, and the §164.514(c) derived-code problem with
      pseudonyms stated.

### Outreach

- [x] **Add `.github/FUNDING.yml`** — done 2026-08-26: the three
      URL-addressable routes from CONTRIBUTING.md's "Money" section
      (PayPal, Venmo, linktr.ee) as `custom:` entries, and only those —
      no GitHub Sponsors, because CONTRIBUTING.md does not list one. Bank
      transfer is arranged by email and has no URL, which the file's
      comment records. YAML validated with `yaml.safe_load`.
- [x] **Add the governance surface to the site** — done 2026-08-26: five
      new routes in `er7-rust.github.io/src/routes/` (`/security/`,
      `/governance/`, `/maintainers/`, `/rfc/`, `/ai-statement/`), each a
      summary of its root document that names the root file as canonical
      and links it, in the same hero/section/prose shape as the existing
      routes; all five linked from the shared footer. `pnpm check` green
      (308 files, 0 errors, 0 warnings); `bin/check-trademarks` green.
      The pages go live on the next `make publish` of the site.
- [x] **Write `help/outreach/index.md` at the workspace level** — done
      2026-08-27, now that CI and the conduct file (both closed
      2026-08-26) had unblocked it: `help/index.md` and
      `help/outreach/index.md`, matching the per-crate `help/` pattern
      and the sibling `hl7-rust` repository's shape. Kept narrower than
      `hl7-rust`'s version on purpose — this repository already has
      `spec/promote/index.md` (the full channel research, written earlier
      in this project's history), so the new file holds only the
      prerequisite gate rule 9 actually asks for, and links out for
      everything else rather than duplicating it. Nine prerequisites
      checked against the tree that day (docs.rs 200 for all three crates
      — caught and fixed a `curl` check that used the crate name instead
      of the module name, which under-reported two of the three as 404;
      `bin/check-trademarks`; the issue templates and `cargo deny` work
      landed the same day). Verdict: every prerequisite met or not
      applicable, including the trademark row, closed here without ever
      needing the written-clearance step `hl7-rust` is still waiting on,
      since no name in this workspace uses a word mark. `AGENTS.md` gained
      a pointer to `help/`; `plan.md` workstream 5 and
      `spec/professionalization/index.md` rule 9's status row updated in
      the same change. No promotion has started — the gate opening is not
      a decision to walk through it.

### Audit and harmonization

- [x] **Re-sync `spec/special-files-for-public-repos/index.md`** — done
      2026-08-26: the four missing entries added (CODE_OF_CONDUCT.md,
      PHI.md, LICENSES/, FUNDING.yml), the two typos fixed
      ("optimizaiton" → "optimization", "Prker" → "Parker", plus a third
      the task did not name: "summries" → "summaries"), and a status
      section added, adapted to this repository (all files exist as of
      2026-08-26; the Covenant 2.0-vs-2.1 mismatch stated honestly). One
      local addition kept: TRADEMARKS.md, which the canonical list omits
      but this repository carries.
- [x] **Per-crate document parity** — resolved 2026-08-26 by recording
      the decision the item offered: the root-level CONTRIBUTING.md,
      CODE_OF_CONDUCT.md, and SECURITY.md cover all three crates, and
      each crate's README now says so in a short "Contributing, conduct,
      and security" section (absolute GitHub links, so the section
      survives on crates.io). No copies were made — one copy cannot
      drift, which is the same reasoning `er7/CONTRIBUTING.md` already
      states; that file stays as the one existing pointer.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
