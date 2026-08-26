[er7-rust](../../index.md) → [spec](../index.md) → HL7® trademarks and fair use

# §4 HL7® trademarks and fair use

This project's name, its crates, its documentation, and its website all use
word marks owned by Health Level Seven International. This section states
what that permits, what it requires of every file in this repository, and
how the requirement is checked.

The short version: **we use the marks descriptively, under fair use, and we
mark them.** We do not use them as part of a product name, a logo, or a
domain, and we say plainly that nobody has endorsed us.

## §4.1 What HL7 International asks for

HL7 International publishes its fair-use terms at
<https://www.hl7.org/legal/fairuse.cfm>. Anyone may use HL7 word marks —
HL7®, FHIR®, CDA®, and others — in fair-use ways, subject to three
requirements, quoted here because a paraphrase of a licence term is a
liability:

> Always include the trademark registration mark® after the first use of
> word marks each page

> Include the following disclaimer on the webpages, material and other
> locations where such marks are used: "HL7®, and FHIR® are the registered
> trademarks of Health Level Seven International and their use of these
> trademarks does not constitute an endorsement by HL7."

> Please refer to the Fast Healthcare Interoperability Resources as the
> "HL7® FHIR® standard". When referencing the HL7® FHIR® standard in a
> website, document, presentation, or otherwise in a place of prominence,
> refer to it as the "HL7® FHIR® standard". In subsequent uses, please
> refer to it as the "HL7® FHIR® standard" or "HL7® FHIR®", using the ®
> symbol as often as is practical, at least once on each page of printed
> matter, generally in connection with the first or dominant usage.

## §4.2 Rules

| Rule | Statement |
| ---- | --------- |
| **T1** | Every **page** that uses a word mark in prose carries `®` immediately after that mark's **first** use on the page. |
| **T2** | Every page that uses a word mark carries the disclaimer from §4.1, verbatim. |
| **T3** | The first or dominant prose reference to Fast Healthcare Interoperability Resources on a page is written **"HL7® FHIR® standard"**. Later references on the same page may be "HL7® FHIR® standard" or "HL7® FHIR®". |
| **T4** | A word mark is never used as, or as part of, this project's own name, a crate name, a binary name, a logo, a domain, or a GitHub organisation. |
| **T5** | Nothing in this project states or implies endorsement, affiliation, certification, or accreditation by HL7 International. |
| **T6** | The canonical notice is [`TRADEMARKS.md`](../../TRADEMARKS.md). It is the one place the full statement is maintained; every page's disclaimer is the quoted text of §4.1, not a paraphrase. |

## §4.3 What counts as a "page"

The guidance is written for printed matter and webpages. This repository
has neither, exactly, so the term is defined here rather than argued about
later:

| Surface | Is it a page? | Why |
| ------- | ------------- | --- |
| A Markdown file | **Yes**, each file | GitHub renders each one as a page, and each is reachable on its own URL |
| A website route under `er7-rust.github.io/src/routes/` | **Yes**, each route, for T1 | Literally a webpage. T2 is satisfied once, in the shared footer of `+layout.svelte`, which renders on every route |
| A Rust source file's `//!` module documentation and `///` item documentation | **Yes**, per file, for T1 | Each becomes its own docs.rs page. T2 is satisfied once per crate, in the crate root's `//!` documentation, which every module page links back to |
| A crate's `Cargo.toml` `description` | **Yes** | It is displayed on the crate's crates.io page and in `cargo search`. For a publishable crate, T2 requires the disclaimer in the description itself — crates.io has no shared footer to satisfy it elsewhere (enforced since 2026-08-26) |
| A command-line tool's `--help` output | **Yes** | It is the tool's own page, and the first thing most users read |
| A sample message, a policy file, a code block, a shell transcript | **No** | Data and commands, not prose. A `®` inside `MSH|^~\&|…` would corrupt the sample |
| A citation block — the BibTeX entry on the website's about page, `CITATION.cff` | **No** | Text meant to be pasted into someone else's toolchain, where a `®` is a defect rather than a courtesy |
| A runtime error or diagnostic string | **No** | Machine-readable output that gets grepped, matched, and pasted into tickets; see §4.5 |
| An identifier in code: a crate name, a keyword, a path, a variable | **No** | Not prose, and not changeable without breaking things |
| A URL, a link target, a repository or event name owned by someone else | **No** | `chat.fhir.org` and `awesome-FHIR` are other people's names, quoted as they are |

## §4.4 Where the mark goes, concretely

```
HL7® v2 messages in the ER7 pipe-hat encoding      ← first use on the page
… the HL7 path that names it                        ← later uses, unmarked
```

`®` goes immediately after the mark, before any following word:
**"HL7® v2"**, not "HL7 v2®". "v2", "v2.5", and "ER7" are not marks and are
never given a `®`.

**"ER7" is not an HL7® word mark**, which is worth saying because this
project is named after it. It is the common name of the pipe-hat encoding —
sometimes written "ER7" for *Encoding Rules 7* — and it appears in the
standard as a description of the encoding rather than as a mark HL7
International claims. This is the reason the crate is `er7` and not
`hl7-something`, and it is what makes T4 easy to keep.

## §4.5 Why diagnostics are exempt

`er7: error: input contains no HL7 segments` stays exactly that, with no
`®`. Three reasons, and they are the same three that keep the sample
messages clean:

1. **Diagnostics are matched, not read.** They are grepped in logs, asserted
   in tests, and pasted into tickets. A non-ASCII character in one is a
   change to an interface, not to prose.
2. **The volume is wrong.** A tool that prints `HL7®` on every one of a
   thousand malformed messages is not marking a trademark; it is shouting.
   HL7 International asks for the mark on first use *per page*, which a
   stream of diagnostics is not.
3. **The pages that carry those strings are already marked.** `--help`
   carries the mark and the disclaimer, and so does the spec section that
   defines each message. A reader who sees the diagnostic can reach a marked
   page in one step.

The same reasoning covers `format!("(HL7 {v})")` in the message summary
line: it is output, and its shape is part of the CLI contract.

## §4.6 How this is checked

A rule that is only prose is a rule that drifts. `bin/check-trademarks`
enforces T1, T2, and T3 across the repository and exits non-zero on a
violation:

```sh
bin/check-trademarks        # or: make check-trademarks
```

It reads source order, not render order. On a Svelte route that matters:
the `<script>` block precedes the markup in the file but its data renders
after, so the check's "first use" and the reader's first use can be
different strings. The practice is to mark both — the guidance itself says
to use the `®` "as often as is practical", so an extra one is never the
error.

It reads every Markdown file, every website route, every Rust source file,
every `Cargo.toml` description (where, for a publishable crate, it also
requires the §4.1 disclaimer verbatim — see §4.3), and the two `--help`
strings; it ignores
fenced code blocks, inline code spans, link targets, citation blocks, and
the surfaces §4.3 excludes. It knows that `&reg;` is `®` in a Svelte
template, and that the `FHIR®` inside the disclaimer is not a reference to
the standard for the purposes of T3. Run it with the four checks before any
release.

T4 and T5 are not machine-checkable, and are not pretended to be. They are
review obligations on the maintainer, and [`NEWS.md`](../../NEWS.md) states
the non-endorsement position for anyone writing about the project.

## §4.7 What this section does not do

It does not make this project's use of the marks lawful by declaring it so,
and it is not legal advice. It records what HL7 International asks of a
fair-use user and what this repository does about it. If HL7 International
tells us this is wrong, they are right and this section changes.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
