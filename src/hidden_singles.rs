use crate::index::ALL_GROUPINGS;
use crate::sudoku::Sudoku;
use std::iter::Iterator;

pub trait HiddenSingles {
    fn find_hidden(&mut self) -> usize;
}

impl HiddenSingles for Sudoku {
    fn find_hidden(&mut self) -> usize {
        let mut solved = 0;
        // For all the grouping types
        for g in ALL_GROUPINGS.iter() {
            // For row,col,box number 0 through 8
            for g_idx in 0..9 {
                let mut counts = [0; 9];
                let mut last_index = [0; 9];
                for i in g.iter(g_idx) {
                    for c in self.positions[i] {
                        let cc = c.trailing_zeros() as usize;
                        counts[cc] += 1;
                        last_index[cc] = i;
                    }
                }

                for (c_idx, count) in counts.iter().enumerate() {
                    if *count == 1 {
                        let s_idx = last_index[c_idx];
                        if self.positions[s_idx].num_candidates() != 1 {
                            self.positions[s_idx].candidates = 1 << c_idx;
                            self.positions[s_idx].set_solved();
                            solved += 1;
                        }
                    }
                }
            }
        }
        solved
    }
}
