[er7-rust](../../index.md) → [spec](../index.md) → trusted publishing

# §7 Trusted Publishing

Trusted Publishing is a secure way to publish Rust crates from supported
CI/CD platforms without manually managing API tokens. It uses OpenID
Connect (OIDC) to verify that a workflow is running from a configured
repository, then provides a short-lived token for publishing.

Instead of storing a long-lived API token, Trusted Publishing allows the
selected release workflow to authenticate directly with crates.io using a
cryptographically signed token that proves the workflow's identity.

The project intends to adopt Trusted Publishing if it explicitly changes
the release-authority rule described below. Support on source-code mirrors
is not a prerequisite: crates are published once to crates.io, not once
per forge.

## §7.1 The release-authority prerequisite

Recorded 2026-08-28, while propagating this intent into
[`MAINTAINERS.md`](../../MAINTAINERS.md) and
[`SECURITY.md`](../../SECURITY.md): Trusted Publishing authenticates a
*CI workflow run* to crates.io. It has no meaning for `cargo publish` run
locally, at a terminal, which is what actually happens in this workspace
today — [`GOVERNANCE.md`](../../GOVERNANCE.md) states it as a rule, not
a habit.

Adopting Trusted Publishing as it exists today would therefore mean
moving the publish step into a CI job first — a governance change, not
a credential swap. This section's stated intent is not withdrawn by that
finding; it is a real prerequisite the original intent did not name, so that
whoever picks this up next does not have to rediscover it before
deciding whether the governance change is one this project actually
wants to make, separately from wanting the credential improvement.

**Updated 2026-09-02: this gap did not close, though a related one
did.** [`GOVERNANCE.md`](../../GOVERNANCE.md) now lets the maintainer
direct an AI coding agent to run `cargo publish` for a release he has
decided on, rather than only ever typing it himself. That is not the
change this section is about: an agent running the command locally, on
the maintainer's own machine, on his direction, is still not a CI
workflow authenticating to crates.io over OIDC — the distinction this
section draws (local terminal vs. CI job) survives unchanged; only which
hands are allowed at the local terminal has widened. Trusted
Publishing's actual prerequisite is still open.

**Updated again, later the same day: which hands widened further, the
distinction did not.** [`GOVERNANCE.md`](../../GOVERNANCE.md) now also
lets an agent working in this repository decide that a specific,
already-scoped release is ready to publish: for `er7` and `er7-redact`,
it may work through the readiness checklist each crate's own
`help/releasing/index.md` states (`serde-er7` has no checklist of its
own yet — its release still leans on the four checks and
`cargo package --list` directly), decide the release meets it, and carry
out the publish step itself — the
maintainer no longer has to tick every box personally before
`cargo publish` runs. That is still a decision made locally, by whoever
the maintainer has directed, against this project's own stated criteria —
not a CI workflow's own judgment, and not authenticated to crates.io by
anything but the same long-lived token this section is about in the first
place. Trusted Publishing's prerequisite — moving the publish step into
an authenticated CI job — is unaffected by either widening and remains
exactly as open as §7.1 originally found it.

## §7.2 Support matrix re-evaluation and decision

Re-evaluated 2026-11-19 against the crates.io team's current
[documentation](https://crates.io/docs/trusted-publishing) and
[development update](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/):

| Surface | Current status | Relevance here |
| ------- | -------------- | -------------- |
| GitHub Actions | Supported | GitHub is the canonical repository and already runs this workspace's CI. |
| GitLab.com CI/CD | Supported in public beta | The GitLab repository is a source mirror; it does not need a second publishing job. |
| Codeberg/Forgejo | Not supported by crates.io | The Codeberg repository is also a source mirror, so this does not prevent a GitHub release job from eliminating the crates.io API token. |
| crates.io | Supported destination | This is the only package registry to which the three Rust crates publish. |
| npm | Not a release destination | The website's private JavaScript dependencies are build inputs; this project does not publish an npm package. |

The previous requirement that every mirror support Trusted Publishing was
therefore based on a false equivalence between *mirroring source* and
*publishing a package*. A GitHub-only release authority would remove the
long-lived crates.io token; no corresponding token is needed by either
mirror.

The governance choice was:

1. Keep publishing locally and accept the long-lived maintainer token; or
2. amend [`GOVERNANCE.md`](../../GOVERNANCE.md) to permit a manually
   dispatched GitHub Actions release job for one maintainer-scoped crate,
   version, and commit, protected by a release environment, with OIDC as
   its only crates.io credential.

Option 2 was selected 2026-11-19. It preserves the existing
rule that no push, merge, tag, or schedule autonomously publishes, while
removing a credential that remains useful between releases.

## §7.3 Activation requirements

The repository side is implemented in
[`.github/workflows/publish.yml`](../../.github/workflows/publish.yml).
The workflow has only a manual trigger; requires an exact crate, manifest
version, full commit SHA, operation, and matching confirmation phrase;
offers a verification operation that proves OIDC without publishing;
requires the commit to be signed and reachable from `main`; repeats the
four checks and package verification; obtains a short-lived token only
after those checks; and publishes exactly one crate only when the publish
operation was explicitly selected. Third-party action code in this
privileged path is pinned to immutable commits.

GitHub environment `release` was created 2026-11-19 with `main`'s protected
branch policy and Joel Parker Henderson as required reviewer. The workflow
cannot be called operational until all three crates configure
`er7-rust/er7-rust`, `.github/workflows/publish.yml`, and environment
`release` as their trusted publisher on crates.io and a hosted run proves
the OIDC exchange. After that proof, enable crates.io's
Trusted-Publishing-only mode for all three crates and remove the local API
token. During this transition, release documentation must state which
credential path is actually active rather than implying configuration that
has not been verified.
