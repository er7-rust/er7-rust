[`er7-redact` specification](../index.md) — section 9 of 17. Section
numbers (§9.x) are stable and cited from code, tests, and commit messages.

# 9. Error handling

Implemented in `src/lib.rs`.

## 9.1 Four variants, four situations [D15]

```rust
pub enum Error {
    /// A policy line could not be read. Carries the line number, the line,
    /// and what was wrong.
    BadPolicy(String),
    /// A path is not a path. Carries the `er7` error.
    Er7(er7::Error),
    /// The `unrecognised` disposition applies an action that writes
    /// nothing (`keep`, `null`, or a custom action returning `None`).
    NoReplacement { action: String },
    /// An action wrote nothing for a leaf that carries text (a custom
    /// action returning `None`), so `Redactor::redact` refuses to leave
    /// the text in place.
    NoOutput { action: String, path: String },
}
```

| Situation | Variant |
| --------- | ------- |
| a policy file with an unknown action, a missing action, or a malformed count | `BadPolicy` |
| a **policy line** naming something that is not an HL7® path | `BadPolicy`, wrapping `er7`'s wording and naming the line number |
| a `Policy::with`, `Rule::new` **call** naming something that is not an HL7 path | `Er7(er7::Error::BadPath(_))` |
| `Redactor::unrecognised` with `Unrecognised::Apply(action)` where `action` writes nothing | `NoReplacement { action }` |
| `Redactor::redact` where an action writes nothing for a leaf that carries text | `NoOutput { action, path }` |

`NoReplacement` fails closed (spec/rust-checked-arithmetic RCA-IN-2,
RCA-IN-6). `Policy::on_unrecognised` and policy files normalise `keep` and
`null` to `pass`, so it is reachable only by assigning the public
`Policy::unrecognised` field or through a custom action. Returning the
payload instead would write unredacted input under a policy that asked for
redaction. The CLI reports it as `message N: ...` and exits 1.

The split follows what the caller can act on. A bad path in a file needs
the line number to be findable at all; a bad path in a function call is
already located by the compiler and the stack, and the caller may want to
match on `er7`'s own variant.

`From<er7::Error>` converts, so `?` works in a function returning either.

## 9.2 Redacting a parsed message fails closed

`Redactor::redact` returns `Result<Report, Error>`. This replaces an earlier
rule, "redaction itself cannot fail", which returned a bare `Report`. It was
a breaking change, made so that no input can leave text in a message that a
policy asked to redact (spec/rust-checked-arithmetic RCA-IN-2, RCA-IN-6).

The one failure is an action that **writes nothing** for a leaf that
carries text. Only a custom action returning `None` does this: `Keep` and
`Null` are handled before an action is asked for text, and `Keep` is the
deliberate way to leave a position alone. Leaving the text in place would
look exactly like success and keep possible PHI, so `redact` returns
`Error::NoOutput` naming the action and the fully qualified path of the
leaf (never its value). The action may be a rule's, the rejecting posture's,
or a known value's in the sweep (§2.10).

`redact` is atomic: it works on a copy and replaces the caller's message
only on success. On `Err` the message is exactly as it was passed in, with
nothing half-redacted, and **it is unredacted**, so the caller must not
write it out. The CLI aborts the run before writing anything and exits 1,
reporting `message N: ...` (§10.4).

Everything else below the policy is still a no-op, not an error:

- a rule that matches nothing does nothing (D8);
- a position that does not exist is not created (D2);
- an empty or null leaf is skipped, whatever the action (D3, D4);
- a delimiter field is skipped (D5);
- replacement text that contains a delimiter is escaped (D11).

This departs from the `er7` spec's R6, below the header nothing fails, on
purpose: a redactor that silently keeps what it was told to remove is the
worse outcome, and a refusal here costs the caller nothing but the
message it must not publish anyway.

## 9.3 Strictness is at the edges

The crate is strict in exactly one place, and it is deliberate: reading a
policy ([§6.4](../06-policy-file-format/index.md)). A typo there means a
value that should have been redacted silently was not.

Everything else is tolerant. A policy naming a segment the message does
not have, a field beyond the end of a segment, a repetition that is not
there — all of it is a no-op.

## 9.4 Messages

An `Error` displays as one complete sentence, with no trailing period and
no error prefix, so it reads correctly whether a caller writes `{e}`,
wraps it, or prefixes it as the CLI does:

```
policy line 4: "PID-5 obfuscate": unknown action "obfuscate"
invalid HL7 path "PID-0": indices are 1-based, so 0 is not a position
```

The second is `er7`'s own text, passed through unchanged rather than
reworded, so that a path error reads the same whichever crate reported it.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
