# Rust MSRV — current N-2

This workspace's **Minimum Supported Rust Version (MSRV)** is **the current
stable Rust release minus two**: if the current stable release is `1.N`, the
MSRV is `1.(N-2)`.

This is a project policy that governs the Rust
toolchain the code in this workspace may assume.

## The rule

- Let `1.N.0` be the latest stable Rust release published by the Rust project.
- The MSRV MUST be `1.(N-2).0`.
- Code, tests, benchmarks, fuzz targets, and examples MUST compile with the
  MSRV toolchain. A language or standard-library feature stabilized after the
  MSRV MUST NOT be used.
- Only the minor version is pinned. Patch releases of the MSRV minor version
  (`1.(N-2).x`) are all acceptable; the recorded value uses `.0`.
- Pre-release channels (beta, nightly) are never the MSRV and MUST NOT be
  required by any workspace target, including the fuzz targets — see
  [`er7/spec/13-testing-strategy/index.md`](../../er7/spec/13-testing-strategy/index.md),
  which keeps the nightly-only fuzz crate in its own workspace
  (`er7/fuzz/`, not a member of the root one) precisely so this rule
  holds.

## Where the MSRV is recorded

Corrected 2026-08-30 against the actual tree, not the structure an earlier
draft of this document assumed: the root `Cargo.toml` has no
`[workspace.package]` table, and no member crate uses
`rust-version.workspace = true` — there is no Cargo-level inheritance
mechanism in play here at all.

| Location | Form |
| -------- | ---- |
| `er7/Cargo.toml`, `er7-redact/Cargo.toml`, `serde-er7/Cargo.toml` | each declares its own `rust-version = "1.(N-2)"` directly, in `[package]` |
| `.github/workflows/ci.yml`, `msrv` job | reads `rust-version` from `er7/Cargo.toml` at run time, cross-checks that `er7-redact` and `serde-er7` declare the identical value (failing the job if they disagree), then installs exactly that toolchain via `dtolnay/rust-toolchain@master` — nothing is hard-coded a second time |

`rust-version` is the single source of truth inside each published crate:
`cargo` refuses to build it with an older toolchain, and downstream
consumers see the value in the crate's own published metadata. The three
crates MUST agree with each other — enforced mechanically by the `msrv`
job's cross-check, not by Cargo inheritance — because a workspace with three
different floors would make "the MSRV" a meaningless phrase.

## Maintenance

When a new stable Rust release `1.N` appears, the MSRV becomes `1.(N-2)`
**in the same change** that observes the release:

1. Set `rust-version` in all three of `er7/Cargo.toml`, `er7-redact/Cargo.toml`,
   and `serde-er7/Cargo.toml` to `1.(N-2)` — there is no single file that
   covers all three.
2. Nothing in `.github/workflows/ci.yml` needs to change: the `msrv` job
   reads the new value from `er7/Cargo.toml` itself.
3. Run `cargo +1.(N-2) check --all-targets --workspace` and fix anything that
   the older toolchain rejects — the MSRV is a floor the code must meet, not a
   ceiling on what the code may need.

Raising the MSRV is therefore routine and expected, not a breaking change to
be avoided. Lowering it below N-2 (to support an older consumer) is a design
decision for `plan.md`, not a convenience.

## CI enforcement

CI MUST verify the MSRV, not merely declare it. The `msrv` job installs the
exact pinned toolchain and runs `cargo check --all-targets --workspace` with
it. `cargo check` (not `cargo build`) is sufficient and fast: the MSRV question
is "does this compile", and the `test` job already answers "does this work" on
stable.

The `msrv` job is separate from the `test` job so a failure names the cause
directly: `test` red means a behavior regression, `msrv` red means the code
started requiring a newer toolchain than the policy allows.

## Current value

As of the most recent update to this document, stable Rust is **1.98**, so the
MSRV is **1.96**. If stable has moved on since, this document is stale in its
example only — the rule above is what binds, and `Cargo.toml` must be brought
back in line with it.
