[AGENTS.md](../AGENTS.md) → release

# Release

How a version gets cut and published. The versioning rules themselves are
[`spec/14-compatibility-and-versioning/index.md`](../spec/14-compatibility-and-versioning/index.md);
this file is the procedure. A tick-box copy is in
[`help/releasing/index.md`](../help/releasing/index.md).

## Deciding the version

Read [§14.2](../spec/14-compatibility-and-versioning/index.md) and classify
the change. The traps specific to this crate:

- **All tree fields are `pub`**, so adding a field to `Message`, `Segment`,
  `Field`, `Repetition`, `Component`, `Subcomponent`, `Separators`,
  `RenderOptions`, or `Path` breaks struct literals. Breaking.
- **`Error` is matched exhaustively**, so a new variant is breaking
  ([§11.3](../spec/11-error-handling/index.md)).
- **The CLI is versioned with the library**
  ([§12.5](../spec/12-command-line-interface/index.md)): removing an
  option, changing an exit code, or changing the outline's label format is
  breaking.
- **Changing what a rule guarantees** is breaking even if the signature is
  identical. Rule IDs are the unit of contract, not function names.

While the crate is `0.x`, Cargo treats `0.1.y → 0.2.0` as the breaking
bump.

## Before releasing

1. **The four checks are clean** ([`workflows.md`](workflows.md)).
2. **Every rule has a test.** Compare the rule index (`spec/01` §1.4)
   against the coverage table (`spec/13` §13.1); the only permitted gap is
   R24.
3. **The spec matches the code.** Any behaviour changed since the last
   release has a matching `spec/` edit.
4. **`spec/17-open-tasks/index.md` reflects reality.** Tasks that shipped
   are deleted; the "next task ID" line is right.
5. **`spec/16-roadmap/index.md` reflects reality.** Items that shipped are
   gone.
6. **The docs still read true** — `index.md`, `docs/**`, `examples/**`.
7. **`spec/index.md` states the version it applies to.**

## Releasing

```sh
# 1. Set the version
$EDITOR Cargo.toml                # version = "x.y.z"
$EDITOR spec/index.md             # "Applies to: er7 x.y.z"
$EDITOR CITATION.cff              # if the description or authors changed

# 2. Verify
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
cargo package --list              # Check what will ship

# 3. Dry run
cargo publish --dry-run

# 4. Commit and tag
git add -A
git commit -m "Release x.y.z"
git tag -a er7-vx.y.z -m "Release x.y.z"   # prefixed: this repo holds 3 crates
git push && git push --tags

# 5. Publish
cargo publish
```

## Checking what ships

`cargo package --list` should show `src/`, `Cargo.toml`, `README.md`,
`LICENSE.md`, and nothing surprising — matching `Cargo.toml`'s own
`include` list, which also ships `spec/**`, `examples/**`, `samples/**`,
and `index.md`. `samples/` ships because the integration tests
`include_str!` them; `target/` never does.

The published crate must build with **no network and no dependencies**
(R25). `cargo publish --dry-run` verifies that.

## After releasing

1. Confirm <https://docs.rs/er7> built. A docs.rs failure is almost always a
   doc-test or an intra-doc link, both of which
   `cargo rustdoc --lib -- -W missing-docs` would have caught locally.
2. Confirm <https://crates.io/crates/er7> shows the right description,
   keywords, categories, and license.
3. Open the next cycle by editing
   [`spec/16-roadmap/index.md`](../spec/16-roadmap/index.md) if the
   priorities moved.

## No changelog file

The commit history is the changelog. Commit messages cite the spec section
they changed (`§6.2`) and the task they close (`T4`), which is what makes
the history readable without a second file to keep in sync. If a release
needs a summary, write it in the git tag annotation.
