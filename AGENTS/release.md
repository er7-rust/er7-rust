[AGENTS.md](../AGENTS.md) → release

# Release

## Before releasing

1. All four checks clean (see [`AGENTS.md`](../AGENTS.md#common-commands)).
2. [`spec/02-wire-shapes.md`](../spec/02-wire-shapes.md) matches the
   actual `Serialize`/`Deserialize` behaviour — if it does not, that is a
   bug per [`spec/index.md`](../spec/index.md)'s own rule ("a code change
   that isn't reflected here is a bug").
3. `Cargo.toml`'s `er7` dependency — if still a path dependency
   (`{ path = "../er7-rust" }`), switch to a version requirement matching
   the `er7` release this version was tested against before publishing to
   crates.io; a path dependency cannot be published as-is.
4. `examples/` all still run: `for e in round_trip_via_json
   build_message_from_json inspect_a_segment_as_json; do cargo run
   --example "$e"; done`.

## Versioning

See [`spec/08-versioning-and-compatibility.md`](../spec/08-versioning-and-compatibility.md)
for what counts as a breaking change in this crate specifically — the wire
shape in [`spec/02-wire-shapes.md`](../spec/02-wire-shapes.md) is part of
the compatibility surface, not only the Rust API.

## What to check in lockstep with an `er7` upgrade

Bumping the `er7` dependency to a new version is not a version-neutral
change here if `er7` itself changed a public type's fields — walk
[`spec/02-wire-shapes.md`](../spec/02-wire-shapes.md)'s table against the
new `er7::Message`/`Segment`/... definitions before publishing, and update
both the code and that table together if anything moved.
