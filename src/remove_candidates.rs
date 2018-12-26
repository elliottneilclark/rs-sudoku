use super::index::get_index_tuple;
use super::sudoku::Sudoku;

pub trait RemoveCandidates {
    fn remove_impossible_candidates(&mut self) -> usize;
}

impl RemoveCandidates for Sudoku {
    fn remove_impossible_candidates(&mut self) -> usize {
        let mut column_solved_set: [u16; 9] = [0; 9];
        let mut row_solved_set: [u16; 9] = [0; 9];
        let mut box_solved_set: [u16; 9] = [0; 9];
        // Get the candidates that already have a single solution.
        for (i, set) in self.positions.iter().enumerate() {
            if set.num_candidates() == 1 {
                let (row_i, col_i, box_i) = get_index_tuple(i as u8);
                row_solved_set[row_i as usize] |= set.candidates;
                column_solved_set[col_i as usize] |= set.candidates;
                box_solved_set[box_i as usize] |= set.candidates;
            }
        }

        let mut changed = 0;
        for (i, set) in self.positions.iter_mut().enumerate() {
            let (row_i, col_i, box_i) = get_index_tuple(i as u8);
            if set.num_candidates() != 1 {
                let new_set = set.candidates
                    & !row_solved_set[row_i as usize]
                    & !column_solved_set[col_i as usize]
                    & !box_solved_set[box_i as usize];
                // If there has been a change then remember that.
                if set.candidates != new_set {
                    changed += 1;
                    set.candidates = new_set;
                }
            }
        }
        changed
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::ONE_LINE;

    #[test]
    fn test_remove_candidates() {
        let mut p = Sudoku::new(ONE_LINE).unwrap();
        assert_eq!(57, p.remove_impossible_candidates());
    }
}
