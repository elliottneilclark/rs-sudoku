use std::char;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub candidates: i16,
}

// Constant with all the candidates turned on.
const ALL_CAND: i16 = 1 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8;

impl Position {
    pub fn new(c: char) -> Option<Position> {
        // Use a match to get a really small lookup table based decoding.
        match c {
            '1' => Some(Position { candidates: 1 << 0 }),
            '2' => Some(Position { candidates: 1 << 1 }),
            '3' => Some(Position { candidates: 1 << 2 }),
            '4' => Some(Position { candidates: 1 << 3 }),
            '5' => Some(Position { candidates: 1 << 4 }),
            '6' => Some(Position { candidates: 1 << 5 }),
            '7' => Some(Position { candidates: 1 << 6 }),
            '8' => Some(Position { candidates: 1 << 7 }),
            '9' => Some(Position { candidates: 1 << 8 }),
            '0' => Some(Position {
                candidates: ALL_CAND,
            }),
            '.' => Some(Position {
                candidates: ALL_CAND,
            }),
            '*' => Some(Position {
                candidates: ALL_CAND,
            }),
            'x' => Some(Position {
                candidates: ALL_CAND,
            }),
            _ => None,
        }
    }
    pub fn num_candidates(&self) -> u32 {
        self.candidates.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_numbers() {
        for i in 1..9 {
            let c = char::from_digit(i, 10).unwrap();
            let p = Position::new(c).unwrap();
            assert_eq!(1 << i - 1, p.candidates);
        }
    }

    #[test]
    fn test_new_none() {
        assert_eq!(None, Position::new('/'));
    }

    #[test]
    fn test_new_zero() {
        for c in ['.', '0', '*'].iter() {
            assert_eq!(ALL_CAND, Position::new(c.clone()).unwrap().candidates);
        }
    }
}
