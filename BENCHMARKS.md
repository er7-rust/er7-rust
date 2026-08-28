# Benchmarks

Measured numbers for the operations that dominate an ER7 workload, plus
the machine they came from and the caveats that make them meaningful.
Covers both `er7` and `er7-redact`; `serde-er7` has none — it wraps
`er7`'s own tree in Serde impls with no processing pass of its own to
measure separately.

The reader-friendly version is <https://er7-rust.github.io/benchmarks/>.

## `er7`

The benchmarks live in [`er7-bench/`](er7-bench/), a workspace member that
is **not published**. It exists so that `er7` itself can keep both
`[dependencies]` and `[dev-dependencies]` empty — a rule its own test
enforces — while Criterion lives one directory over, where it cannot reach
the audit surface of the crate being measured.

### Running them

```sh
cargo bench -p er7-bench

# Record a baseline, change something, then compare against it.
cargo bench -p er7-bench -- --save-baseline before
cargo bench -p er7-bench -- --baseline before

# One group only.
cargo bench -p er7-bench -- parse
```

Criterion writes an HTML report to `target/criterion/report/index.html`.

### The inputs

Two synthetic messages — never real patient data, per
[family policy §1.4](spec/01-family-policy/index.md) — chosen to bracket
what production traffic looks like.

| Input | Shape | Size | Segments |
| ----- | ----- | ---- | -------- |
| **small** | An `ADT^A08`: `MSH`, `EVN`, `PID`, `PV1` | 177 bytes | 4 |
| **large** | An `ORU^R01` with 200 `OBX` segments, each followed by an `NTE` carrying an escape sequence | 21,520 bytes (21.0 KiB) | 402 |

The small message is the shape most interfaces move in bulk. The large one
is the shape that decides whether a parser is fast enough for a day's
traffic.

### Results

Measured **2026-08-26**. Apple M4 Max, macOS 26.6.1, `rustc 1.98.0`,
`aarch64-apple-darwin`, release profile. Criterion, 100 samples per
benchmark; the figure is the median and the bracket is Criterion's
confidence interval.

#### Parsing

| Benchmark | Time | Throughput | Derived |
| --------- | ---- | ---------- | ------- |
| `parse/small` | 2.64 µs [2.60 – 2.70] | 63.9 MiB/s | ≈ 378,000 messages/second |
| `parse/large` | 260.6 µs [257.7 – 264.4] | 78.8 MiB/s | ≈ 3,800 messages/second, ≈ 648 ns/segment |

Parsing is the expensive half of a round trip, and throughput improves with
message size: the large message parses at a higher rate per byte than the
small one, because per-message fixed costs amortise away.

#### Writing

| Benchmark | Time | Derived |
| --------- | ---- | ------- |
| `render/small` | 369 ns [365 – 375] | ≈ 2.7 million messages/second |
| `render/large` | 21.4 µs [21.1 – 21.7] | ≈ 46,800 messages/second |
| `render/large_crlf_trailing` | 21.2 µs [21.0 – 21.6] | Non-default terminator and a trailing terminator cost nothing measurable |

Writing is roughly **12× cheaper than parsing** on the large message. That
matters for the common integration shape: parse once, edit, write many.

#### Escape sequences

| Benchmark | What it measures | Time |
| --------- | ---------------- | ---- |
| `escape/escape_plain` | A value with nothing to escape — the common case | 10.8 ns [10.6 – 11.0] |
| `escape/escape_delimited` | A value full of delimiters | 110.9 ns [109.1 – 113.0] |
| `escape/unescape_sequenced` | A value full of escape sequences to decode | 196.1 ns [192.6 – 200.3] |
| `escape/tokenize_sequenced` | Iterating the sequences without decoding | 122.5 ns [119.3 – 127.9] |

The plain case is the one to watch: escaping a value that needs no escaping
costs about ten nanoseconds and does not allocate, which is what keeps
whole-message writing cheap.

#### Queries

Against the 402-segment large message:

| Benchmark | Query | Time |
| --------- | ----- | ---- |
| `query/subcomponent` | `PID-3.4.2` | 80.3 ns [78.9 – 81.9] |
| `query/last_segment` | `NTE-3` | 149.7 ns [146.3 – 154.5] |
| `query/field` | `PID-3` | 168.8 ns [166.7 – 171.3] |
| `query/all_segments` | `query_all("OBX-5")` — 200 matches | 7.23 µs [7.13 – 7.34] |

