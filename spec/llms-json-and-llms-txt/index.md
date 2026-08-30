[er7-rust](../../index.md) → [spec](../index.md) → llms.json and llms.txt

# §10 `llms.json` and `llms.txt`

This workspace publishes two curated, machine-readable maps of its own
content, in two places, for two different readers:

- **The workspace root** (`llms.txt`, `llms.json`) — for a reader
  working *in* a clone of this repository: an agent with the checkout on
  disk, a contributor browsing `github.com/er7-rust/er7-rust`. Every
  link is **repo-relative** (`AGENTS.md`, `spec/index.md`, `er7/AGENTS.md`
  and the like), which only resolves inside a checkout of this
  repository, not as a URL fetched from anywhere else.
- **The published site** (`er7-rust.github.io/llms.txt`,
  `.../llms.json`) — for a reader, or a crawler, of
  <https://er7-rust.github.io/>. Serving that exact text from the site's
  own `*.github.io/llms.txt` needs website-appropriate versions instead:
  every entry is an **absolute URL** (`https://er7-rust.github.io/...`),
  pointing at wherever it actually resolves from the site's own domain —
  a repo-relative link like `README.md` means nothing to a browser or a
  crawler that only ever sees the rendered site.

Both follow the same two formats:

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

## Where the four files live

`llms.txt` and `llms.json` at the **workspace root** are real,
independent files — not a copy of the site's content with different
link syntax, but this workspace's own file tree curated on its own
terms: the crate entry points, each crate's own spec, the two agent
skills, and the root project documents (`SECURITY.md`, `GOVERNANCE.md`,
and the rest), plus a pointer to the published site's own map for the
same content addressed by URL.

`er7-rust.github.io/static/llms.txt` and `.../static/llms.json` are the
canonical **site** files: `static/` is the one directory SvelteKit's
`adapter-static` copies to the built site verbatim (`svelte.config.js`),
which is exactly where `robots.txt` and `sitemap.xml` already live and
exactly what makes a well-known path like `/llms.txt` actually fetchable
at the published URL — the whole point of the convention.

`er7-rust.github.io/llms.txt` and `.../llms.json` — at that subproject's
own root, alongside its `index.md` and `AGENTS.md` — are symlinks to the
`static/` copies, the same convention every `README.md` in this workspace
already uses to point at its `index.md`: one canonical file, a second
path to it, so a reader browsing the site's own repository root sees the
same map a crawler fetches from the live URL.

## What each one curates

**The workspace-root files** cover, in order: the workspace's own entry
points (`AGENTS.md`, `index.md`, `spec/index.md`, `plan.md`, `tasks.md`),
each crate's own `index.md`/`AGENTS.md`/`spec/index.md`, the two agent
skills, the root project documents (`SECURITY.md` through `CITATION.cff`),
a pointer to the published site, and the sibling `hl7-rust` repository
under `## Optional`.

**The site files** cover the site's own routes, in four sections:

| Section | What | Source |
| ------- | ---- | ------ |
| Docs | The 13 pages in the site's own main navigation, in nav order | `src/lib/site.ts`'s `navLinks` |
| Project | The footer-only pages: agent skills, security, governance, maintainers, RFC, AI statement, trademarks, news | `+layout.svelte`'s footer link list |
| Source | The GitHub repository, and each of the three crates on crates.io (with its docs.rs reference folded into the description rather than given a second row) | — |
| Optional | The two sibling HL7® v2.5 dictionary crates, `hl7-2-from-er7-into-xml`/`-json` — real and relevant, but a different repository's crates, marked `## Optional` per the llms.txt convention for content a shorter context budget can skip | `src/lib/site.ts`'s `crates` |

Every site-file link description is copied from that page's own `<meta
name="description">`, with its own `®` dropped: the mark belongs once,
at the first use on the page (the same rule every other page in this
family already follows —
[`spec/hl7-trademarks-fair-use/index.md`](../hl7-trademarks-fair-use/index.md)),
and here that first use is in the intro blockquote, not a later link
description. The workspace-root files' descriptions are written fresh,
not copied from anywhere, since there is no `<meta>` tag on a Markdown
file to copy from.

## Keeping it from drifting

Nothing currently checks any of these four files against `navLinks`, the
footer, or the workspace's own file tree automatically — unlike
`sitemap.xml`, which is at least fully enumerable by a human diffing two
lists, an `llms.txt`/`llms.json` description is free text a mechanical
check could not verify meaningfully anyway. When a page is added,
renamed, or its description changes — on the site, or a new root
document or crate entry point in the workspace — update the matching
file in the same change, the same way a new site route already needs a
footer link and a `sitemap.xml` entry.

Implemented 2026-08-30, prompted by a direct request rather than a
`tasks.md` item — recorded here because behaviour that reaches a
published URL, or a repository root, belongs in the spec regardless of
how the work was scheduled. In the same change: `sitemap.xml` gained the
six routes it was already missing (`security`, `governance`,
`maintainers`, `rfc`, `ai-statement`, `agent-skill`) — a pre-existing gap
surfaced while building the same curated list for `llms.txt`, not
something this task introduced, and not something to leave stale next to
a newer, more complete map of the same site. The workspace-root pair
(`llms.txt`, `llms.json`) followed as a second request, once it was
clear the site pair's absolute URLs could not double as a map of the
workspace's own file tree.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
