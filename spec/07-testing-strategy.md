[index](index.md) → §7 Testing strategy

# §7 Testing strategy

## 7.1 Four layers

1. **Per-module unit tests** (`#[cfg(test)] mod tests` at the bottom of
   each `src/*.rs`) — the shape and edge cases of that one level in
   isolation: empty vs. absent, custom delimiters, duplicate/missing/unknown
   keys. These are the fastest tests to run and the first place a new edge
   case for one level belongs.
2. **Doctests** (the `Example:` section on every public item, per rule S12)
   — one realistic, runnable use of that item, checked by `cargo test` like
   any other test. Every public item must have one; see
   [`AGENTS/conventions.md`](../AGENTS/conventions.md).
3. **Integration tests** (`tests/integration.rs`) — black-box, through the
   public API only, exercising real message shapes: the `er7` crate's own
   `samples/*.er7` files (read via `include_str!` from the sibling
   checkout, so this crate is tested against the same fixtures `er7` tests
   itself against), plus the specific guarantees in [§4](04-round-trip-guarantee.md)
   and [§5](05-error-handling.md) that only make sense as an end-to-end
   round trip.
4. **Examples** (`examples/*.rs`) — not `#[test]`s themselves, but built
   and run as part of manual verification and documented in
   `examples/README.md`; each is referenced from `docs/usage/index.md` so
   the tutorial and the runnable code cannot drift apart silently.

## 7.2 What each layer is responsible for catching

| Failure mode | Caught by |
|--------------|-----------|
| Wrong shape for one level (§2 violated) | unit test for that module |
| Round trip loses information (§4 violated) | integration test |
| Error message does not name the field (§5 violated) | unit test (`rejects_a_missing_...`) |
| A tutorial example no longer compiles or produces stale output | doctest / `cargo run --example` |
| A real sample message from `er7` fails silently | integration test against `samples/` |

## 7.3 Rule S12: doc coverage is enforced, not aspirational

`src/lib.rs` carries `#![warn(missing_docs)]`, and
`cargo rustdoc --lib -- -W missing-docs` is one of the four required checks
(see [the index](index.md#required-checks)). This applies to struct fields
and enum-like match arms as much as to the types and functions themselves.

## 7.4 Golden-fixture style

Where a test needs a literal message, prefer the shortest text that
exercises the behaviour under test — a two-segment `MSH`/`PID` fragment for
a delimiter edge case, not a full multi-segment ORU. Reach for `er7`'s
`samples/*.er7` (via `tests/integration.rs`) specifically when the point of
the test is "a real message, not one written to order," such as
`round_trips_every_sample_from_the_er7_crate`.
