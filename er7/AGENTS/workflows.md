[AGENTS.md](../AGENTS.md) → workflows

# Workflows

Commands and daily flow. Release mechanics are in
[`release.md`](release.md).

## The four checks

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

All four are clean on `main`. Run all four before finishing any change; run
`cargo test` before starting one, to confirm a green baseline.

## Building and testing

```sh
cargo build                        # Debug build (library + er7 binary)
cargo build --release              # Release build
cargo build --examples             # Compile every example without running

cargo test                         # Everything
cargo test --lib                   # Unit tests only, fastest inner loop
cargo test --test integration      # Integration and CLI tests only
cargo test --doc                   # Doc-tests only
cargo test round_trip              # Every test whose name contains "round_trip"
cargo test -- --nocapture          # Show println!() output
cargo test -- --exact escape::tests::tokenizes_losslessly   # One test
```

## Documentation

```sh
cargo doc --no-deps --open         # Build and open rustdoc for this crate
cargo doc --no-deps                # Build without opening
cargo rustdoc --lib -- -W missing-docs   # Warn on any undocumented public item
cargo test --doc                   # Run the examples inside the doc comments
```

Every public item must be documented and every `Example:` block must run;
see [`conventions.md`](conventions.md) for the doc-comment shape.

Rendered long-form docs live in [`docs/`](../docs/) and are written by hand,
not generated. When you change behaviour, check whether
[`docs/api/index.md`](../docs/api/index.md) still describes it.

## Running the CLI

```sh
cargo run -- samples/oru_r01.er7                      # Outline every value
cargo run -- --query PID-5.1 samples/oru_r01.er7      # One path
cargo run -- -q OBX-5 -q OBX-6 samples/oru_r01.er7    # Several paths
cargo run -- --raw -q OBX-5 samples/oru_r01.er7       # Without decoding escapes
cargo run -- --normalize -t lf samples/oru_r01.er7    # Canonical ER7, readable
cargo run -- --message 2 samples/batch.er7            # One message of a batch
cat samples/batch.er7 | cargo run --                  # From standard input
cargo run -- --help
```

Because ER7 uses carriage returns, a message printed with the default
terminator looks like one line in a terminal. Add `-t lf` when you want to
read it.

## Running examples

```sh
cargo run --example parse_a_message
cargo run --example query_by_path
cargo run --example edit_a_value
cargo run --example escape_sequences
cargo run --example split_a_batch
cargo run --example custom_delimiters
cargo run --example absent_empty_null
```

Each example asserts its own results, so a clean exit means it passed. See
[`examples/README.md`](../examples/README.md).

## Daily flow for a behavioural change

1. `cargo test` — confirm green.
2. Read the matching `spec/` section
   ([`spec-driven-development.md`](spec-driven-development.md)).
3. Edit the spec section to state the target behaviour.
4. Write or update the test that encodes it; watch it fail.
5. Edit the code until it passes and nothing else broke.
6. Update the rule index (`spec/01` §1.4) and coverage table (`spec/13`
   §13.1) if you added or changed a rule.
7. Update derived docs — `index.md`, `docs/**`, `examples/**` — where they
   now read wrong.
8. Run the four checks.
9. Commit with the spec section in the message.

## Inspecting a message quickly

The fastest way to understand an unfamiliar message is the CLI outline,
which labels every value with the path that names it:

```sh
cargo run -- suspect.er7 | less
cargo run -- suspect.er7 | grep '^PID'
cargo run -- --raw suspect.er7 | grep '\\'   # Find every escape sequence
```

## Common pitfalls

| Symptom | Cause |
| ------- | ----- |
| a round-trip test fails on a file you edited | your editor added a trailing newline or converted `\r` to `\n`; the sample files use real `\r` bytes |
| `parse` returns `MissingHeader` on a readable-looking message | the message was pretty-printed and the first line has leading whitespace ([§18.5](../spec/18-open-questions-and-divergences.md)) |
| a doc-test fails but `cargo test --lib` passes | the example in a doc comment is out of date; run `cargo test --doc` |
| clippy passes locally, CI fails | you ran it without `--all-targets`, so `examples/` was skipped |
