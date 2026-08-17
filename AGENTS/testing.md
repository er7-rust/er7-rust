[AGENTS.md](../AGENTS.md) → testing

# Testing

What must be covered is [spec §11](../spec/11-testing-strategy.md). This is
how to write it.

## Where a test goes

| Kind | Home |
| ---- | ---- |
| one module's own behaviour | `#[cfg(test)] mod tests` at the bottom of that module |
| anything crossing modules, or observable only through the public API | `tests/integration.rs` |
| the CLI contract | `tests/integration.rs`, `cli_*` prefix |
| an illustration a reader should see | a rustdoc `Example:` block |

Every rule `D<n>` names its test in the §11.1 coverage table. Adding a rule
means adding a row there in the same change.

## The three assertions

Every redaction test that is not specifically about something else asserts
all three (spec §11.5):

1. the output **parses**;
2. the **shape** is unchanged — use the `shape()` helper;
3. the **original value is gone from the whole message**, not merely from
   the position that was named.

The third catches the bugs. A name survives redaction by being in a second
position nobody listed, and asserting on the redacted position will never
notice.

## Test data

Synthetic, always, and obviously so: `EVERYWOMAN^EVE`, `JONES^WILLIAM`,
`PATID1234`, `444333222`. See [safety](safety.md) — this repository is
exactly where somebody would be tempted to commit a real message to prove
the redaction works.

## The four checks

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc --lib -- -W missing-docs
```

All four are clean on `main` and must stay that way. `--all-targets` is
what compiles `examples/`, which keeps the tutorials honest.
