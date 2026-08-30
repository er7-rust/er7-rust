<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>Agent skill — er7</title>
  <meta
    name="description"
    content="er7-skill: a packaged Claude Code Skill teaching an AI coding agent how to parse, query, edit, write, and redact HL7® v2 ER7 messages with the er7-rust crate family."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">For AI coding agents</p>
  <h1>Agent skill</h1>
  <p class="hero-tagline">
    A packaged skill that teaches an agent which crate to reach for, the rules that are easy to
    get wrong, and worked recipes — so it uses this crate family correctly on the first try.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">What it is</h2>
  <div class="prose">
    <p>
      <a href="https://code.claude.com/docs/en/skills">Claude Code Skills</a> are
      packaged instructions an agent loads when a task matches what the skill covers.
      <code>er7-skill</code> is one such skill for this crate family: which of <code>er7</code>,
      <code>er7-redact</code>, and <code>serde-er7</code> a task needs, the round-trip and
      absent/empty/null rules that are easy to get wrong, how to edit a value without silently
      corrupting the message around it, and short recipes for parsing, querying, editing, batch
      input, and redaction.
    </p>
    <p>
      It teaches a downstream project how to <strong>use</strong> these crates as a dependency. It
      is not the guidance this repository's own agents follow to change the crates themselves —
      that lives in each crate's own <code>AGENTS.md</code> and <code>spec/</code>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Using it</h2>
  <div class="prose">
    <p>
      Copy <a href="{links.repository}/tree/main/er7-skill"><code>er7-skill/</code></a> into a
      project's own <code>.claude/skills/</code> directory. Claude Code picks it up automatically
      the next time a task matches its description — parsing or building an HL7&reg; v2 message in
      Rust, an ER7 or batch-file question, an HL7 path such as <code>PID-5.1</code>, or removing
      patient detail before a message is logged, shared, or committed.
    </p>
    <p>
      <strong>The canonical file is
      <a href="{links.repository}/blob/main/er7-skill/SKILL.md">er7-skill/SKILL.md</a></strong> at
      the repository root; this page is a summary of it, not a copy — read the file itself for the
      full rule list and every recipe.
    </p>
  </div>
</section>
