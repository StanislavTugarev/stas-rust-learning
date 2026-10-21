//! Library crate `rust_test`.
//!
//! Holds only the reusable code that `cargo test` and `cargo bench` need.
//! Everything runnable lives in `examples/`.

pub mod section_07;

use chrono::format::Item::Error;
// Re-exported at the crate root so benches/tests can use
// `rust_test::{sort_algo_1, sort_algo_2, Account, Bank}`.
pub use section_07::bank::{Account, Bank};
pub use section_07::sorting::{sort_algo_1, sort_algo_2};

#[derive(Debug, Default)]
pub struct Student {
    pub id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

// impl Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             age: 20,
//             name: "unknown".to_string(),
//         }
//     }
// }
