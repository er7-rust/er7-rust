[`er7` specification](../index.md) — section 13 of 19. Section numbers
(§13.x) are stable and cited from code, tests, and commit messages.

# 13. Testing strategy

Conventions for writing tests live in
[`AGENTS/testing.md`](../../AGENTS/testing.md). This section says what must be
covered and where.

## 13.1 Rule coverage

Every rule in [§1.4](../01-purpose-and-scope/index.md) names the test that
enforces it. A rule with no test is a bug in this table.

| Rule | Test | File |
| ---- | ---- | ---- |
| R1 | `reads_custom_delimiters`, `honors_a_message_that_chooses_its_own_delimiters` | `src/separators.rs`, `tests/main.rs` |
| R2 | `rejects_unusable_delimiters` | `src/separators.rs` |
| R3 | `fills_in_omitted_encoding_characters`, `stops_reading_encoding_characters_at_the_field_separator` | `src/separators.rs` |
| R4 | `accepts_every_terminator_and_drops_blank_lines` | `src/parse.rs` |
| R5 | `rejects_input_without_a_header` | `src/parse.rs` |
| R6 | `only_a_missing_or_broken_header_is_an_error` | `tests/main.rs` |
| R7 | `an_empty_field_has_no_repetitions` | `src/parse.rs` |
| R8 | `numbers_header_fields_the_way_hl7_does`, `queries_header_delimiters_literally` | `src/parse.rs`, `src/message.rs` |
| R9 | `decodes_escape_sequences_only_on_request` | `tests/main.rs` |
| R10 | `distinguishes_absent_empty_and_null` | `src/message.rs` |
| R11 | `distinguishes_absent_empty_and_null`, `carries_the_corners_through_unchanged` | `src/message.rs`, `tests/main.rs` |
| R12 | `tokenizes_losslessly`, `classifies_every_sequence` | `src/escape.rs` |
| R13 | `unescapes_delimiters_and_hex`, `keeps_undecodable_sequences_literal` | `src/escape.rs` |
| R14 | `escapes_delimiters_and_segment_terminators`, `escapes_the_null_text_so_it_stays_text` | `src/escape.rs` |
| R15 | `escape_and_unescape_round_trip` | `src/escape.rs` |
| R16 | `round_trips_a_canonical_message`, `round_trips_custom_delimiters`, `keeps_empty_positions`, `every_sample_round_trips_byte_for_byte` | `src/render.rs`, `tests/main.rs` |
| R17 | `decodes_only_the_leaves` | `src/render.rs` |
| R18 | `rejects_malformed_paths` | `src/path.rs` |
| R19 | `queries_repetitions_and_occurrences`, `reads_values_by_path` | `src/message.rs`, `tests/main.rs` |
| R20 | `a_missing_position_yields_no_value` | `src/message.rs` |
| R21 | `splits_a_batch_file`, `does_not_mistake_a_local_segment_for_an_envelope`, `keeps_a_headerless_first_message_for_parse_to_reject` | `src/parse.rs` |
| R22 | `reads_the_msh_conveniences`, `msh_conveniences_are_none_when_absent` | `src/message.rs` |
| R23 | `only_a_missing_or_broken_header_is_an_error`, `cli_reports_errors_on_stderr` | `tests/main.rs` |
| R24 | *by review* — enforced by [`AGENTS/safety.md`](../../AGENTS/safety.md), not by a test; there is no way to assert the absence of a feature | — |
| R25 | `the_crate_has_no_runtime_dependencies` | `tests/main.rs` |
| R26 | `first_value_reads_a_leaf_scoped_to_one_segment` | `src/message.rs` |
| R27 | `reads_messages_one_at_a_time_from_a_bufread`, `read_messages_matches_split_messages_on_every_batch_shape` | `src/parse.rs` |

The table is **checked by `cargo test`**, not only by review:
`every_rule_has_a_coverage_row` in `tests/main.rs` reads this file
and [§1.4](../01-purpose-and-scope/index.md) and fails if a rule is
declared with no row here, or covered here without being declared. Its
companion `every_spec_section_is_indexed_and_present` does the same for the
section files and [`index.md`](../index.md).

These two are the mechanism behind "the spec is the single source of
truth": the claim now costs a test run rather than a careful reader.

## 13.2 Where a test belongs

| Kind of test | Home |
| ------------ | ---- |
| a rule about one module's own behaviour | `#[cfg(test)] mod tests` at the bottom of that module |
| anything crossing module boundaries | `tests/main.rs` |
| anything a caller can observe through the public API only | `tests/main.rs` |
| the CLI contract ([§12](../12-command-line-interface/index.md)) | `tests/main.rs`, `cli_*` prefix |
| an invariant of the spec itself — the rule index, the section files | `tests/main.rs` (§13.1) |
| an illustration a reader should see in the docs | a rustdoc `Example:` block, which runs as a doc-test |

## 13.3 The four checks

Every change runs all four, and all four are clean on `main`:

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

The second check is stricter than it looks: `Cargo.toml` sets
`[lints.clippy] pedantic = "warn"`, so **the pedantic group is on** and
`-D warnings` turns any of it into a failure. That is a deliberate choice
for a crate meant to sit at the bottom of a healthcare stack — the group's
`must_use_candidate`, `missing_errors_doc`, and `missing_panics_doc` are
exactly the lints that make a public API hard to misuse from the outside,
and a discarded `to_er7()` or an undocumented `Err` case is a real bug in
this domain. Where a pedantic lint is wrong for a specific line, the fix is
an `#[allow]` **with a `reason`**, not a hole in the group.

