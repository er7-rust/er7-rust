[er7-rust](../../index.md) → [docs](../index.md) → which crate

# Which crate do I need?

Three crates, one layer each. Start at the top and stop at the first "yes".

```text
Do you need HL7® v2 messages without patient detail
(test fixtures, bug reports, vendor tickets, demos)?
├── yes → er7-redact   (and er7, to parse)
└── no
    Do you need a message as JSON, YAML, bincode, or
    another Serde format (an API, a queue, a document store)?
    ├── yes → serde-er7  (and er7, to parse)
    └── no
        Do you need to parse, query, edit, or write a message?
        ├── yes → er7
        └── no  → none of these: see "Not here" below
```

| Crate | Add it with | Depends on | Gives you |
| ----- | ----------- | ---------- | --------- |
| [`er7`](../../er7/index.md) | `cargo add er7` | nothing | `parse`, `Message`, paths and `query`, escapes, `to_er7`, batch reading, and the `er7` CLI |
| [`er7-redact`](../../er7-redact/index.md) | `cargo add er7 er7-redact` | `er7` | `Policy`, `Redactor`, `Report`, pseudonyms, and the `er7-redact` CLI |
| [`serde-er7`](../../serde-er7/index.md) | `cargo add er7 serde-er7` | `er7`, `serde` | `Serialize` and `Deserialize` wrappers for every `er7` type, plus `Strict<T>` |

Add `er7` directly as well whenever your own code calls `er7::parse` or
names an `er7` type. `serde-er7` also re-exports it as `serde_er7::er7`.

Combining them is normal. A service that receives HL7® v2, redacts it, and
stores it as JSON uses all three. Redact first, then serialize, so no
unredacted value ever reaches the store.

## Not here

These crates are the **encoding layer** only, on purpose:

- **No dictionary.** Nothing here knows that `PID-5` is a patient name or
  which fields a message type requires. You name positions; see
  [HL7 v2 for Rust developers](../hl7-v2-for-rust-developers/index.md#no-dictionary).
- **No validation** against a message profile or conformance rules.
- **No transport.** For MLLP framing over TCP, pair `er7` with a codec
  crate. [`COMPARISONS.md`](../../COMPARISONS.md) lists some.
- **No FHIR®.** Converting HL7 v2 to resources of the HL7® FHIR® standard
  is a mapping problem, a separate layer above this one.

[`COMPARISONS.md`](../../COMPARISONS.md) covers when an interface engine,
HAPI, or another Rust crate is the better choice.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
