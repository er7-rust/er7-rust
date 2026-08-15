[`er7` specification](index.md) — section 15 of 19. Section numbers (§15.x) are stable and cited from code, tests, and commit messages.

# 15. Dependencies and build

## 15.1 Zero dependencies [R25]

```toml
[dependencies]
```

**[R25]** The `[dependencies]` table is empty, and stays empty. This is a
feature, not an accident:

- Healthcare integration code is audited, and every transitive dependency
  is another thing to audit and another supply-chain surface.
- The crate is meant to sit at the bottom of a stack of HL7 crates
  ([§1.3](01-purpose-and-scope.md)); a dependency here is a dependency for
  everything above it.
- Nothing in ER7 needs one. The whole format is delimiters and escape
  sequences over `&str`.

There are no dev-dependencies and no build-dependencies either, so
`cargo test` needs nothing but `std`.

`the_crate_has_no_runtime_dependencies` in `tests/integration.rs` reads
`Cargo.toml` and asserts this, so the rule fails loudly rather than
drifting.

## 15.2 Adding a dependency

Do not, without the user asking. If a case arises, it needs:

1. A written justification in this section, naming what it replaces.
2. Agreement that the added audit surface is worth it.
3. `cargo test` still passing with the dependency vendored or offline.

A dependency behind an off-by-default feature flag is still a dependency
for anyone who turns it on, and needs the same justification.

## 15.3 Features

None. The crate has no `[features]` table and no optional dependencies, so
there is exactly one build configuration and exactly one thing to test.

Plausible future features — `serde` derives on the tree types, a
`no_std`+`alloc` mode — are recorded in [§16](16-roadmap.md) rather than
speculatively added.

## 15.4 Package metadata

| Field | Value |
| ----- | ----- |
| name | `er7` |
| version | `0.1.0` |
| edition | `2024` |
| license | `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` |
| repository | <https://github.com/joelparkerhenderson/er7-rust> |
| crate | <https://crates.io/crates/er7> |
| docs | <https://docs.rs/er7> |
| keywords | `hl7`, `er7`, `healthcare`, `parser`, `pipe-delimited` |
| categories | `parser-implementations`, `encoding`, `command-line-utilities` |

The multi-license expression matches the rest of this author's crates, and
lets a downstream project pick whichever of the five fits its own licensing.
See [`LICENSE.md`](../LICENSE.md).

## 15.5 Targets

| Target | Path | Purpose |
| ------ | ---- | ------- |
| library | `src/lib.rs` | the contract |
| binary `er7` | `src/main.rs` | the CLI ([§12](12-command-line-interface.md)) |
| examples | `examples/*.rs` | runnable tutorials ([`examples/README.md`](../examples/README.md)) |
| integration tests | `tests/integration.rs` | [§13](13-testing-strategy.md) |

The binary depends on the library through its public API only, exactly as a
downstream crate would. That is deliberate: if the CLI needs something the
library does not expose, the library is missing it.

## 15.6 Build commands

Day-to-day commands live in
[`AGENTS/workflows.md`](../AGENTS/workflows.md); release mechanics in
[`AGENTS/release.md`](../AGENTS/release.md).
