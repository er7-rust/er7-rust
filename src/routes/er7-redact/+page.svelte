<script lang="ts">
  import { crates } from '$lib/site';

  const crate = crates.find((c) => c.name === 'er7-redact')!;

  const actions = [
    { written: 'keep', becomes: 'PATID1234', use: 'exempting a position from a fallback' },
    { written: 'clear', becomes: '', use: 'an address, a phone number — anything a placeholder would not help' },
    { written: 'null', becomes: '""', use: 'telling a receiver to clear its stored value' },
    { written: 'replace REDACTED', becomes: 'REDACTED', use: 'a name, where a placeholder reads better than a blank' },
    { written: 'mask *', becomes: '*********', use: 'a value whose length is wanted and whose content is not' },
    { written: 'first 4', becomes: 'PATI', use: 'a birth date reduced to its year' },
    { written: 'last 4', becomes: '1234', use: 'an account number reduced to the digits a human matches on' },
    { written: 'pseudonym', becomes: '73f419a72093eda0', use: 'an identifier that must stay linkable across messages' }
  ];

  const options = [
    { flag: '-p, --policy <FILE>', effect: 'Read rules from a policy file; may be repeated' },
    { flag: '-r, --rule <RULE>', effect: 'Add one rule, e.g. "PID-5 replace REDACTED"; may be repeated' },
    { flag: '-a, --all', effect: 'Redact everything except the MSH header' },
    { flag: '-k, --key <KEY>', effect: 'Pseudonym key, a number; default 0' },
    { flag: '-m, --message <N>', effect: 'Use only the Nth message of the input' },
    { flag: '-t, --terminator <KIND>', effect: 'Segment terminator to write: cr (default), lf, crlf' },
    { flag: '-o, --output <FILE>', effect: 'Write to FILE instead of standard output' },
    { flag: '--report', effect: 'Write what would change, and not the message' },
    { flag: '--show-policy', effect: 'Write the policy that would be applied, and exit' }
  ];

  const examples = [
    { name: 'redact_a_message', shows: 'The built-in policy end to end: what goes, what stays, and why the message still works.' },
    { name: 'write_a_policy', shows: 'Three ways to say what to redact — in Rust, from a file, or by extending a built-in.' },
    { name: 'redact_all_but', shows: 'A fallback over everything, with keep rules for what a test needs.' },
    { name: 'pseudonyms_and_linkage', shows: 'Why an identifier becomes a pseudonym rather than a blank, and what that costs.' },
    { name: 'read_the_report', shows: 'The audit trail: one row per position changed, and no values in it.' },
    { name: 'absent_empty_null', shows: 'The three states HL7 keeps apart, and why redaction leaves two of them alone.' }
  ];
</script>

<svelte:head>
  <title>er7-redact — remove patient detail without breaking the message</title>
  <meta
    name="description"
    content="Tutorial, help, and examples for er7-redact: redact HL7 v2 messages in the ER7 encoding with a policy of paths and actions, stable pseudonyms, and a report of every position changed."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Subproject &middot; Tool</p>
  <h1>er7-redact</h1>
  <p class="hero-tagline">
    Remove patient detail from a message — without moving its shape, so everything downstream still
    behaves the way it did on the original.
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
      A bug report arrives with a real message attached, and somebody has to turn it into something
      that can be committed to a repository, pasted into a ticket, or fed to a test suite.
      <code>er7-redact</code> does that: a <strong>policy</strong> of HL7 paths and actions, applied
      to a message, plus a report of every position it changed.
    </p>
    <p>
      The design constraint is that the redacted message has to survive everything that was going to
      be done to the original. So redaction rewrites <strong>leaf text only</strong>: the same
      segments, fields, repetitions, components, and subcomponents come back out, and every path
      that resolved to a value still resolves to one.
    </p>
  </div>

  <div class="pair-grid">
    <figure>
      <figcaption>In</figcaption>
      <pre><code>{`PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F`}</code></pre>
    </figure>
    <figure>
      <figcaption>Out</figcaption>
      <pre><code>{`PID|1||73f419a72093eda0^^^ACME&1.2.840.114398.1.100&ISO^MR||REDACTED^REDACTED^REDACTED||1962|F`}</code></pre>
    </figure>
  </div>
  <p class="er7-caption">
    The record number is a stable pseudonym, the name is a placeholder, the birth date is a year —
    and the assigning authority, the identifier type, and the sex are untouched, because none of
    them identifies the patient.
  </p>
</section>

