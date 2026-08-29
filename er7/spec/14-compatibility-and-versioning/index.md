[`er7` specification](../index.md) — section 14 of 19. Section numbers
(§14.x) are stable and cited from code, tests, and commit messages.

# 14. Compatibility and versioning

## 14.1 Semantic versioning

The crate follows [semver](https://semver.org). Version `0.1.0` is
pre-1.0, so under Cargo's rules a `0.1.x` bump is compatible and a `0.2.0`
bump is not. The intent is to reach `1.0.0` once the API has been exercised
by a second crate ([§16](../16-roadmap/index.md)).

The **CLI is versioned with the library**
([§12.5](../12-command-line-interface/index.md)): removing an option,
changing an exit code, or changing the outline's label format is a breaking
change.

## 14.2 What counts as breaking

| Change | Breaking? |
| ------ | --------- |
| removing or renaming a public item | yes |
| adding a variant to `Error` | yes — callers match exhaustively ([§11.3](../11-error-handling/index.md)) |
| adding a field to a public struct | yes — all fields are `pub`, so struct literals break |
| changing a method's signature or return type | yes |
| changing what a rule in [§1.4](../01-purpose-and-scope/index.md) guarantees | yes |
| adding a new public item | no |
| adding an option to the CLI | no |
| widening what is accepted without changing what is produced | no, unless a rule changes |
| a documentation, comment, or test change | no |

Public struct fields deserve emphasis: `Message`, `Segment`, `Field`,
`Repetition`, `Component`, `Subcomponent`, `Separators`, `RenderOptions`,
and `Path` all expose their fields, which is deliberate — it lets callers
build messages from literals ([§5.1](../05-value-tree/index.md)) — and it
means the struct shapes are part of the contract.

## 14.3 HL7® version compatibility

The crate targets the **ER7 encoding**, which is stable across HL7 v2.1
through v2.9, rather than any single HL7 version. It therefore reads a
message of any v2 version, including one whose MSH-12 it has never seen.

The two version-sensitive points are both handled without version
knowledge:

| Point | Introduced | Handling |
| ----- | ---------- | -------- |
| the truncation character | v2.7 | read from MSH-2 position 5 when present, `None` when not ([§3.1](../03-delimiters/index.md)) |
| MSH-9.3 message structure | v2.3.1 | reported when present, never derived ([§10.3](../10-msh-conveniences/index.md)) |

Anything that *would* require knowing the version is out of scope by R24.

## 14.4 Rust compatibility

| Item | Value |
| ---- | ----- |
| Edition | 2024 |
| Minimum supported Rust version | 1.96 — current stable minus two releases |
| Target support | any target with `std`; no platform-specific code |
| `no_std` | not supported — the crate uses `String` and `Vec` throughout |

The MSRV comes from the workspace-wide **N-2** policy — this family
supports at least current stable Rust minus two releases — stated in
[`spec/rust-msrv-n-minus-2/index.md`](../../../spec/rust-msrv-n-minus-2/index.md)
and shared with `er7-redact` and `serde-er7`. Edition 2024 sets a hard
floor of 1.85, so the effective MSRV is `max(1.85, N-2)`; that floor
stopped binding once stable reached 1.87. The value is pinned as
`rust-version` in `Cargo.toml`, and CI builds and tests against that exact
toolchain on every push (the `msrv` job in
[`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml), reading
the version from this crate's own manifest rather than a second
hard-coded copy — see
[`spec/rust-msrv-n-minus-2/index.md`](../../../spec/rust-msrv-n-minus-2/index.md)).
N-2 is a floor this crate promises to stay above, not a value chased
release by
release: the pin moves when the code genuinely needs something newer, and
that bump is a breaking change, so it lands in a release allowed to break
rather than in a patch.

## 14.5 Stability of spec identifiers

- **Section numbers** (`§N.x`) are stable. A section is never renumbered;
  a new one is appended.
- **Rule IDs** (`R<n>`) are never reused, even after a rule is withdrawn.
- **Task IDs** (`T<n>`) are never reused, even after a task ships
  ([§17](../17-open-tasks/index.md)).

This is what makes a citation in a commit message from two years ago still
resolve.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
