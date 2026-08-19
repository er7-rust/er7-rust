# CLAUDE.md

See **[AGENTS.md](AGENTS.md)** — that file holds the canonical agent
instructions for this workspace, and [`spec/`](spec/) holds the policy
genuinely shared by all three crates. Keeping the guidance in one place
avoids the copies drifting apart.

If your change is specific to one crate, go straight to that crate's own
`CLAUDE.md` / `AGENTS.md` (`er7/`, `er7-redact/`, `serde-er7/`) — this file
covers only the workspace as a whole.

Before finishing any change, run the four checks, either workspace-wide or
scoped to the crate you touched with `-p <crate>`:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
for c in er7 er7-redact serde-er7; do
    cargo rustdoc --lib -p "$c" -- -W missing-docs
done
```
