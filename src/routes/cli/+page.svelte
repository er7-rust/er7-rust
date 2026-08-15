<script lang="ts">
  const options = [
    { flag: '-q, --query <PATH>', effect: 'Print the values at an HL7 path, one per line. May be repeated; outputs appear in the order the options were given.' },
    { flag: '-n, --normalize', effect: 'Rewrite the input as canonical ER7, with a trailing terminator on every message.' },
    { flag: '-m, --message <N>', effect: 'Use only the Nth message of the input, counting from 1.' },
    { flag: '-r, --raw', effect: 'Show text as sent, without decoding escape sequences.' },
    { flag: '-t, --terminator <KIND>', effect: 'Segment terminator to write: cr (default), lf, crlf.' },
    { flag: '-o, --output <FILE>', effect: 'Write to FILE instead of standard output.' },
    { flag: '-h, --help', effect: 'Print usage.' },
    { flag: '-V, --version', effect: 'Print the version.' }
  ];

  const outline = `MSH-1       |
MSH-2       ^~\\&
MSH-9.1     ORU
PID-3.4.2   1.2.840.114398.1.100
PID-5.1     EVERYWOMAN
PID-13[1]   555-555-1111
PID-13[2]   555-555-2222
OBX[1]-3.2  Cholesterol
OBX[2]-3.2  Triglycerides`;
</script>

<svelte:head>
  <title>Command line — er7</title>
  <meta
    name="description"
    content="The er7 command-line tool: outline every value with its HL7 path, query by path, or rewrite a message as canonical ER7."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Reference</p>
  <h1>Command line</h1>
  <p class="hero-tagline">
    Read a pipe-hat message without counting pipes. Install with
    <code>cargo install er7</code>.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Synopsis</h2>
  <pre><code>er7 [OPTIONS] [FILE]</code></pre>
  <div class="prose">
    <p>
      <code>FILE</code> holds one or more messages, or a batch file. <code>-</code> or no argument
      reads standard input. Input is split into messages, and <strong>every message is parsed before
        anything is written</strong>, so a malformed message late in a batch fails the run rather
      than producing half an output.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Option</th>
          <th>Effect</th>
        </tr>
      </thead>
      <tbody>
        {#each options as row (row.flag)}
          <tr>
            <td><code>{row.flag}</code></td>
            <td>{row.effect}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Combining <code>--query</code> with <code>--normalize</code> is an error: they ask for
      different output, and silently preferring one would hide a mistake in a script.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">The default action</span>
    The outline
  </h2>
  <div class="prose">
    <p>
      The reason the command exists: it answers &ldquo;which position is this value actually
      in?&rdquo;, which is the hardest thing about a positional format.
    </p>
  </div>
  <pre><code>{outline}</code></pre>
  <div class="prose">
    <ul>
      <li>
        <strong>Every label is a valid <code>--query</code> argument.</strong> This is the point of
        the format: a path read off the outline can be pasted straight back in.
      </li>
      <li>
        A level with <strong>only one child is not indexed</strong>, so a name sent as a single
        component reads <code>NTE-3</code>, not <code>NTE-3.1</code>. Indices appear exactly where
        they disambiguate.
      </li>
      <li>
        Repeated <strong>segments</strong> are labelled <code>OBX[2]-…</code>; repeated
        <strong>fields</strong> are labelled <code>PID-13[2]</code>.
      </li>
      <li>Positions with <strong>no value are left out</strong> entirely.</li>
      <li>
        An <strong>explicit null is shown as the <code>&quot;&quot;</code></strong> it was sent as,
        since that is exactly what distinguishes it from a field left out.
      </li>
      <li>
        Carriage returns, line feeds, and tabs inside a decoded value are shown as
        <code>\r</code>, <code>\n</code>, and <code>\t</code>, so one value stays on one line.
      </li>
    </ul>
    <p>
      When the input holds more than one message, each outline is preceded by a
      <code># message N</code> heading naming the message code, trigger event, control ID, and
      version where the message supplies them.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Recipes</h2>

  <h3>Inspect an unfamiliar message</h3>
  <pre><code>{`er7 suspect.er7 | less
er7 suspect.er7 | grep '^PID'
er7 --raw suspect.er7 | grep '\\\\'    # find every escape sequence`}</code></pre>

  <h3>Extract values for a script</h3>
  <pre><code>{`# One value
er7 --query PID-5.1 message.er7

# Several paths, in the order given
er7 -q PID-5.1 -q PID-7 -q OBX-5 message.er7

# Exactly as sent, escapes intact
er7 --raw --query OBX-5 message.er7`}</code></pre>

  <h3>Make a message readable</h3>
  <pre><code>{`# ER7 ends segments with a carriage return, so a terminal draws them
# on top of each other. Rewrite with line feeds instead.
er7 --normalize --terminator lf message.er7`}</code></pre>

  <h3>Work through a batch file</h3>
  <pre><code>{`# Outline every message, with headings
er7 batch.er7

# Just the second one
er7 --message 2 batch.er7

# The control ID of every message
er7 --query MSH-10 batch.er7`}</code></pre>

  <h3>From a pipe</h3>
  <pre><code>{`cat message.er7 | er7
er7 - < message.er7
er7 message.er7 --output outline.txt`}</code></pre>
</section>

<section class="section">
  <h2 class="section-heading">Exit codes and diagnostics</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Code</th>
          <th>Meaning</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><code>0</code></td>
          <td>success</td>
        </tr>
        <tr>
          <td><code>1</code></td>
          <td>
            any error: bad arguments, unreadable input, a message that failed to parse, or an
            unwritable output
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Diagnostics go to standard error, prefixed <code>er7: error: </code>, one line. A message that
      failed to parse is identified by its 1-based position in the input.
    </p>
  </div>
  <pre><code>er7: error: message 3: input contains no HL7 segments</code></pre>
  <div class="callout">
    <p>
      <strong>A query that matches nothing prints nothing and still exits 0.</strong> The message
      simply did not carry that value, which is not a failure. Scripts that need to distinguish
      should test for empty output.
    </p>
    <p>
      A closed output pipe (<code>er7 … | head -3</code>) also exits 0, which is what every other
      Unix filter does.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Stability</h2>
  <div class="prose">
    <p>
      The command line is covered by the same semantic-versioning promise as the library: removing
      an option, changing an exit code, or changing the outline&rsquo;s label format is a breaking
      change. Adding an option is not.
    </p>
  </div>
</section>
