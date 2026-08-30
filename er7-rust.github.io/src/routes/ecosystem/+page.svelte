<script lang="ts">
  import { crates } from '$lib/site';
</script>

<svelte:head>
  <title>Ecosystem — er7</title>
  <meta
    name="description"
    content="The crate family: er7 for the ER7 encoding, er7-redact and serde-er7 as tools on top of it, and hl7-2-from-er7-into-xml and hl7-2-from-er7-into-json for the HL7® v2.5 dictionary layer."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">The crate family</p>
  <h1>Ecosystem</h1>
  <p class="hero-tagline">
    One small crate for the encoding, and crates on top of it that each do one job. The split is the
    point.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Why the encoding is its own layer</h2>
  <div class="prose">
    <p>
      ER7 is <strong>one small encoding, stable across every HL7 v2 release</strong> from v2.1 in
      1990 to v2.9. The data-type tables and message structures that give a field its meaning are
      <strong>specific to a version</strong>, and there are nine of them.
    </p>
    <p>
      Fusing the two would mean either shipping one version&rsquo;s tables to everyone, or shipping
      all of them to everyone. Keeping them apart means the encoding is maintained and tested once,
      a dictionary crate chooses its own version, and a user who only needs to route or audit
      messages pays for nothing.
    </p>
    <p>
      What sits on top comes in two kinds. A <strong>dictionary</strong> knows what a position
      <em>means</em> in one HL7 version. A <strong>tool</strong> does one job to any message and
      needs no dictionary at all: <code>er7-redact</code> removes a patient name knowing only which
      position holds it, and <code>serde-er7</code> moves the tree through a Serde format knowing
      only its shape. Neither has to choose an HL7 version, which is why neither is a dictionary.
    </p>
  </div>

  <pre><code>{`  ┌──────────────────────────────────┐  ┌──────────────────────────────────┐
  │  er7-redact       serde-er7      │  │  hl7-2-from-er7-into-xml         │
  │  policies, actions, pseudonyms;  │  │  hl7-2-from-er7-into-json        │
  │  Serde impls for the value tree  │  │  v2.5 types, structures, render  │
  └──────────────────────────────────┘  └──────────────────────────────────┘
     Tools — any HL7 version              HL7 v2.5 dictionary
                   │                                     │
                   └─────────────────┬───────────────────┘
                          depends on │
                                     ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │  er7                                                                 │
  │  delimiters, value tree, escapes, batch input                        │
  └──────────────────────────────────────────────────────────────────────┘   Encoding
                                     │
                                     ▼
                             (no dependencies)`}</code></pre>
</section>

