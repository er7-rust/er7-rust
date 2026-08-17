# `er7-rust.github.io` specification

**Status:** living document, updated alongside every content change.
**Audience:** maintainers and AI agents changing this site.
**Companion docs:** [`AGENTS.md`](../AGENTS.md) for working conventions,
[`index.md`](../index.md) for the overview.

This document is the source of truth for **this site's scope and content
model** — what belongs here, what does not, and how a page is shaped. It is
deliberately *not* a source of truth for anything about the `er7` crate's
behaviour: that lives in the crate's own `spec/` directory (§2 below).

## 1. Purpose

Present the `er7` Rust crate to three audiences, in this order of priority:

1. **An integration engineer who has an ER7 message and a problem.** They
   need to know what the format is, how to read a value out of it, and what
   the command-line tool can tell them.
2. **A Rust developer evaluating the crate.** They need the API surface,
   the scope boundary, and enough of the design rationale to judge whether
   it fits.
3. **Someone who arrived from a search for "ER7" or "HL7 pipe-hat".** They
   need the format explained accurately, whether or not they ever use Rust.

Audience 3 is why `/format/` and `/escapes/` are written to stand alone,
without assuming the reader will install anything.

## 2. Derived, not normative

The crate repository is the source of truth for behaviour. It carries a
19-section specification with numbered rules `R1`–`R25`, each backed by a
test. **This site explains and illustrates that specification.**

Consequences:

- A factual claim here must be traceable to a crate spec section. The
  mapping is in [`AGENTS.md`](../AGENTS.md).
- When the two disagree, the crate is right and this site is the bug —
  unless the crate spec is itself wrong, in which case fix it there first.
- Rule IDs may be cited here (`R16`, `R10`) but are never *defined* here.
- Version-specific numbers — the rule count, the test count, the crate
  version — live in `src/lib/site.ts` or in one page each, so they can be
  updated in one place when the crate releases.

## 3. Scope

### 3.1 In scope

| Route | Covers | Traces to |
| ----- | ------ | --------- |
| `/` | What ER7 is, the positional problem, the three design properties, install | crate §1.5, §2 |
| `/format/` | Hierarchy, delimiters, empty/null, escapes overview, batch files, MLLP, why ER7 persists | crate §2 |
| `/paths/` | Path grammar, the four levels, occurrence indices, repetition special case, the four query methods | crate §8 |
| `/escapes/` | The full sequence table, why half stay literal, decoding, encoding, the tokenizer, the scoping divergence | crate §6, §18.2 |
| `/cli/` | Synopsis, options, the outline format, recipes, exit codes | crate §12 |
| `/api/` | Entry points, the value tree, accessors, configuration, errors | crate §5, §11, `docs/api/` |
| `/ecosystem/` | The encoding/dictionary split, the four crates and the two kinds of layer, building your own | crate §1.3, §18.1 |
| `/about/` | Metadata, spec-driven development, patient safety, contributing, license, citation | crate §14, §15 |

### 3.2 Out of scope

- **A component library.** This site consumes the Lily Design System; it
  does not implement or ship components.
- **Duplicating the rustdoc.** `/api/` is a map that links to
  <https://docs.rs/er7>, not a replacement for it. If a reader needs a full
  signature, send them there.
- **Sibling-crate documentation.** The HL7 v2.5 dictionary crates and
  `er7-redact` have their own repositories and docs; `/ecosystem/` links to
  them and explains the boundary, and stops there. A sibling gets a card
  and a row, never a route.
- **A blog, changelog, or news feed.** The crate's commit history is its
  changelog.
- **Anything interactive that needs a server**, since there is none.

## 4. Content model

### 4.1 Page shape

Every page is one `+page.svelte` with this shape, in order:

1. `<script lang="ts">` — page-local data as `const` arrays, so tables and
   lists are rendered from data rather than hand-written markup.
2. `<svelte:head>` — a `<title>` ending in `— er7`, and a
   `<meta name="description">` of one or two sentences.
3. `<section class="hero">` — eyebrow, `<h1>`, tagline, and on the home
   page a `.button-row`.
4. Optionally `<nav class="toc">` for pages long enough to need it
   (`/format/`, `/paths/`, `/escapes/`).
5. `<section class="section">` per topic, each opening with an
   `<h2 class="section-heading">`.

### 4.2 Prose rules

- **Explain the why, not just the what.** The crate's documentation is
  written that way and the site should match: a reader who knows *why*
  absent, empty, and null are kept apart will not collapse them.
- **Lead with the reader's problem.** Sections open with the situation,
  then the mechanism.
- **Show a real message.** Every concept that can be illustrated with ER7
  text should be, using the same synthetic sample across pages so the
  reader builds familiarity.
- **No marketing superlatives.** No "blazing fast", no "simply", no
  "just". The crate is small and careful; the prose should read that way.

### 4.3 Code samples

- Must be **valid and current**. A sample that no longer compiles against
  the released crate is a bug.
- Rust samples use the crate's public API only.
- Shell samples use the real option names from crate §12.
- Long samples go in `<pre><code>{`…`}</code></pre>` with a template
  literal, so the source stays readable and Svelte does not parse the
  contents.

### 4.4 Data, not markup

Tables and card grids render from a `const` array in the page's script
block. This keeps rows consistent, makes them easy to reorder, and means a
correction is a one-line data edit rather than a markup surgery.

## 5. Design system

The site uses the Lily Design System™ for its tokens and component
classes; [`AGENTS.md`](../AGENTS.md) has the working rules.

The invariant: **Lily's rules are never edited in place.** The base
stylesheet stays a clean copy of `lilydesignsystem.github.io`'s so it can
be refreshed from upstream, and everything this site adds goes in a marked
block at the end of the file.

## 6. Technical constraints

| Constraint | Why |
| ---------- | --- |
| Every route prerendered | GitHub Pages serves static files; there is no server |
| `trailingSlash: 'always'` | A path resolves to its own directory index, with no redirect |
| `strict: true` in the adapter | A link to a page that does not exist fails the build |
| No `paths.base` | The repo is `<org>.github.io`, so the site is served from the domain root |
| `static/.nojekyll` | Stops Pages running Jekyll over SvelteKit's `_app` directory |
| No third-party requests | No analytics, no fonts, no scripts; the site makes no outbound requests at runtime |

## 7. Patient safety

Every ER7 message shown on this site is synthetic, with obviously fictional
names and identifiers. Never publish a real message, even redacted — a
redacted message still carries dates, facility names, and identifier
formats. See the crate's `AGENTS/safety.md` for the full reasoning.

## 8. Maintenance

When the crate releases:

1. Update `version` in `src/lib/site.ts`.
2. Re-check every page against its crate spec section (the mapping in
   `AGENTS.md`), and fix anything the release changed.
3. Update the counts on `/about/` — rules, tests, examples — if they moved.
4. Run `pnpm check` and `pnpm build`.

When a route is added or removed, update `navLinks`, `static/sitemap.xml`,
the routes table in §3.1 above, and the one in `index.md`.
