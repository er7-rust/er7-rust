[er7-redact](../../index.md) → help → releasing

# Releasing

The policy behind these steps is
[`AGENTS/release.md`](../../AGENTS/release.md).

**This checklist is the readiness criteria [`GOVERNANCE.md`'s Release
authority](../../GOVERNANCE.md#release-authority) refers to.** As of
2026-09-02, once the maintainer has scoped and named a release (what
changes, what version), an agent working in this repository may work
through §§1–4 below, decide the release meets them, and carry out §5
itself — the maintainer no longer has to confirm every step personally
before `cargo publish` runs. Scoping and naming the release in the first
place stays the maintainer's alone; see
[`AI_STATEMENT.md`](../../AI_STATEMENT.md#6-human-oversight) for the full
disclosure.

## 1. Confirm the tree is clean

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
cargo build --examples
```

## 2. Confirm the spec and the code agree

- `spec/15-open-tasks/index.md` holds no task this release claims to close.
- Every rule in `spec/01-purpose-and-scope/index.md` §1.4 has a row in
  §11.1.
- A change to `Policy::patient_identifiers` appears in `spec/05` §5.1.
- A change to `pseudonym` — there must not be one. See
  `spec/13-compatibility-and-versioning/index.md` §13.2.

## 3. Bump the version

In `Cargo.toml`, `CITATION.cff`, and the "Applies to" line of
`spec/index.md`.

## 4. Check what ships

```sh
cargo package --list
```

No stray file, and **no message that is not synthetic** — see
[`AGENTS/safety.md`](../../AGENTS/safety.md).

## 5. Publish

```sh
cargo publish --dry-run
cargo publish
git tag -a er7-redact-v0.1.1 -m "er7-redact 0.1.1"   # prefixed: this repo holds 3 crates
git push --tags
```

## 6. Release notes

Name, in this order:

1. anything that changed in `Policy::patient_identifiers` — callers running
   the default need to know what started or stopped being redacted;
2. anything that changed in the policy file format or the CLI;
3. the rest.
