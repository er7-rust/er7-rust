# er7-rust

A Cargo workspace holding three Rust crates for working with HL7 v2
messages in the **ER7** pipe-hat encoding — each independently versioned
and published, sharing this repository and one workspace `Cargo.toml`.

| Crate | What it does |
| ----- | ------------- |
| [`er7`](er7/) | Parse, query, edit, and write ER7 messages, with zero dependencies |
| [`er7-redact`](er7-redact/) | Redact patient detail from an ER7 message without changing its shape |
| [`serde-er7`](serde-er7/) | Serialize and deserialize ER7 message trees with Serde |

`er7-redact` and `serde-er7` depend on `er7` via a path dependency, so a
change to `er7` in this workspace is picked up by its siblings immediately,
without publishing.

Each crate has its own README, specification, examples, and tests — start
in that crate's own directory for anything crate-specific. This root only
holds what the three genuinely share: see
[`spec/01-family-policy.md`](spec/01-family-policy.md) for the shared
dependency, testing, and safety policy, and [`AGENTS.md`](AGENTS.md) for
agent guidance on the workspace as a whole.

The whole family, and the boundary between the layers, is documented at
<https://er7-rust.github.io/ecosystem/>.

## Building

```sh
cargo build --workspace
cargo test --workspace
```

## License

Each crate carries its own `LICENSE.md`; see that crate's own README for
its exact license expression.
