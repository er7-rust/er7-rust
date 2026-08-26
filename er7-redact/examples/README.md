[er7-redact](../index.md) → examples

# Examples

Runnable programs demonstrating the `er7-redact` crate. Each is a single
file with no setup, invoked via `cargo run --example <name>`.

| Example | Purpose |
| ------- | ------- |
| [redact_a_message](redact_a_message.rs) | The built-in policy end to end: what goes, what stays, and why the message still works. |
| [write_a_policy](write_a_policy.rs) | Three ways to say what to redact — in Rust, from a file, or by extending a built-in. |
| [reject_by_default](reject_by_default.rs) | The other posture: reject every value, with `keep` rules accepting what a test needs. |
| [pseudonyms_and_linkage](pseudonyms_and_linkage.rs) | Why an identifier becomes a pseudonym rather than a blank, what that buys, and what it costs. |
| [read_the_report](read_the_report.rs) | The audit trail: one row per position changed, and no values in it. |
| [redact_absent_empty_null](redact_absent_empty_null.rs) | The three states HL7® keeps apart, and why redaction leaves two of them alone. |

## Running

```sh
# Build them all without running — this is what `cargo test` and
# `cargo clippy --all-targets` do, so a broken example fails the build.
cargo build --examples

# Run one.
cargo run --example redact_a_message

# Run them all.
for e in redact_a_message write_a_policy reject_by_default \
         pseudonyms_and_linkage read_the_report redact_absent_empty_null; do
    echo "== $e"; cargo run --quiet --example "$e";
done
```

## Suggested order

If you are new to the crate, read them in this order — each builds on the
one before:

1. **redact_a_message** — what redaction does, and what it leaves.
2. **write_a_policy** — how to say what you want redacted.
3. **read_the_report** — how to check what actually happened.
4. **redact_absent_empty_null** — the distinction that matters most clinically.
5. **pseudonyms_and_linkage** — how to keep a message joinable, and the
   price of doing so.
6. **reject_by_default** — what to do when you do not trust the list.

## Guarantees

- Every example uses only the **published public API** — no `pub(crate)`
  items, no internals. If an example needs something the library does not
  export, the library is missing it.
- Every example **asserts its own results**, so a clean exit means it
  passed. They are compiled by `cargo test` and linted by
  `cargo clippy --all-targets`, so a tutorial that stopped being true
  fails the build.
- Every message is **synthetic**. Names are obviously fictional
  (`EVERYWOMAN^EVE`, `JONES^WILLIAM`) and identifiers obviously fake
  (`PATID1234`, `444333222`). No example contains, or could collide with,
  real patient data — see [`AGENTS/safety.md`](../AGENTS/safety.md), which
  matters more in this repository than in any of its siblings.

## See also

- [`docs/usage/`](../docs/usage/index.md) — the same ground as prose, with
  more explanation
- [`docs/policies/`](../docs/policies/index.md) — the policy file format
  and the built-in tables
- [`docs/api/`](../docs/api/index.md) — the complete public API surface
- [`spec/`](../spec/index.md) — the normative rules these examples
  illustrate
- [`samples/`](../samples/) — the ER7 messages and the policy file the CLI
  examples use

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
