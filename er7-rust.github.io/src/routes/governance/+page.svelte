<script lang="ts">
  import { links } from '$lib/site';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta
    name="description"
    content="How decisions about the ER7 Rust crates get made and recorded: one maintainer, a canonical specification, recorded declines, and forking as legitimate continuity."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Governance</p>
  <h1>Governance</h1>
  <p class="hero-tagline">
    Who decides what happens to this project, how decisions get made and recorded, and how someone
    else can come to share them.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Who decides</h2>
  <div class="prose">
    <p>
      <strong>One person: the maintainer.</strong> There is no steering committee, no technical
      board, no voting, and no legal entity. This is a benevolent-dictator model, and naming it is
      more honest than implying a process. The dictator part is real: one person can accept a pull
      request or change a repository setting, and nobody can overrule them. Publishing a release is
      similar but not identical: the maintainer alone still decides <em>whether</em> a crate
      releases at all &mdash; scoping and naming a release is his, unconditionally &mdash; but as of
      2026-09-02 he may direct an AI coding agent to judge whether a release he has already scoped
      meets this project's own stated readiness criteria and, if so, run <code>cargo publish</code>
      for it. The benevolent part is not a promise &mdash; it is a set of constraints anyone can
      check: a decision about behaviour is written down before it is implemented, a declined idea is
      recorded with its reasoning, a rule without a test fails the build, nothing merges
      automatically, and the project says what it does not have rather than omitting it.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">The specification is the authority</h2>
  <div class="prose">
    <p>
      Each crate&rsquo;s <code>spec/</code> directory is canonical: when the spec and the code
      disagree, the spec is right and the code is a bug. Every behavioural guarantee carries a
      stable rule ID, and a coverage table &mdash; itself checked by <code>cargo test</code> &mdash;
      maps each rule to the test that enforces it. The practical effect is that arguments here are
      about text, not about authority: &ldquo;R13 says an unrecognised sequence stays literal, and
      this code does not&rdquo; is a complete argument the maintainer cannot wave away. That is a
      deliberate transfer of power away from the person holding the merge button, and it is the
      main thing this project offers in place of a committee.
    </p>
    <p>
      A change lands the same way for everyone, maintainer included: the spec section changes
      first, then a test expresses the rule, then the code, then the documentation, then the
      checks pass, then the maintainer reviews and merges. Nothing merges automatically, and no
      tool merges anything, ever.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What gets declined, and why</h2>
  <div class="prose">
    <p>Most declines fall into three groups:</p>
    <ul>
      <li>
        <strong>It crosses a layer boundary.</strong> <code>er7</code> is the ER7
        <em>encoding</em>; validation, typed segment models, code tables, and transports belong to
        the layer above, which is a different project.
      </li>
      <li>
        <strong>It would add a dependency.</strong> The counts are zero, one, and two, each
        justified by name in the family policy. Healthcare integration code gets audited, and
        these crates are meant to sit at the bottom of somebody else&rsquo;s stack.
      </li>
      <li>
        <strong>It would make a claim the project cannot support.</strong> No conformance
        certification, no comparative performance claim without a fair benchmark, no compliance
        statement. Overclaiming in healthcare software is treated as a defect.
      </li>
    </ul>
    <p>
      A decline is written down with its reasoning, in the spec, where you can argue with it.
      <strong>A decision being recorded does not make it right; it makes it arguable.</strong>
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Becoming a maintainer, and forking</h2>
  <div class="prose">
    <p>
      The route to maintainership is open and deliberately unglamorous: contribute, show sustained
      judgement about the specification &mdash; not just the code &mdash; and ask. Nobody is
      expected to wait to be noticed.
    </p>
    <p>
      <strong>Forking is legitimate continuity, not a hostile act</strong>, and the project is
      arranged to make it cheap: five licences at your option, near-zero dependencies, a
      specification with numbered rules bound to tests, no CLA and no copyright assignment. If the
      maintainer becomes unavailable, a fork is the intended answer.
    </p>
    <p>
      <strong>The canonical document is
      <a href="{links.repository}/blob/main/GOVERNANCE.md">GOVERNANCE.md</a></strong> at the
      repository root; this page is a summary of it. Conduct is governed by
      <a href="{links.repository}/blob/main/CODE_OF_CONDUCT.md">CODE_OF_CONDUCT.md</a>, which
      applies across the workspace.
    </p>
  </div>
</section>
