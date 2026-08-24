[`er7-redact` specification](../index.md) — section 13 of 17. Section
numbers (§13.x) are stable and cited from code, tests, and commit messages.

# 13. Compatibility and versioning

## 13.1 Semantic versioning, and what counts as public

The crate follows semantic versioning. Its public surface is larger than
its Rust API, because three other things are depended on by callers who
never write Rust at all:

| Surface | Breaking to change |
| ------- | ------------------ |
| the Rust API | adding a variant to `Action` or `Error`, removing an item, changing a signature |
| the **policy file format** ([§6](../06-policy-file-format/index.md), D18) | how a line is read, an action's spelling, the meaning of `*` |
| the **CLI** ([§10](../10-command-line-interface/index.md)) | removing an option, changing an exit code, changing which policy runs by default, changing the report layout |
| the **posture** a policy or a flag carries ([§2.6](../02-redaction-model/index.md)) | changing which of accept or reject a built-in policy, a policy file line, or a CLI flag means |
| the **pseudonym function** ([§7](../07-pseudonyms/index.md), D12) | any change to the value it returns, for any key and input |

## 13.2 The pseudonym function is frozen

Stronger than the rest, and worth its own clause: `pseudonym(key, value)`
returns the same sixteen characters in every release of this crate, major
versions included.

A pseudonym is a join key. Somebody has a data set redacted last year and
a message redacted today, and the whole point was that the patient matches
across them. A crate that changed the function in 2.0 would silently split
that data set in two, and the breakage would surface as "these are
different patients" — a conclusion, not an error.

If a stronger construction is ever added, it is added **alongside**, as a
new action with a new name
([§16.3](../16-open-questions-and-declined-decisions/index.md)).

## 13.3 The built-in policies may change in a minor release

The opposite treatment, and also deliberate: `Policy::patient_identifiers()`
may gain positions in a minor release.

The alternative is worse. If a position turns out to carry patient detail
and the table cannot change until a major version, then every caller
running the default policy keeps leaking it until they upgrade a major
version — and the crate's first priority is privacy
([§1.5](../01-purpose-and-scope/index.md)).

So: **the set of positions the default policy names is not a compatibility
surface, and grows only.** A position is never *removed* in a minor
release, because that would silently stop redacting something. A caller
who needs a policy frozen against this should write it to a file
(`--show-policy`) and check the file in — which is the reason
`--show-policy` exists.

Every change to the table is a change to
[§5.1](../05-built-in-policies/index.md) in the same commit, and appears in
the release notes.

## 13.4 Dependency on `er7`

`Cargo.toml` depends on `er7` by path (`{ path = "../er7", version = "0" }`)
so the workspace picks up local changes to `er7` immediately; the
`version` requirement is `"0"` while `er7` is pre-1.0, moving to `"1"` when
it releases, and is what governs compatibility once this crate is
published on its own. Types from `er7` — `Message`, `Path`, `Error` —
appear in this crate's public API, so an `er7` major version is a major
version here too.

## 13.5 Before 1.0

While the version is `0.x`:

- a breaking change bumps the minor version (`0.1` → `0.2`);
- the pseudonym freeze ([§13.2](#132-the-pseudonym-function-is-frozen))
  applies from 0.1.0 onward, since data redacted with an early version is
  no less permanent;
- the spec is the contract; if the code disagrees, the spec is right.
