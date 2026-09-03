<script lang="ts">
  import { links } from '$lib/site';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const boilerplate = `ER7 Rust is an open-source Cargo workspace of three Rust crates for
HL7® v2 messages in the ER7 pipe-hat encoding: er7 parses, queries, edits,
and writes them with zero runtime dependencies; er7-redact removes patient
detail without changing the shape of the message; and serde-er7 carries a
message tree through JSON, YAML, or any other Serde data format. It is
multi-licensed under MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or
GPL-3.0-only, at the user's option, and is maintained by Joel Parker
Henderson. https://er7-rust.github.io`;
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta
    name="description"
    content="Announcements, project status, where updates appear, and press contacts for the ER7 Rust crate family."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">News</p>
  <h1>News</h1>
  <p class="hero-tagline">
    Announcements, project status, where updates appear, and press contacts. Not a changelog: this
    is what is worth telling someone who is not reading diffs.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Status at a glance</h2>
  <div class="table-wrap">
    <table class="table">
      <tbody>
        <tr><th>First published</th><td>2026-08-15; the workspace was assembled 2026-08-19</td></tr>
        <tr><th>Crates</th><td>Three, versioned and published independently</td></tr>
        <tr><th>Maturity</th><td><code>0.x</code>. New, and the API may still break in a minor bump.</td></tr>
        <tr><th>Maintainers</th><td>One. The bus factor is stated plainly in <code>MAINTAINERS.md</code>.</td></tr>
        <tr><th>Scope</th><td>The ER7 pipe-hat <em>encoding</em>. Not validation, not a typed segment model, not a transport, not the HL7® FHIR® standard.</td></tr>
        <tr><th>Runtime dependencies</th><td><code>er7</code> zero, <code>er7-redact</code> one, <code>serde-er7</code> two</td></tr>
        <tr><th>Rust</th><td>Current stable minus two releases; today 1.96</td></tr>
        <tr><th>License</th><td>MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only</td></tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-09-02 · serde-er7 0.3.0</h2>
  <h2 class="section-heading">A typo in a hand-written JSON fixture can now name itself</h2>
  <div class="prose">
    <p>
      <code>Strict&lt;T&gt;</code> (for <code>Message</code>, <code>Segment</code>, and
      <code>Separators</code>) is a new, opt-in wrapper that reports an unrecognized JSON key by
      name instead of ignoring it &mdash; the plain types keep their unconditional tolerance for
      an unknown field. Two cases improve: a typo on a required key now names the actual mistake
      instead of a generic &ldquo;missing field&rdquo; that never mentions it, and a typo on the
      one optional key this crate has (<code>Separators::truncation</code>) is caught at all,
      where the plain type silently defaults it to <code>None</code>. Strictness nests, so
      <code>Strict&lt;Message&gt;</code> catches a typo inside a segment or its separators, not
      only at the top level.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-30 · er7-redact 0.4.0</h2>
  <h2 class="section-heading">A redaction rule can now run a caller&rsquo;s own code</h2>
  <div class="prose">
    <p>
      <code>Action</code> gained a ninth variant, <code>Custom</code>, reached through
      <code>Action::custom(f)</code> &mdash; a closure with the same signature as
      <code>Action::apply</code> itself, run at exactly the point a built-in action would run.
      <strong>This is a breaking change</strong>: <code>Action</code> is not marked
      non-exhaustive, so code matching it exhaustively needs a new arm. A worked example ships
      alongside it: shifting an HL7&reg; timestamp by a per-patient offset, declined as a built-in
      action but now expressible as a custom one.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-29 · er7 0.2.0, 0.2.1 · serde-er7 0.2.0 · er7-redact 0.3.0</h2>
  <h2 class="section-heading">The floor moved to 1.96, and known values now get caught wherever they repeat</h2>
  <div class="prose">
    <p>
      All three crates raised the minimum supported Rust version from current stable minus three to
      minus two — 1.95 to 1.96 — which never lands in a patch, so all three took a minor release.
      <code>er7-redact</code> carried one more breaking change of its own:
      <code>Policy</code> gained a public <code>search_known_values</code> field, defaulting to
      <code>true</code>, so a value redacted at one named position is now also redacted wherever
      else it repeats in the same message, case-insensitively and only as a whole word — a policy
      file turns it off with <code>known-values off</code>.
    </p>
    <p>
      Additive alongside it: <code>Redactor::uncovered</code> and the CLI&rsquo;s
      <code>--uncovered</code> flag report every leaf a policy names no rule for;
      <code>er7::read_messages</code> streams a batch one message at a time instead of holding the
      whole input; and <code>Segment::first_value</code> reads one field&rsquo;s first repetition
      and subcomponent straight off a segment already in hand.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-26</h2>
  <h2 class="section-heading">The HL7 marks, marked</h2>
  <div class="prose">
    <p>
      HL7 International lets anyone use its word marks descriptively, and asks three things in
      return: the ® on a mark&rsquo;s first use on each page, a disclaimer wherever the marks
      appear, and Fast Healthcare Interoperability Resources called the &ldquo;HL7® FHIR®
      standard&rdquo;. This project now does all three, everywhere.
    </p>
    <p>
      Two decisions are visible enough to state. Sample messages, error strings, citation blocks,
      and code identifiers are <strong>not</strong> marked: a ® inside
      <code>MSH|^~\&amp;|…</code> corrupts the sample, and <code>no HL7 segments</code> is a
      diagnostic that gets grepped and asserted, not prose that gets read. And none of it rests on
      good intentions — a check in the repository fails the build if a page drifts. See the
      <a href="/trademarks/">trademark notice</a>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-26</h2>
  <h2 class="section-heading">What this project claims, and how to check it</h2>
  <div class="prose">
    <p>
      A set of documents that answer what an evaluation actually asks, rather than what a README
      usually says.
    </p>
    <ul>
      <li>
        <strong><a href="/benchmarks/">Benchmarks with their method</a>.</strong> Measured figures
        with confidence intervals, on a named machine and toolchain, from benchmarks anyone can run.
      </li>
      <li>
        <strong><a href="/comparison/">A comparison that says when to choose something else</a>.</strong>
        Interface engines, HAPI, the other Rust crates, and hand-rolled pipe splitting, in their
        actual categories — including four cases where one of those is the better answer.
      </li>
      <li>
        <strong>A statement of who is behind this, and what happens if he is not.</strong>
        <code>MAINTAINERS.md</code> inventories every publishing identity, says the bus factor is
        one, and lists what is missing rather than omitting it.
      </li>
      <li>
        <strong>A disclosure of how the software is built.</strong>
        <code>AI_STATEMENT.md</code> states where AI tooling is used, at what level, under which
        controls, and with which limitations still standing.
      </li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-23 · er7-redact 0.2.0</h2>
  <h2 class="section-heading">A policy states its posture</h2>
  <div class="prose">
    <p>
      A redaction policy now says explicitly whether it accepts or rejects by default.
      <code>Policy::all_but_the_header()</code> rejects every value no rule accepts, keeping
      <code>MSH</code> intact — the posture to choose when the message is unfamiliar, because the
      failure mode of a missed rule is a redacted value rather than a leaked one.
      <strong>This is a breaking change</strong>: existing policies must say which posture they
      mean.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-20</h2>
  <h2 class="section-heading">Single-value queries stopped walking the whole message</h2>
  <div class="prose">
    <p>
      <code>query</code> now returns on first match. Against a 402-segment message that is 85% off a
      field lookup and 92% off a subcomponent lookup. Fuzz targets and Criterion benchmarks landed
      in the same release, and the minimum supported Rust version — current stable minus three — was
      pinned across the workspace.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading-eyebrow">2026-08-19</h2>
  <h2 class="section-heading">Three repositories became one workspace</h2>
  <div class="prose">
    <p>
      <code>er7</code>, <code>er7-redact</code>, and <code>serde-er7</code> were assembled into one
      Cargo workspace with their histories preserved. They still version and publish independently;
      what they now share is one <code>Cargo.lock</code>, one set of build checks, and one
      workspace-level specification holding the policy common to all three.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Where updates appear</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Channel</th><th>What arrives there</th></tr>
      </thead>
      <tbody>
        <tr><td>This page</td><td>Announcements, written for readers</td></tr>
        <tr><td><a href={links.crate}>crates.io</a></td><td>Every release, per crate; the authoritative version list</td></tr>
        <tr><td><a href={links.repository}>GitHub</a></td><td>Commits, issues, and releases. <em>Watch → Releases only</em> is the low-volume subscription.</td></tr>
        <tr><td><a href="{links.repository}/blob/main/CHANGELOG.md"><code>CHANGELOG.md</code></a></td><td>Change-by-change detail</td></tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      There is no mailing list and no social account. If one is added, it will be announced here
      first.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Press and media</h2>
  <div class="prose">
    <p>
      <strong>Contact:</strong> Joel Parker Henderson,
      <a href={links.email}>joel@joelparkerhenderson.com</a>. Sole maintainer, and the only person
      who can speak for the project. Please say what you are writing and by when; a same-day reply
      is likely but not promised.
    </p>
    <p>
      <strong>Available on request:</strong> background on HL7 v2 and why a pipe-delimited encoding
      designed in the 1980s is still the backbone of hospital integration; why de-identifying a
      message without changing its shape is harder than it looks, and why the shape matters;
      commentary on memory safety and dependency auditing in health IT; the reasoning behind any
      design decision in the project, all of which is written down.
    </p>
    <h3>Boilerplate, ready to quote</h3>
  </div>
  <pre><code>{boilerplate}</code></pre>
  <div class="prose">
    <h3>Facts a story might need, all checkable</h3>
    <ul>
      <li><strong>Three crates</strong>, published on crates.io from 2026-08-15.</li>
      <li>
        <strong>Zero runtime dependencies in <code>er7</code></strong>, enforced by a test that fails
        if one is added.
      </li>
      <li>
        <strong>A byte-for-byte round trip</strong> is a tested guarantee: a message parsed and not
        modified renders back identically.
      </li>
      <li>
        <strong>Redaction preserves structure</strong>, so a redacted message still reproduces the
        bug the original caused — and <code>--report</code> prints paths and actions with no values,
        so the report itself is safe to paste into a ticket.
      </li>
      <li>
        <strong>Five licenses at the user&rsquo;s option</strong>, chosen so that a proprietary
        vendor and a public-sector project can both adopt it without asking.
      </li>
      <li>
        <strong>A minimum supported Rust version of current stable minus two</strong>, chosen
        because hospital toolchains are approved on a cycle measured in quarters.
      </li>
      <li><strong>Benchmarks and their method are published</strong>, with the reasons not to over-read them.</li>
    </ul>
    <h3>What this project will not say</h3>
  </div>
  <div class="callout callout-warning">
    <ul>
      <li>
        <strong>It is not certified or accredited by anyone.</strong> HL7 International has not
        assessed it. No conformance testing body has assessed it.
      </li>
      <li><strong>It is not a medical device</strong>, and it makes no clinical claim.</li>
      <li>
        <strong>It does not validate messages</strong> against the standard&rsquo;s tables, and never
        has. A story describing it as a validator would be wrong.
      </li>
      <li>
        <strong>It has no production track record to cite.</strong> It was first published in 2026.
        Any story implying hospital deployments would be inventing them.
      </li>
      <li>
        <strong>No benchmark comparison against another library exists</strong>, so no
        &ldquo;faster than X&rdquo; claim can be sourced to this project.
      </li>
      <li>
        <strong>No adoption or download figure will be offered as a success metric.</strong> The
        numbers are public on crates.io and are small, as you would expect of a project this age.
      </li>
      <li>
        <strong>No example, sample, or screenshot anywhere contains real patient data</strong>, and
        none ever will. Every one is synthetic, by policy.
      </li>
    </ul>
  </div>
  <div class="prose">
    <h3>Trademark</h3>
    <p>
      HL7® and FHIR® are registered trademarks of Health Level Seven International. This project
      implements a published encoding and is
      <strong>independent of, not affiliated with, and not endorsed by</strong> HL7 International.
      Please carry that qualifier in any coverage; a story that implies otherwise would be wrong in
      a way that matters to a standards body. The full notice is on the
      <a href="/trademarks/">trademarks page</a>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Corrections</h2>
  <div class="prose">
    <p>
      If something on this page — or anywhere in this project — stops being true, that is a defect
      and worth reporting the same way any other defect is. Everything here is written so it can be
      checked rather than believed, which only works if people check.
    </p>
  </div>
</section>
