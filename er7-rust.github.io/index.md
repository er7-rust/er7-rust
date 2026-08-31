# er7-rust.github.io

The public site for the [`er7`](https://github.com/er7-rust/er7-rust) Rust
crate family — parse, query, edit, and write HL7® v2 messages in the ER7
pipe-hat encoding, and the subprojects built on it: `er7-redact` and
`serde-er7`. Every subproject is documented here in full — tutorial, help,
and examples — not merely linked to.

Live at **<https://er7-rust.github.io/>**.

Built with [SvelteKit](https://svelte.dev/docs/kit) and the
[Lily Design System](https://github.com/LilyDesignSystem)™, prerendered to
static HTML by `@sveltejs/adapter-static`, and deployed to GitHub Pages by
GitHub Actions on every push to `main`. There is no server and no
client-side data fetching: every page is a file.

## Contents

- [Quick start](#quick-start)
- [Routes](#routes)
- [How it is built](#how-it-is-built)
- [The Lily Design System](#the-lily-design-system)
- [Deployment](#deployment)
- [Editing content](#editing-content)
- [License](#license)

## Quick start

```sh
pnpm install     # install dependencies
pnpm dev         # dev server with hot reload, http://localhost:5173
pnpm check       # svelte-check: types and accessibility
pnpm build       # prerender to build/
pnpm preview     # serve build/ locally, exactly as Pages will
```

`pnpm check` and `pnpm build` both run in CI on every push; keep both
clean.

## Routes

| Route | What it covers |
| ----- | -------------- |
| `/` | What ER7 is, the problem it poses, and what the crate does about it |
| `/install/` | Installing all three crates and both binaries, building from source, verifying the install |
| `/format/` | The ER7 format itself: hierarchy, delimiters, the explicit null, batch files |
| `/paths/` | HL7 path notation — `PID-5.1`, `OBX[2]-5`, `PID-13[2].1` |
| `/escapes/` | Escape sequences: the full table, what decodes, what stays literal |
| `/cli/` | The `er7` command-line tool, with recipes |
| `/api/` | The complete public API surface |
| `/er7-redact/` | The `er7-redact` subproject: tutorial, help, examples |
| `/serde-er7/` | The `serde-er7` subproject: tutorial, help, examples |
| `/ecosystem/` | The crate family, and the two kinds of layer built on the encoding |
| `/comparison/` | How `er7` compares to interface engines, HAPI, other Rust HL7 crates, and hand-rolled parsing |
| `/benchmarks/` | Measured parsing, writing, escaping, and querying figures, with method and caveats |
| `/about/` | Licensing, citation, contributing, patient-safety constraints |

Not `er7` subproject documentation, but real routes all the same:

| Route | What it covers |
| ----- | -------------- |
| `/agent-skill/` | The two packaged Claude Code Skills this repository ships, `er7-skill` and `er7-rust-maintainer-skill` |
| `/news/` | Announcements, project status, and press contacts |
| `/security/` | How to report a vulnerability, and what is honestly promised |
| `/governance/` | Who decides, and what constrains them |
| `/maintainers/` | The maintainer roster and the stated bus factor |
| `/rfc/` | The specific feedback the project is asking for |
| `/ai-statement/` | How AI tools are used to develop these crates |
| `/trademarks/` | The trademark notice and fair-use posture |

Every route is prerendered. `trailingSlash: 'always'` means a path
resolves to its own directory index, which is what GitHub Pages serves
without a redirect.

## How it is built

```
src/app.html              Document shell; loads the stylesheet
src/lib/site.ts           Navigation, external links, the crate family
src/routes/+layout.svelte Header, nav, footer
src/routes/+layout.ts     prerender = true, trailingSlash = 'always'
src/routes/*/+page.svelte One page per route
static/assets/style.css   Lily Design System base + er7 additions
static/assets/themes/     light.css, dark.css — the ThemePicker's catalog
static/.nojekyll          Tells Pages not to run Jekyll over the output
static/sitemap.xml        Kept in step with the routes table above
.github/workflows/        Build, type-check, deploy
```

The content is the web face of the crate's own documentation. The crate
repository holds the normative version in `spec/` and `docs/`; these pages
explain and illustrate it. **Where the two disagree, the crate's `spec/`
directory is right** — see [`AGENTS.md`](AGENTS.md).

## The Lily Design System

`static/assets/style.css` starts as the Lily Design System's own site
stylesheet, taken from `lilydesignsystem.github.io`. Lily supplies:

- the **token vocabulary** — `--lily-primary`, `--lily-text`,
  `--lily-radius`, `--lily-content-max`, and the rest;
- the **component classes** — `.card`, `.card-grid`, `.callout`, `.hero`,
  `.stat`, `.tag`, `.prose`, `.button`, `.section`, `.site-*`.

An **er7 additions** block at the end of the file adds what this site
needs and Lily does not have: table styling, the ER7 anatomy figure, a
two-column code pair, a definition list, and a page table of contents. All
of it is built from Lily tokens.

To restyle, change a token. To add a component, add it to the additions
block — do not edit Lily's rules in place, so the base stays a clean copy
that can be refreshed from upstream.

The header's three controls — text size, theme, and share — are Lily's
own published Svelte helper packages, not hand-rolled:
[`lily-design-system-svelte-theme-picker`](https://www.npmjs.com/package/lily-design-system-svelte-theme-picker),
[`-text-size-picker`](https://www.npmjs.com/package/lily-design-system-svelte-text-size-picker), and
[`-share-picker`](https://www.npmjs.com/package/lily-design-system-svelte-share-picker), plus
[`lily-design-system-svelte-headless`](https://www.npmjs.com/package/lily-design-system-svelte-headless)
as a dependency for future interactive components (not yet used by any
page). All four ship zero CSS by design — Lily is headless — so the
`.theme-picker*`/`.text-size-picker*`/`.share-picker*` rules, and the
`data-theme`/`data-text-size` token overrides they drive, are this site's
own, in the additions block. `src/app.html` preloads both theme
stylesheets and resolves the active one synchronously before first paint,
so there is no flash from the light default to a stored or OS-preferred
dark theme.

## Deployment

GitHub Actions builds and deploys on every push to `main`
(`.github/workflows/deploy.yml`). The repository is named
`er7-rust.github.io` under the `er7-rust` organization, so the site is
served from the **domain root** — which is why `svelte.config.js` sets no
`paths.base`. A project-pages repository would need one.

One-time setup in the repository settings: **Settings → Pages → Source →
GitHub Actions**.

## Editing content

- Text lives in the `+page.svelte` files, as ordinary HTML with Lily
  classes. There is no CMS and no markdown pipeline.
- Shared values — links, the version, the crate list — live in
  `src/lib/site.ts`. Change a URL there, not in a page.
- Adding a route means adding a directory under `src/routes/`, a link in
  `navLinks`, a row in the routes table above, and an entry in
  `static/sitemap.xml`.
- `svelte.config.js` sets `strict: true`, so a link to a page that does
  not exist **fails the build** rather than shipping a dead link.

## License

The site content is free open source under the same terms as the crate:
MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only, at your
option. See [LICENSE.md](LICENSE.md).

Lily Design System™ is a trademark of its authors, and is itself free open
source. The stylesheet derived from it is used under those terms.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
