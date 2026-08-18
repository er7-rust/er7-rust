[AGENTS.md](../AGENTS.md) → conventions

# Conventions

## Three properties every change protects

1. **The shape does not move** (D1). Actions write leaf text. If a change
   adds, removes, or reorders a node, it is wrong — unless it is `Null`,
   which is the documented exception.
2. **Nothing is invented** (D2, D3, D4). No position is created, no empty
   leaf is filled, no null is overwritten.
3. **The report holds no values** (D13). Not the old text, not the new. A
   report is meant to be pasted into a ticket.

## Reading and writing a value

Always `Subcomponent::value` in, `Subcomponent::set` out. Assigning `raw`
directly is a bug waiting to happen: an unescaped `&` in a replacement
splits the component the next time the message is parsed, shifting every
value after it.

The one place `raw` is assigned is the explicit null, which is structure
rather than text.

## Lints

`cargo clippy --all-targets -- -D warnings` must be clean, and
`Cargo.toml` turns on the **pedantic** group
([spec §12.3](../spec/12-dependencies-and-build.md)), so pedantic findings
fail the build too. Three habits follow:

- **`#[must_use]` on every pure accessor and builder step.** A discarded
  `Report` is a redaction nobody reviewed.
- **`# Errors` on every public `fn` returning `Result`**, naming what it
  refuses — reading a policy is the one place this crate is strict.
- **`# Panics` only where a panic is reachable.** The built-in policies
  `expect` on their own literals; the docs say exactly that, and
  `the_documented_positions_match_the_built_in_policy` proves it cannot
  happen. Where a panic is *not* reachable, restructure until clippy
  agrees rather than documenting a fiction.

Where a lint is wrong for one line, `#[allow(..., reason = "...")]` next to
that line — never a crate-level hole in the group.

## Doc comments

Every public item is documented, and `cargo rustdoc --lib -- -W
missing-docs` stays clean. The shape:

```rust
/// One sentence saying what it is.
///
/// A paragraph of why, where the why is not obvious — especially where a
/// safer-looking alternative was declined.
///
/// Example:
///
/// ```
/// // Assertions, not prints. A doc example is a test.
/// ```
```

Cite the rule and the section for anything normative: `(D4, spec §4.3)`.
Prose in comments explains *why*; the spec says *what*.

## Naming

- A test is named for the rule it enforces, in prose:
  `leaves_an_explicit_null_alone`, not `test_null`.
- An action is named for what a reader gets, not for what the code does:
  `First(4)` keeps the first four characters.
- Nothing is abbreviated. `subcomponent`, not `sub`.

## Style

- `cargo fmt`, always.
- Match `er7`'s idiom: `let Some(x) = … else { continue };` for a position
  that is not there, `checked_sub(1)` for 1-based indexing.
- Small free functions over methods when there is no state to carry.
- No `unwrap()` outside tests and `expect("…")` on a literal that cannot
  fail, with the reason in the message.
