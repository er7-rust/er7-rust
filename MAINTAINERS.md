# Maintainers and access continuity

This file is the roster, and the honest answer to the question a
procurement review asks about any software that will sit in the path of
patient data: *what happens if the person who can ship a fix is
unavailable?*

It is deliberately not aspirational. Everything below describes the project
as it is on the day you read it in git history, not a structure the project
hopes to grow into.

## Roster

| Person | GitHub | Contact | Role | Since |
|---|---|---|---|---|
| Joel Parker Henderson | [@joelparkerhenderson](https://github.com/joelparkerhenderson) | <joel@joelparkerhenderson.com> | Maintainer (sole) | 2026-08-19 |

ORCID: <https://orcid.org/0009-0000-4681-282X>. The date is when this
workspace was assembled; the three crates it absorbed are slightly older —
first published 2026-08-15, 2026-08-16, and 2026-08-17 — and their
histories are still walkable under their own directories.

**The bus factor of this project is one.** There is exactly one person who
can accept a pull request, publish a release, or change a repository
setting. No second maintainer exists, no organisation stands behind the
project, and no legal entity is a party to it. The GitHub organisation
`er7-rust` is an organisation in the GitHub sense only — it exists because
an organisation Pages site must be served from an org-owned repository, not
because there is a group behind it.

Everything else in this file follows from that sentence, and no wording
elsewhere in the repository should be read as softening it.

## Publishing identities and where they live

These are the credentials and configured identities that can put bytes in
front of a user. Naming them is the point: an inventory nobody has written
down is an inventory nobody can hand over.

| Identity | What it publishes | Held by | Recovery if the holder is unavailable |
|---|---|---|---|
| The GitHub organisation `er7-rust` and its owner account | The repository, its issues, its settings | The maintainer's GitHub account, as sole owner | None. GitHub's account-recovery process is the only route, and it is between GitHub and the account holder. |
| A crates.io API token | `er7`, `er7-redact`, `serde-er7` | The maintainer, on his own machine | The crates.io owner list is the recovery surface, and it is the maintainer's account. |
| An SSH key | Pushes to GitHub, and to the GitLab and Codeberg mirrors, which `origin` fans out to on one `git push` | The maintainer, on his own hardware | None; the key is not escrowed. A successor would use their own. |
| The same SSH key, via `make publish` | <https://er7-rust.github.io> — pushed by `git subtree split` into the `er7-rust.github.io` repository, whose own workflow builds and deploys it | The maintainer | As above. Deliberately *not* a CI credential: a workflow doing this would need a token able to write another repository's workflow file, and GitHub refuses that. |

**The honest reading of that table:** every publishing identity terminates
at one person's GitHub account or one person's hardware. There is no
Trusted Publishing, no signing key escrow, and no second holder anywhere.
That is the residual risk, and it is stated rather than mitigated, because
no mitigation is available to a one-person project without a legal entity
behind it.

**Commits and tags are not cryptographically signed.** Said plainly because
a reviewer will check, and an absent signature discovered later reads worse
than one disclosed here. Authorship rests on the GitHub account and the
committer identity in the history.

## What is not here yet

Named rather than quietly omitted, because their absence is itself
information for an evaluation:

- **No hosted-CI track record yet.** A workflow now exists
  ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) running the four
  checks in [§1.2 of the family policy](spec/01-family-policy/index.md), a
  build on the MSRV toolchain, and `bin/check-trademarks` on every push and
  pull request — but it has not yet had a hosted run, so until one goes
  green it is a written gate, not a proven one. Before that, the checks ran
  on a laptop by one person before a release.
- **No signed commits or tags**, as above.
- **No second security responder.** [`SECURITY.md`](SECURITY.md) is the
  policy, and it terminates at one email address —
  <joel@joelparkerhenderson.com>. Pretending otherwise would be worse than
  saying so, which is why that file also tells a reporter to disclose
  publicly after ninety days whether or not a fix exists.
- **No release cadence commitment.** Releases happen when there is
  something to release.

## If the maintainer is unavailable

Stated plainly, so nobody has to guess:

- **Everything already published stays published.** Released versions on
  crates.io are immutable and remain downloadable. The git history, the
  specifications, and the website's source stay public.
- **Nothing new ships.** No releases, no security fixes, no answers to
  issues. This is the case [`SECURITY.md`](SECURITY.md)'s ninety-day
  disclosure clause is written for: report privately, and disclose anyway
  if nobody answers.
- **Forking is legitimate continuity.** The license is a choice of five,
  including permissive and copyleft options, specifically so that a fork is
  straightforward under whichever terms suit the forker. See
  [`LICENSE.md`](LICENSE.md).

If you depend on this software in a clinical setting and that position is
not acceptable to you — and it is entirely reasonable for it not to be —
the mitigation is on your side, not this project's: pin an exact version,
keep a fork you can build from source, and budget for maintaining it.

The crates are built to make that cheap, and this is the one place where
the dependency policy pays off for somebody other than an auditor. `er7`
has zero runtime dependencies, `er7-redact` has one, `serde-er7` has two,
and every behaviour is a numbered rule in a specification directory with a
test bound to it. A fork inherits a readable, testable artefact rather than
an archaeology project.

## What the maintainer does

- Reviews and merges every change; nothing merges automatically.
- Decides specification questions, which are the questions that decide
  behaviour. Each crate's `spec/` is canonical when it and the code
  disagree — see [§1.3](spec/01-family-policy/index.md).
- Cuts releases, one crate at a time. The three crates version
  independently; see [`CHANGELOG.md`](CHANGELOG.md).
- Is accountable for every line in the repository regardless of what tool
  helped write it. See [`AI_STATEMENT.md`](AI_STATEMENT.md).

## Becoming a maintainer

The route is defined, it is open, and the fuller version — including what
"judgement about the specification" means in practice — is
[`GOVERNANCE.md`](GOVERNANCE.md). In short, and deliberately unglamorous:

1. Contribute. A bug report that includes a message reproducing the problem
   — synthetic, never real patient data — is as welcome as code.
2. Show sustained judgement about the specification, not just the code. In
   this project the specification is canonical; someone who can be trusted
   with it can be trusted with the rest.
3. Ask. Email <joel@joelparkerhenderson.com>.

Adding a maintainer means updating three things in one change: the roster
in this file, [`CODEOWNERS`](CODEOWNERS), and the publishing-identity table
above. A maintainer who is not in all three is not really a maintainer.

## How decisions get made

Who decides, how the specification constrains them, how a change lands,
what gets declined, and why forking is treated as legitimate continuity
rather than as a hostile act: [`GOVERNANCE.md`](GOVERNANCE.md).

## Contributing and conduct

[`CONTRIBUTING.md`](CONTRIBUTING.md) and
[`er7/CODE_OF_CONDUCT.md`](er7/CODE_OF_CONDUCT.md) apply across the
workspace, not only to the crate they sit in.
