use crate::hidden_singles::HiddenSingles;
use crate::remove_candidates::RemoveCandidates;
use crate::subset::FindSubset;
use crate::sudoku::Sudoku;

#[derive(Debug)]
pub struct SolveReport {
    /// Is the solution sovled.
    pub is_solved: bool,
    /// Is this still a valid state.
    pub is_valid: bool,
    /// How many were already solved before starting?
    pub given: usize,
    /// Number of hidden singles
    ///
    /// http://sudopedia.enjoysudoku.com/Hidden_Single.html
    ///
    /// The number of positions solved by finding a hidden single
    pub hidden_singles: usize,
    /// Number of naked singles
    ///
    /// http://sudopedia.enjoysudoku.com/Naked_Single.html
    ///
    /// The number of positions solved by having no other options.
    pub naked_singles: usize,
    /// The number of times that a {Naked,Hidden}{Double,Triple} is
    /// used to remove a candidate.
    ///
    /// http://sudopedia.enjoysudoku.com/Solving_Technique.html#Subsets
    pub subsets_used: usize,
    /// The string representation of how the board looks
    ///  after trying to solve the puzzle
    pub state: String,
}

pub trait Solveable {
    fn try_solve(self) -> SolveReport;
}

impl Solveable for Sudoku {
    fn try_solve(mut self) -> SolveReport {
        let mut sr = SolveReport {
            is_solved: false,
            is_valid: false,
            //  remember how many were solved before this
            given: self.num_solved(),
            hidden_singles: 0,
            naked_singles: 0,
            subsets_used: 0,
            state: String::new(),
        };
        // Remember how many were solved with naked singles.
        let mut cont = true;
        while cont && !self.is_solved() {
            // Then remove everything that can't be a candidate anymore.
            let (c_0, _s_0) = self.remove_candidates(false);
            // Try and assign hidden singles
            let hs = self.find_hidden();
            if hs > 0 {
                sr.hidden_singles += hs;
                continue;
            }
            // Remove things setting the solved bit along the way.
            let (c_1, s_1) = self.remove_candidates(true);
            if s_1 > 0 {
                sr.naked_singles += s_1;
                continue;
            }

            let (c_2, _s_2) = self.find_subset();
            if c_2 > 0 {
                sr.subsets_used += c_2;
                continue;
            }
            // We have no more to do because nothing changed.
            if c_0 == 0 && c_1 == 0 {
                cont = false;
            }
        }
        // Copy the final state into the report.
        sr.is_solved = self.is_solved();
        sr.is_valid = self.is_valid();
        sr.state = self.oneline();
        sr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;
    use crate::parse::*;

    #[test]
    fn test_try_solve_easy() {
        let p = parse_sudoku(ONE_LINE).unwrap();
        let sr = p.try_solve();
        assert_eq!(true, sr.is_solved);
        assert_eq!(true, sr.is_valid);
        assert_eq!(41, sr.hidden_singles);
        assert_eq!(16, sr.naked_singles);
        assert_eq!(
            "819637425527841369643529178476218953135796284298354716351962847764183592982475631",
            sr.state
        );
    }

    #[test]
    fn test_bad_fuzz_a() {
        let s = "
            2 . 4 | 7 . . | . . 6
            . . . | . . . | 1 . 4
            . 1 . | 6 5 . | . 8 .
            -------|-------|-------
            . 6 . | . . . | . 1 5
            . . 3 | . 8 . | . 7 .
            . . . | 2 6 . | . . .
            -------|-------|-------
            . . . | 3 4 . | . . .
            . . . | . .. | 6 5 . | 5 . . | . . . | . . 1

            ";
        if let Ok(p) = parse_sudoku(s) {
            p.try_solve();
        }
    }

    #[test]
    fn test_find_naked() {
        let s = "080090030030000000002060108020800500800907006004005070503040900000000010010050020";
        if let Ok(p) = parse_sudoku(s) {
            let sr = p.try_solve();
            assert_eq!(true, sr.is_solved);
            assert_eq!(
                "486591732135278469972463158627814593851937246394625871563142987249786315718359624",
                sr.state
            );
        }
    }

    #[test]
    fn test_try_solve_lots() {
        let c = PUZZLES
            .lines()
            .filter_map(|x| {
                if let Ok(p) = parse_sudoku(x) {
                    Some(p.try_solve())
                } else {
                    None
                }
            })
            .filter(|sr| sr.is_solved)
            .count();
        assert_eq!(12, c)
    }
}
