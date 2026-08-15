[`er7` specification](index.md) — section 7 of 19. Section numbers (§7.x) are stable and cited from code, tests, and commit messages.

# 7. Writing and round trip

Implemented in `src/render.rs`.

## 7.1 The two forms [R17]

Every level of the tree from `Segment` down to `Subcomponent` offers the
same pair:

| Method | Produces |
| ------ | -------- |
| `to_er7(&separators)` | exactly what a receiver would read, escape sequences intact |
| `to_text(&separators)` | the same, with leaf text escape-decoded per [§6.2](06-escape-sequences.md) |

**[R17]** Structural delimiters remain in **both** forms. `SMITH^JOHN`
without its caret is no longer a name and a surname; a "decoded" form that
dropped the caret would be lossy in a way `to_text` callers would not
expect. What `to_text` decodes is the leaf text and nothing else.

Fields 1 and 2 of a header are written literally in both forms
([§4.4.2](04-parsing.md)), since they are the delimiters rather than values
encoded with them.

A consequence worth stating: `to_text` output is **not** re-parseable in
general. A decoded `\F\` becomes a literal `|`, which a parser would read
as a field separator. Use `to_er7` for anything that will be sent, stored,
or parsed again; use `to_text` for display, logging, and database writes.

## 7.2 Whole messages, and the round trip [R16]

```rust
impl Message {
    pub fn to_er7(&self) -> String;
    pub fn to_er7_with(&self, options: RenderOptions) -> String;
}
```

`RenderOptions` has two fields:

| Field | Default | Meaning |
| ----- | ------- | ------- |
| `terminator` | `Terminator::Cr` | what ends each segment ([§3.5](03-delimiters.md)) |
| `trailing_terminator` | `false` | whether the last segment gets one too |

`trailing_terminator` defaults to `false` even though HL7 terminates every
segment including the last, because a trailing terminator surprises callers
that compare or concatenate the result, and the transport — MLLP, or a file
— already marks where the message ends. Set it to `true` for strict wire
output; the CLI's `--normalize` does ([§12](12-command-line-interface.md)).

`Display for Message` is `to_er7()`.

### The guarantee [R16]

For any message this crate parsed, writing it back reproduces the input
**byte for byte**, except where the input was not already canonical.

Text is **canonical** when it has no blank lines and every segment is
terminated by the character `RenderOptions` chose. Parsing normalizes
exactly those two things ([§4.1](04-parsing.md)) and nothing else.
Canonical text round-trips exactly:

```
parse(canonical).to_er7_with(options) == canonical
```

This holds for messages with unusual delimiters, unknown and local
segments, empty positions at every level, explicit nulls, the truncation
character, and escape sequences the crate does not decode. It is the
crate's first design priority ([§1.5](01-purpose-and-scope.md)) and the
property most likely to be broken by a well-meaning change — see
[`AGENTS/conventions.md`](../AGENTS/conventions.md).

## 7.3 How a segment is written

```
name  field1  sep  field2  sep  field3 …
```

with one exception. For a header segment the field separator **is** field 1
and the encoding characters **are** field 2, so both are written literally
with no separator inserted before them:

| Segment | Written as |
| ------- | ---------- |
| `PID` with fields `["1", "", "X"]` | `PID\|1\|\|X` |
| `MSH` with fields `["\|", "^~\\&", "LAB"]` | `MSH\|^~\&\|LAB` |

Within a field, repetitions join with the repetition separator; within a
repetition, components join with the component separator; within a
component, subcomponents join with the subcomponent separator. Empty
children are written as empty strings, which is what preserves their
positions.

## 7.4 Complexity

Writing is a single pass, O(n) in the output length. `to_er7` on a
`Message` allocates one `String`; the per-level `to_er7`/`to_text` methods
allocate one each, so building a large document by concatenating them is
better done by walking the tree once.
