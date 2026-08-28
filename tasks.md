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
- [x] **Tag releases and sign commits/tags going forward; record the
      posture change in MAINTAINERS.md** — tagging was already the
      practice (`er7-v0.1.3`/`serde-er7-v0.1.3`/`er7-redact-v0.2.1`,
      `er7-v0.1.4`/`serde-er7-v0.1.4`/`er7-redact-v0.2.2`, both release
      rounds tagged at the time). Signing itself landed 2026-08-27: SSH
      signing (`gpg.format ssh`), `commit.gpgsign`/`tag.gpgsign` both
      true, set with the maintainer's own existing key rather than a key
      generated on his behalf — asked first, since a signing identity is
      not this agent's to choose. Verified end to end on a scratch branch
      and tag before touching real history: `git log --show-signature`
      and `git tag -v` both report "Good \"git\" signature" against an
      `allowed_signers` file, then the branch and tag were deleted.
      **One piece stays open, and is named rather than implied closed**:
      GitHub's own "Verified" badge needs this key registered there
      specifically as a *signing* key, separate from its existing
      authentication registration, and that registration needs an
      interactive `admin:ssh_signing_key` OAuth grant this agent's `gh`
      session does not have and cannot request on the maintainer's
      behalf. `MAINTAINERS.md` names the exact command
      (`gh auth refresh -h github.com -s admin:ssh_signing_key` then
      `gh ssh-key add`) or the manual alternative
      (github.com/settings/ssh/new, key type "Signing Key"). Commits
      before that step sign and verify locally but show as unverified on
      GitHub; commits after it will not. MAINTAINERS.md, SECURITY.md,
      `plan.md`, and `spec/professionalization/index.md` rule 3's status
      row all corrected in the same change, narrowing "no signed commits"
      to the true, narrower gap rather than leaving the broader claim
      standing next to a contradicting local git config.

      **Superseded the same day.** The maintainer generated a dedicated,
      passphrase-protected code-signing key
      (`SHA256:Ah1MPQNTLGuOy0JwLcU7LbnhSa7cRVqMaDggXwllRXc`) rather than
      continuing to sign with the push-authentication key above, and asked
      for it to be wired in. `user.signingkey` and `allowed_signers` now
      point at it instead. The first signing attempt with it failed
      exactly as expected — `error: Enter passphrase for ...`, because the
      key is not loaded into an `ssh-agent` — which is recorded rather
      than papered over: nothing here holds that passphrase, and signing
      does not work again until the maintainer runs `ssh-add` himself.
      The maintainer unlocked the key himself (`ssh-add
      --apple-use-keychain`, in his own terminal — the passphrase never
      reached this agent) and registered its public half with GitHub as a
      signing key at <https://github.com/settings/ssh/new>, both outside
      this agent's reach by design. **GitHub's "Verified" badge is live
      the same day**: `gh api repos/er7-rust/er7-rust/commits/258f778
      --jq '.commit.verification'` returns `"verified": true, "reason":
      "valid"`; the one earlier commit signed under the *previous* key
      (`c8dc138`) correctly still reads `"unknown_key"`, since that key
      was never registered and never claimed to be. MAINTAINERS.md's
      publishing-identities table gained its own row for the new key,
      separate from the push-authentication one, and its "What is not
      here yet" list dropped the badge bullet entirely rather than
      reword it, since the gap it named no longer exists. SECURITY.md,
      `plan.md`, and `spec/professionalization/index.md` rule 3's status
      row corrected to match in the same change.

      One process note worth recording plainly: the first attempt at
      this used `git commit --allow-empty` on a scratch branch to test
      the newly-unlocked key, not realising the real staged edits to
      MAINTAINERS.md and tasks.md were still staged and would ride along
      into that "test" commit — `--allow-empty` permits a commit with no
      diff, it does not make one empty. Deleting the scratch branch
      afterward orphaned that commit along with the real edits. Recovered
      cleanly because the commit object was still reachable by SHA
      (`git cat-file -t <sha>` confirmed it, `git checkout <sha> --
      MAINTAINERS.md tasks.md` restored the exact content) before it was
      re-committed for real — nothing was lost, but the near-miss is
      worth naming so the next scratch-branch test in this repository
      commits to a path outside the working tree, or stashes first.
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

