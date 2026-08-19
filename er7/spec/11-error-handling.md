[`er7` specification](index.md) — section 11 of 19. Section numbers (§11.x) are stable and cited from code, tests, and commit messages.

# 11. Error handling

Implemented in `src/lib.rs`.

## 11.1 The type [R23]

```rust
pub enum Error {
    Empty,
    MissingHeader(String),
    BadHeader(String),
    BadPath(String),
}
```

**[R23]** Four variants, arising from exactly two situations: a message
with no usable header, and a path that is not a path.

| Variant | When | Raised by |
| ------- | ---- | --------- |
| `Empty` | the input held no non-blank lines | `parse` |
| `MissingHeader(name)` | the first segment is not `MSH`, `FHS`, or `BHS`; carries the name found | `parse` |
| `BadHeader(detail)` | the declared delimiters are unusable ([§3.3](03-delimiters.md)) | `parse`, `Separators::from_header`, `Separators::validate` |
| `BadPath(detail)` | a path could not be read ([§8.1](08-paths-and-queries.md)) | `Path::parse`, `FromStr for Path`, `Message::query`, `Message::query_all` |

`Error` implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Display`, and
`std::error::Error`. `Clone` and `PartialEq` are there so callers can
collect errors across a batch and compare them in tests.

`Display` produces one complete sentence, no trailing period, no error
prefix — so it reads correctly whether a caller writes `{e}`, wraps it in
`anyhow`, or prefixes it as the CLI does.

## 11.2 What is deliberately not an error [R6]

Everything else about ER7 is recoverable, and is recovered rather than
refused. A receiver that rejects a message it could have read is worse than
one that reads it as written ([§1.5](01-purpose-and-scope.md) priority 3).

Not errors:

| Situation | Behaviour |
| --------- | --------- |
| an unknown or local `Z` segment | parsed like any other |
| a segment with no fields | a `Segment` with an empty `fields` vector |
| ragged field counts between two segments of the same name | each keeps what it was sent |
| a component count that does not match any data type | kept as sent; the crate has no data types (R24) |
| an undecodable `\X..\`, or an unknown `\..\` | kept literally ([§6.2](06-escape-sequences.md)) |
| an unterminated escape character | kept literally |
| a query naming a position that is absent | contributes no value (R20) |
| a batch `BTS` count disagreeing with reality | not checked ([§9.4](09-batch-input.md)) |

`parse_with` ([§4.3](04-parsing.md)) has no failure mode at all, and so
returns `Message` rather than `Result<Message, Error>`.

## 11.3 Adding a variant

Adding a variant is a **breaking change** for callers that match
exhaustively ([§14](14-compatibility-and-versioning.md)), and it moves a
situation out of the "recovered" column above. Both need justifying in the
same change, and both need §11.1 and §11.2 updated. The enum is
deliberately not marked `#[non_exhaustive]`: four variants that are unlikely
to grow are more useful matched exhaustively than guarded by a wildcard arm
that hides a future case.
