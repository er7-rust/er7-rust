[index](../index.md) → §9 Roadmap and open questions

# §9 Roadmap and open questions

There is no separate `plan.md` or `tasks.md` — both live here, the same
convention `er7` itself follows (its `AGENTS.md`).

## 9.1 Deliberately deferred

- **A `#[serde(...)]`-attribute-driven mapping from arbitrary user structs
  to ER7 text** (the way `serde_json`/`toml` let *any*
  `#[derive(Serialize)]` struct become JSON/TOML). This is a fundamentally
  different, much larger project: ER7 is positional and segment-based, not
  a general named-field format, so such a mapping would need to invent
  conventions this crate's own scope statement
  ([§1.4](../01-purpose-and-scope/index.md)) rules out — which is exactly
  the "dictionary" layer `er7` itself declines to own. If ever built, it
  belongs in a new, separately-named crate, not bolted onto this one.
- **Borrowing (zero-clone) wrapper types** alongside the current owning
  ones — see [§6.4](../06-ergonomics/index.md). Deferred until a real
  workload shows the clone cost mattering.
- **`serde_yaml`/`bincode` as additional dev-dependencies**, to demonstrate
  more formats in examples. Deferred as a nice-to-have; JSON alone
  demonstrates every wire shape in [§2](../02-wire-shapes/index.md) without
  ambiguity, since JSON has no format-specific behaviour this crate relies
  on that a binary format's demonstration would need to guard against
  separately.

## 9.2 Open questions

- **Should `Terminator` be included at all?** It is a render *option*, not
  message *data* — see `src/terminator.rs`'s own doc comment. It is kept
  for completeness (every public `er7` type gets a wrapper) and because it
  is a useful worked example of the "simple C-like enum" case. If it turns
  out nobody serializes a bare `Terminator` in practice, removing it would
  be a minor scope narrowing, not a design failure.
- ~~**Should a future version offer an opt-in `deny_unknown_fields`
  mode?**~~ **Resolved, 2026-09-02.** Rule S8 stayed unconditional for the
  ordinary `T::deserialize` path — the default is still tolerant, matching
  `er7`'s own R6, exactly as this item asked. What shipped is the opt-in
  itself: `Strict<T>` (S13, [§11](../11-strict-mode/index.md)), for `T` in
  `Message`, `Segment`, `Separators` — the three object-shaped types where
  "unknown field" has meaning. A caller who wants to catch a typo in a
  hand-written JSON fixture (`examples/build_message_from_json.rs`) now
  can, at the one call site that wants it, without changing what any other
  caller's `serde_json::from_str::<Message>(...)` accepts.

## 9.3 Process

A change here follows the same order `er7` itself uses (`er7`'s
`AGENTS/spec-driven-development.md`): update the matching section of this
`spec/` first, then the code, then the tests, then `docs/` and `index.md`
if the change is user-facing.
