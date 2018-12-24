use std::char;

pub type IntCandidateSet = i16;

// Constant with all the candidates turned on.
const ALL_CAND: IntCandidateSet =
    1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8;

const SOLVED: IntCandidateSet = 1 << 9;

pub fn parse_position(c: char) -> Option<IntCandidateSet> {
    // Use a match to get a really small lookup table based decoding.
    match c {
        '1' => Some(SOLVED | 1 << 0),
        '2' => Some(SOLVED | 1 << 1),
        '3' => Some(SOLVED | 1 << 2),
        '4' => Some(SOLVED | 1 << 3),
        '5' => Some(SOLVED | 1 << 4),
        '6' => Some(SOLVED | 1 << 5),
        '7' => Some(SOLVED | 1 << 6),
        '8' => Some(SOLVED | 1 << 7),
        '9' => Some(SOLVED | 1 << 8),
        '0' => Some(ALL_CAND),
        '.' => Some(ALL_CAND),
        '*' => Some(ALL_CAND),
        'x' => Some(ALL_CAND),
        _ => None,
    }
}

pub trait CandidateSet {
    fn num_candidates(&self) -> u32;
    fn is_solved(&self) -> bool;
    fn set_solved(&self) -> Self;
    fn value(&self) -> Option<u8>;
}

impl CandidateSet for IntCandidateSet {
    fn num_candidates(&self) -> u32 {
        (self & !SOLVED).count_ones()
    }
    fn is_solved(&self) -> bool {
        (self & SOLVED) == SOLVED
    }
    fn set_solved(&self) -> IntCandidateSet {
        self | SOLVED
    }
    fn value(&self) -> Option<u8> {
        if self.is_solved() {
            let v = (self & !SOLVED).trailing_zeros() + 1;
            Some(v as u8)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_numbers() {
        for i in 1..10 {
            let c = char::from_digit(i, 10).unwrap();
            let p = parse_position(c).unwrap();
            assert_eq!(true, p.is_solved());
            assert_eq!(Some(i as u8), p.value());
        }
    }

    #[test]
    fn test_new_none() {
        assert_eq!(None, parse_position('/'));
    }

    #[test]
    fn test_new_zero() {
        for c in ['.', '0', '*'].iter() {
            assert_eq!(ALL_CAND, parse_position(c.clone()).unwrap());
        }
    }
}
