<script lang="ts">
  import { crates } from '$lib/site';

  const crate = crates.find((c) => c.name === 'serde-er7')!;

  const shapes = [
    { level: 'Message', shape: 'object, fields "separators" and "segments"', example: '{"separators": {…}, "segments": […]}' },
    { level: 'Segment', shape: 'object, fields "name" and "fields"', example: '{"name": "PID", "fields": […]}' },
    { level: 'Field', shape: 'array of repetitions', example: '[["555-1111"], ["555-2222"]]' },
    { level: 'Repetition', shape: 'array of components', example: '[["SMITH"], ["JOHN"]]' },
    { level: 'Component', shape: 'array of subcomponent strings', example: '["ACME", "1.2.3", "ISO"]' },
    { level: 'Subcomponent', shape: 'a bare string, exactly as sent', example: '"SMITH"' },
    { level: 'Separators', shape: 'object, six named character fields', example: '{"field": "|", "component": "^", …}' },
    { level: 'Terminator', shape: 'one of three strings', example: '"Cr", "Lf", "CrLf"' }
  ];

  const examples = [
    { name: 'round_trip_via_json', shows: 'Parse ER7, serialize to JSON, deserialize, and write the original text back out.' },
    { name: 'build_message_from_json', shows: 'The direction most tutorials skip: JSON in, ER7 text out.' },
    { name: 'inspect_a_segment_as_json', shows: 'Serializing one segment, to see the shape each level chooses for itself.' }
  ];
</script>

<svelte:head>
  <title>serde-er7 — Serde support for the ER7 message tree</title>
  <meta
    name="description"
    content="Tutorial, help, and examples for serde-er7: Serialize and Deserialize implementations for every er7 type, so an HL7® v2 message tree can flow through JSON, YAML, or any Serde format."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Subproject &middot; Tool</p>
  <h1>serde-er7</h1>
  <p class="hero-tagline">
    Serde support for the ER7 message tree — so a parsed message can flow through JSON, YAML, or any
    other Serde format, and come back out unchanged.
  </p>
  <div class="button-row">
    <a class="button button-primary" href="#tutorial">Tutorial</a>
    <a class="button button-secondary" href={crate.docs}>API documentation</a>
    <a class="button button-secondary" href={crate.repository}>Source</a>
  </div>
</section>

<nav class="toc" aria-label="On this page">
  <a href="#what-it-is">What it is</a>
  <a href="#install">Install</a>
  <a href="#tutorial">Tutorial</a>
  <a href="#help">Help</a>
  <a href="#examples">Examples</a>
  <a href="#not">What it does not do</a>
</nav>

<section class="section" id="what-it-is">
  <h2 class="section-heading">What it is</h2>
  <div class="prose">
    <p>
      Once a message is an <code>er7::Message</code>, you often want to hand it to something that
      only speaks Serde: a document database driver, a web framework's JSON response type, a
      structured logger, a snapshot-testing library. <code>serde-er7</code> gives every
      <code>er7</code> type a hand-written <code>Serialize</code> and <code>Deserialize</code>
      implementation, and that is the whole feature surface.
    </p>
    <p>
      It is a separate crate because <code>er7</code> guarantees
      <a href="/about/">zero dependencies</a> — adding Serde there would impose it on every
      consumer, including the ones that never touch a data format. Here the choice is the
      caller's: two dependencies, <code>serde</code> and <code>er7</code>, and nothing else.
    </p>
  </div>
</section>

<section class="section" id="install">
  <h2 class="section-heading">Install</h2>
  <pre><code>{`cargo add serde-er7`}</code></pre>
  <div class="prose">
    <p>
      Plus whichever Serde format you actually use — <code>serde_json</code>, <code>serde_yaml</code
      >, a binary format. This crate never names one: it is format-agnostic on purpose, and JSON
      appears below only because it is the easiest to read on a page.
    </p>
  </div>
</section>

