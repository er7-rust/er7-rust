<script lang="ts">
  import { links, version } from '$lib/site';

  const citation = `@software{henderson_er7,
  author  = {Henderson, Joel Parker},
  title   = {ER7 Rust: Parse, query, edit, and write HL7 v2 messages
             in the ER7 pipe-hat encoding, with zero dependencies},
  url     = {https://github.com/er7-rust/er7-rust},
  version = {${version}}
}`;
</script>

<svelte:head>
  <title>About — er7</title>
  <meta
    name="description"
    content="About the er7 crate: who maintains it, how it is licensed, how to contribute, and how it is built and documented."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">About</p>
  <h1>About er7</h1>
  <p class="hero-tagline">
    A small, focused crate for one job: the ER7 encoding of HL7 v2 messages.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">At a glance</h2>
  <div class="table-wrap">
    <table class="table">
      <tbody>
        <tr><th>Crate</th><td><code>er7</code> {version}</td></tr>
        <tr><th>Purpose</th><td>Parse, query, edit, and write HL7 v2 messages in ER7</td></tr>
        <tr><th>Layer</th><td>Encoding only — no dictionary, no validation, no transport</td></tr>
        <tr><th>Language</th><td>Rust, edition 2024, MSRV 1.85</td></tr>
        <tr><th>Runtime dependencies</th><td>none, and that is a guarantee</td></tr>
        <tr><th>License</th><td>MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only</td></tr>
        <tr><th>Source</th><td><a href={links.repository}>github.com/er7-rust/er7-rust</a></td></tr>
        <tr><th>Crate</th><td><a href={links.crate}>crates.io/crates/er7</a></td></tr>
        <tr><th>API docs</th><td><a href={links.docs}>docs.rs/er7</a></td></tr>
        <tr><th>Maintainer</th><td><a href={links.email}>Joel Parker Henderson</a></td></tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">How it is built</h2>
  <div class="prose">
    <p>
      The crate uses <strong>spec-driven development</strong>. A living specification in the
      repository&rsquo;s <code>spec/</code> directory is the source of truth for behaviour: every
      behavioural change starts there, then propagates to tests, then code, then docs. When the spec
      and the code disagree, the spec is right and the code is a bug.
    </p>
    <p>
      Every guarantee carries a stable rule ID (<code>R1</code>&ndash;<code>R25</code>), and a
      coverage table maps each rule to the test that enforces it &mdash; and the table itself is
      checked by <code>cargo test</code>, so a rule added without a test, or a section file added
      without an index entry, fails the build rather than waiting for a careful reader. Every crate
      in the family does the same, with its own rule prefix. Plans and tasks live as spec
      sections rather than as separate files, so a roadmap item and the rule it changes sit next to
      each other.
    </p>
  </div>
  <div class="stat-row">
    <div class="stat">
      <span class="stat-value">19</span>
      <span class="stat-label">specification sections</span>
    </div>
    <div class="stat">
      <span class="stat-value">25</span>
      <span class="stat-label">numbered behavioural rules</span>
    </div>
    <div class="stat">
      <span class="stat-value">126</span>
      <span class="stat-label">unit, integration, and doc tests</span>
    </div>
    <div class="stat">
      <span class="stat-value">7</span>
      <span class="stat-label">runnable examples</span>
    </div>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Patient safety</h2>
  <div class="callout callout-warning">
    <p>
      This crate handles clinical messages about real patients. Two constraints follow, and they
      shape the whole design:
    </p>
    <ul>
      <li>
        <strong>Absent, empty, and the explicit null stay distinct.</strong> Treating a null as
        empty leaves a withdrawn allergy on a record; treating an empty as a null erases a value
        that was never sent.
      </li>
      <li>
        <strong>A message that arrives correct leaves unchanged.</strong> A silent alteration is
        worse than a visible failure.
      </li>
    </ul>
    <p>
      No test, sample, example, or documentation page in this project contains real patient data.
      Every message is synthetic, with obviously fictional names and identifiers.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Contributing</h2>
  <div class="prose">
    <p>
      Issues and pull requests are welcome at
      <a href={links.repository}>github.com/er7-rust/er7-rust</a>. Two requests:
    </p>
    <ul>
      <li>
        <strong>Include a synthetic message</strong> that reproduces the problem — never a real one,
        even redacted. A &ldquo;redacted&rdquo; message still carries dates, facility names, and
        identifier formats.
      </li>
      <li>
        <strong>Behavioural changes start in the spec.</strong> The repository&rsquo;s
        <code>AGENTS.md</code> and <code>AGENTS/</code> guides describe the change loop and the
        four checks every change must pass.
      </li>
    </ul>
    <p>
      If an answer turns out to be &ldquo;this is deliberate&rdquo;, it gets recorded in the
      spec&rsquo;s open-questions section, so the next person finds the reasoning rather than
      re-asking.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">License</h2>
  <div class="prose">
    <p>
      Multi-licensed. Use it under <strong>any one</strong> of MIT, Apache-2.0, BSD-3-Clause,
      GPL-2.0-only, or GPL-3.0-only, at your option. Pick the one that fits your project and comply
      with that one; you do not need to comply with all five, and you do not need to say which you
      chose.
    </p>
    <p>
      Healthcare integration code ends up inside organisations with very different legal
      constraints: a permissive license suits a vendor integrating into a proprietary product, while
      a copyleft license suits a public-sector project that wants derivatives kept open. Offering
      the choice means neither has to ask.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Citation</h2>
  <div class="prose">
    <p>
      The repository carries a <code>CITATION.cff</code>. If you would rather have BibTeX:
    </p>
  </div>
  <pre><code>{citation}</code></pre>
</section>

<section class="section">
  <h2 class="section-heading">This site</h2>
  <div class="prose">
    <p>
      Built with <a href="https://svelte.dev/docs/kit">SvelteKit</a> and the
      <a href="https://github.com/LilyDesignSystem">Lily Design System</a>™, prerendered to static
      HTML, and deployed to GitHub Pages by GitHub Actions. There is no server and no client-side
      data fetching; every page is a file.
    </p>
    <p>
      Its source is at
      <a href="https://github.com/er7-rust/er7-rust.github.io"
        >github.com/er7-rust/er7-rust.github.io</a
      >. Corrections to these pages are as welcome as corrections to the crate.
    </p>
  </div>
</section>
