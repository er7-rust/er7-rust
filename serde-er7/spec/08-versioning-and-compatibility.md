[index](index.md) → §8 Versioning and compatibility

# §8 Versioning and compatibility

## 8.1 Rule S10: the wire shape is part of the public API

Standard SemVer governs this crate (currently pre-1.0, so the usual
"minor may break" caveat of the `0.y.z` range applies per the Cargo/SemVer
convention — see [`AGENTS/release.md`](../AGENTS/release.md)). What makes
this crate slightly unusual is that its *wire shape* — [§2](02-wire-shapes.md)'s
table — is as much a compatibility surface as its Rust API: a caller who
has stored `Message` JSON on disk, in a database, or in a message queue is
depending on that shape, not merely on the Rust types that produce it.

Concretely, all of the following are breaking changes requiring a major
version bump (or, pre-1.0, a minor bump) and an update to §2 in the same
change:

- Changing any type's shape (object ↔ array, added/removed/renamed field).
- Changing which fields are required vs. optional on deserialize.
- Changing `Terminator`'s wire representation from variant-name strings to
  anything else (indices, different strings).
- Changing `Subcomponent` to serialize `value()` instead of `raw` (this
  would also violate [§4](04-round-trip-guarantee.md), so it should never
  happen without a spec change to that section too, with its rationale
  addressed head-on).

## 8.2 What is not part of the compatibility surface

- Internal helper functions and private visitor types (anything not `pub`).
- The specific error *message* text for a given failure — the error
  *kind* (`missing_field("segments")` naming that field, for instance) is
  meaningful and should stay stable, but the exact `Display` wording a
  given Serde format renders it as is not something this crate controls or
  promises.
- Which methods are reached via `Deref` — `er7`'s own API can grow without
  a version bump here, and every new method it adds becomes reachable
  through the wrapper for free.

## 8.3 Following `er7`'s own versions

This crate's `Cargo.toml` depends on `er7` by path during development
(`../er7-rust`) and should track a semver-compatible range once both are
published to crates.io. A breaking change in `er7`'s own value tree
(renaming a field, changing `Separators`) requires updating this crate's
wire shape to match and bumping this crate's own version accordingly, even
if nothing in this crate's own code changed — the wire shape is defined in
terms of `er7`'s types, so a break there is a break here too.

## 8.4 Rust compatibility

| Item | Value |
| ---- | ----- |
| Edition | 2024 |
| MSRV | 1.95 — current stable minus three releases |
| `no_std` | not supported; the wrapper types own `er7`'s `String`-bearing tree |

The MSRV comes from the workspace-wide **N-3** policy — this family
supports at least current stable Rust minus three releases — stated once in
[`spec/rust-msrv-n-minus-3.md`](../../spec/rust-msrv-n-minus-3.md) and
pinned as `rust-version` in `Cargo.toml`. Edition 2024's own 1.85 floor is
below that and no longer binds.

Two floors constrain this crate that do not constrain `er7`: `serde`'s own
MSRV, and `er7`'s. This crate's floor is the highest of the three. If
`serde` ever declares an MSRV above `N-3`, that becomes the real floor and
this section says so explicitly rather than leaving the manifest to imply
it. Raising the MSRV is a breaking change, so it lands in a release allowed
to break — under the pre-1.0 convention of §8.1, a minor bump.
