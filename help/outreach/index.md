[er7-rust](../../index.md) → [help](../index.md) → outreach

# Outreach

The gate [rule 9 of the professionalization
spec](../../spec/professionalization/index.md) requires: no promotion
while a rule is unmet for the surface being promoted, and this file names
the prerequisites. It does not repeat the channel research — who to reach,
where they gather, what to say and in what order — which lives in
[`spec/promote/`](../../spec/promote/index.md) and is normative for that
question. This page answers a narrower one: *is it time yet*, checked
against the tree as it stands, not assumed from a prior pass.

## Contents

- [Prerequisites](#prerequisites)
- [Verdict](#verdict)
- [Re-checking this page](#re-checking-this-page)

## Prerequisites

A launch that succeeds and then strands people is worse than no launch.
Each row was checked on **2026-08-27**, by running the command shown or
reading the file named — not by recalling an earlier pass.

| Prerequisite | State | Evidence |
| ------------ | ----- | -------- |
| Every crate's docs.rs build is green | **Met** | `curl -sL -o /dev/null -w '%{http_code}' https://docs.rs/<crate>/latest/<module>/` returns 200 for `er7`, `er7-redact` (module `er7_redact`), and `serde-er7` (module `serde_er7`) |
| Each crate's front page answers "what is this, what do I type first" in the first screen | **Met** | `er7/index.md`, `er7-redact/index.md`, and `serde-er7/index.md` each open with a one-line description and a runnable example above the fold, which docs.rs renders from the crate root doc comment |
| No claimed-but-neglected name on crates.io | **Not applicable here** | The sibling `hl7-rust` repository carries this risk — its umbrella crate `hl7` was first claimed in 2019 by an unrelated project — but no crate in this workspace has a prior claim to work around: `er7`, `er7-redact`, and `serde-er7` were all first published from this project, 2026-08-15 through 2026-08-17 |
| A changelog exists | **Met** | [`CHANGELOG.md`](../../CHANGELOG.md) at the workspace root, one file for all three crates, dated per release |
| Issue templates, and a stated response expectation | **Met**, 2026-08-27 | `.github/ISSUE_TEMPLATE/` (`bug_report.md`, `wrong_claim.md`, and a `config.yml` routing security reports to GitHub's private advisory form); [`MAINTAINERS.md`](../../MAINTAINERS.md) states the read-within-a-week target in its own section. The fastest way to lose the integration-engineer audience — [§3.1](../../spec/promote/index.md#31-who-we-are-actually-talking-to) audience A — is an unanswered issue about a vendor dialect |
| The trademark question is answered | **Met** | Unlike `hl7-rust`, whose crate is literally named `hl7` and sought written clearance from HL7® International before promoting under that name, nothing in this workspace's own names uses a word mark — [`TRADEMARKS.md`](../../TRADEMARKS.md) states so plainly, and it is what makes rule T4 of [`spec/hl7-trademarks-fair-use/`](../../spec/hl7-trademarks-fair-use/index.md) easy to keep. `bin/check-trademarks` passes across the tree, checked immediately before writing this row, and no written request to HL7 International is outstanding — none was needed |
| The MSRV policy is documented, and is a genuine selling point to the integration-engineer audience | **Met** | [`spec/rust-msrv-n-minus-2/`](../../spec/rust-msrv-n-minus-2/index.md); already surfaced in [`INSTALL.md`](../../INSTALL.md) and [`COMPARISONS.md`](../../COMPARISONS.md). Hospital toolchains move in quarters — say so explicitly in any promotional copy, per [§3.2](../../spec/promote/index.md#32-what-we-have-to-promote) |
| Dependency auditing runs, so a supply-chain question from audience C ([§3.1](../../spec/promote/index.md#31-who-we-are-actually-talking-to)) has an answer that is not "we haven't checked" | **Met**, 2026-08-27 | `deny.toml`, `.github/workflows/ci.yml`'s `deny` job, `.github/workflows/audit.yml`'s weekly rerun |
| CI actually runs, and has a track record | **Met**, re-checked 2026-09-02 | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) runs on every push; `gh run list -R er7-rust/er7-rust --workflow=ci.yml --limit 200 --json conclusion` shows 44 hosted runs, 43 green — this row was "partly met" as of 2026-08-27, when the track record was one green run; per this page's own [re-checking](#re-checking-this-page) instructions, a row that no longer holds is fixed, not left standing |

## Verdict

**All prerequisites this file gates are met.** The one row that stood at
"partly met" as of 2026-08-27 — CI's track record — needed no decision or
action to close: it accrued automatically as pushes happened, and reading
the tree again on 2026-09-02 found it holding 44 hosted runs, 43 green,
comfortably past "one green run" — a figure re-checked four times over
just this one day (34, 39, 43, 44), which is itself the demonstration:
the track record keeps growing on its own, without anyone deciding it
should. (`SECURITY.md`'s own "what this project does not have" list no
longer names this gap either, closed there in an earlier pass — checked
here rather than left as a stale cross-reference.)

That means [`spec/promote/`](../../spec/promote/index.md)'s [ninety-day
sequence](../../spec/promote/index.md#310-a-ninety-day-sequence) is
workable starting now. Nothing here obliges anyone to start it — that is a
maintainer decision, not a professionalization rule — but the gate rule 9
names is open.

Nothing has been promoted through any channel in
[`spec/promote/`](../../spec/promote/index.md) as of this page's last
check. When the first outreach happens, it is recorded in
[`NEWS.md`](../../NEWS.md), and the baseline
[§3.13](../../spec/promote/index.md#313-measuring-whether-any-of-it-worked)
names is what later movement gets measured against.

## Re-checking this page

Re-run before acting on the verdict above if more than a few weeks have
passed since the date this page states, or after any change to
`TRADEMARKS.md`, `.github/`, or a crate's published version:

```sh
for c in er7 er7-redact serde-er7; do
  mod=$(echo "$c" | tr '-' '_')
  curl -sL -o /dev/null -w "%{http_code} $c\n" "https://docs.rs/$c/latest/$mod/"
done
bin/check-trademarks
```

A row that no longer holds is a defect in this page, reported and fixed
the same way any other stale claim in this repository is — see
[rule 3 of the professionalization spec](../../spec/professionalization/index.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
