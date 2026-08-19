[`er7` specification](index.md) — section 12 of 19. Section numbers (§12.x) are stable and cited from code, tests, and commit messages.

# 12. Command-line interface

Implemented in `src/main.rs`. This is a contract, not an implementation
detail: scripts depend on it, so it is specified here and pinned by the CLI
tests in `tests/integration.rs`.

The CLI adds **no behaviour** of its own. Everything it prints is the
library's output, formatted ([§1.1](01-purpose-and-scope.md)).

## 12.1 Synopsis

```
er7 [OPTIONS] [FILE]
```

`FILE` holds one or more messages, or a batch file. `-` or no argument
reads standard input. Input is split per [§9](09-batch-input.md), and
**every message is parsed before anything is written**, so a malformed
message late in a batch fails the run rather than producing half an output.

| Option | Effect |
|--------|--------|
| `-q, --query <PATH>` | print the values at an HL7 path, one per line; may be repeated, and outputs appear in the order the options were given |
| `-n, --normalize` | rewrite the input as canonical ER7 ([§7.2](07-writing.md)), with a trailing terminator on every message |
| `-m, --message <N>` | use only the Nth message of the input, counting from 1 |
| `-r, --raw` | show text as sent, without decoding escape sequences |
| `-t, --terminator <KIND>` | segment terminator to write: `cr` (default), `lf`, `crlf` |
| `-o, --output <FILE>` | write to `FILE` instead of standard output |
| `-h, --help` | print usage |
| `-V, --version` | print the version |

Combining `--query` with `--normalize` is an error: they ask for different
output, and silently preferring one would hide a mistake in a script.

## 12.2 Actions

Exactly one action runs per invocation:

| Action | Selected by | Output |
| ------ | ----------- | ------ |
| outline | the default | one line per value, labelled with its path (§12.3) |
| query | `--query` | one line per matching value |
| normalize | `--normalize` | canonical ER7 |

## 12.3 The outline

The default output, and the reason the CLI exists: it answers "which
position is this value actually in?", which is the hardest thing about a
positional format.

```
MSH-1       |
MSH-2       ^~\&
MSH-9.1     ORU
PID-3.4.2   1.2.840.114398.1.100
PID-5.1     EVERYWOMAN
PID-13[1]   555-555-1111
PID-13[2]   555-555-2222
OBX[1]-3.2  Cholesterol
OBX[2]-3.2  Triglycerides
```

Rules:

- **Every label is a valid `--query` argument.** This is the point of the
  format: a path read off the outline can be pasted straight back in.
- A level with **only one child is not indexed**, so a name sent as a
  single component reads `NTE-3`, not `NTE-3.1`. Indices appear exactly
  where they disambiguate.
- Repeated **segments** are labelled `OBX[2]-…`; repeated **fields** are
  labelled `PID-13[2]`.
- Positions with **no value are left out** entirely.
- An **explicit null is shown as the `""` it was sent as**, since that is
  exactly what distinguishes it from a field left out
  ([§8.2.1](08-paths-and-queries.md)).
- Carriage returns, line feeds, and tabs inside a decoded value are shown
  as `\r`, `\n`, and `\t`, so one value stays on one line.
- Labels are padded to a common width, clamped between 8 and 28 characters,
  then two spaces before the value.

When the input holds more than one message, each outline is preceded by a
`# message N` heading naming the message code, trigger event, control ID,
and version where the message supplies them, and a blank line separates
messages.

## 12.4 Exit codes and diagnostics

| Code | Meaning |
| ---- | ------- |
| 0 | success |
| 1 | any error: bad arguments, unreadable input, a message that failed to parse, or an unwritable output |

Diagnostics go to standard error, prefixed `er7: error: `, one line. A
message that failed to parse is identified by its 1-based position in the
input: `er7: error: message 3: input contains no HL7 segments`.

**A query that matches nothing prints nothing and still exits 0.** The
message simply did not carry that value, which is not a failure — R20 at
the CLI level. Scripts that need to distinguish should test for empty
output.

A closed output pipe (`er7 … | head -3`) exits 0 rather than reporting a
broken-pipe error, which is what every other Unix filter does.

## 12.5 Stability

The CLI is covered by the same semantic-versioning promise as the library
([§14](14-compatibility-and-versioning.md)): removing an option, changing
an exit code, or changing the outline's label format is a breaking change.
Adding an option is not.
