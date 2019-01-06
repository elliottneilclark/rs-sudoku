use crate::fast_index_data::*;
use std::iter::Iterator;

#[derive(Debug)]
pub struct FastIndexIter {
    incr: usize,
    idx_pos: &'static [u8],
}
impl Iterator for FastIndexIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.incr >= self.idx_pos.len() {
            None
        } else {
            let res = Some(self.idx_pos[self.incr] as usize);
            self.incr += 1;
            res
        }
    }
}

pub fn row_iter(idx: usize) -> FastIndexIter {
    let start = idx * 9;
    let end = start + 9;
    FastIndexIter {
        incr: 0,
        idx_pos: &ROW_DATA[start..end],
    }
}
pub fn column_iter(idx: usize) -> FastIndexIter {
    let start = idx * 9;
    let end = start + 9;
    FastIndexIter {
        incr: 0,
        idx_pos: &COLUMN_DATA[start..end],
    }
}
pub fn box_iter(idx: usize) -> FastIndexIter {
    let start = idx * 9;
    let end = start + 9;
    FastIndexIter {
        incr: 0,
        idx_pos: &BOX_DATA[start..end],
    }
}

#[derive(Debug)]
pub struct CombinationIterator {
    num_idx: usize,
    idx: [usize; 4],
    values: &'static [u8],
}

impl CombinationIterator {
    pub fn new(num_idx: usize, values: &'static [u8]) -> Self {
        assert!(num_idx <= 4);
        assert!(num_idx >= 2);
        let mut idx = [0; 4];
        for (i, idx_item) in idx.iter_mut().enumerate().take(num_idx) {
            *idx_item = i;
        }
        idx[num_idx - 1] -= 1;
        CombinationIterator {
            num_idx,
            idx,
            values,
        }
    }
}

impl Iterator for CombinationIterator {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        // First thing we do is start at the last index
        let mut current_level: usize = self.num_idx - 1;

        // While there are still levels to visit
        // keep trying to increment.
        while current_level < 4 {
            // Move the current level forward one unconditionally
            self.idx[current_level] += 1;

            // Calculate how many positions/numbers are needed
            // to fill out the remaining levels.
            let needed_after = self.num_idx as i32 - (current_level + 1) as i32;
            // If there aren't more values left then try
            // going down a level to start incrementing
            if self.idx[current_level] as i32 + needed_after >= 9 {
                // If this is already the first level then we're done.
                if current_level == 0 {
                    return None;
                }
                current_level -= 1;
            } else {
                // If we aren't at the end then then start the next level
                // one less then the we think it's real value should be
                // That will allow the uncoditional increment above
                // to set it to the real value.
                if current_level < self.num_idx - 1 {
                    self.idx[current_level + 1] = self.idx[current_level];
                }
                // Move forward one level
                current_level += 1;
            }
        }
        Some(
            self.idx[0..self.num_idx]
                .iter()
                .cloned()
                .map(|i| self.values[i] as usize)
                .collect(),
        )
    }
}

pub fn row_comb_iter(idx: usize, sz: usize) -> CombinationIterator {
    let start = idx * 9;
    let end = start + 9;
    CombinationIterator::new(sz, &ROW_DATA[start..end])
}
pub fn column_comb_iter(idx: usize, sz: usize) -> CombinationIterator {
    let start = idx * 9;
    let end = start + 9;
    CombinationIterator::new(sz, &COLUMN_DATA[start..end])
}
pub fn box_comb_iter(idx: usize, sz: usize) -> CombinationIterator {
    let start = idx * 9;
    let end = start + 9;
    CombinationIterator::new(sz, &BOX_DATA[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slow_index;
    #[test]
    fn test_iter() {
        for row in 0..9 {
            let i = row_iter(row).zip(slow_index::row_iter(row));
            for (fast, slow) in i {
                assert_eq!(fast, slow);
            }
        }
        for col in 0..9 {
            let i = column_iter(col).zip(slow_index::column_iter(col));
            for (fast, slow) in i {
                assert_eq!(fast, slow);
            }
        }
        for box_i in 0..9 {
            let i = box_iter(box_i).zip(slow_index::box_iter(box_i));
            for (fast, slow) in i {
                assert_eq!(fast, slow);
            }
        }
    }
    #[test]
    fn test_comb_iter() {
        for sz in 2..5 {
            for row in 0..9 {
                let i = row_comb_iter(row, sz).zip(slow_index::row_comb_iter(row, sz));
                for (fast, slow) in i {
                    assert_eq!(fast, slow);
                }
            }
            for col in 0..9 {
                let i = column_comb_iter(col, sz).zip(slow_index::column_comb_iter(col, sz));
                for (fast, slow) in i {
                    assert_eq!(fast, slow);
                }
            }
            for box_i in 0..9 {
                let i = box_comb_iter(box_i, sz).zip(slow_index::box_comb_iter(box_i, sz));
                for (fast, slow) in i {
                    assert_eq!(fast, slow);
                }
            }
        }
    }
}
