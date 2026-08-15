[`er7` specification](index.md) — section 10 of 19. Section numbers (§10.x) are stable and cited from code, tests, and commit messages.

# 10. MSH conveniences

Implemented in `src/message.rs`.

## 10.1 The five accessors [R22]

| Method | Reads | Example value |
| ------ | ----- | ------------- |
| `Message::message_code()` | MSH-9.1 | `ADT` |
| `Message::trigger_event()` | MSH-9.2 | `A08` |
| `Message::message_structure()` | MSH-9.3 | `ADT_A01` |
| `Message::control_id()` | MSH-10 | `MSG00042` |
| `Message::version()` | MSH-12.1 | `2.5` |

Each returns `Option<String>`. **[R22]** The result is `None` when the
position is absent *or* empty, because for these five fields "sent as
blank" and "not sent" mean the same thing to a caller routing a message.

`version` reads MSH-12**.1** rather than MSH-12 whole, because a v2.5.1+
sender may write `2.5.1^AUS^2.5.1` and only the first component is the
version ID.

## 10.2 Why these five, and only these five

This is the crate's single documented exception to R24 ("no dictionary").
The exception is justified on two grounds, and both must hold for any
future addition:

1. **Universality.** Every tool that touches a message needs these to
   route, log, or acknowledge it. Requiring each one to re-derive
   `query("MSH-10")` is friction with no upside.
2. **Stability.** These positions have not moved in any v2 release, from
   2.1 through 2.9. Reading them requires no version knowledge, so no
   version-specific dictionary leaks into the crate.

A field that fails either test does not belong here. MSH-18 (character
set), for instance, is stable but not universal, and acting on it would
require transcoding the crate does not do ([§11 limitations](#103-what-is-deliberately-absent)).

## 10.3 What is deliberately absent

**Deriving a message structure** from MSH-9.1 and MSH-9.2 when MSH-9.3 is
missing — mapping `ADT^A08` to `ADT_A01`, say. Older senders routinely omit
MSH-9.3, so the need is real, but the mapping is **version-specific**: the
set of trigger events sharing a structure differs between v2.3 and v2.5,
and getting it wrong routes a message to the wrong handler.

That mapping belongs in the dictionary layer above. The sibling crate
[`hl7-2-5-to-xml-using-rust`](https://github.com/joelparkerhenderson/hl7-2-5-to-xml-using-rust)
implements it for v2.5 specifically, and its `root_name` function is the
model to follow.

Also absent, for the same reason: sending/receiving application and
facility (MSH-3 through MSH-6), timestamp parsing (MSH-7), and processing
ID (MSH-11). All are readable with a one-line `query`, and none carry the
universality argument that the five above do. Adding them is tracked as a
declined idea in [§18.4](18-open-questions-and-divergences.md).

## 10.4 Implementation note

The five accessors are implemented over `Message::query_path` with a path
literal, and the parse of that literal is expected never to fail — the
literals are checked by the tests in `src/message.rs`. They are therefore
exactly as fast as the equivalent query and share its semantics, including
[§8.2](08-paths-and-queries.md)'s treatment of missing positions.
