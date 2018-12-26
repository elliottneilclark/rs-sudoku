use super::candidate_set::{parse_position, CandidateSet};
use super::error::SudokuErr;

pub struct Sudoku {
    pub positions: Vec<CandidateSet>,
}

impl Sudoku {
    pub fn new(pzl: &str) -> Result<Sudoku, SudokuErr> {
        // Decode the positions
        let pos_vec: Vec<CandidateSet> = pzl.chars().filter_map(parse_position).collect();
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
            .filter(|x| x.num_candidates() == 1)
            .count()
    }

    pub fn solved(&self) -> bool {
        self.positions.iter().all(|x| x.num_candidates() == 1)
    }

    pub fn toggle_solved(&mut self) -> usize {
        // For all recently changed positions if they now have no other
        // options then set them as solved.
        let mut solved = 0;
        for p in self.positions.iter_mut() {
            if !p.is_solved() && p.num_candidates() == 1 {
                p.set_solved();
                solved += 1;
            }
        }
        solved
    }

    pub fn oneline(&self) -> String {
        self.positions
            .iter()
            .map(|p| p.value().unwrap_or(0).to_string())
            .collect::<Vec<String>>()
            .join("")
    }
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
