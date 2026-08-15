<script lang="ts">
  import { links, version } from '$lib/site';

  const outline = `MSH-1       |
MSH-2       ^~\\&
MSH-3       LAB
MSH-9.1     ORU
MSH-9.2     R01
MSH-9.3     ORU_R01
MSH-10      MSG00042
PID-3.1     444333222
PID-3.4.1   ACME
PID-3.4.2   1.2.840.114398.1.100
PID-5.1     EVERYWOMAN
PID-5.2     EVE
PID-13[1]   555-555-1111
PID-13[2]   555-555-2222
OBX[1]-3.2  Cholesterol
OBX[1]-5    187
OBX[2]-3.2  Triglycerides`;

  const library = `let message = er7::parse(text)?;

assert_eq!(message.control_id().as_deref(), Some("MSG00042"));
assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.query("OBX-3.2")?.as_deref(), Some("Cholesterol"));

// What went in comes back out, byte for byte.
assert_eq!(message.to_er7(), text);`;
</script>

<svelte:head>
  <title>er7 — HL7 v2 messages in the ER7 pipe-hat encoding, in Rust</title>
  <meta
    name="description"
    content="A Rust crate and command-line tool to parse, query, edit, and write HL7 v2 messages in the ER7 pipe-hat encoding. Byte-for-byte round trip, zero dependencies."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Rust crate &amp; CLI &middot; v{version}</p>
  <h1>er7</h1>
  <p class="hero-tagline">
    Parse, query, edit, and write HL7 v2 messages in the ER7 pipe-hat encoding — with a
    byte-for-byte round trip and zero dependencies.
  </p>
  <div class="button-row">
    <a class="button button-primary" href="/format/">What is ER7?</a>
    <a class="button button-secondary" href={links.docs}>API documentation</a>
    <a class="button button-secondary" href={links.repository}>Source</a>
  </div>
</section>

<section class="section">
  <div class="er7-figure">
    <pre class="er7-line"><span class="er7-segment">MSH</span><span class="er7-delimiter"
        >|^~\&amp;|</span
      >LAB<span class="er7-delimiter">|</span>ACME<span class="er7-delimiter">|</span>EHR<span
        class="er7-delimiter">|</span
      >CLINIC<span class="er7-delimiter">|</span>20260815081500<span class="er7-delimiter"
        >||</span
      >ORU<span class="er7-delimiter">^</span>R01<span class="er7-delimiter">^</span>ORU_R01<span
        class="er7-delimiter">|</span
      >MSG00042<span class="er7-delimiter">|</span>P<span class="er7-delimiter">|</span>2.5
<span class="er7-segment">PID</span><span class="er7-delimiter">|</span>1<span
        class="er7-delimiter">||</span
      >444333222<span class="er7-delimiter">^^^</span>ACME<span class="er7-delimiter">&amp;</span
      >1.2.840.114398.1.100<span class="er7-delimiter">&amp;</span>ISO<span class="er7-delimiter"
        >^</span
      >MR<span class="er7-delimiter">||</span>EVERYWOMAN<span class="er7-delimiter">^</span>EVE<span
        class="er7-delimiter">^</span
      >E<span class="er7-delimiter">||</span>19620320<span class="er7-delimiter">|</span>F
