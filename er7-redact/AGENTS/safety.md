[AGENTS.md](../AGENTS.md) → safety

# Safety

This crate handles **clinical messages about real patients**, and its whole
job is to make them shareable. Read this before writing code that touches
behaviour.

## 1. Never put real patient data in this repository

Not in tests, not in samples, not in an example, not in a comment, not in a
commit message — including data a user pastes into a conversation.

This matters more here than in any sibling crate, because a repository
about redaction is exactly where somebody would be tempted to commit a real
message to prove the redaction works. **A redacted real message is still
real patient data**: it carries dates, facility names, identifier formats,
and whatever the policy missed.

If a user shares a real message to reproduce a bug, reproduce it with a
synthetic message of the same shape and use that.

## 2. When in doubt, redact

Priority 1 in [spec §1.5](../spec/01-purpose-and-scope.md). A value that
should have been removed and was not cannot be undone; a value that was
removed and was wanted is a policy edit away. A change that narrows what a
rule reaches needs a reason in the spec, not just a passing test.

## 3. Never claim more than the crate does

The crate is a positional editor, not a compliance tool
([spec §1.3](../spec/01-purpose-and-scope.md)). In documentation, in error
messages, and in conversation with a user:

- do not say a message is "de-identified", "anonymised", or "HIPAA
  compliant". Say which positions were changed;
- do not describe `pseudonym` as secure, hashed-and-therefore-safe, or
  irreversible. It is FNV-1a with a `u64` key
  ([spec §7.3](../spec/07-pseudonyms.md));
- do not let a built-in policy imply completeness. It is a list somebody
  wrote down.

If a user asks whether their output is safe to share, the answer names what
the crate did and what it cannot know.

## 4. Never corrupt the message

- The shape does not move (D1). A redaction that shifts a field is worse
  than no redaction: it is a message that looks fine and says something
  else.
- Never turn "clear this value" into a value (D4), and never invent one
  where nothing was sent (D3).
- Never touch the delimiter fields (D5).
- Write through `Subcomponent::set`, so a delimiter in a replacement is
  escaped rather than splitting the message (D11).

## 5. Never leak in the report or the logs

A report holds paths and actions, and no values (D13). Adding the old value
"for debugging" puts the patient's name into the log, the scrollback, and
the CI transcript.

## 6. Dependencies are an audit surface

One runtime dependency, `er7` (D16). Do not add another without the user
asking, and record what it bought in
[spec §16](../spec/16-open-questions-and-declined-decisions.md).

## If you are unsure

Say so, and stop. Write the uncertainty into
[spec §16](../spec/16-open-questions-and-declined-decisions.md) so the next
reader inherits the question rather than a guess. A recorded open question
is a good outcome; a silent assumption in a crate about privacy is not.
