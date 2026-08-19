[er7](../../index.md) → [help](../) → releasing

# Release checklist

A tick-box copy of [`AGENTS/release.md`](../../AGENTS/release.md), which has
the reasoning. Versioning rules are
[spec §14](../../spec/14-compatibility-and-versioning.md).

## 1. Decide the version

- [ ] Classify the change against [spec §14.2](../../spec/14-compatibility-and-versioning.md).
- [ ] Did a public struct gain a field? All fields are `pub`, so that breaks
      struct literals — **breaking**.
- [ ] Did `Error` gain a variant? Callers match exhaustively — **breaking**.
- [ ] Did a CLI option, exit code, or the outline label format change?
      **Breaking**.
- [ ] Did any rule in [spec §1.4](../../spec/01-purpose-and-scope.md) change
      what it guarantees? **Breaking**, even with an identical signature.

## 2. Check the repository

- [ ] `cargo test` — clean.
- [ ] `cargo clippy --all-targets -- -D warnings` — clean.
- [ ] `cargo fmt --check` — clean.
- [ ] `cargo rustdoc --lib -- -W missing-docs` — clean.
- [ ] Every rule in §1.4 appears in the §13.1 coverage table; the only
      permitted gap is R24.
- [ ] Behaviour changed since the last release has a matching `spec/` edit.
- [ ] `spec/17-open-tasks.md`: shipped tasks deleted, "next task ID" correct.
- [ ] `spec/16-roadmap.md`: shipped items gone.
- [ ] `index.md`, `docs/**`, `examples/**` still read true.
- [ ] Every example still runs: `cargo build --examples`.

## 3. Set the version

- [ ] `Cargo.toml` — `version = "x.y.z"`.
- [ ] `spec/index.md` — "Applies to: er7 x.y.z".
- [ ] `CITATION.cff` — if the description or authors changed.

## 4. Verify the package

- [ ] `cargo package --list` — shows `src/`, `Cargo.toml`, `README.md`,
      `LICENSE.md`, `index.md`, and nothing surprising.
- [ ] `cargo publish --dry-run` — succeeds. This also confirms the crate
      builds with no dependencies (R25).

## 5. Commit, tag, publish

- [ ] `git add -A && git commit -m "Release x.y.z"`
- [ ] `git tag -a vx.y.z -m "Release x.y.z"` — put the release summary in
      the tag annotation; there is no CHANGELOG file.
- [ ] `git push && git push --tags`
- [ ] `cargo publish`

## 6. Afterwards

- [ ] <https://docs.rs/er7> built. A failure is almost always a doc-test or
      an intra-doc link.
- [ ] <https://crates.io/crates/er7> shows the right description, keywords,
      categories, and license.
- [ ] `spec/16-roadmap.md` updated if priorities moved.
