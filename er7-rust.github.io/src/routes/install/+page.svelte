<script lang="ts">
  import { links, version } from '$lib/site';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta
    name="description"
    content="Install er7, er7-redact, and serde-er7: the command-line tools, the libraries, building from source, the requirements, and how to verify the install."
  />
</svelte:head>

<section class="hero">
  <p class="hero-eyebrow">Install</p>
  <h1>Install</h1>
  <p class="hero-tagline">
    Three ways in. If you only want to look at an HL7® v2 message and get on with your day, start
    with the command line — it needs no Rust knowledge at all.
  </p>
</section>

<section class="section">
  <h2 class="section-heading">The command line</h2>
  <div class="prose">
    <p>
      Two binaries. Each is a single self-contained executable with nothing to install alongside it
      — no JVM, no interpreter, no service, no configuration file.
    </p>
  </div>
  <pre><code>{`cargo install er7           # installs \`er7\`
cargo install er7-redact    # installs \`er7-redact\``}</code></pre>
  <div class="prose">
    <p>
      <code>cargo install</code> puts them in <code>~/.cargo/bin</code>, which Rust&rsquo;s installer
      adds to your <code>PATH</code>.
    </p>
  </div>
  <pre><code>{`er7 message.er7                        # every value, with the path that names it
er7 --query PID-5.1 message.er7        # read one value
er7 --query OBX-5 message.er7          # every match, one per line
er7 --raw --query OBX-5 message.er7    # text as sent, escapes not decoded
er7 --normalize --terminator lf m.er7  # rewrite as canonical ER7
er7 --message 2 batch.er7              # the second message of a batch

er7-redact message.er7                 # redact with the built-in policy
er7-redact --report message.er7        # say what would change, change nothing
er7-redact --show-policy               # the built-in policy, as an editable file
er7-redact -p my.policy message.er7    # apply your own policy file
er7-redact -r "NTE-3 clear" m.er7      # or one rule, inline`}</code></pre>
  <div class="prose">
    <p>
      Every label in <code>er7</code>&rsquo;s default outline is itself a valid query, so you can
      read a path off the output and paste it straight back in. Reading is from a named file or
      standard input, writing to standard output or a named file with <code>-o</code>. The
      <a href="/cli/">command-line page</a> documents every flag.
    </p>
  </div>
  <div class="callout callout-warning">
    <p>
      <strong>Before piping patient data anywhere</strong>, note that
      <code>er7-redact --report</code> prints paths and actions and <em>no values</em>, which is the
      form that can safely go into a ticket. Nothing in this family writes a log, opens a socket, or
      reads a configuration file: the only bytes that leave are the ones you asked for.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">As a library</h2>
  <div class="prose">
    <p>Take the layer you want, and skip what you do not.</p>
  </div>
  <pre><code>{`cargo add er7           # parse, query, edit, write. Zero dependencies.
cargo add er7-redact    # redaction. One dependency: er7.
cargo add serde-er7     # Serde support. Two: serde and er7.`}</code></pre>
  <pre><code>{`let message = er7::parse(text)?;

assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.to_er7(), text);   // byte for byte`}</code></pre>
  <div class="prose">
    <p>
      Nothing forces you to take all three. <code>er7</code> depends on nothing at all — that is the
      entire tree, which matters where dependency trees get audited.
      <code>serde-er7</code> is a separate crate rather than a feature flag for exactly that reason:
      adding Serde to <code>er7</code> would cost every user of <code>er7</code> a dependency they
      may not want.
    </p>
    <p>
      There are <strong>no feature flags</strong> on any of the three. Nothing is optional because
      nothing optional exists.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">From source</h2>
  <pre><code>{`git clone https://github.com/er7-rust/er7-rust.git
cd er7-rust
cargo build --workspace
cargo test --workspace`}</code></pre>
  <div class="prose">
    <p>
      One <code>Cargo.lock</code> at the workspace root covers every member. Use
      <code>-p &lt;crate&gt;</code> to scope a command to one of them, and run the four checks that
      define &ldquo;done&rdquo; for any change:
    </p>
  </div>
  <pre><code>{`cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc -p er7 --lib -- -W missing-docs`}</code></pre>
  <div class="prose">
    <p>
      Mirrors of the repository are on <a href="https://gitlab.com/er7-rust">GitLab</a> and
      <a href="https://codeberg.org/er7-rust">Codeberg</a>; issues live on
      <a href={links.repository}>GitHub</a>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Requirements</h2>
  <div class="table-wrap">
    <table class="table">
      <tbody>
        <tr><th>Rust</th><td>Current stable minus two releases. Today that is <strong>1.96</strong>.</td></tr>
        <tr><th>Edition</th><td>2024, which needs 1.85 — no longer the binding constraint</td></tr>
        <tr><th>Platform</th><td>Anything Rust targets. No platform-specific code, no C dependency, no build script.</td></tr>
        <tr><th>Network</th><td>None, at build time or run time</td></tr>
      </tbody>
    </table>
  </div>
  <div class="prose">
    <p>
      The Rust floor is a rolling window. It is that wide because healthcare toolchains are approved
      on a cycle measured in quarters, so a library demanding the compiler released this month is a
      library that cannot be adopted. Check a build against the floor with
      <code>cargo +1.96 check --workspace --all-targets</code>. If you do not have Rust:
      <a href="https://rustup.rs">rustup.rs</a>.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Verifying the install</h2>
  <pre><code>{`er7 --version
printf 'MSH|^~\\\\&|LAB|ACME|EHR|CLINIC|20260814080000||ADT^A08|MSG1|P|2.5\\r' | er7`}</code></pre>
  <div class="prose">
    <p>
      The second prints an outline whose first lines are <code>MSH-1</code>, <code>MSH-2</code>, and
      <code>MSH-3</code>. If it does, you are done. The current release is
      <code>er7</code> {version}.
    </p>
  </div>
</section>

<section class="section">
  <h2 class="section-heading">Uninstalling</h2>
  <pre><code>{`cargo uninstall er7
cargo uninstall er7-redact`}</code></pre>
  <div class="prose">
    <p>
      Nothing is left behind: no configuration directory was created, no cache was written, no
      service was registered.
    </p>
  </div>
</section>