<span class="er7-segment">OBX</span><span class="er7-delimiter">|</span>1<span
        class="er7-delimiter">|</span
      >NM<span class="er7-delimiter">|</span>2093-3<span class="er7-delimiter">^</span>Cholesterol<span
        class="er7-delimiter">^</span
      >LN<span class="er7-delimiter">||</span>187<span class="er7-delimiter">|</span>mg/dL<span
        class="er7-delimiter">|</span
      >&lt;200<span class="er7-delimiter">|</span>N<span class="er7-delimiter">|||</span>F</pre>
  </div>
  <p class="er7-caption">
    A lab result: who sent it, who the patient is, and what the cholesterol was. Delimiters in
    amber, segment names in blue. The same content in HL7&rsquo;s XML encoding runs to several
    kilobytes.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">The problem</span>
    Every value&rsquo;s meaning comes from its position
  </h2>
  <div class="prose">
    <p>
      <code>PID-5.1</code> is a family name because it is fifth and first, not because anything in
      the message says so. One misplaced <code>|</code> silently shifts everything after it, and a
      message that arrives correct but leaves altered is worse than one that fails to parse —
      because the failure is visible and the alteration is not.
    </p>
    <p>
      This crate makes that structure explicit and keeps it intact. Text is stored exactly as it
      arrived and decoded only when you ask for a value.
    </p>
  </div>

  <div class="stat-row">
    <div class="stat">
      <span class="stat-value">0</span>
      <span class="stat-label">dependencies, guaranteed by a test</span>
    </div>
    <div class="stat">
      <span class="stat-value">6</span>
      <span class="stat-label">levels, from message to subcomponent</span>
    </div>
    <div class="stat">
      <span class="stat-value">25</span>
      <span class="stat-label">numbered rules, each with a test</span>
    </div>
    <div class="stat">
      <span class="stat-value">1:1</span>
      <span class="stat-label">byte-for-byte round trip</span>
    </div>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Command line</span>
    Read a message without counting pipes
  </h2>
  <div class="prose">
    <p>
      The default output is an outline: one line per value, labelled with the HL7 path that names
      it. Every label is a valid query, so you can read a path off the output and paste it straight
      back in.
    </p>
  </div>

  <div class="pair-grid">
    <figure>
      <figcaption>er7 samples/oru_r01.er7</figcaption>
      <pre><code>{outline}</code></pre>
    </figure>
    <figure>
      <figcaption>Then query it</figcaption>
      <pre><code>{`# One value
er7 --query PID-5.1 message.er7
#=> EVERYWOMAN

# Every observation in a result
er7 --query OBX-5 message.er7
#=> 187
#=> 102

# Exactly as sent, escapes intact
er7 --raw --query OBX-5 message.er7

# Canonical ER7, readable in a terminal
er7 --normalize --terminator lf message.er7

# The second message of a batch file
cat batch.er7 | er7 --message 2`}</code></pre>
    </figure>
  </div>
  <p><a href="/cli/">Full command-line reference &rarr;</a></p>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Library</span>
    Query by path, or walk the tree
  </h2>
  <pre><code>{library}</code></pre>
  <div class="prose">
    <p>
      <code>query</code> returns the first match and <code>query_all</code> returns every one, so
      <code>OBX-5</code> across a result with three observations gives three values. When you need
      the node itself — to edit it, or to ask whether it is an explicit null — walk the tree with
      1-based accessors that match HL7&rsquo;s own numbering.
    </p>
  </div>
  <p><a href="/api/">Full API surface &rarr;</a></p>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Design</span>
    Three properties, in priority order
  </h2>
  <div class="card-grid">
    <article class="card">
      <h3 class="card-heading">Fidelity</h3>
      <p class="card-description">
        Any message the crate reads, it can write back unchanged — unusual delimiters, unknown
        segments, empty positions, escape sequences and all. Leaf text is stored raw and decoded on
        demand, which is what makes that possible.
      </p>
      <p class="card-meta">Rule R16</p>
    </article>
    <article class="card">
      <h3 class="card-heading">Distinction</h3>
      <p class="card-description">
        Absent, empty, and the explicit null <code>&quot;&quot;</code> are three different answers,
        not one. The null means <em>clear this value</em>; collapsing it into &ldquo;empty&rdquo;
        leaves a withdrawn allergy on a patient record.
      </p>
      <p class="card-meta">Rules R10, R11</p>
    </article>
    <article class="card">
      <h3 class="card-heading">Tolerance</h3>
      <p class="card-description">
        Below the header, nothing fails. Unknown segments, local Z segments, ragged field counts,
        and undecodable escapes are all data. A receiver that rejects a message it could have read
        drops clinical information.
      </p>
      <p class="card-meta">Rule R6</p>
    </article>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Scope</span>
    An encoding, not a dictionary
  </h2>
  <div class="callout">
    <p>
      <strong>This crate does not know what <code>PID-5</code> means.</strong> It does not carry
      segment definitions, data types, message structures, or code tables — all of that is specific
      to an HL7 version, while the ER7 encoding is shared by every release from v2.1 to v2.9.
    </p>
    <p>
      Keeping them apart means the encoding is maintained and tested once, and a dictionary crate
      can choose its own version. See the <a href="/ecosystem/">ecosystem</a> for the crates that
      add the HL7 v2.5 dictionary on top.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Install</span>
    Get started
  </h2>
  <div class="pair-grid">
    <figure>
      <figcaption>Library</figcaption>
      <pre><code>cargo add er7</code></pre>
    </figure>
    <figure>
      <figcaption>Command-line tool</figcaption>
      <pre><code>cargo install er7</code></pre>
    </figure>
  </div>
  <div class="button-row">
    <a class="button button-primary" href="/format/">Learn the format</a>
    <a class="button button-secondary" href="/paths/">Path notation</a>
    <a class="button button-secondary" href="/escapes/">Escape sequences</a>
  </div>
</section>