<section class="section">
  <h2 class="section-heading">The crates</h2>
  <div class="card-grid">
    {#each crates as crate (crate.name)}
      <article class="card">
        <h3 class="card-heading"><code>{crate.name}</code></h3>
        <p class="card-description">{crate.description}</p>
        <p class="card-meta">{crate.layer}</p>
        <div class="tag-list">
          <a class="tag" href={crate.repository}>Source</a>
          <a class="tag" href={crate.crate}>Crate</a>
          <a class="tag" href={crate.docs}>Docs</a>
        </div>
      </article>
    {/each}
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What each layer owns</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Concern</th>
          <th>Lives in</th>
        </tr>
      </thead>
      <tbody>
        <tr><td>Delimiters, read from MSH-1 and MSH-2</td><td><code>er7</code></td></tr>
        <tr><td>The six-level value tree</td><td><code>er7</code></td></tr>
        <tr><td>Escape sequences</td><td><code>er7</code></td></tr>
        <tr><td>Batch splitting</td><td><code>er7</code></td></tr>
        <tr><td>Absent / empty / null</td><td><code>er7</code></td></tr>
        <tr><td>Byte-for-byte round trip</td><td><code>er7</code></td></tr>
        <tr><td>Which positions carry patient detail</td><td><code>er7-redact</code></td></tr>
        <tr><td>Masking a value without moving the shape</td><td><code>er7-redact</code></td></tr>
        <tr><td>Stable pseudonyms across messages</td><td><code>er7-redact</code></td></tr>
        <tr><td>Serialize and deserialize the tree</td><td><code>serde-er7</code></td></tr>
        <tr><td>Which data type each field carries</td><td>dictionary crate</td></tr>
        <tr><td>Composite component names (XPN.1, CX.4, …)</td><td>dictionary crate</td></tr>
        <tr><td>Message-structure grammars and grouping</td><td>dictionary crate</td></tr>
        <tr><td>XML or JSON rendering</td><td>dictionary crate</td></tr>
        <tr><td>MLLP framing, acknowledgements</td><td>neither — out of scope</td></tr>
        <tr><td>Validation of any kind</td><td>neither — out of scope</td></tr>
        <tr><td>Whether a redacted message is safe to share</td><td>none — a judgement</td></tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Worked example</span>
    The same message, five ways
  </h2>
  <div class="prose">
    <p>
      Given <code>PID|1||241900||TEST^FOUAZ</code>, each crate answers a different question about
      it.
    </p>
  </div>

  <h3><code>er7</code> — what is at this position?</h3>
  <pre><code>{`assert_eq!(message.query("PID-5.1")?.as_deref(), Some("TEST"));
assert_eq!(message.to_er7(), text);  // unchanged`}</code></pre>

  <h3><code>er7-redact</code> — what of it can be shared?</h3>
  <pre><code>{`PID|1||33f9458d5c426675||REDACTED^REDACTED`}</code></pre>
  <div class="prose">
    <p>
      Same fields, same components, same delimiters — the record number is a stable pseudonym, so
      two messages about this patient still agree they are about the same one. The crate also
      reports what it changed, as paths and actions with no values in them:
    </p>
  </div>
  <pre><code>{`PID[1]-3[1].1.1  pseudonym
PID[1]-5[1].1.1  replace REDACTED
PID[1]-5[1].2.1  replace REDACTED`}</code></pre>

  <h3><code>serde-er7</code> — how does the tree travel?</h3>
  <pre><code>{`{
  "name": "PID",
  "fields": [
    [[["1"]]],
    [],
    [[["241900"]]],
    [],
    [[["TEST"], ["FOUAZ"]]]
  ]
}`}</code></pre>
  <div class="prose">
    <p>
      Positions, not names: field, repetition, component, subcomponent, nested in that order, with
      <code>PID-2</code> an empty array because it was sent empty. Compare it with the
      <code>hl7-2-from-er7-into-json</code> output below — same message, same format, and the difference
      between them is exactly the dictionary.
    </p>
  </div>

  <h3><code>hl7-2-from-er7-into-xml</code> — what does it mean, as v2.xml?</h3>
  <pre><code>{`<PID>
  <PID.1>1</PID.1>
  <PID.3>
    <CX.1>241900</CX.1>
  </PID.3>
  <PID.5>
    <XPN.1>
      <FN.1>TEST</FN.1>
    </XPN.1>
    <XPN.2>FOUAZ</XPN.2>
  </PID.5>
</PID>`}</code></pre>
  <div class="prose">
    <p>
      The <code>XPN.1</code> and <code>FN.1</code> names come from the v2.5 data-type tables — that
      is exactly the knowledge <code>er7</code> does not carry.
    </p>
  </div>

  <h3><code>hl7-2-from-er7-into-json</code> — the same, as JSON</h3>
  <pre><code>{`"PID": {
  "PID.1": "1",
  "PID.3": { "CX.1": "241900" },
  "PID.5": {
    "XPN.1": { "FN.1": "TEST" },
    "XPN.2": "FOUAZ"
  }
}`}</code></pre>
</section>

<section class="section">
  <h2 class="section-heading">Building your own layer</h2>
  <div class="prose">
    <p>
      If you need a dictionary for a version other than v2.5, a different output format, or a tool
      that does one job to any message, <code>er7</code> is the foundation to build on. The crates
      above are worked examples of both shapes: a dictionary holds its version&rsquo;s tables,
      structure grammars, and renderer, while a tool such as <code>er7-redact</code> holds only the
      positions it acts on — and both delegate everything about the encoding.
    </p>
    <p>Three things they do that yours probably should:</p>
    <ul>
      <li>
        <strong>Keep your own error type</strong> and convert from <code>er7::Error</code> with a
        <code>From</code> impl, so your public API does not leak the parser.
      </li>
      <li>
        <strong>Decide your own whitespace policy.</strong> <code>er7</code> trims nothing because
        it guarantees a round trip; a converter that renders XML or JSON can afford to trim, and
        should do it before parsing.
      </li>
      <li>
        <strong>Take <code>&amp;Separators</code> through your renderer.</strong> Leaf text is
        decoded on demand, at the point it becomes output.
      </li>
    </ul>
  </div>
</section>
