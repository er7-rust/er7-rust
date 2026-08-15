# AGENTS.md

Instructions for coding agents (Claude Code, Codex, or any other) working
in this repository. `CLAUDE.md` is a pointer to this file — keep this one
canonical and don't fork the content between the two.

## What this is

A zero-dependency Rust crate and CLI for the **ER7** encoding of HL7 v2
messages: parse, query, edit, write. See `README.md` for the user-facing
pitch, `spec/er7-format.md` for the format itself, and `spec/index.md` for
the exact, normative rules — **`spec/index.md` is the single source of
truth for behavior.** If you change what the crate does, update that file
in the same change; if you're unsure whether a change is a bug fix or a
behavior change, check it against the spec first.

## Layout

```
src/lib.rs          Crate docs, Error, re-exports.
src/separators.rs   Separators (the delimiter set) and Terminator; reading
                     delimiters from a header, and validating them.
src/escape.rs       Escape sequences: the Escape token vocabulary, the
                     escapes() tokenizer, unescape(), escape(), decode_hex().
src/message.rs      The value tree — Message, Segment, Field, Repetition,
                     Component, Subcomponent — plus accessors, the
                     absent/empty/null predicates, queries, and the five
                     MSH conveniences.
src/parse.rs        Text to tree: parse(), parse_with(), split_messages().
src/render.rs       Tree to text: to_er7()/to_text() at every level, and
                     RenderOptions.
src/path.rs         HL7 paths such as PID-5.1 and OBX[2]-5.
src/main.rs         The CLI: outline, --query, --normalize.
tests/integration.rs Black-box tests through the public API and the CLI.
spec/index.md       Normative specification (source of truth).
spec/er7-format.md  Background on ER7 itself, independent of this crate.
samples/*.er7       Example messages used by the README and by tests.
```

Each module has unit tests in a trailing `#[cfg(test)] mod tests` block;
anything crossing module boundaries or touching the CLI contract goes in
`tests/integration.rs` instead.

## Working conventions

- **Rust edition 2024**, zero runtime dependencies — keep it that way
  unless the user asks otherwise; being dependency-free is part of this
  crate's value.
- Every public item needs a doc comment; `src/lib.rs` carries
  `#![warn(missing_docs)]`. Match the existing register: say what the item
  is, and where the *why* isn't obvious from the code, say that too in a
  sentence.
- Before finishing a change, run all four:
  ```sh
  cargo test                                # unit, integration, doc tests
  cargo clippy --all-targets -- -D warnings
  cargo fmt --check
  cargo rustdoc --lib -- -W missing-docs
  ```
  All four are clean on `main`; keep them that way.
- New behavior needs a test. Prefer a unit test next to the code for
  parsing, escaping, and naming rules; an integration test for anything a
  caller or the CLI can observe.

## The three properties to protect

These are what the crate is for. A change that breaks one of them is wrong
even if every test still passes.

1. **Round trip.** `parse(text).to_er7()` reproduces canonical input byte
   for byte (`spec/index.md` §6.2). This is why leaf text is stored raw and
   decoded on demand, and why nothing but blank lines is trimmed. Don't
   "normalize" values at parse time.
2. **Nothing fails below the header.** Unknown segments, ragged fields,
   odd delimiters, undecodable escapes — all data, never errors
   (`spec/index.md` §10). Don't turn a fallback into a failure.
3. **Absent, empty, and null stay distinct** (`spec/index.md` §4.2).
   Collapsing them is a patient-safety bug, not a simplification: the
   explicit `""` means *clear this value*.

## Making a spec-affecting change

1. Update `spec/index.md` first, or alongside the code, so it states the
   new intended behavior precisely.
2. Implement it, respecting the module boundaries above.
3. Add or update tests that pin the new behavior.
4. Update `README.md` only if the change affects its summary or examples —
   the README intentionally doesn't restate everything the spec covers.
5. Run the four checks.

## Non-goals (don't "fix" these without discussion)

- **Adding a dictionary.** Segment field tables, data types, message
  structures, code tables: all deliberately out of scope
  (`spec/index.md` §1, §11). The sibling crate
  `hl7-2-5-to-xml-using-rust` is where that layer lives.
- **Validation.** Cardinality, required fields, lengths, table membership.
- **Transport.** MLLP framing, TCP, acknowledgement workflows.
- **Interpreting formatting escapes.** `\.br\`, `\H\`, `\Cxxyy\` are
  preserved as written, not rendered (`spec/index.md` §5.2).
- **Deriving a message structure** from MSH-9.1 and MSH-9.2 when MSH-9.3 is
  absent — that mapping is version-specific and belongs above this layer
  (`spec/index.md` §9).
