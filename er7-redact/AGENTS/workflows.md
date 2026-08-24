[AGENTS.md](../AGENTS.md) → workflows

# Workflows

## Commands

```sh
cargo build                                 # build
cargo test                                  # unit, integration, and doc tests
cargo test -- --nocapture                   # show println!() output
cargo doc --no-deps --open                  # build and open rustdoc

cargo run -- samples/adt_a08.er7            # redact a sample
cargo run -- --report samples/adt_a08.er7   # what would change
cargo run -- --show-policy                  # the built-in policy, as a file
cargo run -- --all-but-the-header samples/oru_r01.er7   # reject all but MSH
cargo run --example redact_a_message        # run an example

cargo clippy --all-targets -- -D warnings   # lint
cargo fmt                                   # format
cargo rustdoc --lib -- -W missing-docs      # confirm every public item is documented
```

The last four are the **four checks**; all four are clean on `main` and
must stay that way.

## Daily flow

1. Read the matching `spec/` section, and edit it first if behaviour is
   changing ([spec-driven-development](spec-driven-development.md)).
2. Write or update the test; watch it fail.
3. Change the code until it passes and the rest still do.
4. Update the §11.1 coverage table if a rule was added or moved.
5. Update the derived docs — `index.md`, `docs/**`, `examples/**`.
6. Run the four checks.
7. Commit, naming the spec section and any `T<n>` the change closes.

## Seeing what a message actually holds

The sibling `er7` command is the tool for this, and is worth running before
writing any policy:

```sh
er7 samples/adt_a08.er7                  # outline: every value, labelled with its path
er7 -q PID-3.1 samples/adt_a08.er7       # one position
```

Every label in the outline is a valid rule path, so a policy can be written
by reading the outline.

## Pitfalls

- **A `#` in a policy argument.** It starts a comment, so `mask #` is read
  as a bare `mask`. Build the rule in Rust
  ([spec §16.4](../spec/16-open-questions-and-declined-decisions/index.md)).
- **Samples are CR-terminated.** Editing `samples/*.er7` with a tool that
  normalises line endings turns them into `\n` and breaks the round-trip
  tests.
- **`Policy::accept_all()` is empty and there is no `Default`.** That is
  deliberate ([spec §5.1](../spec/05-built-in-policies/index.md)).
- **Rules apply in order.** A later rule sees the earlier one's output, and
  `Keep` does not undo.
