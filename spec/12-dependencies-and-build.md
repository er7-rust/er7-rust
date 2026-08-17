[`er7-redact` specification](index.md) — section 12 of 17. Section numbers (§12.x) are stable and cited from code, tests, and commit messages.

# 12. Dependencies and build

## 12.1 Exactly one runtime dependency [D16]

```toml
[dependencies]
er7 = "0"
```

And that is the whole table. `er7` is the value tree this crate edits and
the path notation its policies are written in; there is nothing else it
needs.

Healthcare integration code gets audited, and every transitive dependency
is another crate somebody has to review. `er7` guarantees zero
dependencies of its own (its R25), so a project that adds `er7-redact`
adds two crates in total and no transitive graph at all.

Specifically not depended on:

| Not used | Why not | Instead |
| -------- | ------- | ------- |
| a serialization crate | policies are a line format ([§6](06-policy-file-format.md)), read in about forty lines | hand-rolled parser |
| a crypto crate | the honest position on pseudonyms is [§7.3](07-pseudonyms.md), not a stronger primitive with the same key handling | FNV-1a, documented as non-cryptographic |
| a CLI argument crate | the CLI has eleven options and no subcommands | hand-rolled loop, as in `er7` |
| a regex crate | policies name positions, not patterns ([§16.2](16-open-questions-and-declined-decisions.md)) | — |

Adding a dependency requires the user to ask for it, and a note in
[§16](16-open-questions-and-declined-decisions.md) recording what it
bought.

`serde_json` and friends are not dev-dependencies either: nothing in the
test suite needs a data format.

## 12.2 Edition, MSRV, and targets

| Field | Value |
| ----- | ----- |
| Edition | 2024 |
| MSRV | 1.85 — the edition-2024 floor, matching `er7` |
| Targets | anything `std` builds for; no platform-specific code |
| `no_std` | not supported; the crate owns `String`s throughout |

## 12.3 Layout

```
src/lib.rs         crate docs, `Error`, re-exports
src/action.rs      §3 — the eight actions, and applying one to a value
src/policy.rs      §5, §6 — `Rule`, `Policy`, the built-ins, the file format
src/pseudonym.rs   §7 — the keyed hash
src/redact.rs      §2, §4, §8 — `Redactor`, the walk, `Report`
src/main.rs        §10 — the `er7-redact` command
```

The binary uses the **published public API only**, so anything it needs, a
downstream crate has too.
