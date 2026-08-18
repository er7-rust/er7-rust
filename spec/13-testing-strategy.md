[`er7` specification](index.md) — section 13 of 19. Section numbers (§13.x) are stable and cited from code, tests, and commit messages.

# 13. Testing strategy

Conventions for writing tests live in
[`AGENTS/testing.md`](../AGENTS/testing.md). This section says what must be
covered and where.

## 13.1 Rule coverage

Every rule in [§1.4](01-purpose-and-scope.md) names the test that enforces
it. A rule with no test is a bug in this table.

| Rule | Test | File |
| ---- | ---- | ---- |
| R1 | `reads_custom_delimiters`, `honors_a_message_that_chooses_its_own_delimiters` | `src/separators.rs`, `tests/integration.rs` |
| R2 | `rejects_unusable_delimiters` | `src/separators.rs` |
| R3 | `fills_in_omitted_encoding_characters`, `stops_reading_encoding_characters_at_the_field_separator` | `src/separators.rs` |
| R4 | `accepts_every_terminator_and_drops_blank_lines` | `src/parse.rs` |
| R5 | `rejects_input_without_a_header` | `src/parse.rs` |
| R6 | `only_a_missing_or_broken_header_is_an_error` | `tests/integration.rs` |
| R7 | `an_empty_field_has_no_repetitions` | `src/parse.rs` |
| R8 | `numbers_header_fields_the_way_hl7_does`, `queries_header_delimiters_literally` | `src/parse.rs`, `src/message.rs` |
| R9 | `decodes_escape_sequences_only_on_request` | `tests/integration.rs` |
| R10 | `distinguishes_absent_empty_and_null` | `src/message.rs` |
| R11 | `distinguishes_absent_empty_and_null`, `carries_the_corners_through_unchanged` | `src/message.rs`, `tests/integration.rs` |
| R12 | `tokenizes_losslessly`, `classifies_every_sequence` | `src/escape.rs` |
| R13 | `unescapes_delimiters_and_hex`, `keeps_undecodable_sequences_literal` | `src/escape.rs` |
| R14 | `escapes_delimiters_and_segment_terminators` | `src/escape.rs` |
| R15 | `escape_and_unescape_round_trip` | `src/escape.rs` |
| R16 | `round_trips_a_canonical_message`, `round_trips_custom_delimiters`, `keeps_empty_positions`, `every_sample_round_trips_byte_for_byte` | `src/render.rs`, `tests/integration.rs` |
| R17 | `decodes_only_the_leaves` | `src/render.rs` |
| R18 | `rejects_malformed_paths` | `src/path.rs` |
| R19 | `queries_repetitions_and_occurrences`, `reads_values_by_path` | `src/message.rs`, `tests/integration.rs` |
| R20 | `a_missing_position_yields_no_value` | `src/message.rs` |
| R21 | `splits_a_batch_file`, `does_not_mistake_a_local_segment_for_an_envelope`, `keeps_a_headerless_first_message_for_parse_to_reject` | `src/parse.rs` |
| R22 | `reads_the_msh_conveniences`, `msh_conveniences_are_none_when_absent` | `src/message.rs` |
| R23 | `only_a_missing_or_broken_header_is_an_error`, `cli_reports_errors_on_stderr` | `tests/integration.rs` |
| R24 | *by review* — enforced by [`AGENTS/safety.md`](../AGENTS/safety.md), not by a test; there is no way to assert the absence of a feature | — |
| R25 | `the_crate_has_no_runtime_dependencies` | `tests/integration.rs` |

The table is **checked by `cargo test`**, not only by review:
`every_rule_has_a_coverage_row` in `tests/integration.rs` reads this file
and [§1.4](01-purpose-and-scope.md) and fails if a rule is declared with no
row here, or covered here without being declared. Its companion
`every_spec_section_is_indexed_and_present` does the same for the section
files and [`index.md`](index.md).

These two are the mechanism behind "the spec is the single source of
truth": the claim now costs a test run rather than a careful reader.

## 13.2 Where a test belongs

| Kind of test | Home |
| ------------ | ---- |
| a rule about one module's own behaviour | `#[cfg(test)] mod tests` at the bottom of that module |
| anything crossing module boundaries | `tests/integration.rs` |
| anything a caller can observe through the public API only | `tests/integration.rs` |
| the CLI contract ([§12](12-command-line-interface.md)) | `tests/integration.rs`, `cli_*` prefix |
| an invariant of the spec itself — the rule index, the section files | `tests/integration.rs` (§13.1) |
| an illustration a reader should see in the docs | a rustdoc `Example:` block, which runs as a doc-test |

## 13.3 The four checks

Every change runs all four, and all four are clean on `main`:

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

`--all-targets` matters: it is what compiles `examples/` and so keeps the
tutorials from rotting.

## 13.4 Test data

Test messages are either written inline or taken from `samples/`:

| Sample | Exercises |
| ------ | --------- |
| `samples/oru_r01.er7` | a lab result: repeated `OBX`, a repeated field, subcomponents, an escaped `&` |
| `samples/adt_a08.er7` | an admission update: a repeated field with occurrence indices, a local `ZPD` segment |
| `samples/batch.er7` | a batch file: `FHS`/`BHS`/`BTS`/`FTS` envelope, two `ACK` messages, an `ERR` segment |

Plus the `EDGES` constant in `tests/integration.rs`, which packs the
corners into one message: a v2.7 truncation character, an explicit null, a
formatting escape, a decoded delimiter, and hex data.

All sample data is **synthetic**. See
[`AGENTS/safety.md`](../AGENTS/safety.md) — no test may contain real
patient data, and identifiers should be obviously fictional
(`EVERYWOMAN^EVE`, `444333222`).

## 13.5 What is not tested, and why

- **Performance.** There are no benchmarks. The crate is a single-pass
  parser over small inputs; a benchmark suite would cost more to maintain
  than the guidance it gives. Recorded as [T3](17-open-tasks.md).
- **Fuzzing.** Parsing is total below the header (R6), so there is no
  panic-freedom claim a fuzzer would falsify — but that claim is currently
  argued rather than demonstrated. Recorded as [T1](17-open-tasks.md).
- **The absence of features (R24).** No test can assert that a dictionary
  is absent. This is enforced by review.
