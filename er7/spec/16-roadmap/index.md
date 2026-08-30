[`er7` specification](../index.md) — section 16 of 19. Section numbers
(§16.x) are stable and cited from code, tests, and commit messages.

# 16. Roadmap

Work that is **scheduled**, in priority order. Unscheduled ideas live in
[§17](../17-open-tasks/index.md) as tasks; ideas that were considered and
declined live in [§18](../18-open-questions-and-divergences/index.md).

A roadmap item moves here from §17 when it is taken on, and disappears when
it ships. There is no separate `plan.md`.

## 16.1 Toward 0.2.0

Priorities 1 and 2 shipped 2026-08-28 (fuzzing R6 through both `parse` and
`parse_with`, and building/testing on the pinned MSRV toolchain with a
manifest-read version) and are removed from this table along with the
tasks they scheduled — see
[§13.6](../13-testing-strategy/index.md) and
[`spec/rust-msrv-n-minus-2/index.md`](../../../spec/rust-msrv-n-minus-2/index.md)
for the evidence.

| Priority | Item | Task | Rationale |
| -------- | ---- | ---- | --------- |
| 1 | Streaming reader for large batch files | [T4](../17-open-tasks/index.md) | `split_messages` holds the whole input in memory. Batch files in production reach hundreds of megabytes. |

## 16.2 Toward 1.0.0

1.0.0 is reached when three conditions hold:

1. ~~**The API has been exercised by a second crate.**~~ **Met.**
   `hl7-2-from-er7-into-xml` and `hl7-2-from-er7-into-json` both had
   their own copy of an encoding layer; both now depend on `er7` instead
   (task T5, shipped). Their converted output is byte-for-byte identical to
   what it was before the port, and their own test suites pass unchanged.
   See §16.3 below for what the port taught.
2. ~~**R6 is demonstrated, not argued.**~~ **Met, 2026-08-28.**
   `er7/fuzz/parse_with_total` and `er7/fuzz/parse_roundtrip` both fuzz the
   below-the-header parsing logic R6 claims is total, and both ran clean —
   see [§13.6](../13-testing-strategy/index.md) for the run counts and
   durations.
3. ~~**Every rule in [§1.4](../01-purpose-and-scope/index.md) has a
   test**, with the sole documented exception of R24.~~ **Met.** This one
   was true before condition 2 closed, not newly satisfied by it — the
   [§13.1](../13-testing-strategy/index.md) coverage table already listed
   every rule against a test, R24 already carried its documented
   exception, and `every_rule_has_a_coverage_row` already enforced the
   table by machine. Noted here rather than left unmarked once the other
   two were checked off, since an accurate roadmap does not get to skip a
   condition just because closing it took no new work.

**All three conditions are now met, as of 2026-08-28.** No breaking
changes are planned for 1.0.0 — condition 1 surfaced one additive
candidate, `Segment::first_value` (R26, [§5.4](../05-value-tree/index.md)),
shipped 2026-08-29 — and no removals or renames, so the API can be frozen
as it stands. Cutting the release itself is the maintainer's decision, not
a mechanical consequence of this table; this section states readiness,
not intent.

## 16.3 What the T5 port established

Recorded here because it is the evidence behind the 1.0.0 decision.

**Confirmed as designed:**

| Design | How the port exercised it |
|--------|---------------------------|
| The five MSH accessors ([§10](../10-msh-conveniences/index.md)) | Both crates' `root_name` — deriving a message-structure ID from MSH-9 — collapsed to `message_structure()`, `message_code()`, `trigger_event()`. This is exactly the universality argument §10.2 makes. |
| `is_null` at every level ([§5.3](../05-value-tree/index.md)) | Both crates replaced a hand-written comparison against `""` with `Repetition::is_null()`. |
| Decode-on-demand ([§5.2](../05-value-tree/index.md)) | Both now decode with `Subcomponent::value` at the point text becomes XML or JSON, which is where the delimiter set is known. It cost each crate one extra `&Separators` parameter through its node builders — the expected price, and both accepted it. |
| Tolerance below the header (R6) | Neither crate needed a single new fallback; their Z-segment and ragged-field tests passed unchanged. |

**Friction, recorded rather than patched:**

- A per-segment value lookup was written twice, identically — closed
  2026-08-29 as `Segment::first_value` (R26,
  [§5.4](../05-value-tree/index.md)).
- Both crates had to add a `normalize` step to keep their own documented
  trimming, because this crate deliberately trims nothing —
  [§18.5](../18-open-questions-and-divergences/index.md), now with two real
  callers behind it.
- The `Repeat` → `Repetition` rename cost each crate a mechanical
  find-and-replace, as
  [§18.3](../18-open-questions-and-divergences/index.md) predicted.

## 16.4 Explicitly not on the roadmap

These are settled, not pending. Reopening one needs an argument, not a
patch.

| Not planned | Why | Where |
| ----------- | --- | ----- |
| a segment/data-type dictionary | belongs in a layer above | [§1.3](../01-purpose-and-scope/index.md) R24 |
| validation of any kind | same | R24 |
| MLLP or any transport | same | [§9.4](../09-batch-input/index.md) |
| deriving MSH-9.3 from MSH-9.1/9.2 | version-specific | [§10.3](../10-msh-conveniences/index.md) |
| rendering formatted text (`\.br\`, `\H\`) | a presentation concern | [§6.2](../06-escape-sequences/index.md) |
| character-set transcoding for `\Cxxyy\` | needs an encoding library, so it needs a dependency | [§15.1](../15-dependencies-and-build/index.md) R25 |
