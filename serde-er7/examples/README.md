# Examples

Runnable programs, one concept each. Run any of them with:

```sh
cargo run --example <name>
```

| Example | Shows |
| ------- | ----- |
| [`round_trip_via_json`](round_trip_via_json.rs) | ER7 → `Message` → JSON → `Message` → ER7, byte for byte unchanged — the crate's flagship path |
| [`build_message_from_json`](build_message_from_json.rs) | The reverse: hand-written JSON → `Message` → ER7, for building a message from an API request |
| [`inspect_a_segment_as_json`](inspect_a_segment_as_json.rs) | Serializing one `Segment` on its own, and the exact array/object shape each tree level chooses |
| [`catch_a_typo_with_strict`](catch_a_typo_with_strict.rs) | `Strict<Message>` reporting a mistyped key by name, where the plain type either gives a less specific error or, on the one optional key, none at all |

See `docs/usage/index.md` for the walk-through these examples are drawn
from, and `docs/api/index.md` for the full type reference.
