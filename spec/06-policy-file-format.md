[`er7-redact` specification](index.md) — section 6 of 17. Section numbers (§6.x) are stable and cited from code, tests, and commit messages.

# 6. The policy file format

Implemented in `src/policy.rs` (`Policy::parse` and `Display for Policy`).

**The format is part of this crate's public API [D18].** Policy files live
in repositories and are reviewed like code, so a change to how a line is
read is a breaking change ([§13](13-compatibility-and-versioning.md)).

## 6.1 Shape

One rule per line: a path, whitespace, an action.

```
# de-identify.policy — for messages from the LAB interface

PID-3    pseudonym
PID-5    replace REDACTED
PID-7    first 4
PID-11   clear
PID-19   clear
NK1-2    replace
OBX-5    keep
```

- **Blank lines are ignored.**
- **A `#` starts a comment**, either on its own line or after a rule. A
  `#` inside an action's argument ends the line there too, so a policy
  that must write a `#` — as replacement text, or as a mask character —
  should build the rule in Rust rather than in a file
  ([§16.4](16-open-questions-and-declined-decisions.md)).
- Leading and trailing whitespace is ignored; the separator between the
  path and the action is any run of spaces or tabs.
- Rules are kept **in file order**, and order is significant (D7,
  [§2.4](02-redaction-model.md)).

## 6.2 Action grammar

| Written | Action | Notes |
| ------- | ------ | ----- |
| `keep` | `Keep` | |
| `clear` | `Clear` | |
| `null` | `Null` | |
| `replace` | `Replace("REDACTED")` | the argument may be omitted |
| `replace TEXT` | `Replace(TEXT)` | `TEXT` is the rest of the line, trimmed; it may contain spaces |
| `mask` | `Mask('*')` | the argument may be omitted |
| `mask C` | `Mask(C)` | `C` is one character |
| `first N` | `First(N)` | `N` is a non-negative integer |
| `last N` | `Last(N)` | as `first` |
| `pseudonym` | `Pseudonym` | |

Action names are matched **case-insensitively** (`CLEAR` and `clear` are
the same action); replacement text is taken as written.

## 6.3 The fallback line

A path of `*` sets the policy's fallback ([§2.6](02-redaction-model.md))
rather than adding a rule:

```
MSH  keep
*    replace REDACTED
```

`*` may appear anywhere in the file; the fallback always runs last. A
second `*` line replaces the first — a policy has one fallback, and
silently keeping the earlier one would hide an editing mistake.

`* keep` is legal and means "no fallback", which is also the default.

## 6.4 Errors

A malformed line is an error naming the line number and what was wrong
([§9](09-error-handling.md)):

```
policy line 4: "PID-5 obfuscate": unknown action "obfuscate"
policy line 7: "PID-0 clear": invalid HL7 path "PID-0": indices are 1-based, so 0 is not a position
policy line 9: "first 4": expected a path and an action
```

Reading a policy is the one place this crate is strict. A typo in a policy
file means a value that should have been redacted was not, and that
failure is silent, permanent, and exactly what the crate exists to
prevent — so it is caught at load time instead
([§1.5](01-purpose-and-scope.md), priority 1).

## 6.5 Writing a policy back out

`Policy` implements `Display`, and its output re-parses to an equal
policy. The canonical form is one rule per line, with the paths padded to a
common width and at least two spaces before the action, and the fallback
last:

```
PID-3  pseudonym
PID-5  replace REDACTED
*      replace REDACTED
```

This is what the CLI's `--show-policy` prints
([§10](10-command-line-interface.md)), and it is how a caller turns the
built-in policy into a file to edit:

```sh
er7-redact --show-policy > de-identify.policy
```
