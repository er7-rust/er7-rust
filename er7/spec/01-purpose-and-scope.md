[`er7` specification](index.md) — section 1 of 19. Section numbers (§1.x) are stable and cited from code, tests, and commit messages.

# 1. Purpose and scope

## 1.1 Purpose

Read, query, edit, and write HL7 v2 messages in the **ER7** encoding — the
pipe-hat, positional text encoding described in [§2](02-er7-encoding.md)
and defined by chapter 2 of every HL7 v2 release, from 2.1 (1990) through
2.9.

The crate is a **library first** and a command-line tool second. The
library is the contract; the CLI ([§12](12-command-line-interface.md)) is a
thin presentation layer over it and adds no behaviour of its own.

## 1.2 In scope

| Capability | Section |
| ---------- | ------- |
| Reading a message's own delimiter set, including the v2.7 truncation character | [§3](03-delimiters.md) |
| Parsing text into the six-level value tree | [§4](04-parsing.md), [§5](05-value-tree.md) |
| Distinguishing absent, empty, and explicit-null values | [§5.3](05-value-tree.md) |
| Tokenizing, decoding, and encoding escape sequences | [§6](06-escape-sequences.md) |
| Writing the tree back out, byte for byte | [§7](07-writing.md) |
| Reading values by HL7 path (`PID-5.1`, `OBX[2]-5`) | [§8](08-paths-and-queries.md) |
| Splitting batch files and concatenated messages | [§9](09-batch-input.md) |
| Five `MSH` routing accessors | [§10](10-msh-conveniences.md) |

## 1.3 Out of scope

This crate is an **encoding**, not a dictionary. **[R24]** It does not know:

- which fields a given segment has, or what data type each one carries;
- which message structures exist, or how segments group into them;
- what any code-table value means;
- whether a message is valid, complete, or acceptable;
- how a message is framed on the wire (MLLP), or acknowledged.

All of that is version-specific and belongs in a layer above. §10 is the
one narrow, justified exception. The rationale, and the alternatives
considered, are in [§18.1](18-open-questions-and-divergences.md).

### 1.3.1 Where the rest lives — the crate family

This crate is the bottom of a stack, and the layers above it are separate
crates by design (R24, R25). **This subsection is the single source of
truth for the family**; other documents in this repository link here rather
than repeat it.

| Crate | Adds | Depends on |
| ----- | ---- | ---------- |
| [`er7-redact`](https://crates.io/crates/er7-redact) | removing patient detail from a message without changing its shape: a policy of HL7 paths and actions, stable pseudonyms, an audit report | `er7` |
| [`serde-er7`](https://crates.io/crates/serde-er7) | `Serialize`/`Deserialize` for every type in the tree, so a message can flow through any Serde format | `er7`, `serde` |
| [`hl7-2-5-to-xml`](https://crates.io/crates/hl7-2-5-to-xml), [`hl7-2-5-to-json`](https://crates.io/crates/hl7-2-5-to-json) | the HL7 v2.5 dictionary — data types, message structures, and a renderer | `er7` |

The first two are maintained alongside this crate as sibling members of
the same Cargo workspace, in the same repository; the dictionary crates
are separate repositories in the
[`er7-rust`](https://github.com/er7-rust) organisation, and their port
onto `er7` is what [§16.3](16-roadmap.md) records.

Each of these is a crate rather than a feature of this one for the same
reason: a caller who only routes messages should pay for nothing they do
not use, and every optional dependency here would be an audit surface for
everyone ([§15.3](15-dependencies-and-build.md)). The family is presented
together at <https://er7-rust.github.io/ecosystem/>.

## 1.4 Rule index (R1–R25)

Every behavioural rule the crate guarantees, with a stable ID. Prose,
tests, code comments, and commit messages cite these. **IDs are never
reused.** [§13.1](13-testing-strategy.md) maps each rule to the test that
enforces it.

| ID | Rule | Section |
| -- | ---- | ------- |
| R1 | Delimiters are read from the message's own header, never assumed. | [§3.2](03-delimiters.md) |
| R2 | A delimiter set with an alphanumeric, line-ending, or repeated character is rejected. | [§3.3](03-delimiters.md) |
| R3 | Encoding characters the sender omitted fall back to their recommended values. | [§3.2](03-delimiters.md) |
| R4 | Segments divide at `\r`, `\n`, or `\r\n`; blank lines are dropped; nothing else is trimmed. | [§4.1](04-parsing.md) |
| R5 | `parse` requires the first segment to be `MSH`, `FHS`, or `BHS`. | [§4.2](04-parsing.md) |
| R6 | Nothing below the header can fail. | [§4.2](04-parsing.md) |
| R7 | A field the sender left empty has zero repetitions. | [§4.4](04-parsing.md) |
| R8 | Header fields 1 and 2 are the delimiters themselves, stored whole and never decoded. | [§4.4](04-parsing.md) |
| R9 | Leaf text is stored exactly as sent and decoded only on demand. | [§5.2](05-value-tree.md) |
| R10 | Absent, empty, and explicit null are three distinct states. | [§5.3](05-value-tree.md) |
| R11 | `is_empty` and `is_null` are never both true. | [§5.3](05-value-tree.md) |
| R12 | Tokenizing escape sequences is lossless and never fails. | [§6.1](06-escape-sequences.md) |
| R13 | `unescape` resolves only the sequences that stand for characters; the rest stay literal. | [§6.2](06-escape-sequences.md) |
| R14 | `escape` encodes the five structural delimiters, plus `\r` and `\n` as hex. | [§6.3](06-escape-sequences.md) |
| R15 | `unescape(escape(value)) == value` for every value. | [§6.3](06-escape-sequences.md) |
| R16 | Writing reproduces canonical parsed input byte for byte. | [§7.2](07-writing.md) |
| R17 | Structural delimiters remain in `to_text`; only leaf text is decoded. | [§7.1](07-writing.md) |
| R18 | Path indices are 1-based, and `0` is rejected rather than clamped. | [§8.1](08-paths-and-queries.md) |
| R19 | An omitted segment occurrence or repetition index matches every one. | [§8.2](08-paths-and-queries.md) |
| R20 | A position the message does not have yields no value, not an error and not an empty string. | [§8.2](08-paths-and-queries.md) |
| R21 | `split_messages` starts a message at each `MSH` and drops batch envelope segments. | [§9](09-batch-input.md) |
| R22 | The MSH conveniences return `None` when the position is absent or empty. | [§10](10-msh-conveniences.md) |
| R23 | Exactly four error variants exist, arising from exactly two situations. | [§11](11-error-handling.md) |
| R24 | No dictionary, no validation, no transport. | [§1.3](01-purpose-and-scope.md) |
| R25 | Zero runtime dependencies. | [§15](15-dependencies-and-build.md) |

The next rule ID is **R26**.

## 1.5 Design priorities, in order

When two goals conflict, the earlier one wins. This ordering is what makes
the rest of the spec predictable.

1. **Fidelity.** Any message the crate reads, it can write back unchanged
   (R16). A crate that silently alters a clinical message is worse than one
   that refuses to read it.
2. **Distinction.** Absent, empty, and null stay separate (R10, R11).
   Collapsing them corrupts patient records.
3. **Tolerance.** Below the header, nothing fails (R6). A receiver that
   rejects a message it could have read is worse than one that reads it as
   written.
4. **Ergonomics.** Paths, accessors, and the CLI outline exist to make the
   positional structure legible.
5. **Performance.** Zero-copy where it is free (`Cow`, borrowed slices from
   `split_messages`), never at the cost of the four goals above.
