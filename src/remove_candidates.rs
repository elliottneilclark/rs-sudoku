use super::sudoku::Sudoku;
use super::candidate_set::CandidateSet;
use super::index::get_index_tuple;

trait RemoveCandidate {
    fn remove_impossible_candidates(&mut self) -> Vec<usize>;
}

impl RemoveCandidate for Sudoku {
    fn remove_impossible_candidates(&mut self) -> Vec<usize> {
        let mut column_solved_set: [i16; 9] = [0; 9];
        let mut row_solved_set: [i16; 9] = [0; 9];
        let mut box_solved_set: [i16; 9] = [0; 9];
        // Get the candidates that already have a single solution.
        for (i, set) in self.positions.iter().enumerate() {
            if set.num_candidates() == 1 {
                let (row_i, col_i, box_i) = get_index_tuple(i);
                row_solved_set[row_i] |= set;
                column_solved_set[col_i] |= set;
                box_solved_set[box_i] |= set;
            }
        }

        let mut changed = Vec::new();
        self.positions = self
            .positions
            .iter()
            .enumerate()
            .map(|(i, set)| {
                let (row_i, col_i, box_i) = get_index_tuple(i);
                let old_set = set.clone();
                if set.num_candidates() != 1 {
                    let new_set = set
                        & !row_solved_set[row_i]
                        & !column_solved_set[col_i]
                        & !box_solved_set[box_i];
                    // If there has been a change then remember that.
                    if old_set != new_set {
                        changed.push(i);
                    }
                    new_set
                } else {
                    set.clone()
                }
            })
            .collect();
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
        assert_eq!(57, p.remove_impossible_candidates().len());
    }
}
