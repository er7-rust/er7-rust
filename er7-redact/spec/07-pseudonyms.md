[`er7-redact` specification](index.md) — section 7 of 17. Section numbers (§7.x) are stable and cited from code, tests, and commit messages.

# 7. Pseudonyms

Implemented in `src/pseudonym.rs`.

## 7.1 What a pseudonym is for

Clearing `PID-3` destroys the message as test data: nothing joins the
patient in this message to the same patient in the next one, and half of
what an interface does is match on identifiers. A pseudonym replaces the
identifier with a stable stand-in, so `PATID1234` becomes the same
sixteen-character token everywhere it appears, in every message redacted
with the same key.

```
PID|1||PATID1234^^^ADT1^MR||...     →   PID|1||3f2a9c1b7e0d4658^^^ADT1^MR||...
PV1|1|I|...|PATID1234|...           →   PV1|1|I|...|3f2a9c1b7e0d4658|...
```

## 7.2 The function [D12]

```
pseudonym(key: u64, value: &str) -> String
```

- **Stable.** The same key and the same decoded value always produce the
  same sixteen lowercase hexadecimal characters, on every platform, in
  every version of Rust, and — by [§13](13-compatibility-and-versioning.md)
  — in every future 0.x and 1.x release of this crate. Redacting the same
  export twice, months apart, produces the same output.
- **Keyed.** The key is a `u64`, defaulting to `0`. Two data sets redacted
  with different keys share no pseudonyms, so they cannot be joined; two
  redacted with the same key can.
- **Not cryptographic.** The construction is FNV-1a over the key bytes
  followed by the value bytes. It is a hash, not a MAC: it is fast, it is
  deterministic, and it is **not** collision-resistant against an
  adversary, not slow enough to resist a dictionary attack, and not a
  secret-keeping mechanism.

## 7.3 What a pseudonym leaks

Stated plainly, because the failure mode is subtle and the value looks
reassuringly random:

- **Equality.** By construction. Anyone can see that two messages concern
  the same patient, count how many messages each patient generated, and
  rank them — which, joined with an outside data set of the same
  population, can re-identify the largest one.
- **Everything, to anyone who can guess.** Medical record numbers come
  from small spaces. Given the key and the format, an attacker computes
  the pseudonym of every candidate in seconds and inverts the mapping
  completely. The key is the whole defence, and it is a `u64` in a config
  file, not a managed secret.

Therefore:

> **Use `Pseudonym` for data that stays inside your own trust boundary —
> test environments, internal reproductions, CI fixtures. For data leaving
> it, use `Clear` or `Replace`, which leak nothing but the fact that a
> value was there.**

This is recorded as a rule (D12) rather than a comment because it is the
one place in the crate where a caller can reasonably believe they have
more protection than they do.

## 7.4 Why not a real MAC

A keyed cryptographic hash — HMAC-SHA-256, or BLAKE3 with a key — would
answer the second point above, and it is the right answer for a crate that
wants to make a security claim. It is not implemented here for two
reasons, both recorded in
[§16.3](16-open-questions-and-declined-decisions.md):

1. It needs a dependency, and this crate has exactly one (D16).
   Hand-rolling a cryptographic primitive would be worse than either
   alternative.
2. It would make the claim look stronger without making the deployment
   safer, because the key would still be a number in a config file next to
   the data. The honest fix is a managed key, which is an application
   concern.

A caller who needs a real MAC today has to do that pass themselves: walk
the message with `er7` and write each identifier with
`Subcomponent::set`, using whatever construction their threat model calls
for. There is deliberately no `Action` variant that calls back into caller
code — that is the shape the fix would take, and it is recorded as
[T2](15-open-tasks.md) rather than guessed at now.
