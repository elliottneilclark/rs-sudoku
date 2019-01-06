// A module of example puzzles
pub mod examples;

// All the modules that make up index generation
mod fast_index_data;
mod index_helpers;
// This is public so that
// pre_compute_index_data and benchmarks can use it.
pub mod fast_index;
pub mod slow_index;
// The main index entrypoint.
mod index;

mod candidate_set;
mod error;
mod parse;
mod sudoku;

// The modules that make up solve
mod hidden_singles;
mod remove_candidates;
mod subset;

// Does the actual solving
mod solve;

pub use self::error::SudokuErr;
pub use self::parse::parse_sudoku;
pub use self::solve::{SolveReport, Solveable};
pub use self::sudoku::Sudoku;
