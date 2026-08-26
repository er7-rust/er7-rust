<script lang="ts">
  const levels = [
    { path: 'PID', names: 'every PID segment, whole', result: 'PID|1||9|4|SMITH^JOHN^Q' },
    { path: 'PID-5', names: 'field 5', result: 'SMITH^JOHN^Q' },
    { path: 'PID-5.1', names: 'component 1 of field 5', result: 'SMITH' },
    { path: 'PID-5.1.2', names: 'subcomponent 2 of that component', result: '— only one here' }
  ];

  const methods = [
    {
      method: 'query',
      takes: '&str',
      returns: 'Result<Option<String>, Error>',
      decoded: 'yes',
      note: 'the first match'
    },
    {
      method: 'query_all',
      takes: '&str',
      returns: 'Result<Vec<String>, Error>',
      decoded: 'yes',
      note: 'every match'
    },
    {
      method: 'query_path',
      takes: '&Path',
      returns: 'Vec<String>',
      decoded: 'yes',
      note: 'path parsed once'
    },
    {
      method: 'query_path_raw',
      takes: '&Path',
      returns: 'Vec<String>',
      decoded: 'no',
      note: 'exactly as sent'
    }
  ];
</script>

<svelte:head>
  <title>HL7® paths — er7</title>
  <meta
    name="description"
    content="The HL7 path notation used to name one place in a message: PID-5.1, OBX[2]-5, PID-13[2].1. Grammar, occurrence indices, and the four query methods."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Reference</p>
  <h1>HL7 paths</h1>
  <p class="hero-tagline">
    The short notation that names one place in a message: <code>PID-5.1</code>,
    <code>OBX[2]-5</code>, <code>PID-13[2].1</code>.
  </p>
</section>

<nav class="toc" aria-label="On this page">
  <h2>On this page</h2>
  <ul>
    <li><a href="#grammar">Grammar</a></li>
    <li><a href="#levels">The four levels</a></li>
    <li><a href="#occurrences">Occurrence indices</a></li>
    <li><a href="#repetitions">Repetitions</a></li>
    <li><a href="#spellings">Both spellings</a></li>
    <li><a href="#methods">The four query methods</a></li>
    <li><a href="#surprises">Two surprises</a></li>
  </ul>
</nav>

<section class="section">
  <div class="prose">
    <p>
      The notation is a de-facto standard among interface engineers rather than part of HL7 itself,
      so this crate accepts the two spellings that are common in the field and writes the first.
    </p>
  </div>

  <h2 id="grammar" class="anchor-heading section-heading">Grammar</h2>
  <pre><code>{`path       = name occurrence? ( ("-" | ".") index occurrence? ( "." index ( "." index )? )? )?
name       = one or more ASCII letters and digits
occurrence = "[" index "]"
index      = a decimal number, 1 or greater`}</code></pre>
  <div class="prose">
    <p>Surrounding whitespace is ignored.</p>
  </div>
</section>

