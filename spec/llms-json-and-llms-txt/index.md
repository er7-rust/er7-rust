[er7-rust](../../index.md) → [spec](../index.md) → llms.json and llms.txt

# §10 `llms.json` and `llms.txt`

The site publishes two machine-readable maps of its own content, at its
own root — `https://er7-rust.github.io/llms.txt` and `.../llms.json` —
so a language model reading the site can find its most important pages
without crawling the whole thing.

- **`llms.txt`** follows the [llms.txt](https://llmstxt.org/) convention:
  an H1 with the project name, a one-line blockquote summary, short
  context prose, then `##`-delimited sections that are each a flat list
  of `- [title](url): description` links. No dependency, no build step —
  it is a hand-maintained Markdown file.
- **`llms.json`** is a structured twin of the same content, for a
  consumer that wants to parse rather than read Markdown. There is no
  equivalent published standard for a JSON form, so the shape here is
  this project's own: `name`, `summary`, `context` (the same prose
  paragraphs), `trademark_notice`, and `sections` — each an array of
  `{ title, links: [{ title, url, description }] }` — mirroring
  `llms.txt`'s own section structure exactly, so the two stay easy to
  keep in agreement by inspection.

## Where the two files live

`er7-rust.github.io/static/llms.txt` and `.../static/llms.json`, not the
workspace root. `static/` is the one directory SvelteKit's
`adapter-static` copies to the built site verbatim
(`svelte.config.js`), which is exactly where `robots.txt` and
`sitemap.xml` already live and exactly what makes a well-known path like
`/llms.txt` actually fetchable at the published URL — the whole point of
the convention. Putting a copy at the workspace root instead would be a
second file for the same content to drift out of, for no reader that can
reach it: this repository's own `AGENTS.md`/`spec/` documents already
serve an agent working *in* the repository, and `llms.txt` is for a
reader of the *published site*.

## What is curated, and in what order

The same four groups in both files:

| Section | What | Source |
| ------- | ---- | ------ |
| Docs | The 13 pages in the site's own main navigation, in nav order | `src/lib/site.ts`'s `navLinks` |
| Project | The footer-only pages: agent skills, security, governance, maintainers, RFC, AI statement, trademarks, news | `+layout.svelte`'s footer link list |
| Source | The GitHub repository, and each of the three crates on crates.io (with its docs.rs reference folded into the description rather than given a second row) | — |
| Optional | The two sibling HL7® v2.5 dictionary crates, `hl7-2-from-er7-into-xml`/`-json` — real and relevant, but a different repository's crates, marked `## Optional` per the llms.txt convention for content a shorter context budget can skip | `src/lib/site.ts`'s `crates` |

Every link description is copied from that page's own `<meta
name="description">`, not paraphrased separately — one wording, not two
that can drift apart, minus its own `®` where one was present: the mark
belongs once, at the first use on the page (the same rule every other
page in this family already follows —
[`spec/hl7-trademarks-fair-use/index.md`](../hl7-trademarks-fair-use/index.md)),
and here that first use is in the intro blockquote, not inside a later
link description.

## Keeping it from drifting

Nothing currently checks these two files against `navLinks` or the
footer automatically — unlike `sitemap.xml`, which is at least fully
enumerable by a human diffing two lists, `llms.txt`'s descriptions are
free text a mechanical check could not verify meaningfully anyway. When
a page is added, renamed, or its `description` changes, update
`llms.txt` and `llms.json` in the same change, the same way a new route
already needs a footer link and a `sitemap.xml` entry.

Implemented 2026-08-30, prompted by a direct request rather than a
`tasks.md` item — recorded here because behaviour that reaches a
published URL belongs in the spec regardless of how the work was
scheduled. In the same change: `sitemap.xml` gained the six routes it
was already missing (`security`, `governance`, `maintainers`, `rfc`,
`ai-statement`, `agent-skill`) — a pre-existing gap surfaced while
building the same curated list for `llms.txt`, not something this task
introduced, and not something to leave stale next to a newer, more
complete map of the same site.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
