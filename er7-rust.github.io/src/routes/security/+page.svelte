<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>Security — er7</title>
  <meta
    name="description"
    content="The security policy for the ER7 Rust crates: how to report a vulnerability, what is in scope, what is honestly promised, and what the project does not have."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Security</p>
  <h1>Security policy</h1>
  <p class="hero-tagline">
    How to report a vulnerability, what counts as one, and what a one-maintainer project can
    honestly promise about handling it.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Reporting a vulnerability</h2>
  <div class="prose">
    <p>
      <strong>Email <a href={links.email}>joel@joelparkerhenderson.com</a></strong> with
      &ldquo;security&rdquo; in the subject, or use GitHub&rsquo;s private vulnerability reporting
      on <a href={links.repository}>the repository</a> if you prefer a tracked channel. Do
      <strong>not</strong> open a public issue for anything exploitable: there is one maintainer and
      no embargo machinery, so a public report is a public disclosure whether or not it was meant as
      one.
    </p>
  </div>
  <div class="callout callout-warning">
    <p>
      <strong>Never send a real message.</strong> An HL7&reg; v2 message is a clinical record, and a
      real one in a report is a data breach regardless of intent. Redact first &mdash;
      <code>er7-redact message.er7</code> gives you a redacted message with the same shape, and
      <code>er7-redact --report message.er7</code> prints paths and actions with no values at all. A
      real message sent to this project will be deleted rather than used.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What is in scope</h2>
  <div class="prose">
    <p>
      These are libraries and two command-line tools that parse untrusted input and, in one case,
      remove patient detail from it. That gives three real categories:
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Category</th><th>Example</th><th>Why it matters</th></tr>
      </thead>
      <tbody>
        <tr>
          <td>Denial of service through parsing</td>
          <td>An input that makes <code>er7::parse</code> panic, hang, or allocate without bound</td>
          <td>The parser is the untrusted-input surface; a counterexample to rule R6 is a security bug, not just a spec bug</td>
        </tr>
        <tr>
          <td>Redaction failure</td>
          <td><code>er7-redact</code> leaves a value its policy names for removal, or <code>--report</code> prints a value</td>
          <td>This is the crate where being wrong leaks patient data; any instance is treated as a vulnerability</td>
        </tr>
        <tr>
          <td>Silent corruption</td>
          <td>A message round-trips to something other than what went in, without an error</td>
          <td>A clinical record altered silently in transit is a patient-safety problem</td>
        </tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Properties you can check, not just trust</h2>
  <div class="prose">
    <ul>
      <li>
        <strong>No <code>unsafe</code> anywhere</strong> &mdash; every crate root carries
        <code>#![forbid(unsafe_code)]</code>, which the compiler enforces and an
        <code>allow</code> cannot override.
      </li>
      <li>
        <strong>Runtime dependencies: zero, one, and two</strong> &mdash; <code>er7</code> has none
        (enforced by a test), <code>er7-redact</code> has one, <code>serde-er7</code> has two.
      </li>
      <li>
        <strong>No build scripts, and library code performs no I/O at all</strong> &mdash; no
        filesystem, no network, no process spawning, no environment reads, no telemetry.
      </li>
      <li>
        <strong>Fuzzed on the untrusted-input surface</strong> &mdash; four targets under
        <code>er7/fuzz/</code>, smoke-run in CI.
      </li>
    </ul>
    <p>
      The canonical policy names the commands that verify each property, plus the one disclosed
      leak path worth knowing before you log an error
      (<code>er7::Error::MissingHeader</code> carries text taken from the input).
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Disclosure, and what is honestly promised</h2>
  <div class="prose">
    <p>
      Disclosure is coordinated, with a deadline that favours users: <strong>90 days after your
      report, you should disclose publicly whether or not a fix exists.</strong> That is stated in
      advance because a one-maintainer project is exactly the kind that might go quiet.
    </p>
    <p>
      What the project does not have is named rather than omitted: no bug bounty, no second
      responder, no third-party audit, no signed commits or tags, no certification of any kind. The
      full list, and the reasoning, is in the canonical policy.
    </p>
    <p>
      <strong>The canonical document is
      <a href="{links.repository}/blob/main/SECURITY.md">SECURITY.md</a></strong> at the repository
      root; this page is a summary of it.
    </p>
  </div>
</section>
