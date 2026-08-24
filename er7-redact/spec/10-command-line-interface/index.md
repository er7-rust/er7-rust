[`er7-redact` specification](../index.md) — section 10 of 17. Section
numbers (§10.x) are stable and cited from code, tests, and commit messages.

# 10. Command-line interface

Implemented in `src/main.rs`. This is a contract, not an implementation
detail: scripts depend on it, so it is specified here and pinned by the
`cli_*` tests in `tests/integration.rs`.

The CLI adds **no behaviour** of its own. Everything it writes is the
library's output, formatted ([§1.1](../01-purpose-and-scope/index.md)).

## 10.1 Synopsis

```
er7-redact [OPTIONS] [FILE]
```

`FILE` holds one or more messages, or a batch file. `-` or no argument
reads standard input. Input is split with `er7::split_messages`, and
**every payload is parsed before anything is written**, so a payload the
policy refuses ([§2.8](../02-redaction-model/index.md)) fails the run
rather than producing a half-redacted output, however late in a batch it
is.

| Option | Effect |
| ------ | ------ |
| `-p, --policy <FILE>` | read rules from a policy file ([§6](../06-policy-file-format/index.md)); may be repeated, and files are concatenated in the order given |
| `-r, --rule <RULE>` | add one rule, e.g. `-r "PID-5 replace REDACTED"`; may be repeated |
| `--accept-all` | accept by default: applied last, this is the one way to switch off a `reject` a policy file asked for ([§10.2](#102-which-policy-runs)) |
| `--reject-all` | start from `Policy::reject_all()`: reject every value, header included ([§5.6](../05-built-in-policies/index.md)) |
| `--all-but-the-header` | start from `Policy::all_but_the_header()`: reject every value but keep `MSH` ([§5.2](../05-built-in-policies/index.md)). This is what `--all` did |
| `-k, --key <KEY>` | the pseudonym key, a `u64`; default `0` ([§7](../07-pseudonyms/index.md)) |
| `-m, --message <N>` | use only the Nth message of the input, counting from 1 |
| `-t, --terminator <KIND>` | segment terminator to write: `cr` (default), `lf`, `crlf` |
| `-o, --output <FILE>` | write to `FILE` instead of standard output |
| `--report` | write the report ([§8](../08-report/index.md)) instead of the redacted message |
| `--show-policy` | write the policy that would be applied, and exit |
| `-h, --help` | print usage |
| `-V, --version` | print the version |

## 10.2 Which policy runs

| Given | Policy |
| ----- | ------ |
| nothing | `Policy::patient_identifiers()` ([§5.1](../05-built-in-policies/index.md)) |
| `--reject-all` | `Policy::reject_all()` ([§5.6](../05-built-in-policies/index.md)) |
| `--all-but-the-header` | `Policy::all_but_the_header()` ([§5.2](../05-built-in-policies/index.md)) |
| `--policy` or `--rule` | an accepting policy that **refuses** an unrecognised payload, plus those rules |
| `--accept-all` | the same, with the posture forced to accept last |
| a starting flag with `--policy` or `--rule` | that policy, plus those rules |

The CLI's own empty policy is not quite `Policy::accept_all()`: it accepts
by default, as that does, but it refuses a payload it cannot read where
`accept_all` passes one ([§2.8](../02-redaction-model/index.md)). A run
handed nothing but rules has been told nothing about what an unreadable
payload is worth, and refusing is the disposition that loses no value
quietly. Ask for `unrecognised pass` in a policy file to get the other one
— a file that states a disposition is not overruled (D20).

Rules are appended in the order the options were given, with `--policy`
files before `--rule` arguments, and order is significant (D7,
[§2.4](../02-redaction-model/index.md)). Appending never weakens the
defaults the starting policy came with (D20,
[§2.6](../02-redaction-model/index.md)): a policy file of extra rules
cannot turn `--reject-all` back off.

`--accept-all` is the exception, and the only one. It is not a starting
policy: it sets the posture of whatever policy the run assembled, after
every file and rule has been read, and so it wins. That is deliberate —
without it there would be no way at all to run a policy file's rules
without its `reject` line, and a flag that can never change the outcome is
worse than one that can.

```sh
er7-redact --accept-all -p strict.policy message.er7   # rules yes, reject no
```

The built-in default is used **only** when nothing else is asked for. A
caller who names their own rules gets exactly those: silently adding
seventeen more would make the CLI's output impossible to predict from its
arguments, and would make `--show-policy` the only way to find out what
ran. To combine, ask for both:

```sh
er7-redact --show-policy > base.policy   # the built-in default, as a file
er7-redact -p base.policy -r "NTE-3 clear" message.er7
```

`--all` was removed in 0.2 and is an error naming its replacement, rather
than an unknown option:

```
er7-redact: error: --all is now --all-but-the-header (or --reject-all, which redacts the header too)
```

It could not stay: `-a` meant *reject* everything, and the flags that
replaced it are named for the two postures
([§2.6](../02-redaction-model/index.md)). A script that kept working but
had switched posture is the one outcome worth ruling out.

## 10.3 Output

The default output is the redacted messages as canonical ER7, each
segment terminated including the last, which is what a receiver expects
and what makes concatenating two runs safe.

`--report` writes the report instead, one row per change, path and action
in two columns:

```
PID[1]-3[1].1.1  pseudonym
PID[1]-5[1].1.1  replace REDACTED
PID[1]-5[1].2.1  replace REDACTED
PID[1]-7[1].1.1  first 4
```

When the input holds more than one message, each message's rows are
preceded by a `# message N` heading and a blank line separates them. Paths
are padded to a common width, clamped between 8 and 28 characters, then
two spaces before the action — the same layout as the `er7` CLI's outline,
for the same reason.

A payload that was not ER7 and that the policy did not refuse
([§2.8](../02-redaction-model/index.md)) has no positions, and so
contributes no rows. It gets a comment line instead, which is not a change
row and cannot be mistaken for one:

```
# message 2: unrecognised payload, mask *
# message 3: unrecognised payload, passed through
```

**`--report` does not write the redacted message.** It is a dry run: it
says what would change, and changes nothing on disk. To get both, run the
command twice, or redirect them separately.

`--show-policy` writes the policy in the canonical form of
[§6.5](../06-policy-file-format/index.md) and exits without reading any
input, so it works with no `FILE` argument and never blocks on standard
input.

## 10.4 Exit codes and diagnostics

| Code | Meaning |
| ---- | ------- |
| 0 | success |
| 1 | any error: bad arguments, an unreadable policy or input, a message that failed to parse, or an unwritable output |

Diagnostics go to standard error, prefixed `er7-redact: error: `, one
line. A message that failed to parse is identified by its 1-based position
in the input:

```
er7-redact: error: message 3: input contains no HL7 segments
er7-redact: error: reading de-identify.policy: No such file or directory (os error 2)
er7-redact: error: policy line 4: "PID-5 obfuscate": unknown action "obfuscate"
```

**A payload that is not ER7 fails the run only when the policy refuses
one** ([§2.8](../02-redaction-model/index.md)), which the two curated
policies do, and so does every run that names none. A policy that passes
such a payload through, or masks it whole, exits 0 and writes it — the run
did what the policy said. This is the one place a CLI diagnostic depends on
the policy rather than the input.

**A policy that changes nothing is not an error.** The message is written
out unchanged and the run exits 0 — the message simply carried none of the
positions the policy names (D8). A script that needs to know should use
`--report` and test for empty output.

A closed output pipe (`er7-redact … | head -3`) exits 0 rather than
reporting a broken-pipe error, which is what every other Unix filter does.

## 10.5 Stability

The CLI is covered by the same semantic-versioning promise as the library
([§13](../13-compatibility-and-versioning/index.md)): removing an option,
changing an exit code, changing which policy runs by default, or changing
the report's layout is a breaking change. Adding an option is not. Removing
`--all` in 0.2 was such a change, and is why 0.2 is not 0.1.3.
