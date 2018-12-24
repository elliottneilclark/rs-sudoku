use super::candidate_set::*;
pub struct Sudoku {
    pub positions: Vec<IntCandidateSet>,
}

impl Sudoku {
    pub fn new(pzl: &str) -> Result<Sudoku, SudokuErr> {
        // Decode the positions
        let pos_vec: Vec<IntCandidateSet> = pzl.chars().filter_map(parse_position).collect();
        // If there aren't enough then bail
        if pos_vec.len() != 81 {
            return Err(SudokuErr::ParseErr());
        }
        // Return the result
        Ok(Sudoku { positions: pos_vec })
    }

    pub fn num_solved(&self) -> usize {
        self.positions
            .iter()
            .filter(|&x| x.num_candidates() == 1)
            .count()
    }

    pub fn solved(&self) -> bool {
        self.positions.iter().all(|&x| x.num_candidates() == 1)
    }
}

#[derive(Debug)]
pub enum SudokuErr {
    ParseErr(),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::MULTI_LINE;
    use crate::examples::ONE_LINE;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn error() {
        // Shouldn't be valid as it's not enough chars.
        let r = Sudoku::new("badd");
        assert!(!r.is_ok())
    }

    #[test]
    fn test_parse_good_oneline() {
        let r = Sudoku::new(ONE_LINE);
        assert!(r.is_ok())
    }

    #[test]
    fn test_good_multiline() {
        let r = Sudoku::new(MULTI_LINE);
        assert!(r.is_ok());
    }

    #[test]
    fn test_not_solved() {
        let p = Sudoku::new(ONE_LINE).unwrap();
        assert_eq!(false, p.solved());
    }

}
