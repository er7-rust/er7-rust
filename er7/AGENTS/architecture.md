[AGENTS.md](../AGENTS.md) → architecture

# Architecture

How the repository is laid out, what each module owns, and where the
boundaries are.

## Crate layout

This crate is one workspace member among three — see the workspace root's
[`AGENTS.md`](../../AGENTS.md) for how `er7`, `er7-redact`, and `serde-er7`
relate. What follows is `er7`'s own layout, inside `er7/`:

```
Cargo.toml            Package metadata; [dependencies] is empty and stays empty (R25)
index.md              User-facing README (README.md links here)
AGENTS.md             Agent entry point
AGENTS/               Topical agent guides (this file is one)
spec/                 Canonical specification, one file per section
docs/                 Long-form documentation: api, usage, escapes, paths, faq
examples/             Runnable programs, one concept each
fuzz/                 Its own Cargo workspace (nightly + libfuzzer-sys); never touches the published crate's build
help/releasing/       Release checklist
samples/              Example ER7 messages, used by docs and tests
src/                  The crate
tests/integration.rs  Black-box tests through the public API and the CLI
```

## Modules

Each module owns one section of the spec. The mapping is deliberate: if you
cannot say which spec section a change belongs to, it probably does not
belong in the crate.

| Module | Owns | Spec |
| ------ | ---- | ---- |
| `src/lib.rs` | crate docs, `Error`, re-exports | [§11](../spec/11-error-handling/index.md) |
| `src/separators.rs` | `Separators`, `Terminator`; reading and validating a delimiter set | [§3](../spec/03-delimiters/index.md) |
| `src/escape.rs` | `Escape`, `escapes`, `unescape`, `escape`, `decode_hex` | [§6](../spec/06-escape-sequences/index.md) |
| `src/message.rs` | the value tree, accessors, absent/empty/null, queries, MSH conveniences | [§5](../spec/05-value-tree/index.md), [§8](../spec/08-paths-and-queries/index.md), [§10](../spec/10-msh-conveniences/index.md) |
| `src/parse.rs` | `parse`, `parse_with`, `split_messages`, `read_messages`, `MessageReader` | [§4](../spec/04-parsing/index.md), [§9](../spec/09-batch-input/index.md) |
| `src/render.rs` | `to_er7`/`to_text` at every level, `RenderOptions` | [§7](../spec/07-writing/index.md) |
| `src/path.rs` | `Path` and its notation | [§8.1](../spec/08-paths-and-queries/index.md) |
| `src/main.rs` | the CLI | [§12](../spec/12-command-line-interface/index.md) |

## Dependency direction

```
main.rs ──► lib (public API only)
lib.rs  ──► message, parse, render, path, escape, separators
message ──► render, escape, path, separators
parse   ──► message, separators
render  ──► message, separators
escape  ──► separators
path    ──► (Error only)
separators ──► (Error only)
```

Two properties to preserve:

1. **`separators` and `path` are leaves.** They depend on nothing but
   `Error`. Anything that would make them depend on the tree belongs
   elsewhere.
2. **`main.rs` uses the public API only** — no `pub(crate)` items, no
   internals. If the CLI needs something the library does not expose, the
   library is missing it; add it to the library and document it, do not
   reach inside.

## The data model

Six types, one per level of the ER7 hierarchy
([§5.1](../spec/05-value-tree/index.md)):

```
Message { separators: Separators, segments: Vec<Segment> }
└─ Segment { name: String, fields: Vec<Field> }
   └─ Field { repetitions: Vec<Repetition> }
      └─ Repetition { components: Vec<Component> }
         └─ Component { subcomponents: Vec<Subcomponent> }
            └─ Subcomponent { raw: String }
```

Three decisions worth knowing before you change any of it:

- **All fields are `pub`.** A message can be built from literals as well as
  parsed. This makes struct shape part of the public contract
  ([§14.2](../spec/14-compatibility-and-versioning/index.md)).
- **Text lives only at the leaf, and only in `raw`.** Every level above
  holds structure. This is what makes the round trip possible (R16); it is
  the single most important decision in the crate.
- **Nothing is memoized.** `to_er7` and `value()` compute each time. There
  is no cache to invalidate when a caller mutates a `pub` field, which is
  what makes public mutation safe.

## The public API surface

Re-exported at the crate root:

| Item | Kind |
| ---- | ---- |
| `Message`, `Segment`, `Field`, `Repetition`, `Component`, `Subcomponent` | the tree |
| `Separators`, `Terminator`, `RenderOptions` | configuration |
| `Path` | path notation |
| `Error` | errors |
| `parse`, `parse_with`, `split_messages`, `read_messages`, `MessageReader` | entry points |

Reachable through their modules: `er7::escape::{escapes, Escape, Escapes,
unescape, escape, decode_hex}` and `er7::message::NULL`.

Note `er7::parse` names both a module and a function. That is legal — they
live in different namespaces — and intentional: `er7::parse(text)` is the
call every user makes first, and `er7::parse::split_messages` still
resolves. The rendered surface is in
[`docs/api/index.md`](../docs/api/index.md).

## Where things deliberately are not

| Not here | Where instead | Why |
| -------- | ------------- | --- |
| segment/field dictionaries | a layer above | [§18.1](../spec/18-open-questions-and-divergences/index.md) |
| message-structure grammars | a layer above | same |
| MLLP framing | a transport crate | [§9.4](../spec/09-batch-input/index.md) |
| a `Builder` for messages | the public `Vec` fields, or `parse_with` for known text | [§5.5](../spec/05-value-tree/index.md) |
