[AGENTS.md](../AGENTS.md) → workflows

# Workflows

## Commands

```sh
cargo build                                 # build
cargo test                                  # unit, integration, and doc tests
cargo test -- --nocapture                   # show println!() output
cargo doc --no-deps --open                  # build and open rustdoc

cargo run --example round_trip_via_json     # ER7 → JSON → ER7
cargo run --example build_message_from_json # JSON → ER7
cargo run --example inspect_a_segment_as_json
cargo run --example catch_a_typo_with_strict # Strict<T> rejects an unknown field

cargo clippy --all-targets -- -D warnings   # lint
cargo fmt --check                           # format
cargo rustdoc --lib -- -W missing-docs      # confirm every public item is documented
```

The last four are the **four checks**; all four are clean on `main` and
must stay that way.

## Daily flow

1. Read the matching `spec/` section, and edit it first if behaviour is
   changing ([spec-driven-development](spec-driven-development.md)).
2. Write or update the test; watch it fail.
3. Change the code until it passes and the rest still do.
4. Update the §7.1 coverage table if a rule was added or moved.
5. Update the derived docs — `index.md`, `docs/**`, `examples/**`.
6. Run the four checks.
7. Commit, naming the spec section that changed.

## Pitfalls

- **The integration tests read `../er7/samples/`.** `tests/integration.rs`
  pulls the `er7` crate's own sample messages straight from the sibling
  workspace member via `include_str!`. That is deliberate — it tests this
  crate against the same fixtures `er7` tests itself against, rather than
  against literals written to order — and it is why `tests/` is excluded
  from the published package.
- **`er7` is a path dependency, not a registry dependency.** `Cargo.toml`
  says `er7 = { path = "../er7", version = "0" }`, so a local change to
  `er7` is picked up immediately — no `[patch.crates-io]` needed. Only
  when this crate is published on its own does the `version` requirement
  take over.
- **A wire shape is a compatibility surface.** Changing one is a breaking
  change even when the Rust signature is untouched
  ([§8](../spec/08-versioning-and-compatibility/index.md)).
- **`serde_json` is a dev-dependency only.** If it ever appears under
  `[dependencies]`, `no_format_crate_is_a_runtime_dependency` fails, which
  is the point.
