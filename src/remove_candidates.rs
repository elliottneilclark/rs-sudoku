use super::index::get_index_tuple;
use super::sudoku::Sudoku;

pub trait RemoveCandidates {
    fn remove_candidates(&mut self, set_solved: bool) -> (usize, usize);
}

impl RemoveCandidates for Sudoku {
    fn remove_candidates(&mut self, set_solved: bool) -> (usize, usize) {
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
        let mut solved = 0;
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

            // If we've been asked to set the solved bit and
            // we have found a canidate set with only one
            // candidate then set the solved flag.
            if set_solved && !set.is_solved() && set.num_candidates() == 1 {
                set.set_solved();
                solved += 1;
            }
        }
        (changed, solved)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::ONE_LINE;
    use crate::parse::*;

    #[test]
    fn test_remove_candidates() {
        let mut p = parse_sudoku(ONE_LINE).unwrap();
        let (c, _s) = p.remove_candidates(false);
        assert_eq!(12, c);
    }
}
