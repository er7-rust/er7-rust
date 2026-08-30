# AGENTS.md

This is the **workspace root** of the `er7-rust` Cargo monorepo: three
crates, each still maintained and released independently, sharing one
workspace `Cargo.toml` and one `Cargo.lock`.

| Crate | What it does | Its own agent entry point |
| ----- | ------------- | -------------------------- |
| [`er7`](er7/) | Parse, query, edit, and write HL7® v2 messages in ER7, with zero dependencies | [`er7/AGENTS.md`](er7/AGENTS.md) |
| [`er7-redact`](er7-redact/) | Redact patient detail from an ER7 message without changing its shape | [`er7-redact/AGENTS.md`](er7-redact/AGENTS.md) |
| [`serde-er7`](serde-er7/) | Serialize and deserialize ER7 message trees with Serde | [`serde-er7/AGENTS.md`](serde-er7/AGENTS.md) |

`er7-redact` and `serde-er7` depend on `er7` via a path dependency
(`{ path = "../er7", version = "0" }`), so a change to `er7` in this
workspace is picked up by its siblings immediately, without publishing.

Three directories here are **not** crates:

- [`er7-rust.github.io/`](er7-rust.github.io/) holds the source of
  <https://er7-rust.github.io/>, the site that documents all three. It was
  a separate repository until its history was merged into this one, so a
  change to a crate's public surface and the page that teaches it can now
  land together — and a page that still teaches a removed API is a broken
  change, not a follow-up. See
  [`er7-rust.github.io/AGENTS.md`](er7-rust.github.io/AGENTS.md).
- [`er7-skill/`](er7-skill/) holds `SKILL.md`, a packaged
  [Claude Code Skill](https://code.claude.com/docs/en/skills)
  that teaches an AI coding agent how to *use* these crates as a
  dependency — which crate to reach for, the rules that are easy to get
  wrong (the byte-for-byte round trip, absent/empty/null), and worked
  recipes. It is a distributable artefact for a downstream project's own
  `.claude/skills/`, not guidance for changing this workspace itself.
- [`er7-rust-maintainer-skill/`](er7-rust-maintainer-skill/) holds the
  companion `SKILL.md` for the *other* direction: an agent packaging of
  this very file and each crate's `AGENTS.md`, for a downstream fork or
  another repository's tooling that wants this workspace's spec-driven
  workflow, the four checks, and the safety rules as a loadable skill
  rather than a set of files to go read. This file and each crate's own
  `AGENTS.md` remain the canonical source; the skill is a pointer-shaped
  index onto them, not a second copy.

## Where to work

**If your change is specific to one crate**, go straight to that crate's
own `AGENTS.md` — it is the canonical entry point for that crate's spec,
conventions, safety rules, testing, and release process, and none of that
detail is repeated here.

**If your change is about the workspace itself** — the root `Cargo.toml`,
how the three crates relate, a policy genuinely shared by all three — see
[`spec/index.md`](spec/index.md), specifically
[§1 Family policy](spec/01-family-policy/index.md): why dependencies are
kept minimal, what the four build checks are, the shared
spec-driven-development discipline, and the synthetic-data safety rule.

## Common commands

```sh
cargo build --workspace                          # build all three
cargo test --workspace                            # test all three
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check                                  # workspace-wide
cargo test -p er7-redact                           # one crate only
```

## Benchmarks and fuzzing

All of it lives outside the crates it measures, so `er7` and `er7-redact`
can each keep an empty dev-dependency table — `er7`'s is enforced by a
test (`the_crate_has_no_runtime_dependencies`, `er7` spec §15.1, R25).

```sh
cargo bench -p er7-bench                             # criterion; er7-bench/benches/
cargo bench -p er7-redact-bench                       # criterion; er7-redact-bench/benches/
cargo bench -p er7-bench -- --save-baseline before   # then compare a change
```

`er7-bench` and `er7-redact-bench` are workspace members with
`publish = false`; each exists only to own the `criterion` dependency for
the crate it measures. Full figures for both are
[`BENCHMARKS.md`](BENCHMARKS.md).

```sh
cd er7 && cargo +nightly fuzz list                # parse_roundtrip, parse_with_total, escape_roundtrip, query_paths
cd er7 && cargo +nightly fuzz run parse_roundtrip -- -max_total_time=60
```

`er7/fuzz` is its own workspace (nightly plus `libfuzzer-sys`), and each
target asserts a property the spec states rather than merely checking for
panics — rendering is a fixed point, tokenizing is lossless, encoding then
decoding is the identity, `query` agrees with the head of `query_all`,
`parse_with` is total on arbitrary bytes the same way `parse` is. A crash
writes its input to `er7/fuzz/artifacts/<target>/`; reproduce it with
`cargo +nightly fuzz run <target> <that file>`. Corpus and artifacts are
gitignored.

See [§1.2](spec/01-family-policy/index.md#12-the-four-checks) for what each
of the four checks verifies and why they're the same across all three
crates.

## Trademarks

`HL7` and `FHIR` are Health Level Seven International's word marks, used
here descriptively under fair use. Three things follow, and a check
enforces them:

```sh
bin/check-trademarks        # or: make check-trademarks
```

1. **The first use of a mark in prose on any page carries `®`** — every
   Markdown file, every website route, every Rust source file's doc
   comments, every crate `description`, and both `--help` strings.
2. **Every page carries the disclaimer**, verbatim. For the website that is
   the shared footer in `+layout.svelte`; for a crate it is the `//!`
   documentation on `lib.rs`; for a Markdown file it is the trailer.
3. **Sample messages, error strings, citation blocks, code identifiers, and
   crates.io keywords are never marked.** A `®` in `MSH|^~\&|…` corrupts
   the sample, and `no HL7 segments` is a diagnostic that gets grepped and
   asserted, not prose that gets read.

If you add a file that mentions a mark, run the check before you finish;
the reasoning, and the definition of a "page", are
[§4](spec/hl7-trademarks-fair-use/index.md).

## Help

Workspace-level operational checklists — things true of the repository as
a whole, not of one crate — live in [`help/`](help/index.md). Currently:
whether the prerequisites for promoting this project are met
([`help/outreach/`](help/outreach/index.md)). Each crate has its own
`help/` for crate-specific checklists.

## What is not here

There is no root-level `src/`, no root-level rule-ID scheme, and no
root-level test suite — this workspace root exists to wire the three
crates together and to say once what would otherwise be said three times.
Behavioural rules, examples, tutorials, and release checklists all live in
the crate they describe.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
