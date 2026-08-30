<script lang="ts">
  import { links } from '$lib/site';

  const questions = [
    {
      id: 'Q1',
      title: 'Does the value tree match the messages you actually receive?',
      detail:
        'Absent, empty, and the explicit null stay distinct. Does anything your senders emit not fit the fixed hierarchy?'
    },
    {
      id: 'Q2',
      title: 'Are escape sequences decoded in the right places?',
      detail:
        'er7 decodes sequences wherever they appear — a recorded divergence. Has that ever bitten you?'
    },
    {
      id: 'Q3',
      title: 'Is the path syntax the one you would have guessed?',
      detail: 'What did you type that did not work, especially coming from HAPI, Mirth, or another tool?'
    },
    {
      id: 'Q4',
      title: 'What does the built-in redaction policy miss?',
      detail: 'What did you have to add? What did it redact that you needed to keep?'
    },
    {
      id: 'Q5',
      title: 'Should a pseudonym be cryptographic?',
      detail:
        'FNV-1a with a u64 key is a stable stand-in, not a security primitive. Does that reasoning survive your threat model?'
    },
    {
      id: 'Q6',
      title: 'Does "the same patient" need to mean more than "the same text"?',
      detail: 'A pseudonym maps a value, not a patient. Is that a problem you actually have?'
    },
    {
      id: 'Q7',
      title: 'Is the zero-dependency stance load-bearing, or is it theatre?',
      detail: 'Did it matter in a real review at your organisation, or did your process not care?'
    },
    {
      id: 'Q8',
      title: 'Is the minimum Rust version window right?',
      detail: 'Current stable minus two — roughly three months. Is that enough for your organisation?'
    },
    {
      id: 'Q9',
      title: 'Are five licences useful, or just confusing?',
      detail: 'Did the choice help, or did your legal review treat an unusual expression as a red flag?'
    },
    {
      id: 'Q10',
      title: 'Is performance anywhere near your bottleneck?',
      detail: 'Or is I/O what dominates? Batch files in the hundreds of megabytes are the known unsolved case.'
    },
    {
      id: 'Q11',
      title: 'What is missing that stops you adopting this at all?',
      detail: 'No validation, no typed segment model, no transport — deliberately. Did one of those absences stop you?'
    },
    {
      id: 'Q12',
      title: 'Does the documentation earn trust or spend it?',
      detail: 'Did the candour read as rigour, or as a project protesting too much? Which document did you actually use?'
    },
    {
      id: 'Q13',
      title: 'Would "redact what you know" catch what you actually find in free text?',
      detail:
        'When you found an identifier in free text by hand, was it a value that also appeared at a named position — or something else entirely?'
    },
    {
      id: 'Q14',
      title: 'What does a defensible Safe Harbor statement need from a tool like this?',
      detail:
        'If you have taken a data set through a review, what did the reviewer actually ask for — a per-identifier coverage table, or something else?'
    }
  ];
</script>

<svelte:head>
  <title>Request for comments — er7</title>
  <meta
    name="description"
    content="The specific feedback the ER7 Rust project is asking for: fourteen open questions, each recorded in a specification section, each with a decision a good answer would change."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Request for comments</p>
  <h1>What this project wants to learn</h1>
  <p class="hero-tagline">
    A one-maintainer project with no production traffic of its own. The crates are specified,
    tested, and benchmarked &mdash; and none of that says whether they are right for the messages
    real systems send. That gap closes when somebody who runs an interface says what they saw.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">What helps most</h2>
  <div class="prose">
    <ol>
      <li><strong>&ldquo;Here is a message shape you get wrong.&rdquo;</strong> Redacted, minimal, reproducible &mdash; the single most valuable thing anyone can send.</li>
      <li><strong>&ldquo;We evaluated this and chose something else, because X.&rdquo;</strong> A lost evaluation with a reason is worth more than a won one without.</li>
      <li><strong>&ldquo;Your reasoning in &sect;N is wrong, and here is why.&rdquo;</strong> The specs argue in prose precisely so the argument can be attacked.</li>
      <li><strong>&ldquo;We ran it against our traffic and nothing broke.&rdquo;</strong> Boring, and currently unobtainable any other way.</li>
      <li><strong>&ldquo;This claim oversells.&rdquo;</strong> Overclaiming in healthcare software is a defect and is treated as one.</li>
    </ol>
    <p>
      Answer by <a href="{links.repository}/issues">GitHub issue</a>, by
      <a href={links.email}>email</a> for anything about your own environment, or by a pull request
      against the relevant spec section. One paragraph about one question is a real contribution.
      <strong>Never send a real HL7&reg; v2 message</strong> &mdash; redact it first with
      <code>er7-redact</code>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">The questions</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th></th><th>Question</th><th>What we want to know</th></tr>
      </thead>
      <tbody>
        {#each questions as q (q.id)}
          <tr>
            <td>{q.id}</td>
            <td><strong>{q.title}</strong></td>
            <td>{q.detail}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      Every answer gets recorded, whatever it is; a &ldquo;no&rdquo; comes with reasoning, in the
      spec, where you can argue with it; and you get credit unless you ask not to. Question numbers
      are stable and never reused.
    </p>
    <p>
      <strong>The canonical document is
      <a href="{links.repository}/blob/main/RFC.md">RFC.md</a></strong> at the repository root —
      the full version of every question, what an answer would change, which spec section records
      it, and the decisions already made.
    </p>
  </div>
</section>
