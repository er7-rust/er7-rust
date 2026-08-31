<script lang="ts">
  import { links } from '$lib/site';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // crates.io figures, read from its API on 2026-08-26. Recorded as context,
  // not as a ranking: an older crate has had longer to accumulate downloads.
  const crates = [
    {
      name: 'hl7-parser',
      latest: '0.3.0',
      published: '2025-02',
      downloads: '16,640',
      url: 'https://github.com/hamaluik/hl7-parser',
      shape:
        'The most-used and most actively developed alternative. Parses structure without validating; optional Serde, and timestamps into chrono, time, or jiff; message building; cursor-by-character-index. Apache-2.0.'
    },
    {
      name: 'hl7v2-parser',
      latest: '1.2.0',
      published: '2026-03',
      downloads: '209',
      url: 'https://github.com/EffortlessMetrics/hl7v2-rs',
      shape:
        'Newer. Zero-allocation where possible; a companion hl7v2_stream gives event-based streaming with bounded memory.'
    },
    {
      name: 'rust-hl7',
      latest: '0.5.0',
      published: '2021-09',
      downloads: '14,777',
      url: 'https://github.com/wokket/rust-hl7',
      shape:
        'Buffer-copy-free indexing with HL7® notation; self-described as experimental; explicitly no plan for conformance checking. Last published 2021.'
    },
    {
      name: 'hl7-mllp-codec',
      latest: '0.4.0',
      published: '2022-07',
      downloads: '25,755',
      url: 'https://github.com/wokket/hl7-mllp-codec',
      shape:
        'Not a parser at all: a Tokio codec for MLLP framing. Complementary to any of these, including er7.'
    }
  ];
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta
    name="description"
    content="How er7 compares to interface engines, HAPI and the mature libraries, the other Rust HL7 crates, and hand-rolled pipe splitting — including when to choose one of those instead."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Comparison</p>
  <h1>How this compares</h1>
  <p class="hero-tagline">
    Interface engines, the mature libraries on other platforms, the other Rust crates, and the
    pipe-splitting you were about to write yourself. When each is the right answer, and when this is
    the wrong one.
  </p>
</section>