`--all-targets` matters: it is what compiles `examples/` and so keeps the
tutorials from rotting.

The workspace also requires **100% executable-line coverage** across the
three published crates' `src/` trees:

```sh
bin/test-coverage
```

The check runs all targets and reads cargo-llvm-cov's merged LCOV `DA`
records. That distinction is deliberate: the summary table counts the same
generic or linked function again for each integration test and example binary,
including zero-test example harnesses, while an LCOV source line has one merged
execution count. The gate therefore means exactly that every executable source
line ran in at least one target; it does not relabel compiler-generated regions
or duplicate monomorphizations as independent product behavior. Benchmark
crates are excluded because they are measurement harnesses, not published
source. CI pins the coverage driver version that produces the report.

## 13.4 Test data

Test messages are either written inline or taken from `samples/`:

| Sample | Exercises |
| ------ | --------- |
| `samples/oru_r01.er7` | a lab result: repeated `OBX`, a repeated field, subcomponents, an escaped `&` |
| `samples/adt_a08.er7` | an admission update: a repeated field with occurrence indices, a local `ZPD` segment |
| `samples/batch.er7` | a batch file: `FHS`/`BHS`/`BTS`/`FTS` envelope, two `ACK` messages, an `ERR` segment |
| `samples/multilingual.er7` | synthetic UTF-8 text in three scripts, including a decomposed accent, repetitions, an escaped delimiter, and an explicit null |

Plus the `EDGES` constant in `tests/main.rs`, which packs the
corners into one message: a v2.7 truncation character, an explicit null, a
formatting escape, a decoded delimiter, and hex data.

All sample data is **synthetic**. See
[`AGENTS/safety.md`](../../AGENTS/safety.md) — no test may contain real
patient data, and identifiers should be obviously fictional
(`EVERYWOMAN^EVE`, `444333222`).

## 13.5 What is not tested, and why

- **The absence of features (R24).** No test can assert that a dictionary
  is absent. This is enforced by review, and always will be — it is not
  a gap that closes with more test-writing.

## 13.6 Fuzzing and benchmarks

Both used to be recorded here as absent (T1 and T3 in
[§17](../17-open-tasks/index.md), respectively); both are now in place,
and this section is the citation each task's own "done when" asked for.

**Fuzzing.** R6 claims parsing is total below the header — nothing there
can panic, overflow the stack, or hang, whatever bytes arrive. `er7/fuzz/`
carries `parse_roundtrip` (fuzzes `parse`, including the round-trip
idempotence claim in R12) and `parse_with_total` (fuzzes `parse_with`
directly, which is what actually reaches the below-the-header logic on
most fuzzer-generated input — `parse` itself rejects anything without a
recognisable `MSH`/`FHS`/`BHS` header before getting there at all). Both
ran clean, no panics found, on 2026-08-28: `parse_roundtrip` completed
52,355,877 executions in 120 seconds; `parse_with_total` completed
2,487,369 in the same window. **Mutation testing.** `cargo mutants -p er7` (cargo-mutants 27.1.0,
2026-11-26): 378 mutants. 339 were caught, 21 did not compile, and 8 timed
out (mutants that make `read_messages` loop forever, which the tests
detect). 10 survived. Eight were real gaps: the CLI outline's exact layout,
and `query` stopping before a later segment or repetition that has the
value. `cli_outline_is_exact` and
`query_looks_past_a_position_that_lacks_the_value` close them, and a
re-run of the affected functions catches 29 of 30. The two that remain are
equivalent, meaning no input can tell them apart:
- deleting the `Text`/`Unterminated` arm of `unescape`, whose fallback
  writes the same text;
- deleting `!` in the outline's empty-field skip, since a parsed empty
  field has no repetitions and so prints nothing either way.

Three more targets were added on 2026-11-26, each building its input so
that most runs reach the property instead of failing early:
`malformed_header` (any field separator and an MSH-2 of 0–8 characters;
an `Ok` must validate, agree with `Separators::from_header`, and
round-trip), `escape_custom_delimiters` (R12/R14/R15 under a delimiter
set the fuzzer picks), and `repeating_fields` (values written with `set`
come back one per repetition, never as the explicit null). Local runs:
2,219,133 executions in 120 s, 4,836,747 in 60 s, and 705,153 in 60 s,
all clean. The first `malformed_header` run stopped on input `J`: a
letter as field separator gives `MissingHeader`, which §3.3 and §4.4
specify, so the target's assertion was corrected and the parser left as it
was. `escape_roundtrip` and `query_paths` fuzz
adjacent properties (escape/unescape round-tripping, and `query` agreeing
with the head of `query_all`) and are not R6 evidence specifically. The
targets live in `er7/fuzz/`, its own workspace, so `er7` itself keeps R25.
CI builds and briefly smoke-runs all four on every push
(`.github/workflows/ci.yml`'s `fuzz` job); the durations above are a
longer, one-off run, not what CI itself spends per push.

**Benchmarks.** `er7-bench/benches/er7.rs`, a Criterion suite covering
parse, render, escape, and query — deliberately outside the published
crate, the same way the fuzz targets are, so `er7` keeps R25 even here.
Method and current figures are [`BENCHMARKS.md`](../../../BENCHMARKS.md)
at the workspace root; `cargo bench -p er7-bench` reproduces them.
