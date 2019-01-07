use crate::candidate_set::CandidateSet;
use crate::sudoku::Sudoku;
use std::iter::Iterator;

pub trait RemoveMask<T> {
    fn remove_mask(&mut self, m: usize, iter: T) -> usize;
}

impl<T: Iterator<Item = usize>> RemoveMask<T> for Sudoku {
    fn remove_mask(&mut self, m: usize, iter: T) -> usize {
        let mut changed = 0;
        for idx in iter {
            if self[idx].get() & m != 0 {
                let new_cs = CandidateSet::new(self[idx].get() & !m);
                self[idx] = new_cs;
                changed += 1;
            }
        }
        changed
    }
}
