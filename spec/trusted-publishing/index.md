[er7-rust](../../index.md) → [spec](../index.md) → trusted publishing

# §7 Trusted Publishing

Trusted Publishing is a secure way to publish your Rust crates from CI/CD platforms like GitHub Actions and GitLab CI/CD without manually managing API tokens. It uses OpenID Connect (OIDC) to verify that your workflow is running from your repository, then provides a short-lived token for publishing.

Instead of storing long-lived API tokens in your repository secrets, Trusted Publishing allows your CI/CD platform to authenticate directly with crates.io using cryptographically signed tokens that prove the workflow's identity.

We intend to add "Trusted Publishing" when it is production-ready across all our code forges (GitHub.com, GitLab.com, Codeberg.org, etc.) and across all our target destinations (Rust crates.io, NPM npmjs.com, etc.).

## §7.1 A prerequisite this note does not yet name

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
finding; it is a real prerequisite the intent did not yet name, so that
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
