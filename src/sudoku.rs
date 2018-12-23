use super::position::Position;
pub struct Sudoku {
    pub positions: [Position; 81],
}

impl Sudoku {
    pub fn new(pzl: &str) -> Result<Sudoku, SudokuErr> {
        // Decode the positions
        let pos_vec: Vec<Position> = pzl.chars().filter_map(Position::new).collect();
        // If there aren't enough then bail
        if pos_vec.len() != 81 {
            return Err(SudokuErr::ParseErr());
        }
        // put the positions into an array for final location.
        let mut pos: [Position; 81] = [Position { candidates: 0 }; 81];
        for (i, v) in pos_vec.into_iter().enumerate() {
            pos[i] = v;
        }
        // Return the result
        Ok(Sudoku { positions: pos })
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

    const ONE_LINE: &str =
        ".1.....25.2.84.3.9.....91.84.....9.3.....6.......5....35..6.....6.1.....98......1";

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
        let p = " . . . | 2 7 1 | . . .
                 2 . 8 | . . . | 7 . .
                 . . . | . 3 . | . . .
                 ------|-------|-------
                 . . . | 7 5 4 | . . .
                 . . . | . . . | 2 . .
                 . . . | 8 . 9 | . . 5
                 ------|-------|-------
                 1 2 . | . . 3 | 4 8 .
                 8 . 5 | . 4 . | 1 7 .
                 4 3 . | . . . | . 9 .";

        let r = Sudoku::new(p);
        assert!(r.is_ok());
    }

    #[test]
    fn test_not_solved() {
        let p = Sudoku::new(ONE_LINE).unwrap();
        assert_eq!(false, p.solved());
    }

}
