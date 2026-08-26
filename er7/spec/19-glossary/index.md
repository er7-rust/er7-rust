[`er7` specification](../index.md) — section 19 of 19. Section numbers
(§19.x) are stable and cited from code, tests, and commit messages.

# 19. Glossary

Terms as this crate uses them. Where a term is HL7®'s, the HL7 sense is the
one meant.

| Term | Meaning |
| ---- | ------- |
| **absent** | a position the message never sent; the accessor returns `None`. Distinct from *empty* and *null* ([§5.3](../05-value-tree/index.md)) |
| **ACK** | acknowledgement message; a receiver's reply quoting the original's control ID |
| **batch** | a file holding several messages inside an `FHS`/`BHS`/`BTS`/`FTS` envelope ([§9](../09-batch-input/index.md)) |
| **canonical** | ER7 text with no blank lines and one chosen terminator per segment; what writing produces and what round-trips exactly ([§7.2](../07-writing/index.md)) |
| **component** | the fifth level; part of a repetition, separated by `^` |
| **control ID** | MSH-10, the sender's unique identifier for a message |
| **delimiter** | one of the six characters in a `Separators` ([§3.1](../03-delimiters/index.md)) |
| **encoding characters** | MSH-2: the component, repetition, escape, subcomponent, and optional truncation characters |
| **empty** | a position sent with no value, `\|\|`; distinct from *absent* and *null* |
| **ER7** | "Encoding Rules 7", the pipe-hat text encoding for HL7 v2 ([§2](../02-er7-encoding/index.md)) |
| **escape sequence** | the escape character, a body, and the escape character again: `\F\`, `\X0D\` ([§6](../06-escape-sequences/index.md)) |
| **explicit null** | the two-character value `""`, meaning "clear this value"; distinct from *absent* and *empty* |
| **field** | the third level; part of a segment, separated by `\|` |
| **FT** | formatted text; the HL7 data type whose values carry display commands such as `\.br\` |
| **header** | a segment that declares the delimiter set: `MSH`, `FHS`, or `BHS` ([§4.4.2](../04-parsing/index.md)) |
| **HL7 v2** | Health Level Seven version 2, the messaging standard ER7 encodes |
| **leaf** | a subcomponent, the only place text is stored ([§5.2](../05-value-tree/index.md)) |
| **MLLP** | Minimal Lower Layer Protocol, the usual transport framing; out of scope ([§2.7](../02-er7-encoding/index.md)) |
| **MSH** | message header, the first segment of every message |
| **occurrence** | which segment of a repeated name, 1-based; written `OBX[2]` in a path ([§8.1](../08-paths-and-queries/index.md)) |
| **path** | the notation naming one place in a message: `PID-5.1` ([§8.1](../08-paths-and-queries/index.md)) |
| **pipe-hat** | the informal name for ER7, after `\|` and `^` |
| **raw** | text exactly as sent, escape sequences intact; the opposite of *decoded* |
| **repetition** | the fourth level; one occurrence of a repeating field, separated by `~`. Called `Repeat` in the sibling crate ([§18.3](../18-open-questions-and-divergences/index.md)) |
| **round trip** | parsing text and writing it back unchanged (R16, [§7.2](../07-writing/index.md)) |
| **rule** | a numbered behavioural guarantee, `R<n>`, indexed in [§1.4](../01-purpose-and-scope/index.md) |
| **segment** | the second level; one line of a message, named by three characters |
| **segment terminator** | what ends a segment: `\r` on the wire, also `\n` or `\r\n` when read ([§3.5](../03-delimiters/index.md)) |
| **structure** | MSH-9.3, e.g. `ADT_A01`; which segments a message may hold. Reported, never derived ([§10.3](../10-msh-conveniences/index.md)) |
| **subcomponent** | the sixth and deepest level; part of a component, separated by `&` |
| **task** | a numbered unit of pending work, `T<n>` ([§17](../17-open-tasks/index.md)) |
| **trigger event** | MSH-9.2, e.g. `A08`; what happened that caused the message |
| **truncation character** | MSH-2 position 5, marking a value the sender cut short; HL7 v2.7 and later ([§3.1](../03-delimiters/index.md)) |
| **Z segment** | a locally defined segment, named `Z…`; carried through like any other ([§11.2](../11-error-handling/index.md)) |

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
