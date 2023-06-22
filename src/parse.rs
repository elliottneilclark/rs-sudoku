use super::candidate_set::CandidateSet;
use super::error::SudokuErr;
use super::remove_candidates::*;
use super::sudoku::Sudoku;

pub fn parse_sudoku(pzl: &str) -> Result<Sudoku, SudokuErr> {
    // ascii only please
    if !pzl.is_ascii() {
        return Err(SudokuErr::Ascii());
    }

    // Decode the positions
    let mut p = [CandidateSet::default(); 81];
    let mut used = 0;
    for cs in pzl.chars().filter_map(CandidateSet::parse_position) {
        if used < 81 {
            p[used] = cs;
        }
        used += 1;
    }
    // If there aren't enogh then bail
    if used != 81 {
        return Err(SudokuErr::Parse());
    }
    let mut s = Sudoku::new(p);
    // After removing the impossible candidates
    // make sure that everything is still valid.
    s.remove_candidates(false);
    if s.is_valid() {
        // Return the result if the puzzle is not provably invalid.
        Ok(s)
    } else {
        Err(SudokuErr::InvalidPuzzle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::MULTI_LINE;
    use crate::examples::ONE_LINE;

    #[test]
    fn error() {
        // Shouldn't be valid as it's not enough chars.
        let r = parse_sudoku("badd");
        assert!(r.is_err())
    }

    #[test]
    fn test_parse_good_oneline() {
        let r = parse_sudoku(ONE_LINE);
        assert!(r.is_ok())
    }

    #[test]
    fn test_good_multiline() {
        let r = parse_sudoku(MULTI_LINE);
        assert!(r.is_ok());
    }

    #[test]
    fn test_not_solved() {
        let p = parse_sudoku(ONE_LINE).unwrap();
        assert!(!p.is_solved());
    }

    #[test]
    fn test_weird() {
        let s = "000.00..00..00..00....00..000.0000000.0..0.0.0..000...0000000001.1101111.2....0.0";
        let p = parse_sudoku(s);
        assert!(p.is_err());
    }
}
