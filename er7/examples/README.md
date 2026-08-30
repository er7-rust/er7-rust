[er7](../index.md) → examples

# Examples

Runnable programs demonstrating the `er7` crate. Each is a single file with
no setup, invoked via `cargo run --example <name>`.

| Example | Purpose |
| ------- | ------- |
| [parse_a_message](parse_a_message.rs) | The two entry points, a message's own delimiters, and the five MSH routing accessors. |
| [query_by_path](query_by_path.rs) | The four query methods, occurrence indices, and the two behaviours that surprise people. |
| [edit_a_value](edit_a_value.rs) | Editing through `set`, why assigning `raw` is riskier, structural edits, and writing back out. |
| [build_a_message](build_a_message.rs) | Building an ACK from scratch: `parse_with` as the builder for known text, `Vec` fields for what is not text yet. |
| [escape_sequences](escape_sequences.rs) | What decodes, what is kept literal, and rendering formatted text from the token stream. |
| [split_a_batch](split_a_batch.rs) | A full `FHS`/`BHS`/`BTS`/`FTS` envelope, borrowed slices, and carrying on past a bad message. |
| [stream_a_batch](stream_a_batch.rs) | The same batch, read one message at a time from a `BufRead` without holding the whole file in memory. |
| [custom_delimiters](custom_delimiters.rs) | A message using `#*!?@`, the v2.7 truncation character, and what gets rejected. |
| [absent_empty_null](absent_empty_null.rs) | Telling "not sent" from "sent blank" from "clear this value". |

## Running

```sh
# Build them all without running — this is what `cargo test` and
# `cargo clippy --all-targets` do, so a broken example fails the build.
cargo build --examples

# Run one.
cargo run --example parse_a_message

# Run them all.
for e in parse_a_message query_by_path edit_a_value build_a_message \
         escape_sequences split_a_batch stream_a_batch custom_delimiters \
         absent_empty_null; do
    echo "== $e"; cargo run --quiet --example "$e";
done
```

## Suggested order

If you are new to the crate, read them in this order — each builds on the
one before:

1. **parse_a_message** — how text becomes a `Message`.
2. **query_by_path** — how to get values out.
3. **absent_empty_null** — the distinction that matters most clinically.
4. **escape_sequences** — how values carry delimiters.
5. **edit_a_value** — how to change a message safely.
6. **build_a_message** — how to write one that never existed as text at all.
7. **split_a_batch** — how to handle more than one message.
8. **stream_a_batch** — the same, without holding the whole file in memory.
9. **custom_delimiters** — why none of the above assumed `|^~\&`.

## Guarantees

- Every example uses only the **published public API** — no `pub(crate)`
  items, no internals. If an example needs something the library does not
  export, the library is missing it.
- Every example **asserts its own results**, so a clean exit means it
  passed. They are compiled by `cargo test` and linted by
  `cargo clippy --all-targets`, so a tutorial that stopped being true fails
  the build.
- Every message is **synthetic**. Names are obviously fictional
  (`EVERYWOMAN^EVE`, `SMITH^JOHN`) and identifiers obviously fake
  (`444333222`, `MSG00042`). No example contains, or could collide with,
  real patient data — see [`AGENTS/safety.md`](../AGENTS/safety.md).

## See also

- [`docs/usage/`](../docs/usage/index.md) — the same ground as prose, with
  more explanation
- [`docs/api/`](../docs/api/index.md) — the complete public API surface
- [`spec/`](../spec/index.md) — the normative rules these examples
  illustrate
- [`samples/`](../samples/) — the ER7 message files the CLI examples use
