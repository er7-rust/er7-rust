[index](../index.md) → §3 Dependencies and format-agnosticism

# §3 Dependencies and format-agnosticism

## 3.1 Rule S1: exactly two runtime dependencies

```toml
[dependencies]
serde = "1"
er7 = { path = "../er7-rust" }
```

Both are the point of the crate: `serde` is the trait vocabulary being
implemented against, and `er7` is the value tree being wrapped. Neither can
be dropped without dropping the crate's purpose. No third runtime
dependency should be added without updating this section and explaining,
here, what it buys that these two do not.

## 3.2 Rule S2: no format-specific crate is a runtime dependency

`serde_json`, `serde_yaml`, `bincode`, and every other format crate appear
**only** under `[dev-dependencies]`, for tests, doctests, and examples.
This is what "Serde support" means as opposed to "JSON support": the whole
value of building against `serde::Serializer`/`Deserializer` rather than
writing an ER7-to-JSON function directly is that the format is the
caller's choice, made once, in their own `Cargo.toml` — not baked into this
crate's dependency tree.

A consequence: nothing in `src/` may call a function from a format crate,
construct a format-specific type, or reference a format's name in a way
that would break if that dev-dependency were swapped for a different one.
`docs/`, `examples/`, and `spec/` may use `serde_json` freely to
demonstrate, because JSON is simply the most legible format to show in
prose — that is a documentation choice, not an API commitment.

## 3.3 Why not re-export a format

Some Serde bridge crates re-export `to_string`/`from_str` convenience
functions bound to one format, for callers who only ever want that one
format. This crate does not, on the grounds that doing so would make the
"format-agnostic" claim in §3.2 misleading — a crate that ships a
`to_json_string` function has picked JSON, regardless of what its
`Cargo.toml` says. A caller who wants that convenience writes
`serde_json::to_string(&message)` themselves; it is one function call, and
it keeps the choice visible at the call site.

## 3.4 The `er7` re-export

`serde-er7` re-exports the whole `er7` crate as `serde_er7::er7`, mirroring
the convenience `hl7-2-5-to-xml-using-rust` and `hl7-2-5-to-json-using-rust`
extend for the crates they build on. This lets a caller depend on
`serde-er7` alone and still name `er7::Message`, `er7::Error`, and the rest
without a second, separately-versioned dependency on `er7` in their own
`Cargo.toml`. The re-export is exempt from S1/S2: it does not add a
dependency, it exposes one that already exists.

## 3.5 Lints

`Cargo.toml` carries a `[lints.clippy]` table setting the **pedantic**
group to `warn`:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
```

It sits beside the dependency table because it is the same kind of
statement: what this manifest promises a reviewer. The four checks run
`cargo clippy --all-targets -- -D warnings`
([§7.4](../07-testing-strategy/index.md)), so a pedantic finding fails the
build, and `priority = -1` lets a single lint be re-set without turning the
group off.

The lint that matters most here is `missing_errors_doc`: this crate's only
fallible entry point is [`Message::parse`], and a caller needs to know that
its `Err` is `er7`'s, unchanged, with nothing added
([§5](../05-error-handling/index.md)).

Where a pedantic lint is wrong for a line, the fix is an `#[allow]`
carrying a `reason`, next to that line — not a hole in the group.

[`Message::parse`]: https://docs.rs/serde-er7/latest/serde_er7/struct.Message.html#method.parse