<section class="section" id="tutorial">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Tutorial</span>
    A message, out to JSON and back
  </h2>

  <h3>1. Parse into the wrapper type</h3>
  <pre><code>{`use serde_er7::Message;

let text = "MSH|^~\\\\&|LAB|ACME\\r\\
            PID|1||444333222^^^ACME^MR||EVERYWOMAN^EVE^E";

let message = Message::parse(text)?;`}</code></pre>
  <div class="prose">
    <p>
      <code>Message::parse</code> is a thin wrapper over <code>er7::parse</code> and returns the
      same <code>er7::Error</code>, so everything you already know about handling it applies
      unchanged.
    </p>
  </div>

  <h3>2. Serialize it</h3>
  <pre><code>{`let json = serde_json::to_string_pretty(&message)?;`}</code></pre>
  <pre><code>{`{
  "separators": {
    "field": "|",
    "component": "^",
    "repetition": "~",
    "escape": "\\\\",
    "subcomponent": "&",
    "truncation": null
  },
  "segments": [
    { "name": "MSH", "fields": [[[["|"]]], [[["^~\\\\&"]]], [[["LAB"]]], [[["ACME"]]]] },
    {
      "name": "PID",
      "fields": [
        [[["1"]]],
        [],
        [[["444333222"], [""], [""], ["ACME"], ["MR"]]],
        [],
        [[["EVERYWOMAN"], ["EVE"], ["E"]]]
      ]
    }
  ]
}`}</code></pre>
  <div class="prose">
    <p>
      Positions, not names. One array level per level of the ER7 tree — field, repetition,
      component, subcomponent — so <code>[[["1"]]]</code> is a field holding one repetition, one
      component, one subcomponent. <code>PID-2</code> is <code>[]</code> because it was sent empty,
      which is how &ldquo;no value&rdquo; stays distinct from a value that is blank.
    </p>
  </div>

  <h3>3. Deserialize, and write ER7 back out</h3>
  <pre><code>{`let back: Message = serde_json::from_str(&json)?;

assert_eq!(back.to_er7(), text);   // byte for byte`}</code></pre>
  <div class="prose">
    <p>
      The round trip holds because a subcomponent serializes its <strong>raw</strong> text, exactly
      as the sender wrote it, escape sequences included — never the decoded form. Decoding is lossy:
      a formatting escape such as <code>\.br\</code> has no plain-text form to decode back to, so a
      wire format built on decoded values could not be turned back into the original ER7.
    </p>
    <p>
      Note the <code>Deref</code>: <code>back.to_er7()</code> and
      <code>back.query("PID-5.1")</code> are <code>er7</code>'s own methods, reached straight
      through the wrapper. There is no unwrapping step, and no parallel API to learn.
    </p>
  </div>

  <h3>4. Or start from JSON</h3>
  <div class="prose">
    <p>
      The direction most tutorials skip: you have JSON — from a web form, a document store, a test
      fixture — and a legacy receiver that needs ER7.
    </p>
  </div>
  <pre><code>{`let message: serde_er7::Message = serde_json::from_str(json)?;

assert_eq!(message.query("PID-5.1")?.as_deref(), Some("SMITH"));
println!("{}", message.to_er7());`}</code></pre>
</section>

<section class="section" id="help">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Help</span>
    Reference
  </h2>

  <h3>The shape each level serializes as</h3>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Level</th>
          <th>Shape</th>
          <th>Example</th>
        </tr>
      </thead>
      <tbody>
        {#each shapes as shape (shape.level)}
          <tr>
            <td><code>{shape.level}</code></td>
            <td>{shape.shape}</td>
            <td><code>{shape.example}</code></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      This table is a <strong>compatibility surface</strong>: a change to any shape in it is a
      breaking change, because somebody's stored documents are written that way.
    </p>
    <p>Three choices in it are worth the explanation:</p>
    <ul>
      <li>
        <strong>Bare arrays below a segment.</strong> A field, a repetition, and a component each
        hold exactly one thing — the list of the level below — so an object wrapper would add a key
        carrying no information, at every one of the many nodes at that level.
      </li>
      <li>
        <strong>Objects for message and segment.</strong> Each carries two different kinds of
        information, which an array position alone would not distinguish without an undocumented
        convention about which index means what.
      </li>
      <li>
        <strong>Delimiters as six named characters</strong>, not the packed <code>^~\&amp;</code>
        string. A reader can identify each one by name, and every format's own <code>char</code>
        handling stays in play.
      </li>
    </ul>
  </div>

  <h3>Deserializing</h3>
  <div class="prose">
    <p>
      An unknown field in an object is <strong>ignored</strong> rather than rejected, so a document
      that carries extra keys still loads. A missing required field is an error naming that field.
      Anything that parses is accepted — the same posture <code>er7</code> takes below its header.
    </p>
  </div>
</section>

<section class="section" id="examples">
  <h2 class="section-heading">Examples</h2>
  <div class="prose">
    <p>
      Three runnable programs in the crate's <a href={`${crate.repository}/examples`}
        ><code>examples/</code></a
      > directory, each asserting its own results.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Example</th>
          <th>Shows</th>
        </tr>
      </thead>
      <tbody>
        {#each examples as example (example.name)}
          <tr>
            <td><code>{example.name}</code></td>
            <td>{example.shows}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <pre><code>{`cargo run --example round_trip_via_json`}</code></pre>
</section>

<section class="section" id="not">
  <h2 class="section-heading">What it does not do</h2>
  <div class="prose">
    <ul>
      <li>
        <strong>It is not a dictionary.</strong> The JSON above is positional, exactly like the ER7
        it came from. For <code>"PID.5": &#123;"XPN.1": …&#125;</code> — names rather than indices —
        you want a dictionary crate; see the <a href="/ecosystem/">ecosystem</a>, where the two
        outputs sit side by side.
      </li>
      <li>
        <strong>It names no format.</strong> JSON, YAML, and the binary formats are all just Serde
        implementations; none of them appears in this crate's dependencies or public API.
      </li>
      <li>
        <strong>It is not a validator.</strong> Structurally malformed Serde input is rejected by
        <code>Deserialize</code>; anything that parses is accepted.
      </li>
      <li>
        <strong>It is not a parallel API.</strong> Every wrapper <code>Deref</code>s to its
        <code>er7</code> type, so this crate adds a capability rather than a second surface.
      </li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Where to go next</h2>
  <div class="tag-list">
    <a class="tag" href={crate.repository}>Source</a>
    <a class="tag" href={crate.crate}>Crate</a>
    <a class="tag" href={crate.docs}>API documentation</a>
    <a class="tag" href={`${crate.repository}/spec`}>Specification</a>
    <a class="tag" href="/api/">er7 API</a>
    <a class="tag" href="/ecosystem/">Ecosystem</a>
  </div>
</section>
