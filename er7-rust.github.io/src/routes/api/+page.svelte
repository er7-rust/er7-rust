<script lang="ts">
  import { links } from '$lib/site';

  const entries = [
    { item: 'er7::parse', sig: 'fn(&str) -> Result<Message, Error>', note: 'needs an MSH/FHS/BHS header' },
    { item: 'er7::parse_with', sig: 'fn(&str, Separators) -> Message', note: 'headerless fragment; cannot fail' },
    { item: 'er7::split_messages', sig: 'fn(&str) -> Vec<&str>', note: 'batch input; slices borrow' },
    { item: 'er7::read_messages', sig: 'fn<R: BufRead>(R) -> MessageReader<R>', note: 'streamed one message at a time, without holding the whole input' }
  ];

  const messageMethods = [
    { m: 'segment(&str)', r: 'Option<&Segment>', p: 'the first segment with that name' },
    { m: 'segment_at(&str, usize)', r: 'Option<&Segment>', p: 'the 1-based Nth of that name' },
    { m: 'segment_at_mut(&str, usize)', r: 'Option<&mut Segment>', p: 'the same, mutable' },
    { m: 'segments_named(&str)', r: 'impl Iterator', p: 'every segment with that name' },
    { m: 'header()', r: 'Option<&Segment>', p: 'the segment that declared the delimiters' },
    { m: 'query(&str)', r: 'Result<Option<String>, Error>', p: 'first match at a path, decoded' },
    { m: 'query_all(&str)', r: 'Result<Vec<String>, Error>', p: 'every match, decoded' },
    { m: 'query_path(&Path)', r: 'Vec<String>', p: 'every match, path pre-parsed' },
    { m: 'query_path_raw(&Path)', r: 'Vec<String>', p: 'every match, exactly as sent' },
    { m: 'message_code()', r: 'Option<String>', p: 'MSH-9.1' },
    { m: 'trigger_event()', r: 'Option<String>', p: 'MSH-9.2' },
    { m: 'message_structure()', r: 'Option<String>', p: 'MSH-9.3' },
    { m: 'control_id()', r: 'Option<String>', p: 'MSH-10' },
    { m: 'version()', r: 'Option<String>', p: 'MSH-12.1' },
    { m: 'to_er7()', r: 'String', p: 'write with default options' },
    { m: 'to_er7_with(RenderOptions)', r: 'String', p: 'write, choosing the terminator' }
  ];

  const tree = `Message { separators: Separators, segments: Vec<Segment> }
Segment { name: String, fields: Vec<Field> }
Field { repetitions: Vec<Repetition> }
Repetition { components: Vec<Component> }
Component { subcomponents: Vec<Subcomponent> }
Subcomponent { raw: String }`;
</script>

