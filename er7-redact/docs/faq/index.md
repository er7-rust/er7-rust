[er7-redact](../../index.md) → docs → faq

# FAQ

## Is a message this crate redacted safe to share?

No library can answer that, and this one does not pretend to. It removes
the values you named, in the positions you named, and tells you what it
did. Whether what remains identifies anybody depends on your data set, its
recipients, and what else they hold — which is a judgement a person makes
and is accountable for.

What the crate does give you is a report to review before you share, and a
`--all` mode for when "not that I listed" is not a good enough answer.

## Does it find identifiers in free text?

Not yet. A name in an `NTE-3` comment survives every positional policy, and
that is the biggest gap in the crate — recorded as
[T1](../../spec/15-open-tasks.md). Today the answers are to name those
positions (`NTE-3 clear`) or to use `Policy::everything()`.

Pattern matching was considered and declined
([spec §16.2](../../spec/16-open-questions-and-declined-decisions.md)):
`123-45-6789` is an SSN, `1.23-45.6789` is a reference range, and a
redactor that quietly destroys lab values is worse than one that misses an
identifier.

## Why does a redacted name read `REDACTED^REDACTED^REDACTED`?

Because the shape of the message is preserved. `SMITH^JOHN^Q` has three
components, and a receiver that reads `PID-5.2` has to still find something
there. Collapsing the field to one `REDACTED` would shift what everything
after it means to a component-indexing consumer
([spec §16.1](../../spec/16-open-questions-and-declined-decisions.md)).

## Why is an empty field left empty rather than redacted?

Writing `REDACTED` into a field that carried nothing invents a value — and
announces that one used to be there, which is itself a disclosure. Same for
the explicit null `""`, which is an instruction to the receiver rather than
patient data ([spec §4.3](../../spec/04-what-redaction-preserves.md)).

## `clear` or `null`?

`clear` unless you mean it. An empty field says "the sender said nothing
about this"; `""` says "delete your stored value". A redacted copy for
testing should not be telling anything to delete a record.

## Can I get the original values back?

No. There is no mapping table, no key escrow, and no undo. A crate that
could put the names back would be a crate that had to be secured, and the
whole value of a redacted export is that it does not.

## Is `pseudonym` secure?

No, and it says so. It is FNV-1a over the key and the value: stable,
deterministic, and invertible by anyone who has the key and can guess the
identifier space — which, for medical record numbers, is everyone. It
preserves equality on purpose, which is the point of it and is also a leak.

Use it inside your own trust boundary. For data leaving it, use `clear` or
`replace` ([spec §7.3](../../spec/07-pseudonyms.md)).

## Why not HMAC-SHA-256?

It needs a dependency, and this crate has exactly one. More to the point,
it would strengthen the claim more than the deployment: the key would still
be a number in a config file next to the data
([spec §16.3](../../spec/16-open-questions-and-declined-decisions.md)).

## Why does `Policy` have no `Default`?

Because either default would be a silent choice. An empty one would redact
nothing — the failure this crate exists to prevent — and a curated one
would redact forty positions without being asked. Name the policy you mean:
`Policy::new()`, `Policy::patient_identifiers()`, or `Policy::everything()`.

`Redactor::default()` does exist, and is the curated policy with key `0`.

## Why did `keep` not undo my earlier rule?

Rules apply in order, to the message as it stands, so once a value has been
replaced there is nothing left to restore it from. `keep` exempts a
position from the **fallback**, which is what it is for
([spec §2.4](../../spec/02-redaction-model.md)).

## My policy matched nothing and the command still exited 0

That is deliberate. A policy is written against a family of messages, not
one: a rule for `GT1` against a message with no guarantor segment has to be
a no-op, or every policy would need a variant per message type. Use
`--report` and test for empty output if a script needs to know.

## Will the built-in policy change?

It may **grow** in a minor release, and never shrink
([spec §13.3](../../spec/13-compatibility-and-versioning.md)). If a
position turns out to carry patient detail, waiting for a major version to
start redacting it would be the wrong trade.

If you need the exact same redaction next year, pin it:
`er7-redact --show-policy > de-identify.policy`, and check the file in.

## Will a pseudonym change?

Never. `pseudonym(key, value)` is frozen across all future releases,
including major ones, because it is a join key: a data set redacted last
year and a message redacted today have to still agree about which patient
is which ([spec §13.2](../../spec/13-compatibility-and-versioning.md)).

## Does redaction change how long the message is?

Yes, unless every action is `mask`. `Mask` preserves length — which is
exactly why it leaks a little — and `replace`, `first`, `last`, and
`pseudonym` do not.

## Can I redact a batch file?

Yes: the CLI splits input with `er7::split_messages` and redacts each
message, with `--message N` to pick one. In Rust it is a loop, one report
per message ([spec §16.7](../../spec/16-open-questions-and-declined-decisions.md)).

## Why is this a separate crate from `er7`?

`er7` is an encoding and refuses to know what `PID-5` means; that refusal
is what makes it trustworthy at the bottom of a stack. Knowing which
positions carry patient detail is a layer above, and this is that layer
([spec §5.3](../../spec/05-built-in-policies.md)).