<section class="section">
  <div class="callout">
    <p>
      <strong>No performance comparison is claimed here.</strong> Nothing in this family has been
      benchmarked against another library. Comparing fairly means matching what each one actually
      does, and a parser that only splits on pipes is not doing the same work as one that preserves
      escape sequences and round-trips byte for byte. So this page compares
      <em>capability and shape</em>, which is checkable, rather than speed, which would not be. Our
      own measured figures, with their method, are on the
      <a href="/benchmarks/">benchmarks page</a>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">First, which kind of thing do you need?</h2>
  <div class="prose">
    <p>
      Most comparisons in this space go wrong by putting products from three different categories
      into one table. The honest first question is what you are building.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>If you need to…</th><th>You want</th><th>This family</th></tr>
      </thead>
      <tbody>
        <tr>
          <td><em>Run</em> interfaces: routes, retries, queues, monitoring, on-call</td>
          <td>An interface engine</td>
          <td>Not that. Useful alongside one.</td>
        </tr>
        <tr>
          <td>Write an application that happens to speak HL7® v2</td>
          <td>A library</td>
          <td>Yes — <code>er7</code>, <code>er7-redact</code>, <code>serde-er7</code></td>
        </tr>
        <tr>
          <td>Do a one-off transformation at a shell prompt</td>
          <td>A command-line tool</td>
          <td>Yes — two binaries, no Rust knowledge required</td>
        </tr>
        <tr>
          <td>Validate a message against the standard&rsquo;s tables</td>
          <td>A conformance tool</td>
          <td><strong>No.</strong> Deliberately out of scope.</td>
        </tr>
        <tr>
          <td>Speak the HL7® FHIR® standard</td>
          <td>An HL7® FHIR® library</td>
          <td>No. Different standard entirely.</td>
        </tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Interface engines</h2>
  <div class="prose">
    <p>
      <a href="https://github.com/OpenIntegrationEngine">Open Integration Engine</a> — the community
      fork of Mirth Connect, made after Mirth moved to a commercial-only license — and its
      commercial siblings are a different category of thing entirely. An engine gives you channels,
      routing, a management UI, transformers, persistence, retry, alerting, and an operations story.
      It is a system you deploy and run.
    </p>
    <p>
      A library gives you a function call. If your problem is &ldquo;forty interfaces, three
      hospitals, and someone has to be paged when one stops&rdquo;, an engine is the right answer
      and no amount of crate substitutes for it.
    </p>
    <p>These crates are useful <em>alongside</em> an engine rather than instead of it:</p>
    <ul>
      <li>
        The service at the end of a channel, where you would otherwise be writing the v2 parsing
        again in whatever language that service is in.
      </li>
      <li>
        A shell-level check, redaction, or normalisation, using the command-line tools, without
        standing anything up.
      </li>
      <li>
        Getting a message into a bug report. <code>er7-redact --report</code> prints the paths and
        actions with <strong>no values in the output</strong>, which is the shape a ticket can
        carry.
      </li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">HAPI, and the mature libraries</h2>
  <div class="prose">
    <p>
      <a href="https://hapifhir.github.io/hapi-hl7v2/">HAPI HL7v2</a> is the reference open-source
      HL7 v2 library, in Java, dual licensed under MPL 1.1 and GPL 2.0. It has been maintained for
      two decades, ships a generated typed model for every segment and message of every release, and
      has seen far more real traffic than anything here.
      <a href="https://github.com/nHapiNET/nHapi">NHapi</a> is its .NET port;
      <a href="https://github.com/crs4/hl7apy">hl7apy</a> is the best-established Python option.
    </p>
    <p>
      <strong>If your platform is the JVM, use HAPI.</strong> That is not modesty: a twenty-year-old
      library with complete release coverage and a large user base is the lower-risk choice, and
      reimplementing it in a language you were not otherwise using is a bad trade.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th></th><th>The mature libraries</th><th>Here</th></tr>
      </thead>
      <tbody>
        <tr>
          <td>Typed model</td>
          <td>Complete generated model: every segment, every release</td>
          <td><strong>None.</strong> A structural tree; meaning comes from the path</td>
        </tr>
        <tr>
          <td>Validation</td>
          <td>Conformance checking against the standard&rsquo;s tables</td>
          <td><strong>None</strong>, on purpose</td>
        </tr>
        <tr>
          <td>Runtime</td>
          <td>A JVM, a CLR, or a Python interpreter</td>
          <td>A static binary: no runtime, no GC</td>
        </tr>
        <tr>
          <td>Dependency tree</td>
          <td>Substantial, and audited as such</td>
          <td>Zero, one, and two crates respectively</td>
        </tr>
        <tr>
          <td>Round trip</td>
          <td>Varies by parser and options</td>
          <td>Byte for byte, as a tested guarantee</td>
        </tr>
        <tr>
          <td>Track record</td>
          <td>Two decades of production traffic</td>
          <td>First published 2026-08-15. New.</td>
        </tr>
        <tr>
          <td>License</td>
          <td>MPL 1.1 or GPL 2.0, for HAPI</td>
          <td>Five, at your option</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      The license row decides some evaluations outright. A permissive option matters if you are
      linking into a closed-source product; a copyleft option matters if your organisation prefers
      one. Offering five is how this project avoids having that conversation with anyone.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">The other Rust crates</h2>
  <div class="prose">
    <p>
      A small field, and cooperation beats competition in it. The figures below were read from the
      crates.io API on 2026-08-26, as context rather than as a ranking.
    </p>
  </div>
  <div class="card-grid">
    {#each crates as crate (crate.name)}
      <article class="card">
        <h3 class="card-heading"><a href={crate.url}><code>{crate.name}</code></a></h3>
        <p class="card-meta">{crate.latest} · last published {crate.published} · {crate.downloads} downloads</p>
        <p class="card-description">{crate.shape}</p>
      </article>
    {/each}
  </div>
  <div class="prose">
    <h3>When to choose one of those instead</h3>
    <p>These are real reasons, not hedges.</p>
    <ul>
      <li>
        <strong>You want timestamp types.</strong> <code>hl7-parser</code> parses HL7 timestamps into
        <code>chrono</code>, <code>time</code>, or <code>jiff</code>. <code>er7</code> returns the
        text and leaves the interpretation to you, because doing otherwise means a dependency.
      </li>
      <li>
        <strong>You want streaming with bounded memory over very large inputs.</strong>
        <code>hl7v2_stream</code> is built for that; <code>er7</code> parses a message into a tree.
      </li>
      <li>
        <strong>You want Serde on the parser itself.</strong> <code>hl7-parser</code> has a feature
        flag. Here it is a separate crate, precisely so that users who do not want Serde do not pay
        for it.
      </li>
      <li>
        <strong>You are already using one and it works.</strong> Switching a working HL7 parser is
        rarely the highest-value thing on anyone&rsquo;s list.
      </li>
    </ul>
    <h3>Where these crates differ</h3>
    <ul>
      <li>
        <strong>Zero runtime dependencies in <code>er7</code>, enforced by a test.</strong> The
        dependency table is empty and a test fails if that changes. Criterion lives in a separate
        unpublished workspace member so that even the development dependencies stay empty.
      </li>
      <li>
        <strong>A command-line tool, not only a library.</strong> Every label in its output is a
        valid query you can paste back in.
      </li>
      <li>
        <strong>Redaction as a first-class, separately published thing.</strong>
        <code>er7-redact</code> has no equivalent in the list above.
      </li>
      <li>
        <strong>Specification-first development.</strong> Every behaviour is a numbered rule, and
        each rule names the test that enforces it — checkable by reading the tree.
      </li>
      <li><strong>Five licenses</strong>, where the alternatives above are Apache-2.0 or MIT.</li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Splitting on pipes yourself</h2>
  <div class="prose">
    <p>The honest comparison, because it is what most teams actually do and it is sometimes right.</p>
  </div>
  <pre><code>{`fields = line.split("|")          # this works, until it doesn't`}</code></pre>
  <div class="prose">
    <p>
      <strong>When hand-rolling is right:</strong> you read one field, from one sender, whose
      messages you control, in a script that will not outlive the week.
    </p>
    <p><strong>When it stops being right</strong>, in roughly the order teams discover it:</p>
    <ol>
      <li>
        <strong>Escape sequences.</strong> <code>\F\</code> is a literal <code>|</code> in a value.
        Splitting on <code>|</code> cuts a value in half, silently.
      </li>
      <li>
        <strong>The <code>MSH</code> segment is special.</strong> <code>MSH-1</code> <em>is</em> the
        field separator, so the segment does not index like any other.
      </li>
      <li>
        <strong>A sender who does not use the default delimiters.</strong> They are declared in
        <code>MSH-2</code> and you are expected to read them, not assume them.
      </li>
      <li>
        <strong>Repeating fields.</strong> <code>PID-13</code> may be one phone number or four, and
        the code that assumed one is now wrong.
      </li>
      <li><strong>Components and subcomponents</strong> go two levels deeper.</li>
      <li>
        <strong>Writing.</strong> Now you need to escape on the way out, and an unescaped delimiter
        breaks the message for everyone downstream.
      </li>
      <li>
        <strong>Round-tripping.</strong> Someone asks why the message that came out is not
        byte-identical to the one that went in, and the answer is a week.
      </li>
    </ol>
    <p>
      The pitch is not &ldquo;your split is wrong&rdquo;. It is that the seven items above are
      exactly what <code>er7</code> implements, in about the same runtime cost, with a test for each
      and no dependency to audit.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What none of these crates do</h2>
  <div class="prose">
    <p>Stated here so no comparison implies otherwise.</p>
    <ul>
      <li><strong>No validation or conformance checking.</strong> Structure is parsed; content is not judged.</li>
      <li><strong>No typed segment model.</strong> No generated class per segment.</li>
      <li><strong>No transport.</strong> No MLLP, no TCP, no SOAP.</li>
      <li><strong>No HL7® FHIR® standard support</strong>, and no v2-to-FHIR® mapping.</li>
      <li>
        <strong>No clinical claim.</strong> These are encoding libraries, not medical devices, and
        nobody has certified them.
      </li>
    </ul>
    <p>
      Each crate&rsquo;s own specification states its scope and its non-goals precisely; that is the
      authoritative version of this list. The full write-up, with the reasoning, is
      <a href="{links.repository}/blob/main/COMPARISONS.md"><code>COMPARISONS.md</code></a> in the
      repository.
    </p>
  </div>
</section>
