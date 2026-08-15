<script lang="ts">
  const hierarchy = [
    { level: 'message', sep: '—', example: 'the whole text' },
    { level: 'segment', sep: 'carriage return', example: 'PID|1||444333222...' },
    { level: 'field', sep: '|', example: 'EVERYWOMAN^EVE^E' },
    { level: 'repetition', sep: '~', example: '555-1111~555-2222' },
    { level: 'component', sep: '^', example: 'EVERYWOMAN' },
    { level: 'subcomponent', sep: '&', example: '1.2.840.114398.1.100' }
  ];

  const states = [
    {
      wire: 'the field is absent',
      means: 'no information',
      action: 'leave any stored value alone'
    },
    { wire: '||', means: 'present, but no value', action: 'leave any stored value alone' },
    { wire: '|""|', means: 'the explicit null', action: 'clear the stored value' }
  ];
</script>

<svelte:head>
  <title>The ER7 format — er7</title>
  <meta
    name="description"
    content="What ER7 is: the pipe-hat text encoding for HL7 v2 messages. Hierarchy, delimiters, the explicit null, escape sequences, batch files, and why it persists."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Reference</p>
  <h1>The ER7 format</h1>
  <p class="hero-tagline">
    Background on the format itself, independent of any implementation. This is what the crate
    reads and writes.
  </p>
</section>

<nav class="toc" aria-label="On this page">
  <h2>On this page</h2>
  <ul>
    <li><a href="#what-it-is">What ER7 is</a></li>
    <li><a href="#hierarchy">Hierarchy</a></li>
    <li><a href="#delimiters">Delimiters</a></li>
    <li><a href="#null">Empty, and the explicit null</a></li>
    <li><a href="#escapes">Escape sequences</a></li>
    <li><a href="#batch">Batch files</a></li>
    <li><a href="#wire">On the wire</a></li>
    <li><a href="#why">Why it persists</a></li>
  </ul>
</nav>

<section class="section">
  <h2 id="what-it-is" class="anchor-heading section-heading">What ER7 is</h2>
  <div class="prose">
    <p>
      <strong>ER7</strong> — &ldquo;Encoding Rules 7&rdquo; — is the original text encoding for HL7
      version 2 messages, and still the one nearly every production interface speaks. It is defined
      in chapter 2 of every v2 release, from 2.1 in 1990 through 2.9. The nickname
      <em>pipe-hat</em> comes from its two most visible delimiters, <code>|</code> and
      <code>^</code>.
    </p>
    <p>An ER7 message is plain text, positional, and small.</p>
  </div>
</section>

