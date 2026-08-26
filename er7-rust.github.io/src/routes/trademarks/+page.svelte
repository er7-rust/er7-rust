<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>Trademarks — er7</title>
  <meta
    name="description"
    content="The trademark notice for the ER7 Rust crate family: whose marks these are, how they are used here under fair use, and what this project does not claim."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Trademarks</p>
  <h1>Trademarks</h1>
  <p class="hero-tagline">
    HL7® is someone else&rsquo;s word mark. This page says whose, how it is used here, and what
    this project does not claim.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Notice</h2>
  <div class="callout">
    <p>
      HL7®, and FHIR® are the registered trademarks of Health Level Seven International and their
      use of these trademarks does not constitute an endorsement by HL7.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What this project uses, and how</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Mark</th><th>Owner</th><th>How it is used here</th></tr>
      </thead>
      <tbody>
        <tr>
          <td>HL7®</td>
          <td>Health Level Seven International</td>
          <td>Descriptively, to say what these crates read and write: HL7® v2 messages</td>
        </tr>
        <tr>
          <td>FHIR®</td>
          <td>Health Level Seven International</td>
          <td>Descriptively, and only to say that this project does <strong>not</strong> implement the HL7® FHIR® standard</td>
        </tr>
        <tr>
          <td>CDA®</td>
          <td>Health Level Seven International</td>
          <td>Not used</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      This is fair use of a word mark under the terms HL7 International publishes at
      <a href="https://www.hl7.org/legal/fairuse.cfm">hl7.org/legal/fairuse.cfm</a>: the marks name
      the standard the software works with, and nothing more.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What this project does not do</h2>
  <div class="prose">
    <ul>
      <li>
        <strong>No mark is part of any name we own.</strong> The crates are <code>er7</code>,
        <code>er7-redact</code>, and <code>serde-er7</code>; the binaries are <code>er7</code> and
        <code>er7-redact</code>; the organisation is <code>er7-rust</code> and the site is
        <code>er7-rust.github.io</code>. &ldquo;ER7&rdquo; is the common name of the pipe-hat
        encoding — sometimes written for <em>Encoding Rules 7</em> — not an HL7® word mark. That is
        why this crate is <code>er7</code> and not <code>hl7-something</code>.
      </li>
      <li><strong>No logo, badge, or brand element</strong> of HL7 International appears here.</li>
      <li>
        <strong>No claim of endorsement, affiliation, certification, accreditation, or conformity
        assessment</strong> is made anywhere in this project. HL7 International has not assessed
        this software. The <a href="/news/">news page</a> says the same thing in the form a reporter
        needs.
      </li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">How the rule is kept</h2>
  <div class="prose">
    <p>
      A trademark policy that lives only in prose drifts on the next page somebody adds. This one is
      a specification section with numbered rules and a check that enforces them: the ® on a
      mark&rsquo;s first use on every page, the disclaimer wherever the marks appear, and the
      HL7® FHIR® standard called by its full name. The check reads every document, every page of
      this site, every Rust doc comment, every crate description, and both <code>--help</code>
      outputs, and it fails the build on a violation.
    </p>
    <p>
      Sample messages, error strings, citation blocks, code identifiers, and crates.io keywords are
      deliberately left unmarked. A ® inside <code>MSH|^~\&amp;|…</code> corrupts the sample, and
      <code>no HL7 segments</code> is a diagnostic that gets grepped, asserted, and pasted into
      tickets — machine-readable output rather than prose. The reasoning is written out in the
      specification.
    </p>
  </div>
  <div class="button-row">
    <a class="button button-secondary" href="{links.repository}/blob/main/spec/hl7-trademarks-fair-use/index.md">
      The policy
    </a>
    <a class="button button-secondary" href="{links.repository}/blob/main/TRADEMARKS.md">
      The notice, in the repository
    </a>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Other marks</h2>
  <div class="prose">
    <p>
      &ldquo;Rust&rdquo; and the Rust logo are trademarks of the Rust Foundation. This project is
      written in Rust and is not affiliated with or endorsed by the Rust Foundation.
    </p>
    <p>
      Product names of the interface engines and libraries discussed on the
      <a href="/comparison/">comparison page</a> — Mirth Connect, Rhapsody, InterSystems,
      Cloverleaf, HAPI, and others — are the marks of their respective owners, and are used there
      nominatively, to identify the products being compared.
    </p>
    <h2>If we have this wrong</h2>
    <p>
      Trademark owners are welcome to write to <a href={links.email}>joel@joelparkerhenderson.com</a>.
      A correction here is treated the same way as any other defect report: acted on, not argued
      with.
    </p>
  </div>
</section>
