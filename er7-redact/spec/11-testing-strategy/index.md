[`er7-redact` specification](../index.md) — section 11 of 17. Section
numbers (§11.x) are stable and cited from code, tests, and commit messages.

# 11. Testing strategy

Conventions for writing tests live in
[`AGENTS/testing.md`](../../AGENTS/testing.md). This section says what must be
covered and where.

## 11.1 Rule coverage

Every rule in [§1.4](../01-purpose-and-scope/index.md) names the test that
enforces it. A rule with no test is a bug in this table.

| Rule | Test | File |
| ---- | ---- | ---- |
| D1 | `preserves_the_shape`, `every_sample_keeps_its_shape`, `a_cleared_field_reads_back_as_an_empty_one` | `src/redact.rs`, `tests/integration.rs` |
| D2 | `does_not_create_a_position` | `src/redact.rs` |
| D3 | `leaves_an_empty_leaf_empty` | `src/redact.rs` |
| D4 | `leaves_an_explicit_null_alone` | `src/redact.rs` |
| D5 | `never_touches_the_delimiter_fields` | `src/redact.rs` |
| D6 | `null_collapses_the_named_position` | `src/redact.rs` |
| D7 | `applies_rules_in_order` | `src/redact.rs` |
| D8 | `a_rule_that_matches_nothing_does_nothing` | `src/redact.rs` |
| D9 | `rejecting_by_default_covers_what_no_rule_named`, `a_segment_wide_accept_is_not_narrowed` | `src/redact.rs` |
| D10 | `every_action_but_pseudonym_is_idempotent` | `src/action.rs` |
| D11 | `replacement_text_cannot_break_the_message` | `tests/integration.rs` |
| D12 | `pseudonyms_are_stable_and_keyed`, `pseudonyms_link_across_messages` | `src/pseudonym.rs`, `tests/integration.rs` |
| D13 | `a_report_carries_no_values` | `src/redact.rs` |
| D14 | `the_default_policy_names_the_documented_positions`, `the_documented_positions_match_the_built_in_policy` — and *by review* for the compliance claim itself, which no test can make | `src/policy.rs`, `tests/integration.rs` |
| D15 | `reports_a_bad_policy_line`, `cli_reports_errors_on_stderr` | `src/policy.rs`, `tests/integration.rs` |
| D16 | `the_crate_has_one_runtime_dependency` | `tests/integration.rs` |
| D17 | `an_untouched_message_round_trips` | `tests/integration.rs` |
| D18 | `a_policy_round_trips_through_display`, `parses_every_action`, `the_sample_policy_exercises_every_action` | `src/policy.rs`, `src/action.rs`, `tests/integration.rs` |
| D19 | `reject_beats_accept_for_the_same_field`, `reject_segment_beats_a_narrower_accept` | `src/redact.rs` |
| D20 | `appending_never_weakens_the_defaults` | `src/policy.rs` |
| D21 | `an_unrecognised_payload_follows_the_policy`, `cli_masks_an_unrecognised_payload` | `src/redact.rs`, `tests/integration.rs` |
| D22 | `uncovered_lists_every_position_no_rule_names`, `uncovered_ignores_empty_and_null_leaves`, `cli_uncovered_lists_the_documented_gaps` | `src/redact.rs`, `tests/integration.rs` |
| D23 | `known_values_are_redacted_wherever_they_appear`, `known_values_matching_is_case_insensitive`, `known_values_matching_is_whole_word_only`, `known_values_below_the_minimum_length_are_ignored`, `keep_never_becomes_a_known_value`, `known_values_from_a_null_collapsed_field_are_still_learned`, `search_known_values_off_disables_the_sweep`, `known_values_line_parses_and_displays`, `appending_only_turns_known_values_on`, `cli_known_values_default_policy_catches_a_repeated_name` | `src/redact.rs`, `src/policy.rs`, `tests/integration.rs` |
| D24 | `custom_action_runs_the_callers_closure`, `custom_action_equality_is_identity_not_behavior`, `custom_action_writes_a_placeholder_with_no_file_spelling`, `a_policy_mixing_built_in_and_custom_actions_redacts_correctly`, `a_custom_action_reports_instead_of_panicking` | `src/action.rs`, `src/redact.rs` |

