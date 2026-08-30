<script lang="ts">
  import { links } from '$lib/site';

  // Measured 2026-08-26 on an Apple M4 Max, macOS 26.6.1, rustc 1.98.0,
  // aarch64-apple-darwin, release profile. Criterion, 100 samples each; the
  // figure is the median and the bracket is Criterion's confidence interval.
  const parsing = [
    { name: 'parse/small', time: '2.64 µs', range: '2.60 – 2.70', extra: '63.9 MiB/s · ≈ 378,000 messages/second' },
    { name: 'parse/large', time: '260.6 µs', range: '257.7 – 264.4', extra: '78.8 MiB/s · ≈ 3,800 messages/second · ≈ 648 ns/segment' }
  ];

  const writing = [
    { name: 'render/small', time: '369 ns', range: '365 – 375', extra: '≈ 2.7 million messages/second' },
    { name: 'render/large', time: '21.4 µs', range: '21.1 – 21.7', extra: '≈ 46,800 messages/second' },
    { name: 'render/large_crlf_trailing', time: '21.2 µs', range: '21.0 – 21.6', extra: 'A non-default terminator costs nothing measurable' }
  ];

  const escapes = [
    { name: 'escape/escape_plain', time: '10.8 ns', range: '10.6 – 11.0', extra: 'A value with nothing to escape — the common case' },
    { name: 'escape/escape_delimited', time: '110.9 ns', range: '109.1 – 113.0', extra: 'A value full of delimiters' },
    { name: 'escape/unescape_sequenced', time: '196.1 ns', range: '192.6 – 200.3', extra: 'A value full of escape sequences to decode' },
    { name: 'escape/tokenize_sequenced', time: '122.5 ns', range: '119.3 – 127.9', extra: 'Iterating the sequences without decoding' }
  ];

  const queries = [
    { name: 'query/subcomponent', time: '80.3 ns', range: '78.9 – 81.9', extra: 'PID-3.4.2' },
    { name: 'query/last_segment', time: '149.7 ns', range: '146.3 – 154.5', extra: 'NTE-3' },
    { name: 'query/field', time: '168.8 ns', range: '166.7 – 171.3', extra: 'PID-3' },
    { name: 'query/all_segments', time: '7.23 µs', range: '7.13 – 7.34', extra: 'query_all("OBX-5") — 200 matches, ≈ 36 ns each' }
  ];

  const groups = [
    { heading: 'Parsing', rows: parsing },
    { heading: 'Writing', rows: writing },
    { heading: 'Escape sequences', rows: escapes },
    { heading: 'Queries', rows: queries }
  ];
</script>

<svelte:head>
  <title>Benchmarks — er7</title>
  <meta
    name="description"
    content="Measured figures for parsing, writing, escaping, and querying HL7® v2 messages in ER7 — with the machine, the toolchain, the confidence intervals, and how not to read them."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Benchmarks</p>
  <h1>Benchmarks</h1>
  <p class="hero-tagline">
    Measured numbers for the operations that dominate any ER7 workload, plus the machine they came
    from and the caveats that make them meaningful.
  </p>
</section>

