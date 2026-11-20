[er7-rust](../../index.md) → [spec](../index.md) → family policy

# §1 Family policy

The policy every crate in this workspace shares. Each crate's own spec
states its own specific rule (a dependency count, a rule-ID prefix, a
section map); this file states the reasoning behind the pattern once.

## §1.1 Dependency minimalism

Every crate in this family treats a dependency as an audit surface, not a
convenience. This is healthcare-adjacent code — `er7` parses HL7® v2
messages, `er7-redact` strips patient identifiers from them, `serde-er7`
carries them through JSON — and each crate is meant to sit low in a stack
that a downstream integrator has to review, not just trust. A dependency is
one more thing that reviewer has to read, and one more supply-chain risk
they inherit.

That produces three different specific rules, not one shared number:

| Crate | Runtime dependencies | Why |
| ----- | --------------------- | --- |
| `er7` | **Zero.** (`er7` R25) | It is the bottom of the stack; nothing above it needs it to depend on anything. |
| `er7-redact` | **Exactly one: `er7`.** (`er7-redact` D16) | `er7` is the value tree being edited; there is no way to redact an HL7 message without a way to represent one. |
| `serde-er7` | **Exactly two: `serde` and `er7`.** (`serde-er7` S1, S2) | `serde` is the trait vocabulary this crate implements against; `er7` is the tree it wraps. No format crate (`serde_json`, `serde_yaml`, …) is a runtime dependency — this crate is format-agnostic by design. |

A pull request that adds a dependency to any of the three needs to justify
it against this table, in that crate's own spec, not just in a commit
message.

## §1.2 The four checks

Every crate in this workspace uses the same edition, the same lint
posture, and the same four commands as its definition of "done":

```sh
cargo test                                # unit, integration, and doc tests
cargo clippy --all-targets -- -D warnings # lints, including examples and tests
cargo fmt --check                         # formatting
cargo rustdoc --lib -- -W missing-docs    # every public item documented
```

Stable CI runs tests and all-target Clippy for both benchmark crates
as well as the three published crates. Missing public library docs fail
the CI rustdoc step.

CI also runs the supplemental workspace coverage gate:

```sh
bin/test-coverage  # 100% merged executable lines in published src/ trees
```

This is not renamed as a fifth member of the four checks: those four are
the per-crate definition of done and need only Rust and Cargo. Coverage is
a workspace evidence gate with an additional pinned tool. It runs all targets,
excludes the two benchmark-only crates, and evaluates merged LCOV source-line
records. The merge matters because LLVM otherwise counts copies of the same
generic and linked function in every test and example binary, including
zero-test example harnesses. A passing result means every executable source
line in `er7/src`, `er7-redact/src`, and `serde-er7/src` ran in at least one
target; no source module is excluded.

Clippy's pedantic lints are on (`[lints.clippy] pedantic = "warn"`) and the
checks run with `-D warnings`, so a pedantic finding fails the build like
any other. Run them per-crate (`-p er7`, `-p er7-redact`, `-p serde-er7`)
or across the whole workspace with `--workspace`.

### No unsafe code, anywhere

**Every crate root in this workspace carries `#![forbid(unsafe_code)]`** —
the three libraries, both binaries, every example, each benchmark crate
and its bench target, and each fuzz target.

`forbid` rather than `deny` is the point. A `deny` can be turned off by an
`#[allow(unsafe_code)]` on the next function; a `forbid` cannot be
overridden anywhere below it, so the guarantee is a property of the build
rather than a convention someone has to keep. Adding `unsafe` to any of
these crates is a compile error, not a review comment.

This costs nothing here. Nothing in an ER7 encoder needs to reach past the
borrow checker: the whole workload is reading `&str`, walking delimiters,
and building `String`s. If a future change ever seems to need `unsafe`, the
right response is to doubt the change — and, if it survives that, to argue
it into this section before removing the attribute from one crate root.

The claim is load-bearing outside the workspace too: it is one of the
checkable properties [`SECURITY.md`](../../SECURITY.md) publishes for a
reviewer, alongside the dependency counts and the absence of build
scripts.

## §1.3 Spec-driven development

All three crates use **spec-driven development**: every behavioural change
starts in that crate's own `spec/` directory, then propagates outward to
tests, code, and docs. The spec is the source of truth, not the
implementation and not this file.

1. **The `spec/` files are canonical.** When a crate's spec and its code
   disagree, the spec is right and the code is a bug — or the spec is
   right and needs updating *before* the code changes.
2. **No silent behaviour changes.** A change to observable behaviour that
   does not touch the matching `spec/` section is incomplete.