### Audit: stale claims found by re-checking, not by assumption

Three items closed 2026-08-27 by re-verifying earlier status rows against
the live tree, per rule 2 of the professionalization spec ("every
countable claim is re-verified when cited") rather than trusting a prior
pass:

- [x] **Correct `spec/professionalization/index.md` rules 2, 4, and 8** —
      all three had been sitting at "Partly met" citing gaps that were
      already closed elsewhere in this file. Rule 2 said the special-files
      list was missing four entries from the canonical `fhir-rust` copy;
      diffed line by line and found exactly one, a missing "2.1" on the
      CODE_OF_CONDUCT.md entry — fixed in both files. Rule 4 said CI "has
      not yet had a hosted run"; `gh run list` shows 13 runs, 12 green.
      Rule 8 pointed at rules 2 and "per-crate document parity" as open,
      both of which this file already shows closed. All three now read
      **Met**.
- [x] **Correct "exactly one green run" everywhere it was still claimed**
      — MAINTAINERS.md, SECURITY.md, `plan.md`, and AI_STATEMENT.md all
      said CI had a track record of one run, dated 2026-08-26. By
      2026-08-27 that was 13 runs, 12 green, with the one failure
      (`61eb30d`) already root-caused and fixed the same day it happened
      (`574daaf`) — a real track record, not a stronger version of the
      same claim. The bullet was removed outright from MAINTAINERS.md's
      and SECURITY.md's "what is not here" lists (the gap it named no
      longer exists) and replaced with a positive, evidence-cited row in
      SECURITY.md's checkable-properties table. AI_STATEMENT.md's own
      §13 names exactly this trigger — "a claim in this document stops
      being true" — for an off-cycle revision, so §7 and §12 were
      corrected and the document bumped 1.2.0 → 1.3.0 with an Annex A
      entry, rather than edited silently.
- [x] **Investigate the low-severity Dependabot alert flagged, but not
      chased down, at the end of the previous session** — `cookie` in
      `er7-rust.github.io`'s npm tree. Root cause, from Dependabot's own
      job log rather than guessed: `cookie` is a transitive dependency of
      `@sveltejs/kit`, which still declares `cookie: '^0.6.0'` even in its
      latest published release (`2.70.3`, checked via `npm view
      @sveltejs/kit@latest dependencies`) — upstream has not shipped a fix
      yet, which is exactly why Dependabot's own three automatic-fix
      attempts all failed with `security_update_not_possible` rather than
      opening a PR. **Not fixed here.** Forcing `cookie` to `0.7.0+` via a
      `pnpm.overrides` entry would run SvelteKit's server-side cookie
      handling against a dependency version it was not tested with, to
      close a hole that is very likely unreachable anyway: this site is
      prerendered by `adapter-static` with no server at runtime
      (`svelte.config.js` says so explicitly), so the vulnerable code path
      — cookie name/path/domain validation in server-side request
      handling — never executes in production. That trade is not this
      agent's to make unilaterally; the alert is left open, and whether to
      dismiss it (with this reasoning) or wait for an upstream fix is the
      maintainer's call.

### Requested directly, outside `plan.md`'s workstreams

