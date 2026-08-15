<script lang="ts">
  type Row = { sequence: string; meaning: string; decoded: boolean };

  const sequences: Row[] = [
    { sequence: '\\F\\', meaning: 'the field separator as data', decoded: true },
    { sequence: '\\S\\', meaning: 'the component separator as data', decoded: true },
    { sequence: '\\T\\', meaning: 'the subcomponent separator as data', decoded: true },
    { sequence: '\\R\\', meaning: 'the repetition separator as data', decoded: true },
    { sequence: '\\E\\', meaning: 'the escape character as data', decoded: true },
    {
      sequence: '\\Xdd..\\',
      meaning: 'hexadecimal data; pairs of digits, each pair one byte',
      decoded: true
    },
    { sequence: '\\H\\', meaning: 'start highlighting', decoded: false },
    { sequence: '\\N\\', meaning: 'normal text, ending highlighting', decoded: false },
    { sequence: '\\Zdd..\\', meaning: 'locally defined, agreed between the two ends', decoded: false },
    { sequence: '\\Cxxyy\\', meaning: 'switch to a single-byte character set', decoded: false },
    { sequence: '\\Mxxyyzz\\', meaning: 'switch to a multi-byte character set', decoded: false },
    { sequence: '\\.br\\, \\.sp 2\\, …', meaning: 'formatted-text display commands', decoded: false }
  ];

  const commands = [
    { command: '.sp <n>', meaning: 'end the line and skip n vertical spaces' },
    { command: '.br', meaning: 'begin a new output line' },
    { command: '.fi', meaning: 'begin word wrap (the default)' },
    { command: '.nf', meaning: 'begin no-wrap' },
    { command: '.in <n>', meaning: 'indent by n spaces' },
    { command: '.ti <n>', meaning: 'temporarily indent n spaces' },
    { command: '.sk <n>', meaning: 'skip n spaces to the right' },
    { command: '.ce', meaning: 'centre the next line' }
  ];
</script>

<svelte:head>
  <title>Escape sequences — er7</title>
  <meta
    name="description"
    content="How an ER7 value carries characters that would otherwise be read as structure — and which sequences this crate deliberately leaves alone."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Reference</p>
  <h1>Escape sequences</h1>
  <p class="hero-tagline">
    How a value carries characters that would otherwise be read as structure — and, just as
    importantly, which sequences are deliberately left alone.
  </p>
</section>

<nav class="toc" aria-label="On this page">
  <h2>On this page</h2>
  <ul>
    <li><a href="#shape">The shape</a></li>
    <li><a href="#table">The full table</a></li>
    <li><a href="#why-literal">Why half are kept literally</a></li>
    <li><a href="#decoding">Decoding</a></li>
    <li><a href="#encoding">Encoding</a></li>
    <li><a href="#tokenizer">The tokenizer</a></li>
    <li><a href="#divergence">One known divergence</a></li>
  </ul>
</nav>

