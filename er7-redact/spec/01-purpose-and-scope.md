[`er7-redact` specification](index.md) — section 1 of 17. Section numbers (§1.x) are stable and cited from code, tests, and commit messages.

# 1. Purpose and scope

## 1.1 Purpose

Remove or mask patient detail from HL7 v2 messages in the **ER7**
pipe-hat encoding, **without breaking the message**. A redacted message
still parses, still carries its delimiters, and still has a value in every
position that had one before — so the tools downstream of it (an interface
engine, a test harness, a message viewer) behave the same way they did on
the original.

The crate is a **library first** and a command-line tool second. The
library is the contract; the CLI ([§10](10-command-line-interface.md)) is a
thin presentation layer over it and adds no behaviour of its own.

The motivating job: a bug report arrives with a real message attached, and
somebody has to turn it into something that can be committed to a
repository, pasted into a ticket, or fed to a test suite.

## 1.2 In scope

| Capability | Section |
| ---------- | ------- |
| A policy: an ordered list of rules, each an HL7 path and an action | [§2](02-redaction-model.md) |
| Two postures: accept by default, or reject by default | [§2.6](02-redaction-model.md) |
| Eight actions — keep, clear, null, replace, mask, first, last, pseudonym | [§3](03-actions.md) |
| Shape preservation, and the absent/empty/null contract | [§4](04-what-redaction-preserves.md) |
| Four built-in policies, and the curated positions behind two of them | [§5](05-built-in-policies.md) |
| A line-oriented policy file, readable and writable | [§6](06-policy-file-format.md) |
| Stable pseudonyms, so an identifier maps the same way in every message | [§7](07-pseudonyms.md) |
| An audit report of every position that changed | [§8](08-report.md) |

## 1.3 Out of scope

**This crate is a positional editor, not a compliance tool. [D14]** It
does not know:

- whether the positions it redacts are the ones *your* senders use;
- whether what remains is de-identified under HIPAA, GDPR, or any other
  regime — that is a determination about a whole data set, made by a
  person who is accountable for it, not a property of one message;
- what free text means. An identifier written into an `NTE-3` comment or
  an `OBX-5` result stays there unless a rule names that position
  ([§5.4](05-built-in-policies.md));
- how to re-identify anything. There is no key escrow, no mapping table,
  and no undo.

It also does not parse, validate, or transport messages: that is `er7`'s
job and the `er7` spec's §1.3.

A message this crate has redacted is **a message with less in it**, which
is progress, and is not the same thing as a safe one. See
[§5.5](05-built-in-policies.md) for how to think about the gap.

## 1.4 Rule index (D1–D21)

Every behavioural rule the crate guarantees, with a stable ID. Prose,
tests, code comments, and commit messages cite these. **IDs are never
reused.** [§11.1](11-testing-strategy.md) maps each rule to the test that
enforces it.

| ID | Rule | Section |
| -- | ---- | ------- |
| D1 | Redaction rewrites leaf text only; the shape of the message is preserved. | [§4.1](04-what-redaction-preserves.md) |
| D2 | Redaction never creates a position the message did not carry. | [§4.2](04-what-redaction-preserves.md) |
| D3 | A leaf that carried no text is left empty; redaction never invents a value. | [§4.3](04-what-redaction-preserves.md) |
| D4 | An explicit null stays null; redaction never turns "clear this" into a value. | [§4.3](04-what-redaction-preserves.md) |
| D5 | The header's delimiter fields are never redacted, by any rule. | [§4.4](04-what-redaction-preserves.md) |
| D6 | `Null` is the only action that changes shape: it collapses the named position to `""`. | [§3.4](03-actions.md) |
| D7 | Rules apply in order, each to the message as it stands. | [§2.4](02-redaction-model.md) |
| D8 | A rule that matches nothing is not an error. | [§2.5](02-redaction-model.md) |
| D9 | A policy accepts or rejects by default; rejecting applies its action to every leaf no rule named. | [§2.6](02-redaction-model.md) |
| D10 | Every action except `Pseudonym` is idempotent. | [§3.6](03-actions.md) |
| D11 | Replacement text is encoded on the way in, so redaction can never introduce a delimiter. | [§3.5](03-actions.md) |
| D12 | A pseudonym is stable for a given key and value, and is not a cryptographic guarantee. | [§7](07-pseudonyms.md) |
| D13 | A report never contains the text that was redacted. | [§8.2](08-report.md) |
| D14 | The built-in policies are a starting point, not a compliance certification. | [§1.3](01-purpose-and-scope.md), [§5](05-built-in-policies.md) |
| D15 | Errors arise from exactly two situations: a policy that cannot be read, and a path that is not a path. | [§9](09-error-handling.md) |
| D16 | Exactly one runtime dependency: `er7`. | [§12](12-dependencies-and-build.md) |
| D17 | A message no rule touches is written back byte for byte. | [§4.5](04-what-redaction-preserves.md) |
| D18 | The policy file format is one rule per line, and it is part of the public API. | [§6](06-policy-file-format.md) |
| D19 | A reject rule beats an accept rule for the same leaf, whichever order they are in. | [§2.4](02-redaction-model.md) |
| D20 | Appending one policy to another never weakens what it does by default. | [§2.6](02-redaction-model.md) |
| D21 | A payload that is not ER7 is refused, passed through, or acted on whole, as the policy says. | [§2.8](02-redaction-model.md) |

The next rule ID is **D22**.

## 1.5 Design priorities, in order

When two goals conflict, the earlier one wins. This ordering is what makes
the rest of the spec predictable.

1. **Privacy.** When the choice is between redacting too much and too
   little, redact. A value that should have been removed and was not is
   the failure that cannot be undone; a value that was removed and was
   wanted is a policy edit away.
2. **Structural fidelity.** A redacted message must still parse, and every
   value must still be in the position it was in (D1, D2). A redaction
   that shifts a field is worse than no redaction at all: it is a message
   that looks fine and says something else.
3. **Clinical meaning.** Never turn "clear this value" into a value (D4),
   and never invent one where nothing was sent (D3). The `er7` spec's §5.3
   applies here unchanged.
4. **Predictability.** The same policy, key, and message always produce the
   same output, byte for byte. Redaction is run in scripts and in CI, and
   a diff that moves on its own is unusable as an audit trail.
5. **Ergonomics.** Paths, the policy file, and the CLI exist to make the
   positional structure legible. They come last: a convenience that costs
   any of the four goals above is not a convenience.
