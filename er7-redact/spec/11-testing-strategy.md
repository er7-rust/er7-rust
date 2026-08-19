[`er7-redact` specification](index.md) — section 11 of 17. Section numbers (§11.x) are stable and cited from code, tests, and commit messages.

# 11. Testing strategy

Conventions for writing tests live in
[`AGENTS/testing.md`](../AGENTS/testing.md). This section says what must be
covered and where.

## 11.1 Rule coverage

Every rule in [§1.4](01-purpose-and-scope.md) names the test that enforces
it. A rule with no test is a bug in this table.

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
| D9 | `the_fallback_covers_what_no_rule_named` | `src/redact.rs` |
| D10 | `every_action_but_pseudonym_is_idempotent` | `src/action.rs` |
| D11 | `replacement_text_cannot_break_the_message` | `tests/integration.rs` |
| D12 | `pseudonyms_are_stable_and_keyed`, `pseudonyms_link_across_messages` | `src/pseudonym.rs`, `tests/integration.rs` |
| D13 | `a_report_carries_no_values` | `src/redact.rs` |
| D14 | `the_default_policy_names_the_documented_positions`, `the_documented_positions_match_the_built_in_policy` — and *by review* for the compliance claim itself, which no test can make | `src/policy.rs`, `tests/integration.rs` |
| D15 | `reports_a_bad_policy_line`, `cli_reports_errors_on_stderr` | `src/policy.rs`, `tests/integration.rs` |
| D16 | `the_crate_has_one_runtime_dependency` | `tests/integration.rs` |
| D17 | `an_untouched_message_round_trips` | `tests/integration.rs` |
| D18 | `a_policy_round_trips_through_display`, `parses_every_action`, `the_sample_policy_exercises_every_action` | `src/policy.rs`, `src/action.rs`, `tests/integration.rs` |

Three further tests carry no single rule, because they assert the
whole-message properties every rule exists to produce:
`redacts_the_identifiers_a_sample_carries` ([§11.5](#115-the-property-every-action-test-asserts)),
`redaction_is_reproducible` ([§2.7](02-redaction-model.md)), and
`cli_a_policy_that_changes_nothing_is_not_an_error`
([§10.4](10-command-line-interface.md)).

The table is **checked by `cargo test`**, not only by review:
`every_rule_has_a_coverage_row` reads this file and
[§1.4](01-purpose-and-scope.md) and fails if a rule is declared with no row
here, or covered here without being declared.
`every_spec_section_is_indexed_and_present` does the same for the section
files and [`index.md`](index.md), and
`the_documented_positions_match_the_built_in_policy` holds
[§5.1](05-built-in-policies.md) and `Policy::patient_identifiers` to each
other, position by position.

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
| the CLI contract ([§10](10-command-line-interface.md)) | `tests/integration.rs`, `cli_*` prefix |
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
| `samples/de-identify.policy` | a policy file exercising every action, comments, blank lines, and a fallback |

All sample data is **synthetic**, and this matters more here than in any
sibling crate: a repository about redaction is exactly where somebody
would be tempted to commit a real message to prove the redaction works.
Names are obviously fictional (`EVERYWOMAN^EVE`, `JONES^WILLIAM`) and
identifiers obviously fake (`PATID1234`, `444333222`). See
[`AGENTS/safety.md`](../AGENTS/safety.md).

## 11.5 The property every action test asserts

Three assertions, on every redaction test that is not specifically about
something else:

1. the output **parses** — `er7::parse` succeeds on what was written;
2. the **shape** is unchanged — same segment, field, repetition,
   component, and subcomponent counts in the tree, and the same field
   numbering after writing out and reading back (D1,
   [§4.1](04-what-redaction-preserves.md));
3. the **original value is gone** — a search of the whole rendered message
   for the value that was redacted finds nothing.

The third is the one that catches real bugs. A value can survive redaction
by being in a second position nobody listed, and asserting on the position
that was redacted will never notice.

## 11.6 What is not tested, and why

- **That a message is de-identified.** No test can assert it, because it
  is not a property of one message ([§5.5](05-built-in-policies.md)). What
  is tested is that the named positions changed.
- **Pseudonym collision behaviour.** FNV-1a's distribution is a property
  of the construction, not of this crate. Recorded as
  [T3](15-open-tasks.md).
- **Performance.** No benchmarks; one pass over a small message. Recorded
  as [T4](15-open-tasks.md).
