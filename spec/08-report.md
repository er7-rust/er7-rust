[`er7-redact` specification](index.md) — section 8 of 17. Section numbers (§8.x) are stable and cited from code, tests, and commit messages.

# 8. The report

Implemented in `src/redact.rs`.

## 8.1 What it is

`Redactor::redact` returns a `Report`: the list of positions it changed,
each with the action that changed it, in the order the changes were made —
rule by rule, and in message order within each rule.

```rust
let report = redactor.redact(&mut message);

for change in &report.changes {
    println!("{} {}", change.path, change.action);
}
// PID[1]-3[1].1.1 pseudonym
// PID[1]-5[1].1.1 replace REDACTED
// PID[1]-5[1].2.1 replace REDACTED
// PID[1]-7[1].1.1 first 4
```

It answers the question a reviewer actually asks — *what did this thing
do to my message?* — and it answers it about the run that happened, not
about the policy that was intended.

## 8.2 A report never contains redacted text [D13]

A `Change` carries the **path** and the **action**, and nothing else. Not
the old value, not the new one.

A log line reading `PID-5.1: "EVERYWOMAN" → "REDACTED"` puts the patient
name into the log, the terminal scrollback, and the CI transcript — the
three places nobody thought to protect. A report that can be pasted into a
ticket without a second thought is worth more than one that shows its
work.

The new value is not included either, since including it would leak the
old one for `First`, `Last`, and `Mask`.

## 8.3 Paths are fully qualified

Every path in a report carries every index, including the ones that would
be unambiguous:

```
PID[1]-5[1].1.1
OBX[2]-5[1].1.1
```

Two reasons. A fully qualified path is a valid `er7 --query` argument, so
a row can be pasted straight into the `er7` CLI to see what is there now.
And a report is a record: `PID-5` is ambiguous about which of three
repetitions changed, and an audit trail that has to be interpreted is not
one.

## 8.4 One row per change, not per rule

A rule that names a field with three components produces three rows —
one per leaf that actually changed. A rule that matched nothing produces
none (D8, [§2.5](02-redaction-model.md)). A position that two rules both
named produces two rows, in the order the rules ran (D7), which is what
makes an unintended overlap visible.

`Action::Keep` never produces a row: nothing changed. Neither does an
action applied to an empty or null leaf, since those are skipped before
the action runs (D3, D4).

## 8.5 What a report is for

- **Review.** Read it before sharing the message, and check that the
  positions you expected are all there.
- **Regression.** Assert on it in a test. A report is stable output (D7,
  [§2.7](02-redaction-model.md)), so a change to the policy shows up as a
  diff.
- **Evidence.** Keep it beside the redacted message as a record of what
  was done to it.

It is not a guarantee that everything sensitive was found — only that
these positions were changed. See
[§5.5](05-built-in-policies.md).
