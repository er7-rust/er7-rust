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
| A separate, dedicated code-signing SSH key (`SHA256:Ah1MPQNTLGuOy0JwLcU7LbnhSa7cRVqMaDggXwllRXc`) | Signs every commit and tag made from 2026-08-27 onward | The maintainer, passphrase-protected — deliberately not the same key as push authentication above, so a compromise of one does not silently compromise the other | None; not escrowed. A successor generates their own and this table, `git config user.signingkey`, and the `allowed_signers` file all change together. |

**The honest reading of that table:** every publishing identity terminates
at one person's GitHub account or one person's hardware. There is no
Trusted Publishing, no signing key escrow, and no second holder anywhere.
That is the residual risk, and it is stated rather than mitigated, because
no mitigation is available to a one-person project without a legal entity
behind it.

**Commits and tags are cryptographically signed as of 2026-08-27**, with
the dedicated code-signing key in the publishing-identities table above —
`git config commit.gpgsign true` and `tag.gpgsign true`, `gpg.format ssh`,
`user.signingkey` pointed at that key's public half. It was generated
2026-08-27 to replace an earlier configuration that briefly signed with
the same key used for push authentication; keeping the two separate means
a compromised push credential does not also forge history.

The key is passphrase-protected and, as of this writing, not loaded into
an `ssh-agent` on the maintainer's machine — the first signing attempt
after generating it failed for exactly that reason
(`error: Enter passphrase for ...`), which is the honest, checkable state
rather than an assumed one. Before it can sign anything, the maintainer
runs `ssh-add ~/.ssh/id.d/jph-code-signing=*=ssh-ed25519-with-passphrase`
once per session (or configures the agent to retain it, e.g. via macOS
Keychain with `ssh-add --apple-use-keychain`) and enters the passphrase
himself; no automation holds that passphrase, and none should.

Once unlocked, signing verifies locally against an `allowed_signers` file
naming this key — `git log --show-signature` and `git tag -v` both report
a good signature when tested on a scratch branch before this key touched
real history. **GitHub's "Verified" badge is not showing yet**, and that
gap is disclosed rather than implied away: an SSH key must be registered
with GitHub specifically as a *signing* key, and doing that from this
machine needs the `admin:ssh_signing_key` OAuth scope — an interactive
grant the maintainer has to make himself, once, at
<https://github.com/settings/ssh/new> (key type "Signing Key", pasting the
`.pub` file's contents) or via
`gh auth refresh -h github.com -s admin:ssh_signing_key` followed by
`gh ssh-key add <path-to-the-pub-file> --type signing`. A commit made
before that step signs and verifies locally but is not attributed by
GitHub; one made after it is.

## What is not here yet

Named rather than quietly omitted, because their absence is itself
information for an evaluation:

- **A hosted-CI track record of exactly one green run.** The workflow
  ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs the four
  checks in [§1.2 of the family policy](spec/01-family-policy/index.md), a
  build on the MSRV toolchain, a fuzz smoke run, and `bin/check-trademarks`
  on every push and pull request; its first hosted run went green
  2026-08-26, all four jobs. One run is a proof the gate works, not yet a
  track record — before it, the checks ran on a laptop by one person
  before a release.
- **No hosted "Verified" badge on signed commits or tags yet**, as above —
  narrower than the flat "no signed commits" this line said before
  2026-08-27, since signing itself is now on and verified locally; what is
  still missing is one interactive step only the maintainer can take.
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

## What to expect from an issue or pull request

**Read within a week.** That is a target, not a contract — there is one
maintainer, no team rotation, and no on-call. It is stated here instead of
left unsaid, because "someone will look at this eventually" and "nobody
knows when" read very differently to the person waiting. If a week passes
with nothing, a polite ping is not rude; it is a reasonable check that the
issue did not fall through.

A vulnerability report has its own, tighter posture — see
[`SECURITY.md`](SECURITY.md).

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
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) apply across the
workspace.
