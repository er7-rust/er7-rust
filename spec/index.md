[er7-rust](../index.md) → spec

# Workspace-level specification

This is the **workspace root** of a Cargo monorepo holding three crates —
[`er7`](../er7/), [`er7-redact`](../er7-redact/), and
[`serde-er7`](../serde-er7/) — each maintained separately in its own git
history before being merged in here, each still following its own
spec-driven development discipline in its own `spec/` directory.

This root `spec/` is **not** a replacement for those. It holds only the
policy that is genuinely shared across all three crates, stated once so it
does not drift across three restatements. Every behavioural rule, every
rule ID (`R<n>` for `er7`, `D<n>` for `er7-redact`, `S<n>` for
`serde-er7`), and every crate-specific guarantee still lives in that
crate's own `spec/index.md` — start there for anything about what a
specific crate does.

## Contents

| Section | Covers |
| ------- | ------ |
| [§1 Family policy](01-family-policy/index.md) | Dependency minimalism, the four build checks, the spec-driven-development discipline itself, the synthetic-data safety rule, and how workspace path dependencies relate to published version requirements |
| [§2 Rust MSRV: N-3](rust-msrv-n-minus-3/index.md) | The shared minimum supported Rust version — current stable minus three releases — why the window is that wide, and what an MSRV bump implies for a release |
| [§3 Promotion](promote/index.md) | How the family reaches HL7® and Rust professionals: the channels, what each one's rules are, a ninety-day sequence, message templates, and the etiquette that keeps the project welcome in professional communities |
| [§4 HL7® trademarks and fair use](hl7-trademarks-fair-use/index.md) | What HL7 International's fair-use terms require, what counts as a "page" in a repository, where the ® goes and where it deliberately does not, and the check that enforces it |
| [§5 Professionalization](professionalization/index.md) | What "professional" means here and the nine rules that bind the maintainer to it — verified plans, accurate special files, self-declared gaps as promises, CI enforcing what documents claim, trademark discipline, plain-language patient-data answers, conduct, family harmonization, gated outreach — with a dated status table |
| [§6 Free open source funding](free-open-source-funding/index.md) | The checklist behind `.github/FUNDING.yml` and `CONTRIBUTING.md`'s "Money" section — what got set up, and what was checked and found genuinely unavailable rather than skipped |

## What belongs here vs. in a crate's own spec

| Here (workspace-level) | Crate's own `spec/` |
| ----------------------- | -------------------- |
| "Why this family keeps dependencies minimal" | "Why `er7-redact` has exactly one dependency, and what it is" |
| "What the four checks are and why they run" | Rule IDs for behaviour those checks enforce |
| "What spec-driven development means, generically" | This crate's own section map and rule index |
| "Never commit real patient data" | Crate-specific safety consequences (e.g. what a redaction crate must never do with a report) |
| "The MSRV is current stable minus three" | The `rust-version` value that crate's `Cargo.toml` actually declares |

If you are about to write something that is true of one crate but not
necessarily the other two, it belongs in that crate's own `spec/`, not
here.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
