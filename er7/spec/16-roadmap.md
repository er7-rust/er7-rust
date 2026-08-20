[`er7` specification](index.md) — section 16 of 19. Section numbers (§16.x) are stable and cited from code, tests, and commit messages.

# 16. Roadmap

Work that is **scheduled**, in priority order. Unscheduled ideas live in
[§17](17-open-tasks.md) as tasks; ideas that were considered and declined
live in [§18](18-open-questions-and-divergences.md).

A roadmap item moves here from §17 when it is taken on, and disappears when
it ships. There is no separate `plan.md`.

## 16.1 Toward 0.2.0

| Priority | Item | Task | Rationale |
| -------- | ---- | ---- | --------- |
| 1 | Prove panic-freedom by fuzzing the parser | [T1](17-open-tasks.md) | R6 claims nothing below the header fails. Today that is argued from the code, not demonstrated. It is the single most load-bearing unproven claim in the spec. |
| 2 | Check the pinned MSRV in CI | [T2](17-open-tasks.md) | `rust-version = "1.95"` is pinned per the workspace N-3 policy, but no job builds on that toolchain, so the declared floor can drift from the real one silently. |
| 3 | Streaming reader for large batch files | [T4](17-open-tasks.md) | `split_messages` holds the whole input in memory. Batch files in production reach hundreds of megabytes. |

## 16.2 Toward 1.0.0

1.0.0 is reached when three conditions hold:

1. ~~**The API has been exercised by a second crate.**~~ **Met.**
   `hl7-2-5-to-xml-using-rust` and `hl7-2-5-to-json-using-rust` both had
   their own copy of an encoding layer; both now depend on `er7` instead
   (task T5, shipped). Their converted output is byte-for-byte identical to
   what it was before the port, and their own test suites pass unchanged.
   See §16.3 below for what the port taught.
2. **R6 is demonstrated, not argued** — §16.1 priority 1.
3. **Every rule in [§1.4](01-purpose-and-scope.md) has a test**, with the
   sole documented exception of R24 ([§13.1](13-testing-strategy.md)).

No breaking changes are planned for 1.0.0. Condition 1 surfaced one
additive candidate ([T8](17-open-tasks.md)) and no removals or renames, so
if conditions 2 and 3 are met the API can be frozen as it stands.

## 16.3 What the T5 port established

Recorded here because it is the evidence behind the 1.0.0 decision.

**Confirmed as designed:**

| Design | How the port exercised it |
|--------|---------------------------|
| The five MSH accessors ([§10](10-msh-conveniences.md)) | Both crates' `root_name` — deriving a message-structure ID from MSH-9 — collapsed to `message_structure()`, `message_code()`, `trigger_event()`. This is exactly the universality argument §10.2 makes. |
| `is_null` at every level ([§5.3](05-value-tree.md)) | Both crates replaced a hand-written comparison against `""` with `Repetition::is_null()`. |
| Decode-on-demand ([§5.2](05-value-tree.md)) | Both now decode with `Subcomponent::value` at the point text becomes XML or JSON, which is where the delimiter set is known. It cost each crate one extra `&Separators` parameter through its node builders — the expected price, and both accepted it. |
| Tolerance below the header (R6) | Neither crate needed a single new fallback; their Z-segment and ragged-field tests passed unchanged. |

**Friction, recorded rather than patched:**

- A per-segment value lookup was written twice, identically —
  [T8](17-open-tasks.md).
- Both crates had to add a `normalize` step to keep their own documented
  trimming, because this crate deliberately trims nothing —
  [§18.5](18-open-questions-and-divergences.md), now with two real callers
  behind it.
- The `Repeat` → `Repetition` rename cost each crate a mechanical
  find-and-replace, as [§18.3](18-open-questions-and-divergences.md)
  predicted.

## 16.4 Explicitly not on the roadmap

These are settled, not pending. Reopening one needs an argument, not a
patch.

| Not planned | Why | Where |
| ----------- | --- | ----- |
| a segment/data-type dictionary | belongs in a layer above | [§1.3](01-purpose-and-scope.md) R24 |
| validation of any kind | same | R24 |
| MLLP or any transport | same | [§9.4](09-batch-input.md) |
| deriving MSH-9.3 from MSH-9.1/9.2 | version-specific | [§10.3](10-msh-conveniences.md) |
| rendering formatted text (`\.br\`, `\H\`) | a presentation concern | [§6.2](06-escape-sequences.md) |
| character-set transcoding for `\Cxxyy\` | needs an encoding library, so it needs a dependency | [§15.1](15-dependencies-and-build.md) R25 |
