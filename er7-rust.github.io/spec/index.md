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

Present the **`er7` family** — the `er7` crate and every subproject built
on it — to four audiences, in this order of priority:

1. **An integration engineer who has an ER7 message and a problem.** They
   need to know what the format is, how to read a value out of it, and what
   the command-line tools can tell them.
2. **A Rust developer evaluating a crate.** They need the API surface, the
   scope boundary, and enough of the design rationale to judge whether it
   fits.
3. **Someone who arrived from a search for "ER7" or "HL7® pipe-hat".** They
   need the format explained accurately, whether or not they ever use Rust.
4. **Someone who has already picked a subproject** — `er7-redact`,
   `serde-er7` — and needs to get it working today: install it, follow a
   tutorial, copy an example, look up an option.

Audience 3 is why `/format/` and `/escapes/` are written to stand alone,
without assuming the reader will install anything. Audience 4 is why every
subproject has a page of its own (§3.3) rather than a card on
`/ecosystem/`.

### 1.1 What a subproject is

A **subproject** is a crate in the [`er7-rust`](https://github.com/er7-rust)
organisation: today `er7`, `er7-redact`, and `serde-er7`. Each is
documented here in full.

A crate built on `er7` by somebody else — the HL7 v2.5 dictionary crates —
is a **neighbour**. `/ecosystem/` explains the boundary and links to it,
and stops there, because this site cannot promise to keep another
maintainer's documentation current.

## 2. Derived, not normative

**Every crate repository is the source of truth for its own behaviour**,
and each carries a specification with numbered rules, each backed by a
test:

| Crate | Spec | Rules |
| ----- | ---- | ----- |
| `er7` | 19 sections | `R1`–`R27` |
| `er7-redact` | 17 sections | `D1`–`D24` |
| `serde-er7` | 11 sections | `S1`–`S13` |

**This site explains and illustrates those specifications.** It never
defines behaviour, for any crate.

Consequences:

- A factual claim here must be traceable to a section of the spec belonging
  to the crate it is about. The mapping is in [`AGENTS.md`](../AGENTS.md).
- When the two disagree, the crate is right and this site is the bug —
  unless the crate spec is itself wrong, in which case fix it there first.
- Rule IDs may be cited here (`R16`, `D1`, `S3`) but are never *defined*
  here. The prefix says which crate owns the rule.
- Version-specific numbers — the rule count, the test count, the crate
  version — live in `src/lib/site.ts` or in one page each, so they can be
  updated in one place when a crate releases.
- **Every code sample and every command output is taken from a real run**
  of the released crate, not written by hand (§4.3). A page that shows what
  a tool prints is making a factual claim about it.

## 3. Scope

### 3.1 In scope

| Route | Covers | Traces to |
| ----- | ------ | --------- |
| `/` | What ER7 is, the positional problem, the three design properties, install | crate §1.5, §2 |
| `/format/` | Hierarchy, delimiters, empty/null, escapes overview, batch files, MLLP, why ER7 persists | crate §2 |
| `/paths/` | Path grammar, the four levels, occurrence indices, repetition special case, the four query methods | crate §8 |
| `/escapes/` | The full sequence table, why half stay literal, decoding, encoding, the tokenizer, the scoping divergence | crate §6, §18.2 |
| `/cli/` | Synopsis, options, the outline format, recipes, exit codes | crate §12 |
| `/api/` | Entry points, the value tree, accessors, configuration, errors | `er7` §5, §11, `docs/api/` |
| `/er7-redact/` | What redaction preserves, policies and actions, the policy file, pseudonyms, the report, the command | `er7-redact` §2–§10 |
| `/serde-er7/` | Why a wrapper crate, the wire shape per level, the round trip, `Deref` ergonomics | `serde-er7` §1–§6 |
| `/ecosystem/` | The encoding/dictionary split, the crate family and the two kinds of layer, building your own | `er7` §1.3, §18.1 |
| `/about/` | Metadata, spec-driven development, patient safety, contributing, license, citation | `er7` §14, §15 |
| `/install/` | Installing the command-line tools and libraries for all three crates, building from source, verifying the install | workspace `INSTALL.md` |
| `/comparison/` | How `er7` compares to interface engines, HAPI, other Rust HL7 crates, and hand-rolled parsing, and when to choose one of those instead | workspace `COMPARISONS.md` |
| `/benchmarks/` | Measured parsing, writing, escaping, and querying figures: machine, toolchain, confidence intervals, and how not to read them | workspace `BENCHMARKS.md` |

The first six rows are `er7`'s own documentation; `er7` is the one
subproject whose material is spread over several routes, because it is the
one every other page assumes. Every other subproject gets one route, shaped
by §3.3. The last three rows are workspace-level content with no single
owning crate, added after `/about/` covered a version of "install" that a
dedicated route later superseded (§3.1.1 covers what §3.1 does not: the
governance and outreach routes that trace to a root document rather than
a crate spec at all).

#### 3.1.1 Governance, outreach, and agent-facing routes

Not subproject documentation, and not traced to a crate spec section —
each of these instead summarizes one workspace-root document, states
that document as canonical, and stops (§1's four audiences do not
include a compliance reader or an AI agent, but these routes serve them
anyway, the same way the workspace root already carries `SECURITY.md`
etc. alongside the crates' own docs).

| Route | Covers | Traces to |
| ----- | ------ | --------- |
| `/agent-skill/` | The two packaged Claude Code Skills this repository ships | workspace `er7-skill/SKILL.md`, `er7-rust-maintainer-skill/SKILL.md` |
| `/news/` | Announcements, project status, press contacts | workspace `NEWS.md` |
| `/security/` | Vulnerability reporting, what is in scope, what is promised | workspace `SECURITY.md` |
| `/governance/` | Who decides, and what constrains them | workspace `GOVERNANCE.md` |
| `/maintainers/` | The maintainer roster, the stated bus factor | workspace `MAINTAINERS.md` |
| `/rfc/` | The specific feedback the project is asking for | workspace `RFC.md` |
| `/ai-statement/` | How AI tools are used to develop these crates | workspace `AI_STATEMENT.md` |
| `/trademarks/` | The trademark notice and fair-use posture | workspace `TRADEMARKS.md` |

Each of these pages already states, in its own prose, that the linked
root document is canonical and this page is a summary — that sentence is
the mechanism keeping this table honest, not a second thing to check.

### 3.2 Out of scope

- **A component library.** This site consumes the Lily Design System; it
  does not implement or ship components.
- **Duplicating the rustdoc.** A subproject page carries help, a tutorial,
  and examples — the things a reader cannot get from a signature list. For
  the signatures themselves, link to docs.rs. The test: if a change to a
  crate's private implementation would force an edit here, the page has
  gone too deep.
- **Restating a crate's spec.** A page explains and illustrates; the
  numbered rules live in the crate repository, and are cited rather than
  reproduced.
- **Neighbour-crate documentation.** A crate built on `er7` by another
  maintainer (§1.1) gets a card and a row on `/ecosystem/`, never a route.
  This site cannot promise to keep somebody else's documentation
  current.
- **A blog, changelog, or news feed.** The crate's commit history is its
  changelog.
- **Anything interactive that needs a server**, since there is none.

### 3.3 What a subproject page covers

Every subproject in the `er7-rust` organisation is documented **in full**,
on one route, in this order. A reader arriving cold should be able to
install the crate, run it, and look up what they need without leaving the
page.

| Section | Answers | Required |
| ------- | ------- | -------- |
| What it is | the one-sentence job, and the problem it solves | yes |
| Install | `cargo add`, and `cargo install` where there is a binary | yes |
| Tutorial | the shortest path from nothing to a working result, in numbered steps, each with a code sample | yes |
| Help | the reference a reader returns to: CLI options, the action or shape table, the file format | yes |
| Examples | what the crate's own `examples/` directory holds, and what each one shows | yes |
| What it does not do | the scope boundary, in the crate's own terms | yes |
| Where to go next | the crate's spec, docs.rs, source, and the pages that follow from it | yes |

Rules:

- **The crate's own documentation is the source.** A subproject page is
  written from that crate's `index.md`, `docs/`, and `spec/`, and must not
  contradict them (§2).
- **Show the tool's real output.** Anything presented as what a command
  prints is captured from a real run of the released version.
- **Link, do not copy, the API surface.** Signatures live on docs.rs.
- **A subproject page stands alone.** It may link to `/format/` or
  `/paths/` for background, but a reader who never follows those links can
  still get the crate working.

Adding a subproject is the four edits in [`AGENTS.md`](../AGENTS.md) for a
route, plus a `crates` entry in `src/lib/site.ts` and a card on
`/ecosystem/`.

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
- **Output is captured, not written.** Any block presented as what a
  command or a serializer produces is copied from a real run of the
  released crate. Hand-writing it makes the page a claim nobody checked.
- Rust samples use the crate's public API only.
- Shell samples use the real option names from that crate's CLI section.
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

When **`er7`** releases:

1. Update `version` in `src/lib/site.ts`.
2. Re-check every page against its crate spec section (the mapping in
   `AGENTS.md`), and fix anything the release changed.
3. Update the counts on `/about/` — rules, tests, examples — if they moved.
4. Run `pnpm check` and `pnpm build`.

When **a subproject** releases:

1. Re-run every command and sample on its page, and paste the new output
   (§4.3). This is the step that rots first.
2. Re-check its page against that crate's spec, including the rule IDs it
   cites.
3. Update its `description` in `src/lib/site.ts` if the crate's own
   one-sentence description changed.

When a **new subproject** appears, follow §3.3.

When a route is added or removed, update `navLinks`, `static/sitemap.xml`,
the routes table in §3.1 above, and the one in `index.md`.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
