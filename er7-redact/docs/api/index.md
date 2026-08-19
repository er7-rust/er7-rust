[er7-redact](../../index.md) → docs → api

# API reference

The complete public surface. Rendered rustdoc is at
<https://docs.rs/er7-redact/>; this page is the map.

## Modules

| Module | Holds | Spec |
| ------ | ----- | ---- |
| `er7_redact` | `Error`, and re-exports of everything below | [§9](../../spec/09-error-handling.md) |
| `er7_redact::action` | `Action` | [§3](../../spec/03-actions.md) |
| `er7_redact::policy` | `Rule`, `Policy` | [§5](../../spec/05-built-in-policies.md), [§6](../../spec/06-policy-file-format.md) |
| `er7_redact::pseudonym` | `pseudonym` | [§7](../../spec/07-pseudonyms.md) |
| `er7_redact::redact` | `Redactor`, `Report`, `Change` | [§2](../../spec/02-redaction-model.md), [§8](../../spec/08-report.md) |

Everything is re-exported at the crate root, so `use er7_redact::{Action,
Policy, Redactor};` is the usual import.

## `Redactor`

The only thing that edits a message.

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `new` | `fn new(policy: Policy) -> Redactor` | pseudonym key defaults to `0` |
| `with_key` | `fn with_key(self, key: u64) -> Redactor` | builder |
| `redact` | `fn redact(&self, message: &mut er7::Message) -> Report` | edits in place; cannot fail |
| `policy` | `fn policy(&self) -> &Policy` | |
| `key` | `fn key(&self) -> u64` | |
| `Default` | `Redactor::default()` | `patient_identifiers()`, key `0` |

## `Policy`

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `rules` | `Vec<Rule>` | public; applied in order |
| `fallback` | `Option<Action>` | public; applied last, to every leaf no rule named |
| `new` | `fn new() -> Policy` | empty. There is deliberately **no** `Default` |
| `patient_identifiers` | `fn patient_identifiers() -> Policy` | the curated table, [spec §5.1](../../spec/05-built-in-policies.md) |
| `everything` | `fn everything() -> Policy` | `MSH keep`, then a fallback of `replace REDACTED` |
| `with` | `fn with(self, path: &str, action: Action) -> Result<Policy, Error>` | builder |
| `fallback` | `fn fallback(self, action: Action) -> Policy` | `Action::Keep` means "none" |
| `parse` | `fn parse(text: &str) -> Result<Policy, Error>` | the file format |
| `append` | `fn append(&mut self, other: Policy)` | concatenate, order preserved |
| `is_empty` | `fn is_empty(&self) -> bool` | no rules and no fallback |
| `Display` | | the canonical policy file; re-parses to an equal policy |

## `Rule`

| Item | Signature |
| ---- | --------- |
| `path` | `er7::Path` |
| `action` | `Action` |
| `new` | `fn new(path: &str, action: Action) -> Result<Rule, Error>` |
| `parse` | `fn parse(line: &str) -> Result<Rule, Error>` |
| `Display` | `PID-5 replace REDACTED` |

## `Action`

```rust
pub enum Action {
    Keep,
    Clear,
    Null,
    Replace(String),
    Mask(char),
    First(usize),
    Last(usize),
    Pseudonym,
}
```

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `redacted` | `fn redacted() -> Action` | `Replace("REDACTED")` |
| `parse` | `fn parse(text: &str) -> Result<Action, Error>` | the policy file spelling |
| `apply` | `fn apply(&self, value: &str, key: u64) -> Option<String>` | on a **decoded** value; `None` means "write nothing" |
| `Display` | | the policy file spelling |

## `Report` and `Change`

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `Report::changes` | `Vec<Change>` | in the order the changes were made |
| `Report::is_empty` | `fn is_empty(&self) -> bool` | |
| `Report::len` | `fn len(&self) -> usize` | |
| `Report::Display` | | one change per line |
| `Change::path` | `er7::Path` | fully qualified, e.g. `PID[1]-5[1].2.1` |
| `Change::action` | `Action` | |
| `Change::Display` | | `PID[1]-5[1].2.1 replace REDACTED` |

A `Change` deliberately carries **no values** — not the old text, not the
new ([spec §8.2](../../spec/08-report.md)).

## `pseudonym`

```rust
pub fn pseudonym(key: u64, value: &str) -> String
```

Sixteen lowercase hexadecimal characters, stable for a given key and value
in every release of this crate. Not a cryptographic guarantee — read
[spec §7.3](../../spec/07-pseudonyms.md).

## `Error`

```rust
pub enum Error {
    BadPolicy(String),
    Er7(er7::Error),
}
```

Two variants, two situations ([spec §9](../../spec/09-error-handling.md)).
`From<er7::Error>` converts; `Display` is one sentence with no prefix;
`source()` returns the `er7` error for the `Er7` variant. Redaction itself
returns a `Report`, not a `Result`.

## Types from `er7`

`er7::Message`, `er7::Path`, and `er7::Error` appear in this crate's public
API, so an `er7` major version is a major version here too
([spec §13.4](../../spec/13-compatibility-and-versioning.md)).
