[AGENTS.md](../AGENTS.md) → testing

# Testing

See [`spec/07-testing-strategy.md`](../spec/07-testing-strategy.md) for the
normative version of this page; this is the practical how-to.

## Running the suite

```sh
cargo test                    # unit + integration + doc tests, everything
cargo test --lib              # unit tests only (fastest inner loop)
cargo test --test integration # integration tests only
cargo test --doc              # doctests only
cargo test -- --nocapture     # show println!() output from tests/examples
```

## The four required checks

Run all four before finishing any change; all four must be clean:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
```

## Adding a test for a new edge case

1. **A new wire-shape edge case for one level** (e.g. a new kind of
   malformed input) → a unit test in that module's `#[cfg(test)] mod
   tests`.
2. **A new round-trip guarantee, or a regression in one** → an integration
   test in `tests/integration.rs`, following the existing
   parse-serialize-deserialize-assert pattern.
3. **A new realistic message shape worth demonstrating** → consider
   whether it belongs as a new `examples/*.rs` (if it teaches a concept) or
   simply a new fixture in an existing integration test (if it is only
   coverage).

## The sibling fixtures

`tests/integration.rs` reads `er7`'s own `samples/*.er7` files directly via
`include_str!("../../er7-rust/samples/...")`. This means:

- These tests **require the sibling checkout** to exist at
  `../er7-rust` relative to this crate — the same layout the path
  dependency in `Cargo.toml` already requires.
- The sample files end with a trailing segment terminator that `er7`
  itself normalizes away at parse time; `tests/integration.rs`'s `sample()`
  helper trims it before comparing, so the round-trip assertion tests this
  crate's own behaviour, not `er7`'s terminator normalization (which
  `er7`'s own test suite already covers).

## Doctest gotcha: raw strings and `\r`

A raw string literal (`r"..."`) does not interpret `\r` as a carriage
return — it is two literal characters, backslash and `r`. Using one where
an actual segment terminator is needed silently produces a single-segment
message instead of the intended multi-segment one, and the resulting
doctest/test failure can be confusing (an assertion about a segment that
"doesn't exist" even though the literal text clearly contains it). Use a
normal string literal with an escaped backslash for the encoding
characters instead: `"MSH|^~\\&|LAB\rPID|1"`. See
[`AGENTS/conventions.md`](conventions.md) for the same note in context.
