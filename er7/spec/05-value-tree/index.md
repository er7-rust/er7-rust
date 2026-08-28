[`er7` specification](../index.md) — section 5 of 19. Section numbers
(§5.x) are stable and cited from code, tests, and commit messages.

# 5. The value tree

Implemented in `src/message.rs`.

## 5.1 Shape

Six types, one per level of [§2.2](../02-er7-encoding/index.md):

```
Message      separators + segments
└─ Segment   name + fields
   └─ Field  repetitions
      └─ Repetition   components
         └─ Component subcomponents
            └─ Subcomponent  raw: String
```

Every field of every type is `pub`, so a message can be built from literals
as well as parsed. `Message`, and every type below it, derives `Debug`,
`Clone`, `PartialEq`, and `Eq`; the four types below `Segment` also derive
`Default`, so an empty node can be created without ceremony.

The type is named `Repetition`, not `Repeat`, because "repetition
separator" is the standard's own term and this crate defines the vocabulary
its callers use. (The sibling crate `hl7-2-5-to-xml-using-rust` calls it
`Repeat`; see [§18.3](../18-open-questions-and-divergences/index.md).)

## 5.2 Text lives only at the leaf [R9]

Only `Subcomponent` holds text, in its `pub raw: String` field, exactly as
sent. Decoding happens on demand ([§6.2](../06-escape-sequences/index.md)).
This single decision is what makes the round-trip guarantee of
[§7.2](../07-writing/index.md) possible, and it keeps separate the two
questions a receiver asks:

| Question | Answer |
| -------- | ------ |
| What did the sender write? | `subcomponent.raw` |
| What does it mean? | `subcomponent.value(&separators)` |

## 5.3 Absent, empty, and null [R10] [R11]

ER7 distinguishes three states ([§2.4](../02-er7-encoding/index.md)), and
so does this crate.

| State | On the wire | How to ask | Receiver should |
|-------|-------------|-----------|-----------------|
| absent | the field was never sent | the accessor returns `None` | leave the stored value alone |
| empty | `\|\|` | `is_empty()` | leave the stored value alone |
| null | `\|""\|` | `is_null()` | clear the stored value |

**[R11]** `is_empty` and `is_null` are never both true: the explicit null is
text, two `"` characters, so a null node is not empty.

`is_empty` and `is_null` are defined at every level from `Subcomponent` up
to `Field`:

- `Subcomponent::is_empty` — `raw` is the empty string.
- `Subcomponent::is_null` — `raw` is exactly `""` (the constant
  `er7::message::NULL`).
- The three levels above are `is_empty` when every child is, and `is_null`
  when they hold exactly one child and that child is null. A field is
  therefore null only for the precise shape `|""|`, never for
  `|""^X|`.

`Subcomponent::value` reports the explicit null as the empty string, since
that is the value being conveyed. Ask `is_null` when the difference
matters — for a database write, it always does.

## 5.4 Accessors

Every level exposes 1-based accessors matching HL7®'s own numbering, each
returning `Option`, plus a `_mut` variant:

| Level | Accessor | Shortcut |
| ----- | -------- | -------- |
| `Message` | `segment(name)`, `segment_at(name, occurrence)`, `segments_named(name)`, `header()` | |
| `Segment` | `field(n)` | `component(field, component)` |
| `Field` | `repetition(n)` | `component(n)` — first repetition |
| `Repetition` | `component(n)` | |
| `Component` | `subcomponent(n)` | |

`Segment::is_header()` reports whether this segment declares delimiters
(`MSH`, `FHS`, `BHS`), which is what §4.4.2 and §7.1 key on.

Index `0` returns `None` rather than panicking, via `checked_sub`: HL7
numbering starts at 1, so a `0` is a caller's off-by-one and should not be
silently read as element 1.

## 5.5 Editing

Two ways to write a value, with different responsibilities:

| Way | Responsibility |
| --- | -------------- |
| assign `subcomponent.raw` directly | the caller must ensure the text holds no unescaped delimiters |
| `subcomponent.set(value, &separators)` | the crate encodes delimiters for you ([§6.3](../06-escape-sequences/index.md)) |

`set` is the recommended path. A value such as `O'BRIEN & SONS` written
with `set` becomes `O'BRIEN \T\ SONS` and cannot break the structure;
written by assignment, the `&` would split the component in two the next
time the message was parsed.

**`set` takes text, not ER7.** Everything given to it is data, so every
delimiter in it is encoded — `~` included. Handing it a whole field's ER7
therefore collapses that field: `A~B~C` becomes one value holding two
`\R\` sequences, which is `set` doing exactly what it promises and the
wrong tool for the job. Copying a value that is more than one leaf — a
repeating field, a composite — is a structural edit, below.

Structural edits — adding a repetition, extending a segment with more
fields, moving a field from one message to another — are done by
manipulating the public `Vec` fields directly. Every level is a public
`Vec` and every node is `Clone`, so a repeating field moves as itself:

```rust,ignore
let ids = source.segment("PID").unwrap().field(3).unwrap().clone();
let pid = target.segment_at_mut("PID", 1).unwrap();
if pid.fields.len() < 3 {
    pid.fields.resize(3, Field::default());   // 1-based position 3
}
pid.fields[2] = ids;                          // repetitions stay repetitions
```

The crate does not offer `push_field`- or `set_field`-style helpers; `Vec`
already has them, and wrapping them would only add surface without adding
meaning.

**Building a message that never existed as text is not a different
problem.** There is no `Message::builder()`, and in practice a message
built from nothing is rare: an ACK, the case that comes up most, is mostly
values the inbound message already carries — sending and receiving
application swap places, the control ID being acknowledged is copied — so
by the time it is written it is known text with a few values spliced in.
`er7::parse_with` turns that text into segments, which makes parsing the
builder for anything expressible as ER7; a field assembled from pieces
that are not text yet, rather than a string, still goes through the same
`Vec` fields above. `examples/build_a_message.rs` works a full ACK both
ways side by side. Two tools cover every case seen so far; a third would
only be a worse way to spell one of them.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