<section class="section" id="install">
  <h2 class="section-heading">Install</h2>
  <div class="pair-grid">
    <figure>
      <figcaption>Library</figcaption>
      <pre><code>{`cargo add er7-redact`}</code></pre>
    </figure>
    <figure>
      <figcaption>Command line</figcaption>
      <pre><code>{`cargo install er7-redact`}</code></pre>
    </figure>
  </div>
</section>

<section class="section" id="tutorial">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Tutorial</span>
    From a message you cannot share to one you can
  </h2>

  <h3>1. Redact with the built-in policy</h3>
  <pre><code>{`er7-redact message.er7 > redacted.er7`}</code></pre>
  <div class="prose">
    <p>Or, in Rust:</p>
  </div>
  <pre><code>{`use er7_redact::{Policy, Redactor};

let mut message = er7::parse(text)?;
let report = Redactor::new(Policy::patient_identifiers()).redact(&mut message);

println!("{}", message.to_er7());`}</code></pre>
  <div class="prose">
    <p>
      The built-in policy names about forty positions across <code>PID</code>, <code>NK1</code>,
      <code>PV1</code>, <code>GT1</code>, and <code>IN1</code> — the fields that carry a patient
      identifier in every HL7 v2 release from 2.3 to 2.9.
    </p>
  </div>

  <h3>2. Check what it did, before sharing anything</h3>
  <pre><code>{`er7-redact --report message.er7`}</code></pre>
  <pre><code>{`PID[1]-3[1].1.1  pseudonym
PID[1]-5[1].1.1  replace REDACTED
PID[1]-5[1].2.1  replace REDACTED
PID[1]-5[1].3.1  replace REDACTED
PID[1]-7[1].1.1  first 4`}</code></pre>
  <div class="prose">
    <p>
      One row per position that actually changed, each a fully qualified path you can paste into
      <code>er7 --query</code> to see what is there now. A report carries <strong>no values</strong>
      — not the old text, not the new — so it is safe to paste into a ticket. An empty report means
      nothing was found to redact, which is either good news or a wrong policy.
    </p>
  </div>

  <h3>3. Say what you want redacted</h3>
  <pre><code>{`er7-redact -r "PID-5 replace REDACTED" -r "PID-7 first 4" message.er7`}</code></pre>
  <div class="prose">
    <p>
      Naming your own rules replaces the built-in policy rather than adding to it, so what runs can
      be predicted from the arguments. In Rust the same policy reads:
    </p>
  </div>
  <pre><code>{`use er7_redact::{Action, Policy};

let policy = Policy::new()
    .with("PID-5", Action::redacted())?
    .with("PID-7", Action::First(4))?;`}</code></pre>
  <div class="prose">
    <p>
      Paths are <a href="/paths/"><code>er7</code> paths</a>, so everything that notation does works
      here: <code>OBX-5</code> covers every <code>OBX</code> in the message,
      <code>PID-13[2]</code> names one repetition, <code>PID-5.1</code> reaches one component.
    </p>
  </div>

  <h3>4. Keep the built-in list, and pin it</h3>
  <pre><code>{`er7-redact --show-policy > de-identify.policy   # the built-in list, as a file
$EDITOR de-identify.policy
er7-redact --policy de-identify.policy message.er7`}</code></pre>
  <div class="prose">
    <p>
      The built-in list may <em>grow</em> in a minor release — if a position turns out to carry
      patient detail, waiting for a major version to start redacting it would be the wrong trade —
      so a repository that needs the same redaction next year should check the file in.
    </p>
  </div>

  <h3>5. When you do not trust the list</h3>
  <pre><code>{`er7-redact --all message.er7`}</code></pre>
  <div class="prose">
    <p>
      Redact everything except the <code>MSH</code> header, then name what to keep. This is the only
      thing that covers a local <code>Z</code> segment nobody has documented, and the only honest
      answer to &ldquo;is there anything else in there?&rdquo;.
    </p>
  </div>
  <pre><code>{`let policy = Policy::everything()
    .with("OBX-2", Action::Keep)?
    .with("OBX-3", Action::Keep)?
    .with("OBX-5", Action::Keep)?;`}</code></pre>
</section>

