[AGENTS.md](../AGENTS.md) → testing

# Testing

What to test, where to put it, and how to write it. The *coverage
obligation* — which rule needs which test — is in
[`spec/13-testing-strategy/index.md`](../spec/13-testing-strategy/index.md);
this file is about craft.

## The four checks

Run all four before finishing any change. All four are clean on `main`.

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

`--all-targets` is not optional: it is what compiles `examples/`, and an
example that no longer compiles is a tutorial that lies.

## Where a test goes

| Kind | Home |
| ---- | ---- |
| a rule about one module's own behaviour | `#[cfg(test)] mod tests` at the bottom of that module |
| anything crossing module boundaries | `tests/integration.rs` |
| anything observable only through the public API | `tests/integration.rs` |
| the CLI contract | `tests/integration.rs`, `cli_*` prefix |
| an illustration a reader should see | a rustdoc `Example:` block |

When in doubt, prefer the unit test: it fails closer to the cause.

## Naming

A test name is a **sentence about behaviour**, not about the function under
test. Read it as "it …":

```
✔ keeps_undecodable_sequences_literal
✔ numbers_header_fields_the_way_hl7_does
✔ a_missing_position_yields_no_value
✘ test_unescape
✘ unescape_works
✘ test_case_3
```

The name is what a reader sees when it fails, so it should say what broke.

## Shape

- **One behaviour per test**, but several assertions are fine when they are
  the same behaviour at different depths — `queries_each_depth` asserting
  field, component, and subcomponent is one behaviour.
- **Assert the value, not just the shape.** `assert_eq!(…, Some("SMITH"))`
  beats `assert!(result.is_some())`.
- **Loop over cases** when a rule has many instances, and pass the case into
  the failure message so a failure names itself:

  ```rust
  for text in ["", "-5", "PID-", "PID-0"] {
      assert!(Path::parse(text).is_err(), "expected {text:?} to be rejected");
  }
  ```

- **Cite the spec in a comment** when a test exists because of a rule that
  is not obvious from the assertion:

  ```rust
  // The explicit null is text, so a null field is not empty (R11).
  assert!(!obx.field(5).unwrap().is_empty());
  ```

## Round-trip tests

The most valuable test in this crate is the cheapest to write:

```rust
assert_eq!(er7::parse(text).unwrap().to_er7(), text);
```

Add one whenever you add a parse rule, a write rule, or a sample. Use
canonical text — `\r` terminators, no blank lines — or pass
`RenderOptions` that match the input
([§7.2](../spec/07-writing/index.md)).

## Doc-tests

A rustdoc `Example:` block is a test that a reader also sees, which makes it
the highest-leverage test in the crate. Requirements:

- It must **run**, not be marked `ignore` or `no_run`. If an example cannot
  run, it is the wrong example.
- It must **assert**, not just print. A doc-test that only calls a function
  proves nothing.
- Prefer `?` with a `# fn main() -> Result<(), er7::Error> {` wrapper over
  `.unwrap()`, so the example models what a caller should write.
- Keep the message text short. The point is the call, not the data.

## Test data

- **All test data is synthetic.** See [`safety.md`](safety.md). Never paste
  a real message, even redacted.
- Use the `samples/` files for realistic shapes and inline strings for
  focused cases.
- Names should be obviously fictional (`EVERYWOMAN^EVE`, `SMITH^JOHN`),
  identifiers obviously fake (`444333222`, `MSG00042`).

## CLI tests

The `cli()` helper in `tests/integration.rs` spawns the built binary via
`env!("CARGO_BIN_EXE_er7")` and returns `(success, stdout, stderr)`. Use it
for anything in [§12](../spec/12-command-line-interface/index.md), and
assert on:

- exit status,
- exact `stdout` where the format is part of the contract,
- a distinctive substring of `stderr` plus the `er7: error: ` prefix,
- that `stdout` is **empty** when the command fails.

## What not to test

- Private helpers, except through the behaviour they implement.
- `Debug` output — it is not a contract.
- Exact error message wording beyond a distinctive substring; the wording
  should be improvable without breaking tests.
- The absence of a feature (R24). No test can assert that; review enforces
  it.
