use crate::sudoku::Sudoku;

#[derive(Debug)]
pub struct SolveReport {
    // Is the solution sovled.
    is_solved: bool,
    // How many were already solved before starting?
    given: usize,
    // Number of hidden singles
    //
    // http://sudopedia.enjoysudoku.com/Hidden_Single.html
    //
    // The number of positions solved by finding a hidden single
    hidden_singles: usize,
    // Number of naked singles
    //
    // http://sudopedia.enjoysudoku.com/Naked_Single.html
    //
    // The number of positions solved by having no other options.
    naked_singles: usize,
}

trait Solveable {
    fn try_solve(&mut self) -> SolveReport;
}

impl Solveable for Sudoku {
    fn try_solve(&mut self) -> SolveReport {
        SolveReport {
            is_solved: false,
            given: self.num_solved(),
            hidden_singles: 0,
            naked_singles: 0,
        }
    }
}
