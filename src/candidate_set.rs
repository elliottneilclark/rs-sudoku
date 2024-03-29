use std::borrow::{Borrow, BorrowMut};
use std::char;
use std::convert::From;
use std::fmt;
use std::iter::{IntoIterator, Iterator};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct CandidateSet(usize);

// Constant with all the candidates turned on.
const ALL_CAND: usize = 1 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8;

const SOLVED: usize = 1 << 9;

impl CandidateSet {
    pub fn parse_position(c: char) -> Option<Self> {
        // Use a match to get a really small lookup table based decoding.
        match c {
            '1' => Some(CandidateSet::new(SOLVED | 1)),
            '2' => Some(CandidateSet::new(SOLVED | 1 << 1)),
            '3' => Some(CandidateSet::new(SOLVED | 1 << 2)),
            '4' => Some(CandidateSet::new(SOLVED | 1 << 3)),
            '5' => Some(CandidateSet::new(SOLVED | 1 << 4)),
            '6' => Some(CandidateSet::new(SOLVED | 1 << 5)),
            '7' => Some(CandidateSet::new(SOLVED | 1 << 6)),
            '8' => Some(CandidateSet::new(SOLVED | 1 << 7)),
            '9' => Some(CandidateSet::new(SOLVED | 1 << 8)),
            '0' | '.' | '*' | 'x' => Some(CandidateSet::new(ALL_CAND)),
            _ => None,
        }
    }

    pub fn new(v: usize) -> Self {
        CandidateSet(v)
    }

    pub fn num_candidates(self) -> usize {
        (self.0 & !SOLVED).count_ones() as usize
    }

    pub fn is_solved(self) -> bool {
        (self.0 & SOLVED) == SOLVED
    }
    pub fn set_solved(&mut self) {
        debug_assert!(
            self.0.count_ones() == 1,
            "Only set solved on really solved sets."
        );
        self.0 |= SOLVED;
    }
    pub fn value(self) -> Option<usize> {
        if self.is_solved() {
            let v = (self.0 & !SOLVED).trailing_zeros() + 1;
            Some(v as usize)
        } else {
            None
        }
    }
    pub fn get_candidates(self) -> usize {
        self.0 & !SOLVED
    }
    pub fn get(self) -> usize {
        self.0
    }
}

impl Borrow<usize> for CandidateSet {
    fn borrow(&self) -> &usize {
        &self.0
    }
}

impl BorrowMut<usize> for CandidateSet {
    fn borrow_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl From<CandidateSet> for usize {
    fn from(value: CandidateSet) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CandidateSetIterator(usize);
impl CandidateSetIterator {
    pub fn new(c: usize) -> Self {
        CandidateSetIterator(c)
    }
}

impl Iterator for CandidateSetIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 > 0 {
            // Keep a copy of the current value
            let sav = self.0;
            // Remove the last set bit
            // and save it in self.candidates
            self.0 &= self.0 - 1;
            // Now return the difference
            Some(sav - self.0)
        } else {
            None
        }
    }
}

impl IntoIterator for CandidateSet {
    type Item = usize;
    type IntoIter = CandidateSetIterator;
    fn into_iter(self) -> CandidateSetIterator {
        debug_assert!(
            self.get_candidates().count_ones() <= 9,
            "We should always have 9 or fewer digits set."
        );
        CandidateSetIterator::new(self.get_candidates())
    }
}

impl fmt::Debug for CandidateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For every mask convert it to the
        // possible value, convert that to
        // string, then collect that into a vec, then join into a string.
        let cstr = self
            .into_iter()
            .map(|c| (c.trailing_zeros() + 1).to_string())
            .collect::<Vec<String>>()
            .join(", ");
        // Add that all to the formatter.
        write!(
            f,
            "CandidateSet{{candidates: [{}], is_solved: {}, num_candidates: {} }}",
            cstr,
            self.is_solved(),
            self.num_candidates()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_numbers() {
        for i in 1..10 {
            let c = char::from_digit(i, 10).unwrap();
            let p = CandidateSet::parse_position(c).unwrap();
            assert!(p.is_solved());
            if let Some(v) = p.value() {
                // Values should be equal
                assert_eq!(i as usize, v);
                // We should get the mask back
                for iv in p {
                    assert_eq!(1 << (i - 1), iv);
                }
            }
        }
    }

    #[test]
    fn test_new_none() {
        assert_eq!(None, CandidateSet::parse_position('/'));
    }

    #[test]
    fn test_new_zero() {
        for c in ['.', '0', '*'].iter() {
            assert_eq!(ALL_CAND, CandidateSet::parse_position(*c).unwrap().into());
        }
    }

    #[test]
    fn test_iterator_one() {
        let mut iter = CandidateSetIterator::new(1 << 1);
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_multiple() {
        let mut iter = CandidateSetIterator::new(1 | 1 << 4 | 1 << 5);
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(1 << 4), iter.next());
        assert_eq!(Some(1 << 5), iter.next());
        assert_eq!(None, iter.next());
    }
}
