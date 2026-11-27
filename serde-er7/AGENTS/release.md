[AGENTS.md](../AGENTS.md) → release

# Release

## Before releasing

1. All four checks clean (see [`AGENTS.md`](../AGENTS.md#common-commands)).
2. [`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md)
   matches the actual `Serialize`/`Deserialize` behaviour — if it does not,
   that is a bug per [`spec/index.md`](../spec/index.md)'s own rule ("a
   code change that isn't reflected here is a bug").
3. `Cargo.toml`'s `er7` dependency is a path dependency during development
   (`{ path = "../er7", version = "0.4" }`) so the workspace picks up local
   `er7` changes immediately. The `version` is the first `er7` release whose
   behaviour this crate relies on, never a bare `"0"` (workspace
   family-policy §1.6). Publish `er7` first. Before publishing, confirm the
   requirement matches the `er7` release this version was tested against
   — `cargo publish` strips the `path` and publishes against the version
   requirement alone, so an out-of-date requirement would silently pull a
   different `er7` than what was tested.
4. `examples/` all still run: `for e in round_trip_via_json
   build_message_from_json inspect_a_segment_as_json
   catch_a_typo_with_strict; do cargo run --example "$e"; done`.
5. `cargo package --locked --list` contains only the intended source,
   specification, documentation, examples, and root package files; then
   `cargo package --locked` builds the exact archive Cargo would publish.
6. From the green CI run for the release commit, retain
   `serde-er7-<version>-x86_64-unknown-linux-gnu.cdx.json` from the
   `cyclonedx-sboms` artifact. Attach it if a GitHub Release is created;
   workspace spec §15 explains its target and assurance boundaries.

## Who may dispatch publishing

The maintainer alone scopes and names a release; once he has, he may
direct an agent to judge readiness against the checklist above and
dispatch the protected manual OIDC workflow with `crate=serde-er7`,
the exact version, full signed release-commit SHA, and confirmation phrase
`publish serde-er7 <version>` — see
[`GOVERNANCE.md`](../../GOVERNANCE.md)'s "Release authority" section.

## Versioning

See
[`spec/08-versioning-and-compatibility/index.md`](../spec/08-versioning-and-compatibility/index.md)
for what counts as a breaking change in this crate specifically — the wire
shape in [`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md)
is part of the compatibility surface, not only the Rust API.

## What to check in lockstep with an `er7` upgrade

Bumping the `er7` dependency to a new version is not a version-neutral
change here if `er7` itself changed a public type's fields — walk
[`spec/02-wire-shapes/index.md`](../spec/02-wire-shapes/index.md)'s table
against the new `er7::Message`/`Segment`/... definitions before publishing,
and update both the code and that table together if anything moved.
