#[derive(Debug)]
pub enum SudokuErr {
    AsciiErr(),
    ParseErr(),
    InvalidPuzzleErr(),
}
