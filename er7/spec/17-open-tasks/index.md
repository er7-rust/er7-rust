[`er7` specification](../index.md) — section 17 of 19. Section numbers
(§17.x) are stable and cited from code, tests, and commit messages.

# 17. Open tasks (backlog)

Each task is a small, specific unit of work. A task moves into the roadmap
([§16](../16-roadmap/index.md)) when it is scheduled; it leaves this
section when it ships, and the change should leave the spec in a state
where the task is no longer needed. Completed tasks are **deleted, not
archived** — the commit history is the changelog.

Tasks are labelled `T<number>` so they can be referenced from commits and
issues. **IDs are never reused** even after a task ships, so future
references stay unambiguous.

The next task ID is **T9**. Tasks are listed **in ID order**, which is not
priority order — [§16](../16-roadmap/index.md) is where priority lives.

---

## T4 — Streaming reader for large batch files

**Scheduled:** [§16.1](../16-roadmap/index.md) priority 3.

`split_messages` takes `&str`, so the whole input must be in memory.
Production batch files reach hundreds of megabytes.

Done when: an iterator yields messages from a `BufRead` without holding the
whole input; `split_messages` remains as the in-memory form; and
[§9](../09-batch-input/index.md) documents both, including which one keeps
the borrowed-slice guarantee.

Open sub-question: whether the streaming form can keep zero-copy slices at
all, or must yield `String`. That answer decides the API shape.

## T7 — Convenience for building a message from scratch

[§5.5](../05-value-tree/index.md) says structural edits go through the
public `Vec` fields, which is honest but verbose: constructing an `ACK` by
hand is a dozen nested literals.

Done when: either a builder or a small set of constructors exists and §5.5
documents it, or §5.5 argues that `Vec` is enough and this task is deleted.
Whichever way it goes, `examples/` should show the recommended way to build
an `ACK`, since that is the case every integration hits.

## T8 — Add a per-segment value lookup

Raised by the [T5](../16-roadmap/index.md) port, which shipped in 0.1.0.

Both `hl7-2-5-to-xml-using-rust` and `hl7-2-5-to-json-using-rust` needed
"the decoded text of SEG-*f*.*c* on **this** segment, empty treated as
absent" — to read OBX-2 while iterating segments — and both wrote the same
eight-line helper:

```rust
segment.component(field, component)?
    .subcomponent(1)?
    .value(separators)
    .into_owned()
```

[`Message::query`](../08-paths-and-queries/index.md) does not fit: it
searches the whole message for the first segment of that name, and here the
segment is already in hand. Two independent callers writing identical glue
is the evidence [§10.2](../10-msh-conveniences/index.md) asks for.

The open question is the signature. `Segment` does not carry its
`Separators`, so decoding cannot happen on `Segment` alone without passing
them — `segment.value(2, 1, &separators)` reads awkwardly for what is
otherwise a positional accessor. The alternatives are a raw
`Segment::value(f, c) -> Option<&str>` that leaves decoding to the caller,
or leaving the glue where it is.

Done when: either the accessor exists with
[§5.4](../05-value-tree/index.md) updated, a rule and test added, and both
sibling crates simplified to use it; or §5.4 records the decision not to
add it and this task is deleted.
