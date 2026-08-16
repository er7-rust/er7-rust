[index](index.md) → §1 Purpose and scope

# §1 Purpose and scope

## 1.1 What this crate is

`serde-er7` gives every public type in the [`er7`](https://github.com/er7-rust/er7-rust)
crate — `Message`, `Segment`, `Field`, `Repetition`, `Component`,
`Subcomponent`, `Separators`, `Terminator` — a hand-written `Serialize` and
`Deserialize` implementation, via a same-named wrapper type per level. That
is the entire feature surface.

## 1.2 Why a wrapper crate rather than an added dependency

`er7` has zero dependencies by its own rule (its spec §15, rule R25), and
that is deliberate: it is meant to sit at the bottom of a stack of HL7
crates in a domain where dependencies are audited. Adding `serde` to `er7`
directly would impose that dependency on every consumer, including ones
that never touch Serde. A separate crate lets the choice be the caller's:
depend on `er7` alone, or add `serde-er7` on top when a Serde format is
actually needed.

## 1.3 What problem this solves

Once a message is an `er7::Message`, a caller often wants to hand it to
something that only speaks Serde: a document database driver, a web
framework's JSON response type, a structured logger, a snapshot-testing
library. Without this crate, that means hand-writing a conversion for every
such consumer. With it, `Message` implements `Serialize`/`Deserialize`
directly, and every Serde-compatible sink or source works unmodified.

## 1.4 Non-goals

- **A dictionary.** This crate does not know what any segment, field, or
  code table means — same as `er7`. See the sibling crates
  `hl7-2-5-to-xml-using-rust` and `hl7-2-5-to-json-using-rust` for that
  layer.
- **A format.** This crate never mentions JSON, YAML, or any other format
  in its own runtime dependencies or public API. See [§3](03-dependencies-and-format-agnosticism.md).
- **A validator.** Structurally malformed *Serde* input (a required field
  missing, an array where a string was expected) is rejected with a
  `Deserialize` error; anything that parses is accepted, exactly as `er7`
  accepts anything below its header.
- **A replacement for `er7`'s own API.** Every wrapper `Deref`s to its
  `er7` type ([§6](06-ergonomics.md)), so this crate adds a capability
  rather than a parallel API surface.

## 1.5 Which goal wins when two conflict

See [the table in the index](index.md#which-goal-wins-when-two-conflict).
