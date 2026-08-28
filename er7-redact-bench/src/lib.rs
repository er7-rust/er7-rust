//! Benchmarks for `er7-redact`, kept in their own crate for the same
//! reason `er7-bench` is: `er7-redact` carries exactly one runtime
//! dependency (D16, `er7-redact` spec §12.1) and, as of this crate's
//! addition, still zero development ones — Criterion's own dependency
//! tree lives here instead, one directory over, where it cannot reach the
//! audit surface of the crate being measured.
//!
//! There is no library here; the code is in `benches/`. Run it with:
//!
//! ```sh
//! cargo bench -p er7-redact-bench
//! cargo bench -p er7-redact-bench -- --save-baseline before   # then compare
//! ```
#![forbid(unsafe_code)]
