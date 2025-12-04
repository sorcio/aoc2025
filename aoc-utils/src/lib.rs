//! # Advent of Code Utils
//!
//! My personal collection of utilities for solving Advent of Code problems.
//! While I set a personal rule to not use external crates for Advent of Code
//! solutions, I decided it's okay to collect my own utilities over time and use
//! them in my solutions.

pub mod range;
pub mod testing;
pub mod utils;

pub use range::*;
pub use testing::*;
pub use utils::*;

pub use unindent::{unindent, unindent_bytes};