A single `query` returns on first match rather than walking the whole
message, which is why `PID-3.4.2` against a 402-segment message costs about
as much as against a 4-segment one. `query_all` necessarily walks
everything: 7.23 µs for 200 matches is about 36 ns per match found.

### Optimisation history

Changes made because a benchmark said so, rather than because the code
looked slow:

| Change | Effect |
| ------ | ------ |
| [`821a7dc`](https://github.com/er7-rust/er7-rust/commit/821a7dc) *Stop `query` walking the whole message to return one value* | `query/field` −85%, `query/subcomponent` −92%. Single-value lookup went from proportional to message length to effectively constant |

### How to read these numbers, and how not to

1. **These are single-machine numbers on fast hardware.** An M4 Max is not
   an interface engine in a hospital data centre. Treat the ratios — write
   is 12× cheaper than parse, a single query does not scale with message
   length — as the durable finding, and the absolute figures as an upper
   bound.
2. **No comparison to other libraries is claimed here.** Benchmarking
   someone else's library fairly is hard, and benchmarking it unfairly is
   worse than not doing it. [`COMPARISONS.md`](COMPARISONS.md) compares
   design and scope, on the record, without inventing numbers.
3. **Criterion's outlier counts are not noise to ignore.** Runs on a laptop
   routinely report 5–9% high-severe outliers from scheduling. Compare
   against a saved baseline on the same machine rather than against a
   number in this file.
4. **Nothing here is a latency guarantee.** These are library operations,
   not an end-to-end interface benchmark; a real feed spends most of its
   time in I/O, TLS, and the receiving system.

### Fuzzing

Performance work is only safe next to correctness work. The crate carries
`cargo-fuzz` targets alongside these benchmarks:

```sh
cargo +nightly fuzz run parse_roundtrip -- -max_total_time=60
```

Four targets: `parse_roundtrip`, `parse_with_total`, `escape_roundtrip`,
and `query_paths`. See [`er7/fuzz/`](er7/fuzz/) and
[`er7/spec/13-testing-strategy/index.md`](er7/spec/13-testing-strategy/index.md)
§13.6, which cites the actual run counts.

## `er7-redact`

The benchmarks live in
[`er7-redact-bench/`](er7-redact-bench/), a workspace member that is
**not published** — the same reasoning as `er7-bench`, one directory
over: `er7-redact` carries exactly one runtime dependency and, until this
crate existed, zero development ones too, and Criterion's own tree stays
out of that count.

### Running them

```sh
cargo bench -p er7-redact-bench

# Record a baseline, change something, then compare against it.
cargo bench -p er7-redact-bench -- --save-baseline before
cargo bench -p er7-redact-bench -- --baseline before
```

### The inputs

The HL7® v2 standard's own reference `ADT^A08` example — the same message
every sample and doc-test in this crate already uses — and a batch of 50
independently parsed copies of it, standing in for an export. Parsing
happens once, outside the timed closure; what is measured is redaction
alone.

### Results

Measured **2026-08-28**. Apple M4 Max, macOS 26.6.1, `rustc 1.98.0`,
`aarch64-apple-darwin`, release profile. Criterion, 100 samples per
benchmark.

| Benchmark | Time | Derived |
| --------- | ---- | ------- |
| `redact/small` | 17.56 µs [17.50 – 17.62] | ≈ 57,000 messages/second |
| `redact/batch_of_50` | 793.6 µs [791.0 – 796.4] | ≈ 15.9 µs/message — consistent with the single-message figure, as it should be: nothing in `Redactor::redact` shares state across messages |
| `posture/accept_by_default` | 17.39 µs [17.33 – 17.46] | `Policy::patient_identifiers()` — only the leaves a rule names are visited |
| `posture/reject_by_default` | 19.01 µs [18.97 – 19.05] | `Policy::all_but_the_header()` — every leaf in the message is visited and judged |

Rejecting by default costs about **9% more** than accepting by default on
this message, which tracks the shape of the work: a reject-by-default
policy has to walk and judge every leaf, not just the ones a rule names.
That gap will grow on a message with more untouched leaves than this
eight-segment example carries, and shrink toward zero on one where nearly
every leaf is named — this figure is one data point, not a general ratio.

### How to read these numbers, and how not to

The same four cautions as `er7`'s own results apply here without
qualification — see above. One addition specific to this crate:
**redaction cost is not de-identification cost.** Whether a message ends
up de-identified is a property of the policy you write, not of how fast
this crate applies it — see
[`er7-redact` §5.5](er7-redact/spec/05-built-in-policies/index.md).

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
