//! Library crate `rust_test`.
//!
//! Holds only the reusable code that `cargo test` and `cargo bench` need.
//! Everything runnable lives in `examples/`.

pub mod section_07;

// Re-exported at the crate root so benches/tests can use
// `rust_test::{sort_algo_1, sort_algo_2, Account, Bank}`.
pub use section_07::bank::{Account, Bank};
pub use section_07::sorting::{sort_algo_1, sort_algo_2};
