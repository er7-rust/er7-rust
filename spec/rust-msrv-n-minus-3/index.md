[er7-rust](../../index.md) → [spec](../index.md) → Rust MSRV

# §2 Rust MSRV: N-3

Every crate in this workspace supports **current stable Rust minus three
releases**, and pins that value as `rust-version` in its `Cargo.toml`.

At the time of writing stable is 1.98, so the MSRV is **1.95**.

## §2.1 Why N-3 and not something else

The number is a compromise between two costs that pull in opposite
directions, and the reasoning matters more than the value:

- **Too narrow** (say, "latest stable") makes these crates unusable to the
  people most likely to need them. Healthcare integration code runs inside
  organisations whose toolchains are approved on a cycle measured in
  quarters, not days; a library that demands the toolchain released this
  month is a library they cannot adopt.
- **Too wide** (say, "the oldest Rust that still compiles it") turns every
  new language feature into a research question and quietly costs
  maintenance forever. It also tends to be a fiction: nobody tests the
  claim, so the real floor drifts up on the first convenient `let ... else`
  and the declared one becomes a lie.

Three releases is roughly six months of Rust — long enough for a
distribution or an internal toolchain to catch up, short enough that the
window is still testable and the code is not written against a language
from another era.

It is a *rolling* window, not a fixed version: as stable moves, so does the
floor. That is deliberate. A fixed floor only ever ages, and the decision
to abandon it eventually gets made in a hurry by whoever is blocked.

## §2.2 The edition floor

Edition 2024 requires 1.85, so the effective minimum is `max(1.85, N-3)`.
That floor stopped binding once stable reached 1.88 and is now historical,
but it is the reason the MSRV did not simply track N-3 from the start.

## §2.3 What a bump implies

Raising the MSRV is a **breaking change** for a consumer whose toolchain
sits below the new floor, and it is treated as one:

- A bump lands in a release that is allowed to break — a minor bump while a
  crate is `0.x`, a major one afterwards — never in a patch release.
- The new value is pinned in `Cargo.toml` in the same change, so `cargo`
  reports a clear "requires rustc 1.x" rather than an error from the middle
  of a build.
- The crate's own compatibility section is updated in the same change:
  [`er7` §14.4](../../er7/spec/14-compatibility-and-versioning/index.md),
  and the equivalent section in `er7-redact` and `serde-er7`.

The window moving is not by itself a reason to bump. N-3 is the *minimum*
this family promises to support, not a target to track release by release:
the pin only has to move when the code actually needs something newer, or
when the declared floor has fallen so far behind that nobody is testing it
any more.

## §2.4 The gap this policy used to have

Until 2026-08-26 nothing in CI built against the pinned toolchain, so a
declared floor could drift from the real one the moment a contributor used
a newer feature — the failure mode described above, arriving by accident
rather than by decision. Closed in two steps:

- **2026-08-26.** The `msrv` job in
  [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) started
  running `cargo check --workspace --all-targets` on the pinned toolchain,
  on every push and pull request.
- **2026-08-28.** The job now also runs `cargo test --workspace` — a
  check alone proves the code compiles, not that it behaves, and `er7`
  T2's own "done when" asked for both. The toolchain version is read from
  `er7/Cargo.toml`'s `rust-version` at run time rather than hard-coded in
  the workflow, with a step that fails loudly if `er7`, `er7-redact`, and
  `serde-er7` ever disagree on the value — closing this policy's own
  possible failure mode (a silent per-crate drift) at the same time as
  the original one.

This closed [`er7` T2](../../er7/spec/17-open-tasks/index.md), deleted
from that section's backlog in the same change.
