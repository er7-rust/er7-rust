[er7-rust](../../index.md) → [spec](../index.md) → dependabot

# §8 Dependabot

- Enable GitHub Dependabot security updates at the repo level.
- Enable GitHub Dependabot `.github/dependabot.yml` for scheduled update
  PRs.

Implemented 2026-08-26; `tasks.md` carries the verified-done record. Two
things, not one, and they are independent:

**Repository-level security updates** — Dependabot alerts, automated
security fixes, private vulnerability reporting, and secret scanning —
are a GitHub setting, not a file in this repository, and confirmed live
by re-checking the API rather than trusting the earlier record: `GET
/repos/er7-rust/er7-rust` reports `dependabot_security_updates` and
`secret_scanning` both `enabled`; `GET .../automated-security-fixes`
reports `enabled: true`; `GET .../private-vulnerability-reporting`
reports `enabled: true`.

**`.github/dependabot.yml`** registers the manifests those security
updates cover — the root workspace (`er7`, `er7-redact`, `serde-er7`,
`er7-bench`, `er7-redact-bench` — the last added 2026-08-28, one
lockfile) and `er7/fuzz/`, deliberately its own workspace so
`libfuzzer-sys` never touches `er7`'s tree — plus the
`github-actions` ecosystem, each on a weekly schedule. The two `cargo`
entries carry `open-pull-requests-limit: 0`, deliberately: this checklist
only asked for updates to be *enabled*, not for routine version-bump PRs
to become a treadmill. With the limit at 0, Dependabot still watches
every manifest and opens a PR for a security advisory — that is the
separate, repository-level setting above, which the limit does not
touch — while staying quiet about an ordinary version bump. The posture
mirrors the sibling `fhir-rust` repository's, adopted after that
repository's first hour with default limits opened 47 PRs, most of them
routine bumps. Raising a limit, ecosystem by ecosystem, is one line in
that file if the posture changes.

[`SECURITY.md`](../../SECURITY.md) states the same settings for a
security researcher reading that file rather than this one; the two are
kept in agreement rather than one citing the other as the source of
truth.
