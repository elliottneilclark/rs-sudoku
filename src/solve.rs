use crate::hidden_singles::HiddenSingles;
use crate::remove_candidates::RemoveCandidates;
use crate::sudoku::Sudoku;

#[derive(Debug)]
pub struct SolveReport {
    /// Is the solution sovled.
    pub is_solved: bool,
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
    /// The string representation of how the board looks
    ///  after trying to solve the puzzle
    pub state: String,
}

pub trait Solveable {
    fn try_solve(self) -> SolveReport;
}

impl Solveable for Sudoku {
    fn try_solve(mut self) -> SolveReport {
        //  remember how many were solved before this
        let given = self.num_solved();
        // Remember how many were solved with naked singles.
        let mut naked_singles = 0;
        let mut hidden_singles = 0;
        let mut cont = true;
        while cont {
            // First assign anything that can only be
            println!("Not Done yet {}", self.num_solved());
            // Then remove everything that can't be a candidate anymore.
            self.remove_impossible_candidates();
            // Try and assign hidden singles
            let hs = self.handle_hidden_singles();
            if hs > 0 {
                hidden_singles += hs;
                continue;
            }
            // Remove things.
            self.remove_impossible_candidates();
            // Try and toggle solved.
            let ns = self.toggle_solved();
            if ns > 0 {
                naked_singles += ns;
                continue;
            }
            // We have no more
            cont = false;
        }

        SolveReport {
            is_solved: self.solved(),
            given: given,
            hidden_singles: hidden_singles,
            naked_singles: naked_singles,
            state: self.oneline(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;
    use crate::sudoku::Sudoku;

    #[test]
    fn test_try_solve_easy() {
        let p = Sudoku::new(ONE_LINE).unwrap();
        let sr = p.try_solve();
        assert_eq!(true, sr.is_solved);
        assert!(sr.naked_singles >= 2);
        assert_eq!(
            "819637425527841369643529178476218953135796284298354716351962847764183592982475631",
            sr.state
        );
    }
}