<section class="section">
  <div class="stat-row">
    <div class="stat">
      <span class="stat-value">260 µs</span>
      <span class="stat-label">to parse a 402-segment lab result</span>
    </div>
    <div class="stat">
      <span class="stat-value">12×</span>
      <span class="stat-label">cheaper to write a message than to parse it</span>
    </div>
    <div class="stat">
      <span class="stat-value">80 ns</span>
      <span class="stat-label">for one query, whatever the message length</span>
    </div>
    <div class="stat">
      <span class="stat-value">0</span>
      <span class="stat-label">dependencies linked into any of it</span>
    </div>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">The inputs</h2>
  <div class="prose">
    <p>
      Two synthetic messages — never real patient data — chosen to bracket what production traffic
      looks like.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Input</th><th>Shape</th><th>Size</th><th>Segments</th></tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>small</strong></td>
          <td>An <code>ADT^A08</code>: <code>MSH</code>, <code>EVN</code>, <code>PID</code>, <code>PV1</code></td>
          <td>177 bytes</td>
          <td>4</td>
        </tr>
        <tr>
          <td><strong>large</strong></td>
          <td>An <code>ORU^R01</code> with 200 <code>OBX</code> segments, each followed by an <code>NTE</code> carrying an escape sequence</td>
          <td>21,520 bytes</td>
          <td>402</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      The small message is the shape most interfaces move in bulk. The large one is the shape that
      decides whether a parser is fast enough for a day&rsquo;s traffic.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Results</h2>
  <div class="prose">
    <p>
      Measured <strong>2026-08-26</strong>. Apple M4 Max, macOS 26.6.1, <code>rustc 1.98.0</code>,
      <code>aarch64-apple-darwin</code>, release profile. Criterion, 100 samples per benchmark; the
      figure is the median and the bracket is Criterion&rsquo;s confidence interval.
    </p>
  </div>
  {#each groups as group (group.heading)}
    <h3>{group.heading}</h3>
    <div class="table-wrap">
      <table class="table">
        <thead>
          <tr><th>Benchmark</th><th>Time</th><th>Interval</th><th>Notes</th></tr>
        </thead>
        <tbody>
          {#each group.rows as row (row.name)}
            <tr>
              <td><code>{row.name}</code></td>
              <td>{row.time}</td>
              <td>{row.range}</td>
              <td>{row.extra}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/each}
  <div class="prose">
    <p>
      Three things are worth carrying away. Parsing gets <em>more</em> efficient per byte as
      messages grow, because per-message fixed costs amortise away. Writing is roughly twelve times
      cheaper than parsing, which matters for the common integration shape: parse once, edit, write
      many. And a single <code>query</code> returns on first match rather than walking the whole
      message, so looking up <code>PID-3.4.2</code> in a 402-segment message costs about what it
      costs in a four-segment one.
    </p>
    <p>
      The plain escape case is the one to watch: escaping a value that needs no escaping costs about
      ten nanoseconds and does not allocate, which is what keeps whole-message writing cheap.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Running them yourself</h2>
  <div class="prose">
    <p>
      The benchmarks live in <code>er7-bench/</code>, a workspace member that is
      <strong>not published</strong>. It exists so that <code>er7</code> itself can keep both
      <code>[dependencies]</code> and <code>[dev-dependencies]</code> empty — a rule its own test
      enforces — while Criterion lives one directory over, where it cannot reach the audit surface
      of the crate being measured.
    </p>
  </div>
  <pre><code>{`cargo bench -p er7-bench

# Record a baseline, change something, then compare against it.
cargo bench -p er7-bench -- --save-baseline before
cargo bench -p er7-bench -- --baseline before

# One group only.
cargo bench -p er7-bench -- parse`}</code></pre>
  <div class="prose">
    <p>Criterion writes an HTML report to <code>target/criterion/report/index.html</code>.</p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Optimisation history</h2>
  <div class="prose">
    <p>Changes made because a benchmark said so, rather than because the code looked slow.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Change</th><th>Effect</th></tr>
      </thead>
      <tbody>
        <tr>
          <td>
            <a href="{links.repository}/commit/821a7dc"><code>821a7dc</code></a>
            — stop <code>query</code> walking the whole message to return one value
          </td>
          <td>
            <code>query/field</code> &minus;85%, <code>query/subcomponent</code> &minus;92%. Single-value
            lookup went from proportional to message length to effectively constant.
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">How to read these numbers, and how not to</h2>
  <div class="callout callout-warning">
    <ol>
      <li>
        <strong>These are single-machine numbers on fast hardware.</strong> An M4 Max is not an
        interface engine in a hospital data centre. Treat the ratios as the durable finding, and the
        absolute figures as an upper bound.
      </li>
      <li>
        <strong>No comparison to another library is claimed.</strong> Benchmarking someone
        else&rsquo;s library fairly is hard, and benchmarking it unfairly is worse than not doing
        it. The <a href="/comparison/">comparison page</a> compares design and scope instead,
        without inventing numbers.
      </li>
      <li>
        <strong>Criterion&rsquo;s outlier counts are not noise to ignore.</strong> Runs on a laptop
        routinely report 5&ndash;9% high-severe outliers from scheduling. Compare against a saved
        baseline on the same machine rather than against a number on this page.
      </li>
      <li>
        <strong>Nothing here is a latency guarantee.</strong> These are library operations, not an
        end-to-end interface benchmark; a real feed spends most of its time in I/O, TLS, and the
        receiving system.
      </li>
    </ol>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Fuzzing</h2>
  <div class="prose">
    <p>
      Performance work is only safe next to correctness work. The crate carries
      <code>cargo-fuzz</code> targets alongside these benchmarks — <code>parse_roundtrip</code>,
      <code>parse_with_total</code>, <code>escape_roundtrip</code>, and <code>query_paths</code> —
      and each asserts a property the specification states rather than merely checking for panics.
    </p>
  </div>
  <pre><code>{`cargo +nightly fuzz run parse_roundtrip -- -max_total_time=60`}</code></pre>
  <div class="prose">
    <p>
      The full write-up, with the method and the raw Criterion output, is
      <a href="{links.repository}/blob/main/BENCHMARKS.md"><code>BENCHMARKS.md</code></a> in the
      repository.
    </p>
  </div>
</section>