<svelte:head>
  <title>API surface — er7</title>
  <meta
    name="description"
    content="The complete public API of the er7 crate: entry points, the six-level value tree, accessors, configuration, escape sequences, and errors."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Reference</p>
  <h1>API surface</h1>
  <p class="hero-tagline">
    The complete public API in one page. Rendered rustdoc with full signatures is at
    <a href={links.docs}>docs.rs/er7</a>.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Entry points</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Item</th>
          <th>Signature</th>
          <th>Notes</th>
        </tr>
      </thead>
      <tbody>
        {#each entries as row (row.item)}
          <tr>
            <td><code>{row.item}</code></td>
            <td><code>{row.sig}</code></td>
            <td>{row.note}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="callout">
    <p>
      <code>er7::parse</code> names both a module and a function. That is legal — they live in
      different namespaces — so <code>er7::parse(text)</code> calls the function and
      <code>er7::parse::split_messages</code> resolves the module path.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">The value tree</h2>
  <div class="prose">
    <p>
      Six types, one per level of the ER7 hierarchy. All fields are <code>pub</code>, so a message
      can be built from literals as well as parsed. All derive <code>Debug</code>,
      <code>Clone</code>, <code>PartialEq</code>, <code>Eq</code>; the four below
      <code>Segment</code> also derive <code>Default</code>.
    </p>
  </div>
  <pre><code>{tree}</code></pre>
  <div class="prose">
    <p>
      Only <code>Subcomponent</code> holds text, in <code>raw</code>, exactly as sent. Decoding
      happens on demand — that single decision is what makes the byte-for-byte round trip possible.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading"><code>Message</code></h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Method</th>
          <th>Returns</th>
          <th>Purpose</th>
        </tr>
      </thead>
      <tbody>
        {#each messageMethods as row (row.m)}
          <tr>
            <td><code>{row.m}</code></td>
            <td><code>{row.r}</code></td>
            <td>{row.p}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Also <code>impl Display for Message</code>, equivalent to <code>to_er7()</code>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Levels below <code>Message</code></h2>
  <div class="prose">
    <p>
      Every level exposes 1-based accessors matching HL7®&rsquo;s own numbering, each returning
      <code>Option</code>, plus a <code>_mut</code> variant. Index <code>0</code> returns
      <code>None</code> rather than element 1.
    </p>
  </div>
  <dl class="defs">
    <dt>Segment</dt>
    <dd>
      <code>field(n)</code>, <code>field_mut(n)</code>, <code>component(field, component)</code>,
      <code>first_value(field, component, &amp;seps)</code>, <code>is_header()</code>,
      <code>to_er7(&amp;seps)</code>, <code>to_text(&amp;seps)</code>
    </dd>
    <dt>Field</dt>
    <dd>
      <code>repetition(n)</code>, <code>repetition_mut(n)</code>, <code>component(n)</code>,
      <code>is_empty()</code>, <code>is_null()</code>, <code>to_er7</code>, <code>to_text</code>
    </dd>
    <dt>Repetition</dt>
    <dd>
      <code>component(n)</code>, <code>component_mut(n)</code>, <code>is_empty()</code>,
      <code>is_null()</code>, <code>to_er7</code>, <code>to_text</code>
    </dd>
    <dt>Component</dt>
    <dd>
      <code>subcomponent(n)</code>, <code>subcomponent_mut(n)</code>, <code>is_empty()</code>,
      <code>is_null()</code>, <code>to_er7</code>, <code>to_text</code>
    </dd>
    <dt>Subcomponent</dt>
    <dd>
      <code>new(text)</code>, <code>value(&amp;seps)</code>, <code>set(value, &amp;seps)</code>,
      <code>is_empty()</code>, <code>is_null()</code>, <code>to_er7</code>, <code>to_text</code>
    </dd>
  </dl>
  <div class="callout callout-warning">
    <p>
      <code>to_text</code> decodes leaf text but keeps the structural delimiters, so its output is
      <strong>not re-parseable</strong>: a decoded <code>\F\</code> becomes a literal <code>|</code>.
      Use <code>to_er7</code> for anything that will be sent, stored, or parsed again.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Configuration</h2>
  <dl class="defs">
    <dt>Separators</dt>
    <dd>
      Fields <code>field</code>, <code>component</code>, <code>repetition</code>,
      <code>escape</code>, <code>subcomponent</code>: <code>char</code>; <code>truncation</code>:
      <code>Option&lt;char&gt;</code>. Methods <code>default()</code>,
      <code>from_header(&amp;str)</code>, <code>validate()</code>, <code>is_delimiter(char)</code>,
      <code>encoding_characters()</code>. Derives <code>Copy</code>.
    </dd>
    <dt>Terminator</dt>
    <dd>
      <code>Cr</code> (default), <code>Lf</code>, <code>CrLf</code>. Method
      <code>as_str()</code>.
    </dd>
    <dt>RenderOptions</dt>
    <dd>
      <code>terminator: Terminator</code>, <code>trailing_terminator: bool</code>. Derives
      <code>Default</code> (<code>Cr</code>, <code>false</code>).
    </dd>
    <dt>Path</dt>
    <dd>
      <code>segment: String</code>; <code>segment_occurrence</code>, <code>field</code>,
      <code>repetition</code>, <code>component</code>, <code>subcomponent</code>:
      <code>Option&lt;usize&gt;</code>. Implements <code>FromStr</code>, <code>Display</code>,
      <code>Hash</code>.
    </dd>
  </dl>
</section>

<section class="section">
  <h2 class="section-heading">Escape sequences</h2>
  <div class="prose">
    <p>Module <code>er7::escape</code>.</p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Item</th>
          <th>Signature</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><code>escapes</code></td>
          <td><code>fn(&amp;str, &amp;Separators) -&gt; Escapes</code></td>
        </tr>
        <tr>
          <td><code>unescape</code></td>
          <td><code>fn(&amp;str, &amp;Separators) -&gt; Cow&lt;str&gt;</code></td>
        </tr>
        <tr>
          <td><code>escape</code></td>
          <td><code>fn(&amp;str, &amp;Separators) -&gt; Cow&lt;str&gt;</code></td>
        </tr>
        <tr>
          <td><code>decode_hex</code></td>
          <td><code>fn(&amp;str) -&gt; Option&lt;String&gt;</code></td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      <code>Escape</code> variants: <code>Text</code>, <code>Field</code>, <code>Component</code>,
      <code>Subcomponent</code>, <code>Repetition</code>, <code>EscapeCharacter</code>,
      <code>Hex</code>, <code>Highlight</code>, <code>Normal</code>, <code>Formatting</code>,
      <code>Local</code>, <code>SingleByteCharacterSet</code>, <code>MultiByteCharacterSet</code>,
      <code>Unknown</code>, <code>Unterminated</code>.
    </p>
  </div>
  <p><a href="/escapes/">The escape-sequence reference &rarr;</a></p>
</section>

<section class="section">
  <h2 class="section-heading">Errors</h2>
  <div class="prose">
    <p>
      Four variants, arising from exactly two situations: a message with no usable header, and a
      path that is not a path.
    </p>
  </div>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Variant</th>
          <th>When</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><code>Empty</code></td>
          <td>the input held no non-blank lines</td>
        </tr>
        <tr>
          <td><code>MissingHeader(String)</code></td>
          <td>the first segment is not MSH, FHS, or BHS; carries the name found</td>
        </tr>
        <tr>
          <td><code>BadHeader(String)</code></td>
          <td>the declared delimiters are unusable</td>
        </tr>
        <tr>
          <td><code>BadPath(String)</code></td>
          <td>a path could not be read</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Everything else about ER7 is recovered rather than refused: unknown segments, local Z
      segments, ragged field counts, undecodable escapes, and missing positions are all data.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Not in the API</h2>
  <div class="callout">
    <p>
      No dictionary, no validator, no transport, no dependencies. See the
      <a href="/ecosystem/">ecosystem</a> for the crates that add the HL7 v2.5 dictionary on top.
    </p>
  </div>
</section>
