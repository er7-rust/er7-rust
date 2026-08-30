<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>Agent skills — er7</title>
  <meta
    name="description"
    content="Two packaged Claude Code Skills for the er7-rust crate family: er7-skill for using er7, er7-redact, and serde-er7 correctly, and er7-rust-maintainer-skill for changing this repository itself."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">For AI coding agents</p>
  <h1>Agent skills</h1>
  <p class="hero-tagline">
    Two packaged skills, for the two different audiences that read this repository: a user of
    these crates, and a maintainer changing them.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">What a skill is</h2>
  <div class="prose">
    <p>
      <a href="https://code.claude.com/docs/en/skills">Claude Code Skills</a> are packaged
      instructions an agent loads when a task matches what the skill covers. This repository ships
      two, at its root, for two different jobs.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading"><code>er7-skill</code> — using this crate family</h2>
  <div class="prose">
    <p>
      ER7 concepts and terminology (segments, fields, repetitions, components, paths, delimiters),
      which of <code>er7</code>, <code>er7-redact</code>, and <code>serde-er7</code> a task needs,
      the round-trip and absent/empty/null rules that are easy to get wrong, and short recipes for
      parsing, querying, editing, batch input, and redaction.
    </p>
    <p>
      It teaches a downstream project how to <strong>use</strong> these crates as a dependency. It
      is not the guidance this repository's own agents follow to change the crates themselves —
      that is the other skill.
    </p>
    <p>
      Copy <a href="{links.repository}/tree/main/er7-skill"><code>er7-skill/</code></a> into a
      project's own <code>.claude/skills/</code> directory. Claude Code picks it up automatically
      the next time a task matches its description — parsing or building an HL7&reg; v2 message in
      Rust, an ER7 or batch-file question, an HL7 path such as <code>PID-5.1</code>, or removing
      patient detail before a message is logged, shared, or committed.
    </p>
    <p>
      <strong>The canonical file is
      <a href="{links.repository}/blob/main/er7-skill/SKILL.md">er7-skill/SKILL.md</a></strong>;
      this page is a summary of it, not a copy — read the file itself for the full rule list and
      every recipe.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading"><code>er7-rust-maintainer-skill</code> — changing this repository</h2>
  <div class="prose">
    <p>
      The other direction: an agent packaging of this repository's own <code>AGENTS.md</code>
      files, for a downstream fork or another project's tooling that wants this workspace's
      conventions as a loadable skill rather than a set of files to go read first. It indexes,
      rather than duplicates: spec-driven development (spec first, then tests, then code, then
      docs), the four checks, and the rules that are easy to violate by skipping straight to the
      code — never real patient data, never collapsing absent/empty/null, no dictionary or
      validator added to a crate that deliberately has neither.
    </p>
    <p>
      <strong>The canonical file is
      <a href="{links.repository}/blob/main/er7-rust-maintainer-skill/SKILL.md"
        >er7-rust-maintainer-skill/SKILL.md</a
      ></strong>
      — read it, or the workspace <a href="{links.repository}/blob/main/AGENTS.md">AGENTS.md</a> it
      indexes, before changing this repository.
    </p>
  </div>
</section>