<section class="section">
  <h2 id="levels" class="anchor-heading section-heading">The four levels</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Path</th>
          <th>Names</th>
          <th>On <code>PID|1||9|4|SMITH^JOHN^Q</code></th>
        </tr>
      </thead>
      <tbody>
        {#each levels as row (row.path)}
          <tr>
            <td><code>{row.path}</code></td>
            <td>{row.names}</td>
            <td><code>{row.result}</code></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      A path that stops above the leaf returns that subtree <strong>as written</strong>, with its
      structural delimiters intact. Only the leaf text is decoded.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="occurrences" class="anchor-heading section-heading">Occurrence indices</h2>
  <div class="prose">
    <p>There are two, and they mean different things.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Position</th>
          <th>Selects</th>
          <th>Example</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>after the <strong>segment name</strong></td>
          <td>which segment of that name</td>
          <td><code>OBX[2]-5</code> — the second OBX</td>
        </tr>
        <tr>
          <td>after the <strong>field number</strong></td>
          <td>which repetition of that field</td>
          <td><code>PID-13[2]</code> — the second phone number</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Both are 1-based. Both may be omitted, and omitting one means &ldquo;every one&rdquo;. They
      compose: <code>OBX[2]-5[1].1.2</code> is subcomponent 2 of component 1 of the first repetition
      of field 5 of the second <code>OBX</code>.
    </p>
  </div>
  <pre><code>{`// Three OBX segments, so three answers.
assert_eq!(message.query_all("OBX-5")?, ["187", "102", ""]);

// One, pinned down.
assert_eq!(message.query_all("OBX[2]-5")?, ["102"]);`}</code></pre>
  <div class="callout">
    <p>
      Every index is 1-based, and <code>0</code> is rejected with an error rather than clamped — it
      is almost always a caller&rsquo;s off-by-one, and silently reading it as <code>1</code> would
      return a plausible wrong answer.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="repetitions" class="anchor-heading section-heading">Repetitions have a special case</h2>
  <div class="prose">
    <p>
      A path that <strong>stops at the field</strong> returns the whole field, repetition separators
      included. A path that goes <strong>deeper</strong> splits into one answer per repetition.
    </p>
  </div>
  <pre><code>{`// PID-13 is \`555-1111~555-2222\`
assert_eq!(message.query("PID-13")?.as_deref(), Some("555-1111~555-2222"));
assert_eq!(message.query_all("PID-13.1")?, ["555-1111", "555-2222"]);
assert_eq!(message.query_all("PID-13[2].1")?, ["555-2222"]);`}</code></pre>
  <div class="prose">
    <p>
      This is deliberate: <code>PID-13</code> as a whole field is a meaningful thing to ask for, and
      joining its repetitions back with <code>~</code> is the only honest way to return it as one
      string.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="spellings" class="anchor-heading section-heading">Both spellings</h2>
  <div class="prose">
    <p>
      <code>PID-5.1</code> and <code>PID.5.1</code> parse identically. <code>Display</code> writes
      the first, and round-tripping through it preserves meaning, because occurrence indices the
      path left open are left out rather than defaulted to 1.
    </p>
  </div>
  <pre><code>{`let a: er7::Path = "PID.5.1".parse()?;
let b: er7::Path = "PID-5.1".parse()?;
assert_eq!(a, b);
assert_eq!(a.to_string(), "PID-5.1");`}</code></pre>
</section>

<section class="section">
  <h2 id="methods" class="anchor-heading section-heading">The four query methods</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Method</th>
          <th>Takes</th>
          <th>Returns</th>
          <th>Decoded</th>
          <th>Note</th>
        </tr>
      </thead>
      <tbody>
        {#each methods as row (row.method)}
          <tr>
            <td><code>{row.method}</code></td>
            <td><code>{row.takes}</code></td>
            <td><code>{row.returns}</code></td>
            <td>{row.decoded}</td>
            <td>{row.note}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      The <code>&amp;str</code> forms parse the path each call and can fail with a bad-path error.
      The <code>&amp;Path</code> forms take an already-parsed path, which is what you want when
      applying one path to many messages.
    </p>
  </div>
  <pre><code>{`let path: er7::Path = "PID-5.1".parse()?;
let names: Vec<String> = messages
    .iter()
    .flat_map(|message| message.query_path(&path))
    .collect();`}</code></pre>
  <div class="prose">
    <p>
      <code>Path</code> implements <code>Clone</code>, <code>PartialEq</code>, <code>Eq</code>, and
      <code>Hash</code>, so a set of paths can be map keys or deduplicated.
    </p>
  </div>
</section>

<section class="section">
  <h2 id="surprises" class="anchor-heading section-heading">Two surprises</h2>
  <div class="card-grid">
    <article class="card">
      <h3 class="card-heading">Header delimiters come back literally</h3>
      <p class="card-description">
        <code>MSH-1</code> is the field separator and <code>MSH-2</code> is the encoding characters.
        They are the delimiters, not values encoded with them, so they are never decoded:
        <code>MSH-1</code> gives <code>|</code> and <code>MSH-2</code> gives
        <code>^~\&amp;</code>.
      </p>
    </article>
    <article class="card">
      <h3 class="card-heading">A missing position contributes nothing</h3>
      <p class="card-description">
        Not an empty string — no entry at all. So
        <code>query_all(&quot;OBX-5&quot;).len()</code> counts the <code>OBX</code> segments that
        actually carried a fifth field, which is usually what you want and occasionally a surprise.
      </p>
    </article>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Reading paths off the command line</h2>
  <div class="prose">
    <p>
      The <code>er7</code> command&rsquo;s default output labels every value with the path that
      names it, and every one of those labels is a valid query.
    </p>
  </div>
  <pre><code>{`er7 message.er7 | grep Cholesterol
#=> OBX[1]-3.2  Cholesterol

er7 --query 'OBX[1]-3.2' message.er7
#=> Cholesterol`}</code></pre>
  <div class="callout callout-warning">
    <p>
      Quote the path in a shell: <code>[</code> and <code>]</code> are glob characters.
    </p>
  </div>
  <p><a href="/cli/">Full command-line reference &rarr;</a></p>
</section>
