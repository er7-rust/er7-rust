<script lang="ts">
  import { page } from '$app/state';
  import { links, navLinks, themes, shareTargets } from '$lib/site';
  import ThemePicker from 'lily-design-system-svelte-theme-picker';
  import TextSizePicker from 'lily-design-system-svelte-text-size-picker';
  import SharePicker from 'lily-design-system-svelte-share-picker';

  let { children } = $props();

  function isCurrent(href: string): boolean {
    return page.url.pathname === href;
  }

  let theme = $state('');
  let textSize = $state('');

  // page.data.title convention: every route's +page.ts sets `title` via
  // its load function (the same string its own <svelte:head><title>
  // reads, so the two cannot drift), and page.data — the merged data of
  // every load function on the current route — carries it up here. The
  // fallback only matters for a route that has not adopted the
  // convention yet; every route in this repository has.
  let shareTitle = $derived(
    (page.data as { title?: string }).title ??
      'er7 — HL7® v2 messages in the ER7 pipe-hat encoding, in Rust'
  );

  // Four destinations, chosen deliberately, not exhaustively: LinkedIn,
  // Mastodon, Bluesky, Reddit. A share link does not require this project
  // to have an account there itself — see AGENTS.md's "Header controls"
  // — and it is not a third-party *request*: it is a plain <a href> the
  // visitor chooses to follow, the same as the GitHub/crates.io/docs.rs
  // links already in this file's footer, so spec/index.md §6's "no
  // third-party requests" is unaffected.
</script>

<a class="skip-link" href="#main">Skip to main content</a>

<header class="site-header">
  <div class="site-header-inner">
    <a class="site-brand" href="/" aria-label="er7 home">
      <span>er7</span>
    </a>
    <nav class="site-nav" aria-label="Main">
      {#each navLinks as link (link.href)}
        <a href={link.href} aria-current={isCurrent(link.href) ? 'page' : undefined}>
          {link.label}
        </a>
      {/each}
      <a href={links.repository}>GitHub</a>
    </nav>
    <div class="site-controls">
      <TextSizePicker
        label="Text size"
        sizes={['small', 'medium', 'large', 'x-large']}
        bind:value={textSize}
        storageKey="lily-text-size"
      />
      <ThemePicker
        label="Theme"
        themesUrl="/assets/themes/"
        themes={[...themes]}
        bind:value={theme}
        storageKey="lily-theme"
        detectFromSystem
      />
      <SharePicker
        label="Share this page"
        title={shareTitle}
        targets={shareTargets}
        copyLabel="Copy link"
        copiedLabel="Link copied"
        copyFailedLabel="Could not copy — copy it from the address bar"
      />
    </div>
  </div>
</header>

<main id="main" class="site-main">
  {@render children()}
</main>

<footer class="site-footer">
  <div class="site-footer-inner">
    <p>
      <code>er7</code> — parse, query, edit, and write HL7® v2 messages in the ER7 pipe-hat
      encoding. Free open source: MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, or GPL-3.0.
    </p>
    <p class="site-footer-trademark">
      Built with SvelteKit and the
      <a href="https://github.com/LilyDesignSystem">Lily Design System</a>™.
    </p>
    <p class="site-footer-trademark">
      HL7&reg;, and FHIR&reg; are the registered trademarks of Health Level Seven International and
      their use of these trademarks does not constitute an endorsement by HL7.
      <a href="/trademarks/">Trademark notice</a>.
    </p>
    <div class="site-footer-links">
      <a href={links.repository}>Source</a>
      <a href={links.crate}>Crate</a>
      <a href={links.docs}>API docs</a>
      <a href="/ecosystem/">Ecosystem</a>
      <a href="/agent-skill/">Agent skills</a>
      <a href="/news/">News</a>
      <a href="/security/">Security</a>
      <a href="/governance/">Governance</a>
      <a href="/maintainers/">Maintainers</a>
      <a href="/rfc/">RFC</a>
      <a href="/ai-statement/">AI statement</a>
      <a href="/trademarks/">Trademarks</a>
      <a href="/about/">About</a>
    </div>
  </div>
</footer>
