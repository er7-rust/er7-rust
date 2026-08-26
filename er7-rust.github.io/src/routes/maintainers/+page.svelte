<script lang="ts">
  import { links } from '$lib/site';
</script>

<svelte:head>
  <title>Maintainers — er7</title>
  <meta
    name="description"
    content="The maintainer roster for the ER7 Rust crates, the stated bus factor of one, and what happens if the person who can ship a fix is unavailable."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Maintainers</p>
  <h1>Maintainers and continuity</h1>
  <p class="hero-tagline">
    The honest answer to the question a procurement review asks: what happens if the person who can
    ship a fix is unavailable?
  </p>
</section>

<section class="section">
  <h2 class="section-heading">Roster</h2>
  <div class="table-wrap">
    <table class="table">
      <thead>
        <tr><th>Person</th><th>GitHub</th><th>Contact</th><th>Role</th></tr>
      </thead>
      <tbody>
        <tr>
          <td>Joel Parker Henderson</td>
          <td><a href="https://github.com/joelparkerhenderson">@joelparkerhenderson</a></td>
          <td><a href={links.email}>joel@joelparkerhenderson.com</a></td>
          <td>Maintainer (sole)</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="callout callout-warning">
    <p>
      <strong>The bus factor of this project is one.</strong> There is exactly one person who can
      accept a pull request, publish a release, or change a repository setting. No second
      maintainer exists, no organisation stands behind the project, and no legal entity is a party
      to it. Nothing elsewhere in the repository should be read as softening that sentence.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">What is not here yet</h2>
  <div class="prose">
    <p>Named rather than quietly omitted, because absence is itself information:</p>
    <ul>
      <li>
        <strong>No hosted-CI track record yet.</strong> A workflow exists and runs the four
        checks, an MSRV build, a fuzz smoke run, and the trademark checker &mdash; but until its
        first hosted run goes green it is a written gate, not a proven one.
      </li>
      <li><strong>No signed commits or tags</strong>, and no key escrow.</li>
      <li>
        <strong>No second security responder.</strong> The security policy terminates at one email
        address, which is why it tells a reporter to disclose publicly after ninety days whether or
        not a fix exists.
      </li>
      <li><strong>No release cadence commitment.</strong> Releases happen when there is something to release.</li>
    </ul>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">If the maintainer is unavailable</h2>
  <div class="prose">
    <p>
      Everything already published stays published: released versions on crates.io are immutable,
      and the git history, the specifications, and this site&rsquo;s source stay public. But
      nothing new ships &mdash; no releases, no security fixes, no answers to issues.
    </p>
    <p>
      <strong>Forking is legitimate continuity.</strong> If you depend on this software in a
      clinical setting and a bus factor of one is not acceptable &mdash; and it is entirely
      reasonable for it not to be &mdash; the mitigation is on your side: pin an exact version,
      keep a fork you can build from source, and budget for maintaining it. The crates are built to
      make that cheap: near-zero dependencies, and every behaviour a numbered rule with a test
      bound to it, so a fork inherits a readable, testable artefact rather than an archaeology
      project.
    </p>
    <p>
      <strong>The canonical document is
      <a href="{links.repository}/blob/main/MAINTAINERS.md">MAINTAINERS.md</a></strong> at the
      repository root; this page is a summary of it, and the full version includes the
      publishing-identity inventory &mdash; every credential that can put bytes in front of a
      user, and where each one terminates.
    </p>
  </div>
</section>
