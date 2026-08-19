# AGENTS.md

This is the **workspace root** of the `er7-rust` Cargo monorepo: three
crates, each still maintained and released independently, sharing one
workspace `Cargo.toml` and one `Cargo.lock`.

| Crate | What it does | Its own agent entry point |
| ----- | ------------- | -------------------------- |
| [`er7`](er7/) | Parse, query, edit, and write HL7 v2 messages in ER7, with zero dependencies | [`er7/AGENTS.md`](er7/AGENTS.md) |
| [`er7-redact`](er7-redact/) | Redact patient detail from an ER7 message without changing its shape | [`er7-redact/AGENTS.md`](er7-redact/AGENTS.md) |
| [`serde-er7`](serde-er7/) | Serialize and deserialize ER7 message trees with Serde | [`serde-er7/AGENTS.md`](serde-er7/AGENTS.md) |

`er7-redact` and `serde-er7` depend on `er7` via a path dependency
(`{ path = "../er7", version = "0" }`), so a change to `er7` in this
workspace is picked up by its siblings immediately, without publishing.

## Where to work

**If your change is specific to one crate**, go straight to that crate's
own `AGENTS.md` — it is the canonical entry point for that crate's spec,
conventions, safety rules, testing, and release process, and none of that
detail is repeated here.

**If your change is about the workspace itself** — the root `Cargo.toml`,
how the three crates relate, a policy genuinely shared by all three — see
[`spec/index.md`](spec/index.md), specifically
[§1 Family policy](spec/01-family-policy.md): why dependencies are kept
minimal, what the four build checks are, the shared spec-driven-development
discipline, and the synthetic-data safety rule.

## Common commands

```sh
cargo build --workspace                          # build all three
cargo test --workspace                            # test all three
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check                                  # workspace-wide
cargo test -p er7-redact                           # one crate only
```

See [§1.2](spec/01-family-policy.md#12-the-four-checks) for what each of
the four checks verifies and why they're the same across all three crates.

## What is not here

There is no root-level `src/`, no root-level rule-ID scheme, and no
root-level test suite — this workspace root exists to wire the three
crates together and to say once what would otherwise be said three times.
Behavioural rules, examples, tutorials, and release checklists all live in
the crate they describe.
