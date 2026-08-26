[`er7` specification](../index.md) — section 15 of 19. Section numbers
(§15.x) are stable and cited from code, tests, and commit messages.

# 15. Dependencies and build

## 15.1 Zero dependencies [R25]

```toml
[dependencies]
```

**[R25]** The `[dependencies]` table is empty, and stays empty. This is a
feature, not an accident:

- Healthcare integration code is audited, and every transitive dependency
  is another thing to audit and another supply-chain surface.
- The crate is meant to sit at the bottom of a stack of HL7® crates
  ([§1.3](../01-purpose-and-scope/index.md)); a dependency here is a
  dependency for everything above it.
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

A `no_std`+`alloc` mode is recorded in [§16](../16-roadmap/index.md) rather
than speculatively added.

**Serde support is settled, and it is not a feature here.** It ships as the
separate crate [`serde-er7`](https://crates.io/crates/serde-er7), which
depends on this one and adds hand-written `Serialize`/`Deserialize` impls
for every type in the tree. A `serde` feature in this crate would put an
optional dependency in front of every caller who never serialises anything,
and R25 exists to stop exactly that. Task T6 closed on this basis.

## 15.4 Package metadata

| Field | Value |
| ----- | ----- |
| name | `er7` |
| version | `0.1.1` |
| edition | `2024` |
| rust-version | `1.95` (the N-3 MSRV — see [§14.4](../14-compatibility-and-versioning/index.md)) |
| license | `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` |
| repository | <https://github.com/er7-rust/er7-rust> |
| crate | <https://crates.io/crates/er7> |
| docs | <https://docs.rs/er7> |
| website | <https://er7-rust.github.io> |
| keywords | `hl7`, `er7`, `healthcare`, `parser`, `pipe-delimited` |
| categories | `parser-implementations`, `encoding`, `command-line-utilities` |

The multi-license expression matches the rest of this author's crates, and
lets a downstream project pick whichever of the five fits its own licensing.
See [`LICENSE.md`](../../LICENSE.md).

## 15.5 Targets

| Target | Path | Purpose |
| ------ | ---- | ------- |
| library | `src/lib.rs` | the contract |
| binary `er7` | `src/main.rs` | the CLI ([§12](../12-command-line-interface/index.md)) |
| examples | `examples/*.rs` | runnable tutorials ([`examples/README.md`](../../examples/README.md)) |
| integration tests | `tests/integration.rs` | [§13](../13-testing-strategy/index.md) |

The binary depends on the library through its public API only, exactly as a
downstream crate would. That is deliberate: if the CLI needs something the
library does not expose, the library is missing it.

## 15.6 Build commands

Day-to-day commands live in
[`AGENTS/workflows.md`](../../AGENTS/workflows.md); release mechanics in
[`AGENTS/release.md`](../../AGENTS/release.md).

## 15.7 Lints

`Cargo.toml` carries a `[lints.clippy]` table setting the **pedantic**
group to `warn`:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
```

This is build configuration rather than a dependency, and it is recorded
here for the same reason the empty dependency table is: it is part of what
a reviewer sees when they open the manifest.

The four checks run `cargo clippy --all-targets -- -D warnings`
([§13.3](../13-testing-strategy/index.md)), so a pedantic finding fails the
build like any other warning. `priority = -1` lets a specific lint be
re-set without turning the group off.

Where a pedantic lint is wrong for a particular line, the fix is an
`#[allow]` carrying a `reason`, next to the code it excuses — not a hole in
the group.

**Every crate root also carries `#![forbid(unsafe_code)]`** — the library,
the binary, every example, and each fuzz target. `forbid` rather than `deny`, so no
`#[allow(unsafe_code)]` further down can reopen it: `unsafe` here is a
compile error rather than a review comment. The workspace-level reasoning
is [§1.2 of the family policy](../../../spec/01-family-policy/index.md).


---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
