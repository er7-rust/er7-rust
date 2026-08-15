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
| 2 | Pin and check the MSRV | [T2](17-open-tasks.md) | [§14.4](14-compatibility-and-versioning.md) states 1.85 but nothing enforces it, so it can drift silently. |
| 3 | Streaming reader for large batch files | [T4](17-open-tasks.md) | `split_messages` holds the whole input in memory. Batch files in production reach hundreds of megabytes. |

## 16.2 Toward 1.0.0

1.0.0 is reached when three conditions hold:

1. **The API has been exercised by a second crate.** The sibling
   `hl7-2-5-to-xml-using-rust` has its own encoding layer today; porting it
   to build on `er7` would test whether this crate's model is actually the
   right foundation, and would surface anything missing before the API is
   frozen ([T5](17-open-tasks.md)).
2. **R6 is demonstrated, not argued** — §16.1 priority 1.
3. **Every rule in [§1.4](01-purpose-and-scope.md) has a test**, with the
   sole documented exception of R24 ([§13.1](13-testing-strategy.md)).

No breaking changes are planned for 1.0.0 beyond whatever condition 1
surfaces. If it surfaces nothing, 0.2.0 becomes 1.0.0 unchanged.

## 16.3 Explicitly not on the roadmap

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
