# Install

Three ways in, depending on what you are doing. If you only want to look at
a message and get on with your day, start with the command line — it needs
no Rust knowledge at all.

- [The command line](#the-command-line)
- [As a library](#as-a-library)
- [From source](#from-source)
- [Requirements](#requirements)
- [Verifying the install](#verifying-the-install)
- [Uninstalling](#uninstalling)

Full documentation is at <https://er7-rust.github.io>; the [install page](https://er7-rust.github.io/install/)
there covers the same ground with more examples.

## The command line

Two binaries ship in this workspace. Each is a single self-contained
executable with no runtime to install alongside it — no JVM, no
interpreter, no service.

```sh
cargo install er7           # installs `er7`
cargo install er7-redact    # installs `er7-redact`
```

`cargo install` puts them in `~/.cargo/bin`, which Rust's installer adds to
your `PATH`.

```sh
er7 message.er7                        # every value, with the path that names it
er7 --query PID-5.1 message.er7        # read one value
er7 --query OBX-5 message.er7          # every match, one per line
er7 --raw --query OBX-5 message.er7    # text as sent, escapes not decoded
er7 --normalize --terminator lf m.er7  # rewrite as canonical ER7
er7 --message 2 batch.er7              # the second message of a batch

er7-redact message.er7                 # redact with the built-in policy
er7-redact --report message.er7        # say what would change, change nothing
er7-redact --show-policy               # the built-in policy, as an editable file
er7-redact -p my.policy message.er7    # apply your own policy file
er7-redact -r "NTE-3 clear" m.er7      # or one rule, inline
```

Every label in `er7`'s default outline is itself a valid query, so you can
read a path off the output and paste it straight back in.

Reading is from a named file or standard input, writing to standard output
or a named file with `-o`. `er7 --help` and `er7-redact --help` list the
rest; the full contracts are
[`er7/spec/12-command-line-interface/index.md`](er7/spec/12-command-line-interface/index.md)
and
[`er7-redact/spec/10-command-line-interface/index.md`](er7-redact/spec/10-command-line-interface/index.md).

**Before piping patient data anywhere**, read
[§1.4 of the family policy](spec/01-family-policy/index.md). `er7-redact
--report` prints paths and actions and *no values*, which is the form that
can safely go into a ticket.

## As a library

Take the layer you want, and skip what you do not:

```sh
cargo add er7           # parse, query, edit, write. Zero dependencies.
cargo add er7-redact    # redaction. One dependency: er7.
cargo add serde-er7     # Serde support. Two: serde and er7.
```

```rust
let message = er7::parse(text)?;

assert_eq!(message.query("PID-5.1")?.as_deref(), Some("EVERYWOMAN"));
assert_eq!(message.to_er7(), text);   // byte for byte
```

Nothing forces you to take all three. `er7` depends on nothing at all —
that is the entire tree, which matters where dependency trees get audited.
`serde-er7` exists as a separate crate rather than a feature flag for
exactly that reason: adding Serde to `er7` would cost every user of `er7` a
dependency they may not want. See
[§1.1 of the family policy](spec/01-family-policy/index.md).

There are **no feature flags** on any of the three. Nothing is optional
because nothing optional exists.

## From source

```sh
git clone https://github.com/er7-rust/er7-rust.git
cd er7-rust
cargo build --workspace
cargo test --workspace
```

One `Cargo.lock` at the workspace root covers every member; a crate does
not carry its own. Use `-p <crate>` to scope a command to one member:

```sh
cargo test -p er7-redact
cargo run -p er7 --bin er7 -- --query PID-5.1 er7/samples/oru_r01.er7
cargo bench -p er7-bench
```

The four checks that define "done" for any change:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
cargo rustdoc -p er7 --lib -- -W missing-docs
```

Mirrors of the repository are on
[GitLab](https://gitlab.com/er7-rust) and
[Codeberg](https://codeberg.org/er7-rust); issues live on GitHub.

## Requirements

| | |
|---|---|
| Rust | Current stable minus three releases. Today that is **1.95**. |
| Edition | 2024, which needs 1.85 — no longer the binding constraint. |
| Platform | Anything Rust targets. No platform-specific code, no C dependency, no build script. |
| Network | None, at build time or run time. |

The Rust floor is a rolling window, and the policy behind it is
[`spec/rust-msrv-n-minus-3/index.md`](spec/rust-msrv-n-minus-3/index.md).
It exists because healthcare toolchains are approved on a cycle measured in
quarters, so a library demanding the compiler released this month is a
library that cannot be adopted.

Check a build against the floor with:

```sh
cargo +1.95 check --workspace --all-targets
```

If you do not have Rust: <https://rustup.rs>.

## Verifying the install

```sh
er7 --version
printf 'MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260814080000||ADT^A08|MSG1|P|2.5\r' | er7
```

The second prints an outline whose first lines are `MSH-1`, `MSH-2`, and
`MSH-3`. If it does, you are done.

## Uninstalling

```sh
cargo uninstall er7
cargo uninstall er7-redact
```

Nothing is left behind: no config directory is created, no cache is
written, no service is registered.
