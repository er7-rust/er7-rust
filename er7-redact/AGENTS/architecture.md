[AGENTS.md](../AGENTS.md) → architecture

# Architecture

## Layout

```
src/lib.rs         crate docs, `Error`, re-exports
src/action.rs      spec §3  — the eight actions, and applying one to a value
src/policy.rs      spec §5, §6 — `Rule`, `Policy`, the built-ins, the file format
src/pseudonym.rs   spec §7  — the keyed hash
src/redact.rs      spec §2, §4, §8 — `Redactor`, the walk, `Report`
src/main.rs        spec §10 — the `er7-redact` command
```

Dependencies run one way: `redact` uses `policy`, `policy` uses `action`,
`action` uses `pseudonym`, and nothing points back up. `lib.rs` holds only
the error type and the re-exports.

## The one dependency

`er7`, and nothing else (D16). It supplies the value tree, the path
notation, the escaping, and the absent/empty/null distinction. Anything
this crate is tempted to reimplement from that list is a bug: use
`Subcomponent::value` to read, `Subcomponent::set` to write, and
`er7::Path` to name a position.

The binary uses the **published public API only**, so anything it needs, a
downstream crate has too.

## The redaction pass

`Redactor::redact` is one pass with a small amount of state, in
`src/redact.rs`:

1. **Label first.** Segment names and occurrence numbers are collected up
   front, before anything is borrowed mutably, so a change can be labelled
   with the path that names it.
2. **Rules, in order.** Each rule descends segment → field → repetition →
   component → subcomponent, taking the position the path pins down or
   every position where it does not.
3. **Record what was named.** Every leaf a rule reached goes into a
   `HashSet<Position>`, whether or not it changed. That is what makes
   `Keep` exempt a position from the fallback.
4. **Fallback last.** A second walk over every leaf not in that set.

`Position` is `(segment index, field, repetition, component,
subcomponent)`, all 1-based below the segment.

## Where a change goes

| Changing | File | Spec |
| -------- | ---- | ---- |
| what an action does to a value | `src/action.rs` | §3 |
| the policy file format | `src/policy.rs` | §6 |
| the built-in tables | `src/policy.rs` | §5 |
| which positions a rule reaches | `src/redact.rs` | §2, §4 |
| what a report holds | `src/redact.rs` | §8 |
| an option or an output format | `src/main.rs` | §10 |

## The public API surface

`Redactor`, `Policy`, `Rule`, `Action`, `Report`, `Change`, `pseudonym`,
`Error`. Everything is re-exported at the crate root; the modules exist so
that the code has somewhere to live, not so that callers navigate them.

`Policy` and `Rule` have public fields, and that is deliberate: a policy is
data, and a caller assembling one from a configuration table should not
have to go through a builder.
