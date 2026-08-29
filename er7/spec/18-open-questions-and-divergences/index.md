[`er7` specification](../index.md) — section 18 of 19. Section numbers
(§18.x) are stable and cited from code, tests, and commit messages.

# 18. Open questions and known divergences

Where the crate knowingly differs from the standard, from a sibling crate,
or from what a reader might reasonably expect — recorded so the difference
is visible rather than rediscovered.

A divergence recorded here is a **decision**, not a bug. A divergence
*not* recorded here is a bug. If you find one, write it down before
deciding what to do about it.

## 18.1 The crate stops at the encoding

**Decision:** no dictionary, no validation, no transport (R24).

**Why this is a real question:** every user of this crate eventually needs a
dictionary. Splitting the encoding from the dictionary means two crates
where a user might have wanted one.

**Why it was decided this way:** the dictionary is version-specific and
large — segment tables, data types, message structures, code tables, for
each of v2.1 through v2.9 — while the encoding is small and stable across
all of them. Fusing them would mean either shipping one version's tables to
everyone or shipping all of them to everyone. Keeping the encoding separate
lets a dictionary crate choose its own version, and lets a user who only
needs to route or audit messages pay for nothing.

**Settled by the T5 port.** `hl7-2-5-to-xml-using-rust` and
`hl7-2-5-to-json-using-rust` now both depend on `er7` and keep only their
v2.5 dictionary — data-type tables, message-structure grammars, renderer.
Each dropped an identical 350-line copy of the encoding layer, and their
converted output did not change by a byte. The boundary is in the right
place; see [§16.3](../16-roadmap/index.md) for the detail.

## 18.2 Escape sequences are decoded in every field

**Divergence from the standard.** HL7® scopes escaping to `ST`, `TX`, and
`FT` fields and to the fourth component of `ED`
([§2.5.1](../02-er7-encoding/index.md)). This crate decodes sequences
wherever they appear.

**Why:** applying the standard's scope requires knowing each field's data
type, which requires the dictionary this crate does not have (§18.1).

**The risk:** a false positive — a value that legitimately contains the
escape character, in a field where escaping does not apply, read as a
sequence. In practice this is rare, because a bare backslash in a field
where escaping does not apply is itself unusual.

**The mitigations:** R13 keeps unrecognized sequences literal, so a false
positive usually round-trips unchanged anyway; and `Subcomponent::raw`
always holds exactly what arrived, so a caller who knows the data type can
always override.

**Not planned to change.** A dictionary-layer crate that knows the types
can call `unescape` selectively.

## 18.3 `Repetition`, where the sibling crate says `Repeat`

**Divergence from `hl7-2-5-to-xml-using-rust`,** whose equivalent type is
`Repeat`.

**Why:** "repetition separator" is the standard's own term
([§2.2](../02-er7-encoding/index.md)), and this crate is meant to define
the vocabulary its callers use.

**Cost, now measured:** the T5 port renamed `Repeat` to `Repetition` and
`repeats` to `repetitions` in both sibling crates. It was a mechanical
find-and-replace in one file each, caught entirely by the compiler. The
cost was as bounded as predicted.

## 18.4 MSH accessors that were declined

[§10.3](../10-msh-conveniences/index.md) lists what is deliberately absent:
deriving MSH-9.3, and accessors for MSH-3 through MSH-7 and MSH-11.

**Why they were declined:** each fails at least one of the two tests in
[§10.2](../10-msh-conveniences/index.md) — universality and
version-stability. Deriving MSH-9.3 fails stability outright. The rest fail
universality: they are one `query` call away and not every tool needs them,
so adding them would grow the exception to R24 without earning it.

**What would reopen it:** evidence that a caller cannot express one of them
with `query`. None is known.

## 18.5 Trimming: fidelity versus tidiness

**Divergence from `hl7-2-5-to-xml-using-rust`,** which trims whitespace
from every segment line. This crate trims nothing but blank lines
([§4.1](../04-parsing/index.md) R4).

**Why:** that crate converts to XML, where a stray leading space is noise;
this crate guarantees a byte-for-byte round trip (R16), where a stray
trailing space might be data. The crate cannot tell the difference, so it
keeps what it was given.

**Consequence to be aware of:** a message that has been pretty-printed with
indentation will parse with that indentation inside the segment name. The
name of the first segment would then not be `MSH`, and `parse` would
return `MissingHeader` — a surprising error for what looks like a readable
message. Handling that would mean guessing which whitespace is data, so it
is left to the caller to trim before parsing.

**Confirmed by the T5 port, and left as is.** Both sibling crates hit
exactly this and both added the same six-line `normalize` — split, trim,
drop blanks, rejoin with `\r` — before calling `parse`. That is the
intended division: they convert to XML and JSON, where leading whitespace
is noise, so they can afford to trim; this crate guarantees a byte-for-byte
round trip (R16), so it cannot. Two callers writing the same six lines is
acceptable where two callers writing the same eight-line *value lookup*
was not, because the trimming encodes a policy the callers own and the
lookup encoded one this crate owns — which is why the lookup, and not the
trimming, became `Segment::first_value` (R26,
[§5.4](../05-value-tree/index.md)).

## 18.6 Open question: should `to_text` exist at every level?

**Unresolved.** `to_text` decodes leaves but keeps structural delimiters
(R17), so its output is not re-parseable ([§7.1](../07-writing/index.md)).
That is documented, but it is still a shape that invites misuse: a caller
who reaches for `to_text` on a `Field` and writes the result into a new
message has silently corrupted it.

**Options considered:** removing `to_text` above `Subcomponent`; renaming
it to something that does not sound like a safe default; keeping it and
relying on the documentation.

**Current state:** kept, because the CLI outline and any human-facing
display genuinely want a decoded field. Revisit if a user reports the
misuse. No task is open, because there is no evidence of the problem yet.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
