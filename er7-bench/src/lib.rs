//! Benchmarks for [`er7`], kept in their own crate so that `er7` itself can
//! carry no dependencies at all — not even development ones, which its own
//! test enforces (`the_crate_has_no_runtime_dependencies`, `er7` spec
//! §15.1, R25).
//!
//! There is no library here; the code is in `benches/`. Run it with:
//!
//! ```sh
//! cargo bench -p er7-bench
//! cargo bench -p er7-bench -- --save-baseline before   # then compare
//! ```
