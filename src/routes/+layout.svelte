<script lang="ts">
  import { page } from '$app/state';
  import { links, navLinks } from '$lib/site';

  let { children } = $props();

  function isCurrent(href: string): boolean {
    return page.url.pathname === href;
  }
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
  </div>
</header>

<main id="main" class="site-main">
  {@render children()}
</main>

<footer class="site-footer">
  <div class="site-footer-inner">
    <p>
      <code>er7</code> — parse, query, edit, and write HL7 v2 messages in the ER7 pipe-hat
      encoding. Free open source: MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, or GPL-3.0.
    </p>
    <p class="site-footer-trademark">
      Built with SvelteKit and the
      <a href="https://github.com/LilyDesignSystem">Lily Design System</a>™.
    </p>
    <div class="site-footer-links">
      <a href={links.repository}>Source</a>
      <a href={links.crate}>Crate</a>
      <a href={links.docs}>API docs</a>
      <a href="/ecosystem/">Ecosystem</a>
      <a href="/about/">About</a>
    </div>
  </div>
</footer>
