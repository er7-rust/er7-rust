# AGENTS.md

Guidance for AI coding agents (Claude Code, Codex, Copilot, Cursor, Aider,
etc.) working in this repository.

See [`index.md`](index.md) for the human-oriented overview, and
[`spec/index.md`](spec/index.md) for this site's scope and content model.

## Project snapshot

| Field | Value |
| ----- | ----- |
| Package | `er7-rust.github.io` |
| Purpose | The public site for the `er7` Rust crate |
| Live at | https://er7-rust.github.io/ |
| Stack | SvelteKit 2 + Svelte 5 (runes), `@sveltejs/adapter-static`, TypeScript |
| Design system | Lily Design System™ |
| Package manager | pnpm |
| Deploy | GitHub Actions → GitHub Pages, on push to `main` |
| Crate repository | https://github.com/er7-rust/er7-rust |
| Maintainer | Joel Parker Henderson — joel@joelparkerhenderson.com |

## The one rule that matters most

**This site is derived, not normative.** The `er7` crate repository holds
the source of truth for behaviour in its `spec/` directory, with numbered
rules `R1`–`R25` and a test for each. These pages explain and illustrate
that spec; they do not define it.

So: **never state a behaviour here that the crate's spec does not state.**
If you find a discrepancy, the crate is right and this site is the bug —
unless the crate's spec is itself wrong, in which case fix it there first
and then update these pages. Do not "improve" a rule in prose.

Concretely, before changing any factual claim on a page, check it against
the matching section of the crate's spec:

| Page | Crate spec section |
| ---- | ------------------ |
| `/format/` | §2 The ER7 encoding |
| `/paths/` | §8 Paths and queries |
| `/escapes/` | §6 Escape sequences |
| `/cli/` | §12 Command-line interface |
| `/api/` | §5 value tree, §11 errors, and `docs/api/index.md` |
| `/ecosystem/` | §1.3 scope, §18.1 the layer boundary |
| `/about/` | §14 versioning, §15 metadata |

## Layout

```
index.md                   Overview (README.md links here)
AGENTS.md                  This file
spec/index.md              This site's scope and content model
src/app.html               Document shell; loads the stylesheet
src/lib/site.ts            Navigation, external links, crate family, version
src/routes/+layout.svelte  Header, nav, footer
src/routes/+layout.ts      prerender = true, trailingSlash = 'always'
src/routes/*/+page.svelte  One page per route
static/assets/style.css    Lily base + an "er7 additions" block
static/.nojekyll           Stops Pages running Jekyll over the output
static/sitemap.xml         Must stay in step with the routes
.github/workflows/deploy.yml  Build, type-check, deploy
```

## Working conventions

- **Svelte 5 runes.** `$props()`, `$state()`, `$derived()`. Not
  `export let`, not stores, unless there is a reason to be written down.
- **Keyed `{#each}`.** Always `{#each items as item (item.key)}`.
- **TypeScript everywhere**, including `<script lang="ts">` in components.
- **No new dependencies** without the user asking. The site needs
  SvelteKit and nothing else; it has no runtime JavaScript beyond what
  SvelteKit ships.
- **Shared values live in `src/lib/site.ts`.** Links, the crate version,
  the crate family. Never hard-code a URL in a page — a rename should be
  one edit.
- **Content is HTML with Lily classes** inside `+page.svelte`. There is no
  markdown pipeline; do not add one without discussion.
- Use the **Svelte MCP server** when writing or changing components: run
  `svelte-autofixer` on a component before considering it done.

## Styling

`static/assets/style.css` is the Lily Design System's site stylesheet with
an additions block appended.

- **To restyle, change a token** (`--lily-*`) — not a rule.
- **To add a component, add it to the "er7 additions" block** at the end
  of the file. Do not edit Lily's rules in place: the base stays a clean
  copy so it can be refreshed from upstream
  (`~/git/lilydesignsystem/lily-design-system/lilydesignsystem.github.io/static/assets/style.css`).
- Prefer an existing Lily class over a new one. The vocabulary is
  `.hero`, `.section`, `.section-heading`, `.card`, `.card-grid`,
  `.callout`, `.button`, `.stat`, `.stat-row`, `.tag`, `.tag-list`,
  `.prose`, `.site-*`.
- The additions block adds `.table`, `.table-wrap`, `.er7-figure`,
  `.er7-line`, `.pair-grid`, `.defs`, `.toc`, `.anchor-heading`.

## Checks

Both run in CI on every push; keep both clean.

```sh
pnpm check       # svelte-check: types and accessibility
pnpm build       # prerender; fails on a dead internal link
```

`svelte.config.js` sets `strict: true`, so **a link to a page that does
not exist fails the build**. That is deliberate — it is the check that
keeps the site's cross-references honest. Do not add `handleHttpError` to
silence it; add the missing page, or fix the link.

## Adding a route

Four edits, all required:

1. `src/routes/<name>/+page.svelte` — the page, with a `<svelte:head>`
   holding a `<title>` and a `<meta name="description">`.
2. `src/lib/site.ts` — a `navLinks` entry.
3. `static/sitemap.xml` — a `<url>` entry.
4. `index.md` and `spec/index.md` — a row in the routes table.

## Patient safety

Every ER7 message shown on this site is **synthetic**, with obviously
fictional names (`EVERYWOMAN^EVE`, `SMITH^JOHN`) and identifiers
(`444333222`, `MSG00042`).

Never paste a real message into a page, an example, or a commit message —
including one a user shares to illustrate a point. Reproduce its shape
with a synthetic message instead. A "redacted" message still carries
dates, facility names, and identifier formats.

## Non-goals

- **A component library.** This site consumes Lily; it does not implement
  or ship components.
- **A markdown pipeline, a CMS, or a blog.** Eight hand-written pages do
  not need one.
- **Client-side search, analytics, or any third-party script.** The site
  ships no tracking and makes no third-party requests.
- **Duplicating the rustdoc.** `/api/` is a map with links to
  <https://docs.rs/er7>; it is not a substitute for it.
