#[derive(Debug)]
pub enum SudokuErr {
    Ascii(),
    Parse(),
    InvalidPuzzle(),
}
