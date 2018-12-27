use std::char;
use std::iter::{IntoIterator, Iterator};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CandidateSet {
    pub candidates: u16,
}

// Constant with all the candidates turned on.
const ALL_CAND: u16 = 1 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8;

const SOLVED: u16 = 1 << 9;

pub fn parse_position(c: char) -> Option<CandidateSet> {
    // Use a match to get a really small lookup table based decoding.
    match c {
        '1' => Some(CandidateSet {
            candidates: SOLVED | 1,
        }),
        '2' => Some(CandidateSet {
            candidates: SOLVED | 1 << 1,
        }),
        '3' => Some(CandidateSet {
            candidates: SOLVED | 1 << 2,
        }),
        '4' => Some(CandidateSet {
            candidates: SOLVED | 1 << 3,
        }),
        '5' => Some(CandidateSet {
            candidates: SOLVED | 1 << 4,
        }),
        '6' => Some(CandidateSet {
            candidates: SOLVED | 1 << 5,
        }),
        '7' => Some(CandidateSet {
            candidates: SOLVED | 1 << 6,
        }),
        '8' => Some(CandidateSet {
            candidates: SOLVED | 1 << 7,
        }),
        '9' => Some(CandidateSet {
            candidates: SOLVED | 1 << 8,
        }),
        '0'| '.' | '*' | 'x' => Some(CandidateSet {
            candidates: ALL_CAND,
        }),
        _ => None,
    }
}

impl CandidateSet {
    pub fn num_candidates(self) -> u32 {
        (self.candidates & !SOLVED).count_ones()
    }
    pub fn is_solved(self) -> bool {
        (self.candidates & SOLVED) == SOLVED
    }
    pub fn set_solved(&mut self) {
        debug_assert!(
            self.candidates.count_ones() == 1,
            "Only set solved on really solved sets."
        );
        self.candidates |= SOLVED;
    }
    pub fn value(self) -> Option<u8> {
        if self.is_solved() {
            let v = (self.candidates & !SOLVED).trailing_zeros() + 1;
            Some(v as u8)
        } else {
            None
        }
    }
    pub fn get_candidates(self) -> u16 {
        (self.candidates & !SOLVED)
    }
}

#[derive(Clone, Debug)]
pub struct CandidateSetIterator {
    candidates: u16,
}

impl Iterator for CandidateSetIterator {
    type Item = u16;
    fn next(&mut self) -> Option<u16> {
        if self.candidates > 0 {
            let ans = 1 << self.candidates.trailing_zeros();
            self.candidates &= self.candidates - 1;
            Some(ans as u16)
        } else {
            None
        }
    }
}

impl IntoIterator for CandidateSet {
    type Item = u16;
    type IntoIter = CandidateSetIterator;
    fn into_iter(self) -> CandidateSetIterator {
        debug_assert!(
            (self.candidates & !SOLVED) != 0,
            "Candidate sets should never be empty"
        );
        CandidateSetIterator {
            candidates: self.candidates & !SOLVED,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn test_new_numbers() {
        for i in 1..10 {
            let c = char::from_digit(i, 10).unwrap();
            let p = parse_position(c).unwrap();
            assert_eq!(true, p.is_solved());
            if let Some(v) = p.value() {
                // Values should be equal
                assert_eq!(i as u8, v);
                // We should get the mask back
                for iv in p {
                    assert_eq!(1 << (i - 1), iv);
                }
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_new_none() {
        assert_eq!(None, parse_position('/'));
    }

    #[test]
    fn test_new_zero() {
        for c in ['.', '0', '*'].iter() {
            assert_eq!(ALL_CAND, parse_position(*c).unwrap().candidates);
        }
    }

    #[test]
    fn test_struct_size() {
        assert_eq!(2, mem::size_of::<CandidateSet>());
    }

    #[test]
    fn test_iterator_one() {
        let mut iter = CandidateSetIterator { candidates: 1 << 1 };
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_multiple() {
        let mut iter = CandidateSetIterator {
            candidates: 1 | 1 << 4 | 1 << 5,
        };
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(1 << 4), iter.next());
        assert_eq!(Some(1 << 5), iter.next());
        assert_eq!(None, iter.next());
    }
}
