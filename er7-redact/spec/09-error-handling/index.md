[`er7-redact` specification](../index.md) — section 9 of 17. Section
numbers (§9.x) are stable and cited from code, tests, and commit messages.

# 9. Error handling

Implemented in `src/lib.rs`.

## 9.1 Two variants, two situations [D15]

```rust
pub enum Error {
    /// A policy line could not be read. Carries the line number, the line,
    /// and what was wrong.
    BadPolicy(String),
    /// A path is not a path. Carries the `er7` error.
    Er7(er7::Error),
}
```

| Situation | Variant |
| --------- | ------- |
| a policy file with an unknown action, a missing action, or a malformed count | `BadPolicy` |
| a **policy line** naming something that is not an HL7® path | `BadPolicy`, wrapping `er7`'s wording and naming the line number |
| a `Policy::with`, `Rule::new` **call** naming something that is not an HL7 path | `Er7(er7::Error::BadPath(_))` |

The split follows what the caller can act on. A bad path in a file needs
the line number to be findable at all; a bad path in a function call is
already located by the compiler and the stack, and the caller may want to
match on `er7`'s own variant.

`From<er7::Error>` converts, so `?` works in a function returning either.

## 9.2 Redaction itself cannot fail

`Redactor::redact` returns a `Report`, not a `Result`. Once a policy has
been read and a message parsed, there is nothing left that can go wrong:

- a rule that matches nothing does nothing (D8);
- a position that does not exist is not created (D2);
- an empty or null leaf is skipped (D3, D4);
- a delimiter field is skipped (D5);
- replacement text that contains a delimiter is escaped (D11).

This mirrors the `er7` spec's R6 — below the header, nothing fails — for
the same reason: a redactor that refuses a message leaves the caller
holding the unredacted original, which is the worse outcome.

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