<section class="section">
  <h2 id="hierarchy" class="anchor-heading section-heading">Hierarchy</h2>
  <div class="prose">
    <p>Six levels, each with its own delimiter.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Level</th>
          <th>Separated by</th>
          <th>Example</th>
        </tr>
      </thead>
      <tbody>
        {#each hierarchy as row (row.level)}
          <tr>
            <td><strong>{row.level}</strong></td>
            <td><code>{row.sep}</code></td>
            <td><code>{row.example}</code></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      A <strong>segment</strong> is three characters of name followed by its fields:
      <code>MSH</code> is the message header, <code>PID</code> patient identification,
      <code>OBX</code> an observation. Names beginning with <code>Z</code> are local extensions,
      defined by whoever is at the two ends and by nobody else.
    </p>
    <p>
      Everything is <strong>positional</strong>. <code>PID-5.1</code> is a family name because it is
      fifth and first, not because anything in the message says so. This is why one misplaced
      <code>|</code> corrupts everything after it, and why a message needs a dictionary — the HL7
      standard for that version — before its values mean anything.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="delimiters" class="anchor-heading section-heading">Delimiters</h2>
  <div class="prose">
    <p>
      Only the segment terminator is fixed: a carriage return, <code>\r</code>, hex 0D. The standard
      is explicit that implementers cannot change it. In practice many systems store messages in
      files with <code>\n</code> or <code>\r\n</code> instead, so a tolerant reader accepts all
      three.
    </p>
    <p>The rest are declared by the message itself, in its first two fields.</p>
  </div>

  <div class="er7-figure">
    <pre class="er7-line"><span class="er7-segment">MSH</span><span class="er7-delimiter"
        >|^~\&amp;|</span
      >
<span class="er7-note">   ^^^^^</span>
<span class="er7-note">   |||||</span>
<span class="er7-note">   ||||+- subcomponent separator  (MSH-2 position 4)</span>
<span class="er7-note">   |||+-- escape character        (MSH-2 position 3)</span>
<span class="er7-note">   ||+--- repetition separator    (MSH-2 position 2)</span>
<span class="er7-note">   |+---- component separator     (MSH-2 position 1)</span>
<span class="er7-note">   +----- field separator         (MSH-1, 4th character of the message)</span></pre>
  </div>

  <div class="prose">
    <p>
      MSH-1 is a field whose value <em>is</em> the field separator, and MSH-2 is a field whose value
      <em>is</em> the encoding characters. This is circular by design, and it is why those two
      fields can never be split or escaped like ordinary ones. It also means a reader learns the
      delimiters from bytes 4&ndash;8 of the message and must not assume <code>|^~\&amp;</code>,
      however universal that choice is in practice.
    </p>
    <p>
      HL7 v2.7 added a fifth encoding character, the <strong>truncation character</strong>
      (recommended <code>#</code>), marking a value the sender cut short to fit a length limit. Most
      messages omit it.
    </p>
    <p>
      The batch envelope segments <code>FHS</code> and <code>BHS</code> declare delimiters the same
      way, since a batch file may begin with either.
    </p>
  </div>

  <div class="callout">
    <p>
      <strong>In the crate:</strong> the delimiter set is read from every message, never hardcoded,
      and a set that reuses one character for two roles is rejected — that message could not be read
      back as the sender meant it.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="null" class="anchor-heading section-heading">Empty, and the explicit null</h2>
  <div class="prose">
    <p>Three states, easily confused, and the difference is clinical.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>On the wire</th>
          <th>Means</th>
          <th>A receiver must</th>
        </tr>
      </thead>
      <tbody>
        {#each states as row (row.wire)}
          <tr>
            <td><code>{row.wire}</code></td>
            <td>{row.means}</td>
            <td>{row.action}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Trailing fields a sender has nothing for may simply be dropped, so a <code>PID</code> ending
      after field 8 is normal and says nothing about fields 9 onward. The two-character
      <code>&quot;&quot;</code> is the only way to say &ldquo;delete what you have&rdquo;.
    </p>
  </div>
  <div class="callout callout-warning">
    <p>
      <strong>Getting this wrong is a patient-safety bug.</strong> Treating a null as empty leaves a
      withdrawn allergy on the record. Treating an empty as a null erases a value that was never
      sent. The crate keeps all three apart, at every level of the tree.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="escapes" class="anchor-heading section-heading">Escape sequences</h2>
  <div class="prose">
    <p>
      A value that needs to contain a delimiter escapes it. A sequence is the escape character, a
      body, and the escape character again: <code>\F\</code>, <code>\X0D\</code>,
      <code>\.br\</code>.
    </p>
    <p>
      Five of them stand for the delimiters themselves; the rest are hexadecimal data, character-set
      switches, display formatting, or locally agreed extensions.
    </p>
  </div>
  <p><a href="/escapes/">The full escape-sequence reference &rarr;</a></p>
</section>

<section class="section">
  <h2 id="batch" class="anchor-heading section-heading">Batch files</h2>
  <div class="prose">
    <p>Several messages can share a file, wrapped in an envelope.</p>
  </div>
  <pre><code>{`FHS   file header
  BHS   batch header
    MSH   message ...
    MSH   message ...
  BTS   batch trailer
FTS   file trailer`}</code></pre>
  <div class="prose">
    <p>
      The envelope segments describe the <em>file</em>, not any message in it: who sent the batch,
      how many messages it holds, when it was written. A reader that wants the messages drops them
      and starts a new message at each <code>MSH</code>.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="wire" class="anchor-heading section-heading">On the wire</h2>
  <div class="prose">
    <p>
      ER7 messages are usually carried by <strong>MLLP</strong> (Minimal Lower Layer Protocol): each
      message is wrapped in a start byte (0x0B) and an end sequence (0x1C 0x0D) over a TCP
      connection, and the receiver answers with an <code>ACK</code> message quoting the
      original&rsquo;s MSH-10 control ID.
    </p>
    <p>
      The framing is a separate concern from the encoding, which is why this crate handles only the
      latter. Strip the framing bytes before parsing.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="why" class="anchor-heading section-heading">Why it persists</h2>
  <div class="prose">
    <p>
      The tradeoffs are stark, and they have kept ER7 in place for thirty-five years.
    </p>
  </div>
  <div class="card-grid">
    <article class="card">
      <h3 class="card-heading">In its favour</h3>
      <p class="card-description">
        Messages are tiny, so an interface engine can move millions a day. The format is trivially
        streamable. And it is embedded in hundreds of thousands of production interfaces, most of
        which will never be rewritten.
      </p>
    </article>
    <article class="card">
      <h3 class="card-heading">Against it</h3>
      <p class="card-description">
        It is positional, so it is brittle and unreadable without tooling. There is no schema in the
        message itself. And the same field number means different things in different versions.
      </p>
    </article>
  </div>
  <div class="prose">
    <p>
      HL7 published an XML encoding in v2.3.1 and FHIR later, but neither displaced ER7 in the
      installed base.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Sources</h2>
  <ul class="prose">
    <li>
      <a href="https://www.hl7.eu/HL7v2x/v25/std25/ch02.html">HL7 v2.5 chapter 2, control</a> —
      encoding rules, delimiter table, escape sequences
    </li>
    <li>
      <a href="https://www.hl7.eu/HL7v2x/v28/std28/ch02.html">HL7 v2.8 chapter 2, control</a> — the
      same, with the truncation character
    </li>
    <li>
      <a href="http://v2plus.hl7.org/2021Jan/xml-encoding-rules.html">HL7 v2+ XML encoding syntax</a>
      — the alternative encoding ER7 is contrasted with
    </li>
    <li>
      <a href="https://caristix.com/help-center/v3/test/task/hl7-er7-encoding/"
        >Caristix: HL7 ER7 encoding</a
      >
    </li>
    <li>
      <a href="https://rhapsody.health/resources/hl7-escape-sequences/"
        >Rhapsody: HL7 escape sequences</a
      >
    </li>
  </ul>
</section>
