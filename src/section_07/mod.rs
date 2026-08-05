//! Section 7 — unit tests & benchmarks.
//!
//! - `sorting` — two sorting algorithms (benched in `benches/sorting.rs`)
//! - `bank`    — a tiny in-memory bank (tested inline + in `tests/s07_bank_stress.rs`,
//!               benched in `benches/bank.rs`)
//! - `shapes`  — a unit-testing demo (`cargo test`)

pub mod bank;
pub mod shapes;
pub mod sorting;
