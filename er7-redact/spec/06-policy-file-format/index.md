[`er7-redact` specification](../index.md) — section 6 of 17. Section
numbers (§6.x) are stable and cited from code, tests, and commit messages.

# 6. The policy file format

Implemented in `src/policy.rs` (`Policy::parse` and `Display for Policy`).

**The format is part of this crate's public API [D18].** Policy files live
in repositories and are reviewed like code, so a change to how a line is
read is a breaking change
([§13](../13-compatibility-and-versioning/index.md)).

## 6.1 Shape

One rule per line: a path, whitespace, an action. Four reserved first
words — `accept`, `reject`, `unrecognised`, and `known-values` — set what
the policy does by default instead of naming a position
([§6.3](#63-the-default-lines)).

```
# de-identify.policy — for messages from the LAB interface

PID-3    pseudonym
PID-5    replace REDACTED
PID-7    first 4
PID-11   clear
PID-19   clear
NK1-2    replace
OBX-5    keep

accept
```

- **Blank lines are ignored.**
- **A `#` starts a comment**, either on its own line or after a rule. A
  `#` inside an action's argument ends the line there too, so a policy
  that must write a `#` — as replacement text, or as a mask character —
  should build the rule in Rust rather than in a file
  ([§16.4](../16-open-questions-and-declined-decisions/index.md)).
- Leading and trailing whitespace is ignored; the separator between the
  path and the action is any run of spaces or tabs.
- Rules are kept **in file order**, and order is significant (D7,
  [§2.4](../02-redaction-model/index.md)) — except between a `keep` rule
  and a rejecting rule for the same position, where the rejecting one wins
  whichever came first (D19).
- A file that states no posture **accepts by default**, so a file of
  nothing but rules means "redact these".

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

## 6.3 The default lines

Four first words are reserved. Each sets one of the policy's defaults
([§2.6](../02-redaction-model/index.md),
[§2.8](../02-redaction-model/index.md),
[§2.10](../02-redaction-model/index.md)) rather than adding a rule, and
each may appear anywhere in the file:

| Written | Means |
| ------- | ----- |
| `accept` | accept by default: a leaf no rule named is left as it is |
| `reject` | reject by default with `replace REDACTED` |
| `reject ACTION` | reject by default with `ACTION`, in the grammar of [§6.2](#62-action-grammar) |
| `unrecognised refuse` | a payload that is not ER7 fails the run |
| `unrecognised pass` | a payload that is not ER7 is written out unchanged |
| `unrecognised ACTION` | a payload that is not ER7 has `ACTION` applied to it whole |
| `known-values on` | a value found at a named position is redacted wherever else it appears (D23) — the default |
| `known-values off` | positional rules and the posture are the whole of what this policy does |

```
MSH  keep

reject        replace REDACTED
unrecognised  mask *
known-values  off
```

The reserved words are matched **case-insensitively**, like action names,
and `unrecognized` is accepted for `unrecognised`. Segment names are three
characters, so none of the four can collide with a path.

A second `accept` or `reject` line replaces the first, and so does a
second `unrecognised` or `known-values` line: a policy has one of each,
and silently keeping the earlier one would hide an editing mistake.

A file that never mentions `known-values` gets `on` — the same "state it
or get the safer answer" the other two defaults already use. Turning it
off is always explicit, in the file or with
`Policy::search_known_values(false)`, and appending a policy can only turn
it on, never off, for the same reason a posture can only get stricter
(D20, [§2.6](../02-redaction-model/index.md)).

`reject keep` is legal and means `accept`, and so does `unrecognised keep`
for `unrecognised pass` — rejecting a leaf by leaving it alone is not
rejecting it. Each is normalised on the way in, so that a policy written
back out says what it does ([§6.5](#65-writing-a-policy-back-out)).

Concatenating files is **not** the same as writing one: appending a policy
can only make the defaults stricter, never looser (D20,
[§2.6](../02-redaction-model/index.md)).

### The `*` line, removed

Before 0.2 the posture was written `* ACTION`, with `* keep` for "no
fallback". A `*` line is now an error naming its replacement:

```
policy line 9: "* replace REDACTED": the default line is now "reject replace REDACTED", not "*"
```

That is a breaking change to a compatibility surface (D18,
[§13](../13-compatibility-and-versioning/index.md)), and it is an error
rather than a silent synonym on purpose: `*` said nothing about which of
the two postures it meant, which is the whole of what this format now has
to say.

## 6.4 Errors

A malformed line is an error naming the line number and what was wrong
([§9](../09-error-handling/index.md)):

```
policy line 4: "PID-5 obfuscate": unknown action "obfuscate"
policy line 7: "PID-0 clear": invalid HL7 path "PID-0": indices are 1-based, so 0 is not a position
policy line 9: "first 4": expected a path and an action
policy line 11: "accept everything": "accept" takes no argument, but got "everything"
policy line 13: "unrecognised": "unrecognised" wants "refuse", "pass", or an action
```

Reading a policy is the one place this crate is strict. A typo in a policy
file means a value that should have been redacted was not, and that
failure is silent, permanent, and exactly what the crate exists to
prevent — so it is caught at load time instead
([§1.5](../01-purpose-and-scope/index.md), priority 1).

## 6.5 Writing a policy back out

`Policy` implements `Display`, and its output re-parses to an equal
policy. The canonical form is one rule per line, with the paths padded to a
common width and at least two spaces before the action; then a blank line;
then the three default lines, always all three, whatever they say:

```
PID-3  pseudonym
PID-5  replace REDACTED

reject        replace REDACTED
unrecognised  refuse
known-values  on
```

The defaults are written out **even when they are the quiet ones**. A
policy file that ends `accept` / `unrecognised refuse` / `known-values on`
is longer than one that ends nowhere, and it is the difference between a
reader knowing what happens to everything the file does not name and a
reader assuming it.

This is what the CLI's `--show-policy` prints
([§10](../10-command-line-interface/index.md)), and it is how a caller
turns the built-in policy into a file to edit:

```sh
er7-redact --show-policy > de-identify.policy
```
