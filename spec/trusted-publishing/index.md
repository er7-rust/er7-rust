[er7-rust](../../index.md) → [spec](../index.md) → trusted publishing

# §7 Trusted Publishing

Trusted Publishing is a secure way to publish your Rust crates from CI/CD platforms like GitHub Actions and GitLab CI/CD without manually managing API tokens. It uses OpenID Connect (OIDC) to verify that your workflow is running from your repository, then provides a short-lived token for publishing.

Instead of storing long-lived API tokens in your repository secrets, Trusted Publishing allows your CI/CD platform to authenticate directly with crates.io using cryptographically signed tokens that prove the workflow's identity.

We intend to add "Trusted Publishing" when it is production-ready across all our code forges (GitHub.com, GitLab.com, Codeberg.org, etc.) and across all our target destinations (Rust crates.io, NPM npmjs.com, etc.).

## §7.1 A prerequisite this note does not yet name

Recorded 2026-08-28, while propagating this intent into
[`MAINTAINERS.md`](../../MAINTAINERS.md) and
[`SECURITY.md`](../../SECURITY.md): Trusted Publishing authenticates a
*CI workflow run* to crates.io. It has no meaning for a human running
`cargo publish` at their own terminal, which is what actually happens in
this workspace today — [`GOVERNANCE.md`](../../GOVERNANCE.md) states it
as a rule, not a habit: "a release is a decision, not an automation; no
workflow publishes; one person runs `cargo publish`."

Adopting Trusted Publishing as it exists today would therefore mean
moving the publish step into a CI job first — a governance change, not
a credential swap. This section's stated intent is not withdrawn by that
finding; it is a real prerequisite the intent did not yet name, so that
whoever picks this up next does not have to rediscover it before
deciding whether the governance change is one this project actually
wants to make, separately from wanting the credential improvement.
