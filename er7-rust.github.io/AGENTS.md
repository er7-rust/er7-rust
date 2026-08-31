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
| Deploy | `make github-pages` (from the monorepo root; `git subtree push`, `make publish` still works as an alias) exports this directory to the sibling `er7-rust/er7-rust.github.io` repo; GitHub Actions there builds and deploys on push to *its* `main` — [`index.md`](index.md)'s "Deployment" section, policy in [`spec/monorepo-github-pages/index.md`](../spec/monorepo-github-pages/index.md) |
| Crate repositories | https://github.com/er7-rust — `er7`, `er7-redact`, `serde-er7` |
| Maintainer | Joel Parker Henderson — joel@joelparkerhenderson.com |

## The one rule that matters most

**This site is derived, not normative.** Every crate repository holds the
source of truth for its own behaviour, in its `spec/` directory, with
numbered rules and a test for each — `R1`–`R27` for `er7`, `D1`–`D24` for
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
src/routes/*/+page.ts      load() returns { title } — page.data.title convention
src/routes/*/+page.svelte  One page per route
static/assets/style.css    Lily base + an "er7 additions" block
static/assets/themes/      light.css, dark.css — ThemePicker's catalog
static/.nojekyll           Stops Pages running Jekyll over the output
static/sitemap.xml         Must stay in step with the routes
.tool-versions             Pins Node for local dev (mise/asdf); matches deploy.yml
.github/workflows/deploy.yml  Build, type-check, deploy — Node 26
```

## Working conventions

- **Svelte 5 runes.** `$props()`, `$state()`, `$derived()`. Not
  `export let`, not stores, unless there is a reason to be written down.
- **Keyed `{#each}`.** Always `{#each items as item (item.key)}`.
- **TypeScript everywhere**, including `<script lang="ts">` in components.
- **No new dependencies** without the user asking. As of 2026-08-31 the
  user has asked for four: `lily-design-system-svelte-theme-picker`,
  `-text-size-picker`, `-share-picker`, and `-headless` (the last, unused
  by any page yet, for future interactive components) — see "Header
  controls" below. Nothing else; the site still ships no other runtime
  JavaScript beyond what SvelteKit and these four provide.
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
  `.er7-line`, `.pair-grid`, `.defs`, `.toc`, `.anchor-heading`,
  `.site-controls`, and the `.theme-picker*`/`.text-size-picker*`/
  `.share-picker*` rules for the three header controls below.

## Header controls

`+layout.svelte`'s header renders three Lily Svelte helper packages —
text size, theme, and share — each an icon button opening a WAI-ARIA APG
listbox (verify against the installed package's own
`dist/*.svelte`, not its README: the text-size-picker's published README
still describes a native `<select>` from an earlier version; 0.1.1
actually ships the same button-plus-listbox markup as the other two).

- **Theme.** Attribute-based, and built for a multi-stylesheet catalog,
  not just the two entries it ships with today. `themesUrl="/assets/themes/"`,
  `themes={[...themes]}` (from `src/lib/site.ts`, the one place the
  catalog is listed — see its own doc comment for the four places adding
  a theme touches), `storageKey="lily-theme"`, `detectFromSystem`.
  `static/assets/themes/light.css` and `dark.css` each scope their
  `--lily-*` overrides to `:root[data-theme="…"]`; `src/app.html` preloads
  both (so switching is instant, no per-switch fetch) and runs a small
  inline script, before the stylesheet link, that resolves `data-theme`
  synchronously from storage or `prefers-color-scheme` — matching the
  picker's own resolution order — so there is no flash of the light
  default on a dark-preferring visit. Two of Lily's own (unedited) rules
  hardcode a light-only background tint (`#eef2ff`, in `.tag`,
  `.site-nav a[aria-current="page"]`, and `.button-secondary:hover`)
  rather than a token; overridden for `data-theme="dark"` in the
  additions block rather than in Lily's rules, per the invariant above.
- **Text size.** `sizes={['small', 'medium', 'large', 'x-large']}`,
  `storageKey="lily-text-size"`. Sets `data-text-size` on `<html>`; the
  additions block maps each slug to a `font-size` on `:root`.
- **Share.** `title={shareTitle}` reads the page.data.title convention
  below (never a fixed string); `targets={shareTargets}`
  (`src/lib/site.ts`) is four networks — LinkedIn, Mastodon, Bluesky,
  Reddit — chosen deliberately, not exhaustively. Each `href` builds that
  network's own share-intent URL from `url`/`title`; the package ships no
  endpoints of its own (see its README). A share link does not put this
  project on that network — it gives a visitor one path there — and it is
  not a third-party *request*: a plain `<a href>` the visitor chooses to
  follow, no different from the GitHub/crates.io/docs.rs links already in
  this file's footer, so `spec/index.md` §6's "no third-party requests"
  still holds.

All three ship zero CSS by design (Lily is headless); every rule for them
lives in the additions block, built from `--lily-*` tokens like every
other addition.

## page.data.title convention

Every route's `+page.ts` sets `title` via its `load` function — see
"Adding a route" above for the exact shape. `+page.svelte` reads it back
as `{data.title}` in `<svelte:head>`; `+layout.svelte` reads the same
value from `page.data.title` (with a home-page-title fallback, for a
route that has not adopted the convention — none currently haven't) and
passes it to the SharePicker above. One string, read in two places,
never duplicated into two that can drift.

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

Five edits, all required:

1. `src/routes/<name>/+page.ts` — `export const load: PageLoad = () => ({
   title: '…' })`. This is the **page.data.title convention**: the one
   place a route's title is written down. `+page.svelte`'s
   `<svelte:head>` reads it back as `{data.title}` rather than repeating
   the string, and `+layout.svelte` reads the same value from
   `page.data.title` for the header's SharePicker — one string, never two
   that can drift. Every route has one; there is no route without a
   title.
2. `src/routes/<name>/+page.svelte` — the page. Its script block starts
   `import type { PageData } from './$types';` then
   `let { data }: { data: PageData } = $props();`, before any other
   import or const. `<svelte:head>` holds `<title>{data.title}</title>`
   and a `<meta name="description">` (this one still hand-written per
   page — only the title is shared).
3. A link in `+layout.svelte`: a `navLinks` entry in `src/lib/site.ts` for
   a main-navigation page, or a hard-coded link in the footer's own link
   list for a governance/outreach/agent-facing page that does not belong
   in the main nav (the pattern every route under §3.1.1 of
   [`spec/index.md`](spec/index.md) already follows — `security`,
   `governance`, `maintainers`, `rfc`, `ai-statement`, `trademarks`,
   `news`, `agent-skill`).
4. `static/sitemap.xml` — a `<url>` entry.
5. `index.md` and `spec/index.md` — a row in the routes table.

A trademark trap worth naming: if a route's title carries a mark's first
use (e.g. `paths`'s "HL7® paths — er7"), that ® lives in `+page.ts` now,
not `+page.svelte` — `bin/check-trademarks` scans per file, so a page
whose *next* "HL7" mention (typically the `<meta name="description">`)
has no ® of its own will fail, even though the rendered page's real first
use (the `<title>`, which the browser paints before the meta tag) is
covered. Give that next mention its own ® rather than relying on the
title in the other file to cover it — `/` and `/paths/` both needed this
fix when the convention landed.

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
- **A markdown pipeline, a CMS, or a blog.** Twenty-one hand-written pages
  do not need one.
- **Client-side search, analytics, or any third-party script.** The site
  ships no tracking and makes no third-party requests.
- **Duplicating the rustdoc.** `/api/` is a map with links to
  <https://docs.rs/er7>; it is not a substitute for it.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
