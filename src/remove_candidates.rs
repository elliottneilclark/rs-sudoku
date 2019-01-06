use crate::candidate_set::CandidateSet;
use crate::index_helpers::get_index_tuple;
use crate::sudoku::Sudoku;

pub trait RemoveCandidates {
    fn remove_candidates(&mut self, set_solved: bool) -> (usize, usize);
}

impl RemoveCandidates for Sudoku {
    fn remove_candidates(&mut self, set_solved: bool) -> (usize, usize) {
        let mut column_solved_set: [usize; 9] = [0; 9];
        let mut row_solved_set: [usize; 9] = [0; 9];
        let mut box_solved_set: [usize; 9] = [0; 9];
        // Get the candidates that already have a single solution.
        for (i, set) in self.iter().enumerate() {
            if set.num_candidates() == 1 {
                let (row_i, col_i, box_i) = get_index_tuple(i);
                // We explicitly don't use get_candidates here.
                // The masks that we're generating will only be used for
                // positions with num_candidates > 1 and hence no
                // is_solved bit set.
                let v: usize = set.get();
                row_solved_set[row_i] |= v;
                column_solved_set[col_i] |= v;
                box_solved_set[box_i] |= v;
            }
        }
        let mut solved = 0;
        let mut changed = 0;
        for (i, set) in self.iter_mut().enumerate() {
            let (row_i, col_i, box_i) = get_index_tuple(i);
            if set.num_candidates() != 1 {
                let new_set: usize = set.get()
                    & !row_solved_set[row_i]
                    & !column_solved_set[col_i]
                    & !box_solved_set[box_i];
                // If there has been a change then remember that.
                if set.get() != new_set {
                    changed += 1;
                    *set = CandidateSet::new(new_set);
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