<section class="section">
  <h2 id="shape" class="anchor-heading section-heading">The shape</h2>
  <div class="prose">
    <p>
      A sequence is the message&rsquo;s escape character, a body, and the escape character again.
    </p>
  </div>
  <pre><code>{`\\F\\        \\X0D\\        \\.br\\        \\Z0102\\
^ ^        ^    ^
| body     | body
escape     escape`}</code></pre>
  <div class="prose">
    <p>
      The escape character is whatever MSH-2 position 3 declared — conventionally <code>\</code>,
      but a message using <code>?</code> writes <code>?F?</code>. Every function takes the
      message&rsquo;s delimiter set for exactly that reason.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="table" class="anchor-heading section-heading">The full table</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Sequence</th>
          <th>Meaning</th>
          <th>Decoded?</th>
        </tr>
      </thead>
      <tbody>
        {#each sequences as row (row.sequence)}
          <tr>
            <td><code>{row.sequence}</code></td>
            <td>{row.meaning}</td>
            <td>{row.decoded ? 'yes' : 'no — kept literally'}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <h3>Display commands</h3>
  <div class="prose">
    <p>Used inside formatted-text (<code>FT</code>) fields.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Command</th>
          <th>Meaning</th>
        </tr>
      </thead>
      <tbody>
        {#each commands as row (row.command)}
          <tr>
            <td><code>{row.command}</code></td>
            <td>{row.meaning}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 id="why-literal" class="anchor-heading section-heading">Why half are kept literally</h2>
  <div class="prose">
    <p>
      The sequences marked &ldquo;no&rdquo; say something about <strong>presentation or encoding</strong>
      that a plain string cannot carry. There are three options and only one of them is honest.
    </p>
  </div>
  <div class="card-grid">
    <article class="card">
      <h3 class="card-heading">Drop them</h3>
      <p class="card-description">Loses information the sender sent.</p>
    </article>
    <article class="card">
      <h3 class="card-heading">Guess at them</h3>
      <p class="card-description">Invents information the sender did not send.</p>
    </article>
    <article class="card">
      <h3 class="card-heading">Keep them as written</h3>
      <p class="card-description">
        The caller sees exactly what arrived. A caller who understands display commands can act on
        them; a caller who does not sees text that is at worst ugly rather than wrong.
      </p>
      <p class="card-meta">What the crate does</p>
    </article>
  </div>
</section>

<section class="section">
  <h2 id="decoding" class="anchor-heading section-heading">Decoding</h2>
  <pre><code>{`use er7::{Separators, escape::unescape};

let separators = Separators::default();

// Sequences that stand for characters decode.
assert_eq!(unescape(r"Smith \\T\\ Jones", &separators), "Smith & Jones");
assert_eq!(unescape(r"a\\F\\b", &separators), "a|b");
assert_eq!(unescape(r"\\X0D\\", &separators), "\\r");
assert_eq!(unescape(r"\\X4142\\", &separators), "AB");

// Everything else is kept exactly as written, and not damaged.
assert_eq!(unescape(r"line\\.br\\next", &separators), r"line\\.br\\next");
assert_eq!(unescape(r"\\H\\loud\\N\\", &separators), r"\\H\\loud\\N\\");
assert_eq!(unescape(r"a\\Fb", &separators), r"a\\Fb");   // unterminated`}</code></pre>
  <div class="prose">
    <p>
      <code>\X..\</code> decodes only when its body is whole pairs of ASCII hexadecimal digits;
      <code>\XZZ\</code> and <code>\X123\</code> are kept literally, because a body that is not hex
      was not hex data. Bytes are read as UTF-8 with the usual lossy replacement, which is the best a
      receiver can do for a sender that meant some other repertoire.
    </p>
    <p>
      <code>unescape</code> returns a borrowed string when the text contains no escape character at
      all — the overwhelmingly common case, and free.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="encoding" class="anchor-heading section-heading">Encoding</h2>
  <pre><code>{`use er7::escape::escape;

assert_eq!(escape("Smith & Jones", &separators), r"Smith \\T\\ Jones");
assert_eq!(escape("a|b^c~d&e", &separators), r"a\\F\\b\\S\\c\\R\\d\\T\\e");
assert_eq!(escape(r"a\\b", &separators), r"a\\E\\b");
assert_eq!(escape("line\\r\\nnext", &separators), r"line\\X0D\\\\X0A\\next");`}</code></pre>
  <div class="prose">
    <p>Two details worth knowing:</p>
    <ul>
      <li>
        <strong>The escape character is encoded first</strong>, so a value containing it is encoded
        once, not twice.
      </li>
      <li>
        <strong><code>\r</code> and <code>\n</code> become <code>\X0D\</code> and
          <code>\X0A\</code>.</strong> A literal carriage return in a value would end the segment and
        truncate the message — the one corruption an ER7 writer must never commit.
      </li>
    </ul>
    <p>
      The truncation character (<code>#</code>, HL7 v2.7) is <em>not</em> encoded: it is structural
      only inside MSH-2, so a <code>#</code> in a value is just a <code>#</code>.
    </p>
  </div>
  <div class="callout callout-warning">
    <p>
      <strong>Always encode when you write.</strong> <code>Subcomponent::set</code> does it for you.
      Assigning the raw text directly with an unescaped <code>&amp;</code> silently splits the
      component the next time the message is parsed, shifting every value after it.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="tokenizer" class="anchor-heading section-heading">The tokenizer</h2>
  <div class="prose">
    <p>
      <code>escapes</code> is the layer both functions above are built on. It turns text into a
      stream of classified tokens, which is what you want when you need more than &ldquo;decode or
      don&rsquo;t&rdquo;.
    </p>
  </div>
  <pre><code>{`use er7::escape::{escapes, Escape};

let tokens: Vec<_> = escapes(r"Dr\\S\\Who\\.br\\", &separators).collect();

assert_eq!(tokens, vec![
    Escape::Text("Dr"),
    Escape::Component,
    Escape::Text("Who"),
    Escape::Formatting("br"),
]);`}</code></pre>
  <div class="prose">
    <p>Two properties make it usable as a foundation:</p>
    <ul>
      <li>
        <strong>It never fails.</strong> Text that does not form a valid sequence comes back as
        <code>Unknown</code> or <code>Unterminated</code>, not an error.
      </li>
      <li>
        <strong>It is lossless.</strong> Writing every token back reproduces the input exactly.
      </li>
    </ul>
    <p>
      The crate will not render <code>\.br\</code> for you — that is a presentation concern and out
      of scope — but the token stream gives you everything needed to do it yourself.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="divergence" class="anchor-heading section-heading">One known divergence</h2>
  <div class="callout">
    <p>
      HL7 scopes escaping to <code>ST</code>, <code>TX</code>, and <code>FT</code> fields and to the
      fourth component of <code>ED</code>. This crate decodes sequences <strong>wherever they
        appear</strong>, because applying the standard&rsquo;s scope requires knowing each
      field&rsquo;s data type, and that requires a dictionary this crate deliberately does not have.
    </p>
    <p>
      The risk is a false positive on a value that legitimately contains a backslash in a field where
      escaping does not apply. The mitigations: unrecognized sequences stay literal, so such a value
      usually round-trips unchanged anyway; and the raw text is always available, so a caller who
      knows the data type can override.
    </p>
  </div>
</section>