<section class="section" id="help">
  <h2 class="section-heading">
    <span class="section-heading-eyebrow">Help</span>
    Reference
  </h2>

  <h3>The eight actions</h3>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Written</th>
          <th><code>PATID1234</code> becomes</th>
          <th>Use for</th>
        </tr>
      </thead>
      <tbody>
        {#each actions as action (action.written)}
          <tr>
            <td><code>{action.written}</code></td>
            <td><code>{action.becomes}</code></td>
            <td>{action.use}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      <code>clear</code> and <code>null</code> are not interchangeable. An empty field says
      &ldquo;the sender said nothing about this&rdquo;; <code>""</code> says &ldquo;delete your
      stored value&rdquo;. A redacted copy for testing should not be telling anything to delete a
      record.
    </p>
  </div>

  <h3>The policy file</h3>
  <pre><code>{`# de-identify.policy

PID-3.1  pseudonym          # keep linkage, lose the record number
PID-5    replace REDACTED
PID-7    first 4            # a birth date reduced to its year
PID-11   clear
NTE-3    clear              # free text, where identifiers hide

*        replace REDACTED   # optional: the fallback, always applied last`}</code></pre>
  <div class="prose">
    <p>
      One rule per line: a path, whitespace, an action. Blank lines are ignored, <code>#</code>
      starts a comment, and rules apply <strong>in order</strong>. A path of <code>*</code> sets the
      fallback, which turns the policy from &ldquo;redact these&rdquo; into &ldquo;redact everything
      except these&rdquo; — and <code>keep</code> is how a position is exempted from it.
    </p>
    <p>
      A malformed line is refused at load time, with the line number. That strictness is deliberate:
      a typo in a policy means a value that silently was not redacted.
    </p>
  </div>

  <h3>Command-line options</h3>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr>
          <th>Option</th>
          <th>Effect</th>
        </tr>
      </thead>
      <tbody>
        {#each options as option (option.flag)}
          <tr>
            <td><code>{option.flag}</code></td>
            <td>{option.effect}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <h3>Pseudonyms</h3>
  <div class="prose">
    <p>
      Clearing an identifier destroys the message as test data: nothing ties the patient here to the
      same patient in the next message. <code>pseudonym</code> replaces it with a stable stand-in
      instead, so every message redacted with the same key maps
      <code>PATID1234</code> the same way, and different keys produce unrelated mappings.
    </p>
  </div>
  <div class="callout">
    <p>
      <strong>A pseudonym is not a cryptographic guarantee.</strong> It is a keyed hash that
      preserves equality on purpose — that is what makes messages joinable, and it is also what
      leaks. Anyone holding the key can invert the mapping by trying every candidate identifier, and
      record numbers come from small spaces.
    </p>
    <p>
      Use it inside your own trust boundary — test environments, reproductions, CI fixtures. For
      data leaving it, use <code>clear</code> or <code>replace</code>, which leak nothing but the
      fact that a value was there.
    </p>
  </div>
</section>

<section class="section" id="examples">
  <h2 class="section-heading">Examples</h2>
  <div class="prose">
    <p>
      Six runnable programs in the crate's <a href={`${crate.repository}/tree/main/examples`}
        ><code>examples/</code></a
      > directory. Each asserts its own results, so a clean exit means it passed.
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
  <pre><code>{`cargo run --example redact_a_message`}</code></pre>
</section>

<section class="section" id="not">
  <h2 class="section-heading">What it does not do</h2>
  <div class="prose">
    <p>
      <strong>This is a positional editor, not a compliance tool.</strong>
    </p>
    <ul>
      <li>
        It cannot tell you whether the result is de-identified. That is a judgement about a whole
        data set, its recipients, and what else they hold — made by a person who is accountable for
        it.
      </li>
      <li>
        It does not know which positions <em>your</em> senders use. Run
        <code>er7 message.er7</code> and read what is actually in there before trusting any policy,
        including the built-in one.
      </li>
      <li>
        It does not find an identifier written into free text. A name in an <code>NTE-3</code>
        comment survives every positional policy; name that position, or use <code>--all</code>.
      </li>
      <li>There is no way back: no mapping table, no key escrow, no undo.</li>
    </ul>
    <p>
      A message this crate has redacted is a message with less in it, which is progress, and is not
      the same thing as a safe one.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Where to go next</h2>
  <div class="tag-list">
    <a class="tag" href={crate.repository}>Source</a>
    <a class="tag" href={crate.crate}>Crate</a>
    <a class="tag" href={crate.docs}>API documentation</a>
    <a class="tag" href={`${crate.repository}/tree/main/spec`}>Specification</a>
    <a class="tag" href="/paths/">Path notation</a>
    <a class="tag" href="/ecosystem/">Ecosystem</a>
  </div>
</section>
