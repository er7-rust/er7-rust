[AGENTS.md](../AGENTS.md) → release

# Release

The operational checklist is
[`help/releasing/`](../help/releasing/index.md); this is the policy behind
it.

## Versioning

Semantic versioning, over a public surface that is larger than the Rust API
([spec §13](../spec/13-compatibility-and-versioning/index.md)):

| Surface | Breaking to change |
| ------- | ------------------ |
| the Rust API | adding an `Action` or `Error` variant, removing an item, changing a signature |
| the policy file format | how a line is read, an action's spelling, the meaning of `*` |
| the CLI | removing an option, changing an exit code or the default policy, changing the report layout |
| `pseudonym` | **any** change to what it returns, ever |

While `0.x`, a breaking change bumps the minor version.

## The two asymmetries

- **`pseudonym` is frozen forever**, major versions included. It is a join
  key across data sets redacted years apart.
- **The built-in policy may grow in a minor release**, and never shrinks. A
  position that turns out to carry patient detail should start being
  redacted at the next release, not the next major version.

Both are spelled out in
[spec §13](../spec/13-compatibility-and-versioning/index.md); neither is
negotiable in a release without editing that section first.

## Before publishing

1. The four checks are clean ([workflows](workflows.md)).
2. `spec/15-open-tasks/index.md` has no task that this release claims to
   close.
3. Any change to `Policy::patient_identifiers` is reflected in
   [spec §5.1](../spec/05-built-in-policies/index.md) and named in the
   release notes.
4. `cargo package --list` holds no stray file, and no message that is not
   synthetic.
5. Version bumped in `Cargo.toml`, `CITATION.cff`, and the spec index's
   "Applies to" line.