- [x] **Free CI disk headroom: clean each crate's `target/` after
      processing, and strip preinstalled runner bloat** — done 2026-08-28.
      Two changes to `.github/workflows/ci.yml`:
      1. **The `checks` job split into two phases with a `cargo clean`
         between them.** `er7-bench`'s only dev-dependency is criterion,
         which alone pulls in roughly 50 transitive crates that the three
         published crates' own test/clippy/doc steps never touch. Measured
         with a clean `target/` before each, not estimated: phase 1 alone
         (the three published crates, all four checks) peaks at 226 MiB;
         phase 2 alone (`er7-bench`, criterion and all) peaks at 215 MiB;
         the old single-phase shape, run fresh with nothing scoped or
         cleaned, peaked at 391 MiB — splitting roughly halves the job's
         peak footprint rather than summing the two. A package-scoped
         `cargo clean -p er7-bench` was tried first and rejected: measured
         at only ~330 MiB freed of the ~1.8 GiB a long-lived local
         `target/` had accumulated over this session's own work, because a
         package-scoped clean does not walk into its exclusive dependency
         tree — it leaves criterion's own compiled output sitting there.
      2. **A "free preinstalled runner bloat" step**, first in the
         `checks`, `msrv`, and `fuzz` jobs (not `trademarks` or `deny`,
         neither of which compiles anything): removes the .NET SDK,
         Android SDK, GHC, CodeQL, PowerShell, and cached Node modules the
         standard `ubuntu-latest` image ships preinstalled and this
         workflow never touches, plus pruning cached Docker images. Every
         removal is `|| true`-guarded so a path that has moved in a future
         runner-image update fails silently rather than breaking the job
         — a real maintenance risk of hardcoding paths from an external
         image, named rather than hidden. `df -h /` before and after, in
         every job that runs it, so the effect is verifiable in the run
         log rather than assumed to have worked — and on this change's own
         first hosted run (`33149419145`, `ba0e1e2`, all five jobs green),
         it was: **87G available before, 109G after, in all three jobs**
         that run it — roughly 21 GiB of headroom this workflow was never
         using, freed before a single line of this workspace's own code
         gets checked out. That number is larger than the few hundred
         megabytes the `target/`-splitting change above recovers; the
         runner-bloat step is the one doing most of the actual work here,
         and this file says so rather than letting the more interesting
         engineering (the phase split) read as the bigger win.

      Deliberately **not** a third-party disk-cleanup action
      (`jlumbroso/free-disk-space` and similar exist and are well known):
      the job is a dozen lines of `rm -rf` against well-documented paths,
      and this workspace's own stated position on dependencies — an audit
      surface, not a convenience — applies to CI tooling as much as to
      published crates. Written inline, so the whole thing is readable in
      `ci.yml` without trusting an external action's own supply chain.

      Verified locally before trusting it on a hosted runner: both phases
      of the `checks` split ran for real (`cargo test`/`clippy`/`rustdoc`
      per phase, matching what the workflow now runs, not a paraphrase of
      it), and every `run:` block in the file was checked with `bash -n`
      via a small script that parses the workflow YAML and syntax-checks
      each step — a `rm -rf` typo would previously not have been caught
      until it reached a hosted runner.

- [x] **Implement `spec/free-open-source-funding/index.md`** — done
      2026-08-28. Two of its five items were genuinely achievable; the
      other three were checked, not assumed, and one turned out to
      already be true.
      - **GitHub Sponsors was already set up** — confirmed via GitHub's
        GraphQL API (`hasSponsorsListing: true` for `joelparkerhenderson`)
        before doing anything, rather than trusting that a
        `github.com/sponsors/<user>` URL resolving to 200 meant a real
        listing existed (it can return 200 either way). Surfaced it:
        `.github/FUNDING.yml` gained `github: joelparkerhenderson`, and
        `CONTRIBUTING.md`'s "Money" table gained the row.
      - **Open Collective is genuinely not achievable here, and is not
        pretended to be.** Checked against Open Collective's own GraphQL
        API: an account exists at `joelparkerhenderson`, but as an
        `INDIVIDUAL` — a personal contributor profile, not a fundable
        project collective — and no collective exists at `er7-rust` at
        all. Creating a real one needs the maintainer's own sign-in and a
        fiscal-host choice, which is not a decision this agent makes on
        his behalf. The sibling `fhir-rust` repository hit the identical
        wall on the identical spec item and left a clear, honest write-up
        of exactly this reasoning — read first, and the finding here
        matches it rather than rediscovering it independently. Recorded
        as a stated absence, not a silent one, in `CONTRIBUTING.md`,
        `.github/FUNDING.yml`'s own comments, and `NEWS.md`.
      - **`.github/FUNDING.yml`, `CONTRIBUTING.md`, and `NEWS.md`
        updated to match**, per the spec's last three items.
      - **One stale claim found and fixed along the way**: `CONTRIBUTING.md`
        offered "funding the CI that MAINTAINERS.md currently lists as
        missing" as an alternative to a donation — CI has not been
        missing from that list since this session's earlier "exactly one
        green run" correction. Swapped for the gap that is actually still
        listed there (a second security responder), checked against the
        live file rather than assumed unchanged.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
