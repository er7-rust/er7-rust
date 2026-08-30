[`er7-redact` specification](../index.md) — section 12 of 17. Section
numbers (§12.x) are stable and cited from code, tests, and commit messages.

# 12. Dependencies and build

## 12.1 Exactly one runtime dependency [D16]

```toml
[dependencies]
er7 = { path = "../er7", version = "0" }
```

And that is the whole table. `er7` is the value tree this crate edits and
the path notation its policies are written in; there is nothing else it
needs. The `path` points at the sibling workspace member so local changes
to `er7` are picked up immediately; the `version` requirement is kept
alongside it so the manifest is still valid once this crate is packaged and
published on its own (see
[§13.4](../13-compatibility-and-versioning/index.md)).

Healthcare integration code gets audited, and every transitive dependency
is another crate somebody has to review. `er7` guarantees zero
dependencies of its own (its R25), so a project that adds `er7-redact`
adds two crates in total and no transitive graph at all.

Specifically not depended on:

| Not used | Why not | Instead |
| -------- | ------- | ------- |
| a serialization crate | policies are a line format ([§6](../06-policy-file-format/index.md)), read in about forty lines | hand-rolled parser |
| a crypto crate | the honest position on pseudonyms is [§7.3](../07-pseudonyms/index.md), not a stronger primitive with the same key handling | FNV-1a, documented as non-cryptographic |
| a CLI argument crate | the CLI has eleven options and no subcommands | hand-rolled loop, as in `er7` |
| a regex crate | policies name positions, not patterns ([§16.2](../16-open-questions-and-declined-decisions/index.md)) | — |

Adding a dependency requires the user to ask for it, and a note in
[§16](../16-open-questions-and-declined-decisions/index.md) recording what
it bought.

`serde_json` and friends are not dev-dependencies either: nothing in the
test suite needs a data format.

## 12.2 Edition, MSRV, and targets

| Field | Value |
| ----- | ----- |
| Edition | 2024 |
| MSRV | 1.96 — current stable minus two, matching `er7` |
| Targets | anything `std` builds for; no platform-specific code |
| `no_std` | not supported; the crate owns `String`s throughout |

The MSRV comes from the workspace-wide **N-2** policy — this family
supports at least current stable Rust minus two releases — stated once in
[`spec/rust-msrv-n-minus-2/index.md`](../../../spec/rust-msrv-n-minus-2/index.md)
and pinned as `rust-version` in `Cargo.toml`. Edition 2024's own 1.85 floor
is below that and no longer binds. This crate never holds a floor older
than `er7`'s: it depends on `er7`, so a caller who can build this crate
must be able to build that one too.

## 12.3 Lints

`Cargo.toml` carries a `[lints.clippy]` table setting the **pedantic**
group to `warn`:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
```

The four checks run `cargo clippy --all-targets -- -D warnings`
([§11.3](../11-testing-strategy/index.md)), so a pedantic finding fails the
build. `priority = -1` lets a specific lint be re-set without turning the
group off.

**Every crate root also carries `#![forbid(unsafe_code)]`** — the library,
the binary, and every example. `forbid` rather than `deny`, so no
`#[allow(unsafe_code)]` further down can reopen it: `unsafe` here is a
compile error rather than a review comment. The workspace-level reasoning
is [§1.2 of the family policy](../../../spec/01-family-policy/index.md).


Three of the group's lints earn their place here in particular:

| Lint | Why it matters in this crate |
| ---- | ---------------------------- |
| `must_use_candidate` | a discarded `Report` is a redaction nobody reviewed |
| `missing_errors_doc` | reading a policy is the one place this crate is strict ([§9.3](../09-error-handling/index.md)); a caller needs to know what it refuses |
| `missing_panics_doc` | the built-in policies `expect` on their own literals — the docs say so, and a test proves it cannot happen |

Where a pedantic lint is wrong for a line, the fix is an `#[allow]`
carrying a `reason`, next to that line — not a hole in the group.

## 12.4 Layout

```
src/lib.rs         crate docs, `Error`, re-exports
src/action.rs      §3 — the eight built-in actions, the caller-supplied
                   ninth, and applying one to a value
src/policy.rs      §5, §6 — `Rule`, `Policy`, the built-ins, the file format
src/pseudonym.rs   §7 — the keyed hash
src/redact.rs      §2, §4, §8 — `Redactor`, the walk, `Report`
src/main.rs        §10 — the `er7-redact` command
```

The binary uses the **published public API only**, so anything it needs, a
downstream crate has too.
