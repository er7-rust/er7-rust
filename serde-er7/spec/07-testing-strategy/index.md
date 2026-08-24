[index](../index.md) → §7 Testing strategy

# §7 Testing strategy

## 7.1 Rule coverage

Every rule in [the index](../index.md#rule-index) names the test that
enforces it. A rule with no test is a bug in this table.

| Rule | Test | File |
| ---- | ---- | ---- |
| S1 | `the_crate_has_exactly_two_runtime_dependencies` | `tests/integration.rs` |
| S2 | `no_format_crate_is_a_runtime_dependency` | `tests/integration.rs` |
| S3 | `serializes_as_a_bare_string`, `keeps_escape_sequences_raw_through_json` | `src/subcomponent.rs`, `tests/integration.rs` |
| S4 | `round_trips_several_subcomponents`, `round_trips_repeated_values`, `round_trips_nested_components` | `src/component.rs`, `src/field.rs`, `src/repetition.rs` |
| S5 | `round_trips_a_full_message_through_json`, `round_trips_a_segment` | `src/message.rs`, `src/segment.rs` |
| S6 | `round_trips_the_default_delimiters`, `round_trips_custom_delimiters_and_truncation` | `src/separators.rs` |
| S7 | `round_trips_every_variant`, `rejects_an_unknown_variant` | `src/terminator.rs` |
| S8 | `ignores_unknown_fields`, `treats_a_missing_truncation_key_as_none` | `src/segment.rs`, `src/separators.rs` |
| S9 | `rejects_a_missing_name`, `rejects_a_missing_required_field`, `error_messages_still_name_the_missing_field` | `src/segment.rs`, `src/separators.rs`, `tests/integration.rs` |
| S10 | `round_trips_through_json`, `pretty_json_keeps_the_tree_shape`, and every shape test above | `tests/integration.rs`, `src/message.rs` |
| S11 | `deref_reaches_query`, `deref_reaches_the_inner_api` | `src/message.rs`, `src/subcomponent.rs` |
| S12 | *by `cargo rustdoc --lib -- -W missing-docs`*, which is one of the four checks (§7.4) | — |

The table is **checked by `cargo test`**, not only by review:
`every_rule_has_a_coverage_row` reads this file and the rule index and
fails if a rule is declared without a row here, or covered here without
being declared. `every_spec_section_is_indexed_and_present` does the same
for the section files and [`index.md`](../index.md).

That is what makes "the spec is the single source of truth" a property of
the build rather than a habit.

## 7.2 Four layers

1. **Per-module unit tests** (`#[cfg(test)] mod tests` at the bottom of
   each `src/*.rs`) — the shape and edge cases of that one level in
   isolation: empty vs. absent, custom delimiters, duplicate/missing/unknown
   keys. These are the fastest tests to run and the first place a new edge
   case for one level belongs.
2. **Doctests** (the `Example:` section on every public item, per rule S12)
   — one realistic, runnable use of that item, checked by `cargo test` like
   any other test. Every public item must have one; see
   [`AGENTS/conventions.md`](../../AGENTS/conventions.md).
3. **Integration tests** (`tests/integration.rs`) — black-box, through the
   public API only, exercising real message shapes: the `er7` crate's own
   `samples/*.er7` files (read via `include_str!` from the sibling
   workspace member, so this crate is tested against the same fixtures
   `er7` tests itself against), plus the specific guarantees in
   [§4](../04-round-trip-guarantee/index.md) and
   [§5](../05-error-handling/index.md) that only make sense as an
   end-to-end round trip.
4. **Examples** (`examples/*.rs`) — not `#[test]`s themselves, but built
   and run as part of manual verification and documented in
   `examples/README.md`; each is referenced from `docs/usage/index.md` so
   the tutorial and the runnable code cannot drift apart silently.

## 7.3 What each layer is responsible for catching

| Failure mode | Caught by |
|--------------|-----------|
| Wrong shape for one level (§2 violated) | unit test for that module |
| Round trip loses information (§4 violated) | integration test |
| Error message does not name the field (§5 violated) | unit test (`rejects_a_missing_...`) |
| A tutorial example no longer compiles or produces stale output | doctest / `cargo run --example` |
| A real sample message from `er7` fails silently | integration test against `samples/` |

## 7.4 Rule S12: doc coverage is enforced, not aspirational

`src/lib.rs` carries `#![warn(missing_docs)]`, and `cargo rustdoc --lib --
-W missing-docs` is one of the four required checks (see
[the index](../index.md#required-checks)). This applies to struct fields
and enum-like match arms as much as to the types and functions themselves.

## 7.5 Golden-fixture style

Where a test needs a literal message, prefer the shortest text that
exercises the behaviour under test — a two-segment `MSH`/`PID` fragment for
a delimiter edge case, not a full multi-segment ORU. Reach for `er7`'s
`samples/*.er7` (via `tests/integration.rs`) specifically when the point of
the test is "a real message, not one written to order," such as
`round_trips_every_sample_from_the_er7_crate`.
