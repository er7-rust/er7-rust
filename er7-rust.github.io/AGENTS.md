# AGENTS.md

Guidance for AI coding agents (Claude Code, Codex, Copilot, Cursor, Aider,
etc.) working in this repository.

See [`index.md`](index.md) for the human-oriented overview, and
[`spec/index.md`](spec/index.md) for this site's scope and content model.

## Project snapshot

| Field | Value |
| ----- | ----- |
| Package | `er7-rust.github.io` |
| Purpose | The public site for the `er7` Rust crate family, documenting every subproject in full |
| Live at | https://er7-rust.github.io/ |
| Stack | SvelteKit 2 + Svelte 5 (runes), `@sveltejs/adapter-static`, TypeScript |
| Design system | Lily Design System™ |
| Package manager | pnpm |
| Deploy | GitHub Actions → GitHub Pages, on push to `main` |
| Crate repositories | https://github.com/er7-rust — `er7`, `er7-redact`, `serde-er7` |
| Maintainer | Joel Parker Henderson — joel@joelparkerhenderson.com |

## The one rule that matters most

**This site is derived, not normative.** Every crate repository holds the
source of truth for its own behaviour, in its `spec/` directory, with
numbered rules and a test for each — `R1`–`R26` for `er7`, `D1`–`D24` for
`er7-redact`, `S1`–`S12` for `serde-er7`. These pages explain and
illustrate those specs; they do not define anything.

So: **never state a behaviour here that the owning crate's spec does not
state.** If you find a discrepancy, the crate is right and this site is the
bug — unless the crate's spec is itself wrong, in which case fix it there
first and then update these pages. Do not "improve" a rule in prose.

And **never hand-write a tool's output.** Anything shown as what a command
prints or a serializer produces is captured from a real run of the released
crate (site spec §4.3).

Concretely, before changing any factual claim on a page, check it against
the matching section of the owning crate's spec:

| Page | Crate | Spec section |
| ---- | ----- | ------------ |
| `/format/` | `er7` | §2 The ER7 encoding |
| `/paths/` | `er7` | §8 Paths and queries |
| `/escapes/` | `er7` | §6 Escape sequences |
| `/cli/` | `er7` | §12 Command-line interface |
| `/api/` | `er7` | §5 value tree, §11 errors, and `docs/api/index.md` |
| `/er7-redact/` | `er7-redact` | §2 model, §3 actions, §4 preservation, §5 policies, §6 file format, §7 pseudonyms, §8 report, §10 CLI |
| `/serde-er7/` | `serde-er7` | §1 purpose, §2 wire shapes, §4 round trip, §5 errors, §6 ergonomics |
| `/ecosystem/` | `er7` | §1.3 scope, §18.1 the layer boundary |
| `/about/` | `er7` | §14 versioning, §15 metadata |

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

## Adding a subproject

A new crate in the `er7-rust` organisation is documented **in full**, on
one route, following the section order in
[`spec/index.md`](spec/index.md) §3.3: what it is, install, tutorial, help,
examples, what it does not do, where to go next.

That means the four edits above, plus:

5. `src/lib/site.ts` — a `crates` entry, so it appears on `/ecosystem/`.
6. `/ecosystem/` — a row in "what each layer owns", and a step in the
   worked example if the crate has something to say about the shared
   message.

Write the page from the crate's own `index.md`, `docs/`, and `spec/`, and
run its commands to capture their real output. A crate outside the
organisation is a *neighbour*, not a subproject: it gets a card on
`/ecosystem/` and nothing more (site spec §1.1).

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