3. **Tests express the spec.** A unit test or doc-test is the executable
   form of a spec clause; each crate's testing-strategy section maps every
   rule to the test that enforces it.
4. **Docs follow the spec.** Each crate's `index.md`, `docs/**`, and
   `examples/**` are *derived* — they explain and illustrate the spec,
   they do not define it.

| Artefact | Answers | Example |
| -------- | ------- | ------- |
| `spec/**` | what the crate **does** | "an empty leaf is left empty" |
| `AGENTS/**` | how the code is **written and changed** | "use `checked_sub(1)?` so index 0 yields `None`" |
| `docs/**`, `examples/**`, `index.md` | how a caller **uses** it | "here is how to pull a patient name out of an ADT" |

Rules and tasks have stable IDs, and IDs are never reused, even after a
rule is withdrawn or a task is finished — the commit history is the
archive. Each crate uses its own letter prefix (`R` for `er7`, `D` for
`er7-redact`, `S` for `serde-er7`) precisely so a rule ID unambiguously
names both its crate and its rule; do not read an `R`-numbered guarantee
as applying to `er7-redact`, or a `D`-numbered one as applying to `er7`.

None of the three crates maintains a separate `plan.md` or `tasks.md` —
each crate's own spec holds a roadmap section and an open-tasks section
with stable `T<n>` IDs, and that is the whole planning surface.

For the exact change-loop steps (edit spec → update rule index → write the
failing test → change the code → update the coverage table → update
derived docs → run the four checks → commit), and for which spec section
owns which kind of change, see the `AGENTS/spec-driven-development.md`
file inside the crate you are changing — that mapping is necessarily
crate-specific.

## §1.4 Safety: synthetic data only

No real patient data, ever, in any of the three crates' repositories — not
in tests, not in samples, not in an example, not in a comment, not in a
commit message, not in a conversation pasted into an issue. A redacted
real message is still real patient data; redaction is not a reason to
relax this. Every sample message across the workspace uses obviously
fictional names (`EVERYWOMAN^EVE`, `SMITH^JOHN`, `JONES^WILLIAM`) and
obviously fake identifiers (`444333222`, `PATID1234`, `MSG00042`).

This matters most in `er7-redact`, whose entire purpose is handling
patient identifiers, and each crate's own `AGENTS/safety.md` states what
follows from this rule specifically for that crate — what it is and is not
safe to claim about its own output.

### Multilingual test data

The shared UTF-8 [fixture](../../er7/samples/multilingual.er7) exercises
Arabic and Japanese text, a decomposed Latin accent, and an escaped field
separator. The parser and Serde suites compare canonical ER7 bytes across
their round trips. The redaction suite checks the selected leaf and the
exact text around it. These checks cover text after decoding to UTF-8.
For a feed in another encoding, the integration must decode its bytes
before passing text to the crates. MSH-18 alone does not perform that step.

## §1.5 Workspace layout and versioning

`er7`, `er7-redact`, and `serde-er7` are members of one Cargo workspace,
sharing one root `Cargo.toml`, one workspace `Cargo.lock`, and one
`target/`. `er7-redact` and `serde-er7` depend on `er7` via a **path**
dependency (`er7 = { path = "../er7", version = "0" }`) so that local
changes to `er7` are picked up immediately across the workspace without a
crates.io round-trip — the `version` requirement is kept alongside the
path so the manifest is still valid once each crate is packaged and
published independently. Each crate is versioned, released, and published
to crates.io on its own schedule; the workspace does not imply lock-step
releases.

Each crate still has its own git history, preserved via `git subtree` when
the three were merged into this workspace, and its own release checklist
in `AGENTS/release.md` — publishing one crate does not require publishing
the others.

## §1.6 Release artifacts are built in CI

The four checks exercise the workspace checkout, where `er7-redact` and
`serde-er7` see the local `er7` path dependency and every test file is
available. A crates.io release is a different build surface: Cargo applies
each manifest's `include` list, strips the path part of a published
dependency, resolves its version requirement from the registry, and builds
from the resulting archive.

CI therefore runs the following for every publishable crate:

```sh
cargo package --locked -p er7
cargo package --locked -p er7-redact
cargo package --locked -p serde-er7
```

This is deliberately a package-build invariant, not a substitute for the
four checks. The integration suites are not part of the published archives;
they run against the workspace earlier in CI. The package job proves the
complementary facts: each archive can be assembled from its allow-listed
files, its normalized manifest is valid, and the code that actually ships
builds against the registry dependencies a downstream user will receive.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
