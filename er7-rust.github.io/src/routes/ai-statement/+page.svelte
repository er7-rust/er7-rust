<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>AI statement — er7</title>
  <meta
    name="description"
    content="How AI tools are used to develop the ER7 Rust crates: what they touch, who is accountable, which controls bound the work, and the limitations that survive all of it."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">AI statement</p>
  <h1>AI statement</h1>
  <p class="hero-tagline">
    A disclosure of how artificial-intelligence tools are used to build these crates, written for
    evaluators and regulated adopters performing supplier due diligence.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">The crates ship no AI</h2>
  <div class="prose">
    <p>
      No model is trained, embedded, or called at run time. Nothing performs inference, and nothing
      in library code reaches a network at all. AI is used to <em>build</em> the software &mdash;
      in the same sense a compiler and a linter are used to build it. The EU AI Act imposes no
      obligation on this project, because it binds providers and deployers of AI systems and this
      workspace is not one; the disclosure is voluntary, and no standard is claimed as conformity.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Where AI is used, and who is accountable</h2>
  <div class="prose">
    <p>
      The tooling is agentic AI coding assistance &mdash; Claude Code, by Anthropic &mdash; in
      sessions the maintainer directs and reviews. Crate code, tests, specifications, and
      documentation are <strong>ai-generated</strong> in the W3C AI Content Disclosure
      vocabulary&rsquo;s sense: AI-generated with human prompting and review. What the crates do
      and do not do &mdash; scope, non-goals, what a release claims &mdash; is decided by the
      maintainer, with no AI involvement. Nothing is <strong>autonomous</strong>: no commit or
      release is automated, and nothing lands on its own authority.
    </p>
    <p>
      <strong>One named human is accountable for every change</strong>, whatever tool produced the
      bytes. Commit trailers record tool <em>participation</em>; the author field of every commit
      records accountability, and a tool is never named as an author or signer of anything.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Controls, prohibitions, and limitations</h2>
  <div class="prose">
    <p>
      Every change, whoever or whatever wrote it, passes the same gates: the test suite, pedantic
      lints denied, formatting, documented public items, the minimum-Rust-version build, and the
      trademark checker. The specification is the control that catches a plausible-but-wrong
      implementation regardless of who wrote it: every rule is numbered, backed by a test, and a
      tool cannot quietly redefine what correct means. Tests and expectations are never weakened to
      make a build pass.
    </p>
    <p>In this project, AI is prohibited from:</p>
    <ul>
      <li>committing or merging anything on its own authority, or signing anything;</li>
      <li>deciding whether to accept a contribution from someone else;</li>
      <li>deciding what the HL7&reg; v2 standard means where it is silent, or what a release claims;</li>
      <li>weakening a test, an expectation, a spec rule, or a gate to make something pass;</li>
      <li>adding a dependency to any of the three crates;</li>
      <li>generating a sample or fixture by paraphrasing a real message &mdash; synthetic means constructed, not laundered.</li>
    </ul>
    <p>
      And the limitations are stated, because a disclosure without them is marketing: the gates
      prove what they test, not correctness; review depth is one person&rsquo;s; provenance
      uncertainty survives; and the whole document is a self-declaration that no third party has
      audited. The checkable artefacts &mdash; the specs, the tests, the commit trailers, the
      published benchmark method &mdash; are the counterweight: they can disagree with the
      document, and if they do, the document is wrong.
    </p>
    <p>
      <strong>The canonical document is
      <a href="{links.repository}/blob/main/AI_STATEMENT.md">AI_STATEMENT.md</a></strong> at the
      repository root &mdash; versioned, with an effective date, a change log, and a
      machine-readable summary; this page is a summary of it. No patient data, no personally
      identifiable health information, and no customer data exists anywhere in this project, and
      therefore in any prompt.
    </p>
  </div>
</section>
