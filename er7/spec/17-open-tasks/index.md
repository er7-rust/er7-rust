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

