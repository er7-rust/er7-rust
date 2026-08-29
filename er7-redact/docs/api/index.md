[er7-redact](../../index.md) → docs → api

# API reference

The complete public surface. Rendered rustdoc is at
<https://docs.rs/er7-redact/>; this page is the map.

## Modules

| Module | Holds | Spec |
| ------ | ----- | ---- |
| `er7_redact` | `Error`, and re-exports of everything below | [§9](../../spec/09-error-handling/index.md) |
| `er7_redact::action` | `Action` | [§3](../../spec/03-actions/index.md) |
| `er7_redact::policy` | `Rule`, `Policy`, `Posture`, `Unrecognised` | [§5](../../spec/05-built-in-policies/index.md), [§6](../../spec/06-policy-file-format/index.md) |
| `er7_redact::pseudonym` | `pseudonym` | [§7](../../spec/07-pseudonyms/index.md) |
| `er7_redact::redact` | `Redactor`, `Report`, `Change` | [§2](../../spec/02-redaction-model/index.md), [§8](../../spec/08-report/index.md) |

Everything is re-exported at the crate root, so `use er7_redact::{Action,
Policy, Redactor};` is the usual import.

## `Redactor`

The only thing that edits a message.

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `new` | `fn new(policy: Policy) -> Redactor` | pseudonym key defaults to `0` |
| `with_key` | `fn with_key(self, key: u64) -> Redactor` | builder |
| `redact` | `fn redact(&self, message: &mut er7::Message) -> Report` | edits in place; cannot fail |
| `uncovered` | `fn uncovered(&self, message: &er7::Message) -> Vec<er7::Path>` | every leaf with text that no rule names; read-only |
| `unrecognised` | `fn unrecognised(&self, payload: &str) -> Option<String>` | what to write for a payload that is not ER7; `None` means the policy refuses it |
| `policy` | `fn policy(&self) -> &Policy` | |
| `key` | `fn key(&self) -> u64` | |
| `Default` | `Redactor::default()` | `patient_identifiers()`, key `0` |

## `Policy`

| Item | Signature | Notes |
| ---- | --------- | ----- |
| `rules` | `Vec<Rule>` | public; applied in order |
| `posture` | `Posture` | public; what every leaf no rule named gets |
| `unrecognised` | `Unrecognised` | public; what a payload that is not ER7 gets |
| `search_known_values` | `bool` | public; sweep for a value found at a named position wherever else it appears (D23, [spec §2.10](../../spec/02-redaction-model/index.md)); defaults to `true` |
| `accept_all` | `fn accept_all() -> Policy` | no rules; accepts, and passes an unrecognised payload |
| `reject_all` | `fn reject_all() -> Policy` | no rules; `replace REDACTED` over everything, masks an unrecognised payload |
| `patient_identifiers` | `fn patient_identifiers() -> Policy` | the curated table, [spec §5.1](../../spec/05-built-in-policies/index.md); accepts, refuses |
| `all_but_the_header` | `fn all_but_the_header() -> Policy` | `MSH keep`, then `reject replace REDACTED`; refuses |
| `with` | `fn with(self, path: &str, action: Action) -> Result<Policy, Error>` | builder |
| `posture` | `fn posture(self, posture: Posture) -> Policy` | `Reject(Keep)` normalises to `Accept` |
| `on_unrecognised` | `fn on_unrecognised(self, u: Unrecognised) -> Policy` | `Apply(Keep)` and `Apply(Null)` normalise to `Pass` |
| `search_known_values` | `fn search_known_values(self, search: bool) -> Policy` | builder for the field above |
| `parse` | `fn parse(text: &str) -> Result<Policy, Error>` | the file format; a file that says nothing accepts, **refuses**, and searches known values |
| `append` | `fn append(&mut self, other: Policy)` | rules in order; the stricter posture (D20); the appended disposition; `search_known_values` only turns on |
| `is_empty` | `fn is_empty(&self) -> bool` | no rules, and accepts by default |
| `Display` | | the canonical policy file, all three defaults stated; re-parses to an equal policy |

There is deliberately **no** `Policy::new` and no `Default`: a policy
cannot exist without saying which posture it takes.

## `Posture` and `Unrecognised`

```rust
pub enum Posture {
    Accept,
    Reject(Action),
}

pub enum Unrecognised {
    Pass,
    Apply(Action),
    Refuse,
}
```

`Posture` is what every leaf no rule named gets
([spec §2.6](../../spec/02-redaction-model/index.md)); `Unrecognised` is
what a payload that is not ER7 gets
([spec §2.8](../../spec/02-redaction-model/index.md)). Both implement
`Display` in the policy file spelling — `accept`, `reject clear`, `refuse`,
`pass`, `mask *`.

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
new ([spec §8.2](../../spec/08-report/index.md)).

## `pseudonym`

```rust
pub fn pseudonym(key: u64, value: &str) -> String
```

Sixteen lowercase hexadecimal characters, stable for a given key and value
in every release of this crate. Not a cryptographic guarantee — read
[spec §7.3](../../spec/07-pseudonyms/index.md).

## `Error`

```rust
pub enum Error {
    BadPolicy(String),
    Er7(er7::Error),
}
```

Two variants, two situations
([spec §9](../../spec/09-error-handling/index.md)). `From<er7::Error>`
converts; `Display` is one sentence with no prefix; `source()` returns the
`er7` error for the `Er7` variant. Redaction itself returns a `Report`, not
a `Result`.

## Types from `er7`

`er7::Message`, `er7::Path`, and `er7::Error` appear in this crate's public
API, so an `er7` major version is a major version here too
([spec §13.4](../../spec/13-compatibility-and-versioning/index.md)).
