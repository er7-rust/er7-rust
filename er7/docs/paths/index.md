[er7](../../index.md) → [docs](../) → paths

# HL7 paths

The short notation that names one place in a message: `PID-5.1`,
`OBX[2]-5`, `PID-13[2].1`.

The notation is a de-facto standard among interface engineers rather than
part of HL7 itself. This crate accepts the two spellings that are common in
the field and writes the first. Normative rules are
[spec §8](../../spec/08-paths-and-queries/index.md).

## Grammar

```
path       = name occurrence? ( ("-" | ".") index occurrence? ( "." index ( "." index )? )? )?
name       = one or more ASCII letters and digits
occurrence = "[" index "]"
index      = a decimal number, 1 or greater
```

Surrounding whitespace is ignored.

## The four levels

| Path | Names | On `PID\|1\|\|9\|4\|SMITH^JOHN^Q` |
| ---- | ----- | --- |
| `PID` | the whole segment | `PID\|1\|\|9\|4\|SMITH^JOHN^Q` |
| `PID-5` | field 5 | `SMITH^JOHN^Q` |
| `PID-5.1` | component 1 of field 5 | `SMITH` |
| `PID-5.1.2` | subcomponent 2 of that component | — (there is only one) |

A path that stops above the leaf returns that subtree **as written**, with
its structural delimiters intact. Only the leaf text is decoded.

## Occurrence indices

There are two, and they mean different things.

| Position | Selects | Example |
| -------- | ------- | ------- |
| after the **segment name** | which segment of that name | `OBX[2]-5` — the second `OBX` |
| after the **field number** | which repetition of that field | `PID-13[2]` — the second phone number |

Both are 1-based. Both may be omitted, and omitting one means "every one":

```rust
// Three OBX segments, so three answers.
assert_eq!(message.query_all("OBX-5")?, vec!["187", "102", ""]);

// One, pinned down.
assert_eq!(message.query_all("OBX[2]-5")?, vec!["102"]);
```

They compose: `OBX[2]-5[1].1.2` is subcomponent 2 of component 1 of the
first repetition of field 5 of the second `OBX`.

## Repetitions have a special case

A path that **stops at the field** returns the whole field, repetition
separators included. A path that goes **deeper** splits into one answer per
repetition.

```rust
// PID-13 is `555-1111~555-2222`
assert_eq!(message.query("PID-13")?.as_deref(), Some("555-1111~555-2222"));
assert_eq!(message.query_all("PID-13.1")?, vec!["555-1111", "555-2222"]);
assert_eq!(message.query_all("PID-13[2].1")?, vec!["555-2222"]);
```

This is deliberate: `PID-13` as a whole field is a meaningful thing to ask
for, and joining its repetitions back with `~` is the only honest way to
return it as one string.

## Both spellings

`PID-5.1` and `PID.5.1` parse identically. `Display` writes the first.

```rust
let a: er7::Path = "PID.5.1".parse()?;
let b: er7::Path = "PID-5.1".parse()?;
assert_eq!(a, b);
assert_eq!(a.to_string(), "PID-5.1");
```

Round-tripping through `Display` preserves meaning, because occurrence
indices the path left open are left out rather than defaulted to 1.

## Indices are 1-based, and 0 is an error

HL7 numbers from 1. A `0` is almost always a caller's off-by-one, and
silently reading it as `1` would return a plausible wrong answer — so it is
rejected instead.

```rust
assert!("PID-0".parse::<er7::Path>().is_err());
assert!("PID[0]-5".parse::<er7::Path>().is_err());
```

Other rejections: an empty path, a missing field number (`PID-`), a
trailing dot (`PID-5.`), an unclosed bracket (`PID[2-5`), trailing junk
(`PID-5x`), and a separator that is neither `-` nor `.` (`PID/5`).

## The four query methods

| Method | Takes | Returns | Decoded? |
| ------ | ----- | ------- | -------- |
| `Message::query` | `&str` | `Result<Option<String>, Error>` — first match | yes |
| `Message::query_all` | `&str` | `Result<Vec<String>, Error>` — every match | yes |
| `Message::query_path` | `&Path` | `Vec<String>` — every match | yes |
| `Message::query_path_raw` | `&Path` | `Vec<String>` — every match | no, as sent |

The `&str` forms parse the path each call, so they can fail with
`Error::BadPath`. The `&Path` forms take an already-parsed path, which is
what you want when applying one path to many messages:

```rust
let path: er7::Path = "PID-5.1".parse()?;
let names: Vec<String> = messages
    .iter()
    .flat_map(|message| message.query_path(&path))
    .collect();
```

`Path` implements `Clone`, `PartialEq`, `Eq`, and `Hash`, so a set of paths
can be map keys or deduplicated.

## Two things that surprise people

**Header delimiter fields come back literally.** `MSH-1` is the field
separator and `MSH-2` is the encoding characters; they are the delimiters,
not values encoded with them, so they are never decoded.

```rust
assert_eq!(message.query("MSH-1")?.as_deref(), Some("|"));
assert_eq!(message.query("MSH-2")?.as_deref(), Some(r"^~\&"));
```

**A missing position contributes nothing at all** — no entry in the vector,
not an empty string. So `query_all("OBX-5").len()` counts the `OBX`
segments that actually carried a fifth field, which is usually what you
want and occasionally a surprise.

## Reading paths off the CLI

The `er7` command's default output labels every value with the path that
names it, and every one of those labels is a valid query. Read a path off
the outline, paste it into `--query`:

```sh
er7 samples/oru_r01.er7 | grep Cholesterol
#=> OBX[1]-3.2  Cholesterol

er7 --query 'OBX[1]-3.2' samples/oru_r01.er7
#=> Cholesterol
```

Quote the path in a shell: `[` and `]` are glob characters.

## Paths or accessors?

| Use | When |
| --- | ---- |
| paths | the position is a string known at compile time, or comes from configuration or a user; you want every match without a loop |
| accessors | you need the node itself — to edit it, to ask `is_null()`, or to avoid the intermediate `String` |

See [docs/usage §4](../usage/index.md) for the accessor walk-through.