Further tests carry no single rule, because they assert the whole-message
properties every rule exists to produce, or a contract that is a table
rather than a rule: `redacts_the_identifiers_a_sample_carries`
([§11.5](#115-the-property-every-action-test-asserts)),
`redaction_is_reproducible` ([§2.7](../02-redaction-model/index.md)),
`cli_a_policy_that_changes_nothing_is_not_an_error`
([§10.4](../10-command-line-interface/index.md)),
`the_two_bare_postures_say_what_they_are`
([§5.6](../05-built-in-policies/index.md)), and the `cli_*` tests that pin
[§10.2](../10-command-line-interface/index.md)'s table of which policy
runs.

The table is **checked by `cargo test`**, not only by review:
`every_rule_has_a_coverage_row` reads this file and
[§1.4](../01-purpose-and-scope/index.md) and fails if a rule is declared
with no row here, or covered here without being declared.
`every_spec_section_is_indexed_and_present` does the same for the section
files and [`index.md`](../index.md), and
`the_documented_positions_match_the_built_in_policy` holds
[§5.1](../05-built-in-policies/index.md) and `Policy::patient_identifiers`
to each other, position by position.

Those three are what make "the spec is the single source of truth" a
property of the build rather than a habit — and the third matters most,
because §5.1 is the one table a reader is most likely to trust without
checking the code.

## 11.2 Where a test belongs

| Kind of test | Home |
| ------------ | ---- |
| a rule about one module's own behaviour | `#[cfg(test)] mod tests` at the bottom of that module |
| anything crossing module boundaries | `tests/integration.rs` |
| anything a caller can observe through the public API only | `tests/integration.rs` |
| the CLI contract ([§10](../10-command-line-interface/index.md)) | `tests/integration.rs`, `cli_*` prefix |
| an invariant of the spec itself — the rule index, the section files, the §5.1 table | `tests/integration.rs` (§11.1) |
| an illustration a reader should see in the docs | a rustdoc `Example:` block, which runs as a doc-test |

## 11.3 The four checks

Every change runs all four, and all four are clean on `main`:

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

`--all-targets` matters: it is what compiles `examples/` and so keeps the
tutorials from rotting.

## 11.4 Test data

Test messages are either written inline or taken from `samples/`:

| Sample | Exercises |
| ------ | --------- |
| `samples/adt_a08.er7` | an admission update carrying the full identifier set: `PID`, `NK1`, `PV1`, `GT1`, `IN1`, and a local `ZPD` segment |
| `samples/oru_r01.er7` | a lab result: repeated `OBX`, a repeated field, subcomponents, an escaped `&`, and free text no positional policy touches |
| `samples/de-identify.policy` | a policy file exercising every action, comments, blank lines, and both default lines |

All sample data is **synthetic**, and this matters more here than in any
sibling crate: a repository about redaction is exactly where somebody
would be tempted to commit a real message to prove the redaction works.
Names are obviously fictional (`EVERYWOMAN^EVE`, `JONES^WILLIAM`) and
identifiers obviously fake (`PATID1234`, `444333222`). See
[`AGENTS/safety.md`](../../AGENTS/safety.md).

## 11.5 The property every action test asserts

Three assertions, on every redaction test that is not specifically about
something else:

1. the output **parses** — `er7::parse` succeeds on what was written;
2. the **shape** is unchanged — same segment, field, repetition,
   component, and subcomponent counts in the tree, and the same field
   numbering after writing out and reading back (D1,
   [§4.1](../04-what-redaction-preserves/index.md));
3. the **original value is gone** — a search of the whole rendered message
   for the value that was redacted finds nothing.

The third is the one that catches real bugs. A value can survive redaction
by being in a second position nobody listed, and asserting on the position
that was redacted will never notice.

## 11.6 What is not tested, and why

- **That a message is de-identified.** No test can assert it, because it is
  not a property of one message ([§5.5](../05-built-in-policies/index.md)).
  What is tested is that the named positions changed.
- **Pseudonym collision behaviour.** FNV-1a's distribution is a property
  of the construction, not of this crate, so a test asserting on it would
  be testing someone else's code under a false name. This used to be
  recorded as T3, on the premise that §7 claimed collisions were
  negligible and needed a test to back that up — but §7 makes no such
  claim and never did in its current form: [§7.2](../07-pseudonyms/index.md)
  states plainly that the construction is "not collision-resistant against
  an adversary," and [§7.3](../07-pseudonyms/index.md) goes further, warning
  that an attacker can invert the whole mapping by brute force over a
  realistic identifier space. T3's own fallback closure — "the claim in
  §7 is weakened to what is actually demonstrated" — was already true by
  the time anyone reread the task against the spec it cited, so the task
  is deleted rather than carried forward against a claim that does not
  exist.
- **Nothing else.** Performance used to be listed here as untested
  (recorded as T4); see [§11.7](#117-benchmarks) for why it no longer is.

## 11.7 Benchmarks

`er7-redact-bench/`, a workspace member that is **not published** —
mirroring `er7-bench` one crate over, for the same reason: this crate
carries exactly one runtime dependency (D16) and, until this crate
existed, zero development ones, and Criterion's own tree stays out of
that count.

Measured 2026-08-28: redacting the standard reference `ADT^A08` example
under the default policy costs about 17.6 µs; a batch of 50 costs about
793.6 µs, or roughly 15.9 µs/message — consistent with the single-message
figure, which is the sanity check that matters, since nothing in
`Redactor::redact` shares state across messages. Rejecting by default
costs about 9% more than accepting by default on this message, tracking
the shape of the work: a reject-by-default policy walks and judges every
leaf, not just the ones a rule names. Full figures, method, and the
caveats that make them meaningful are in
[`BENCHMARKS.md`](../../../BENCHMARKS.md) at the workspace root.

This closes [T4](../15-open-tasks/index.md).
