use std::iter::Iterator;

pub type IndexTuple = (usize, usize, usize);

pub fn get_index_tuple(i: usize) -> IndexTuple {
    let row_i = i / 9;
    let col_i = i % 9;
    let box_i = ((row_i / 3) * 3) + (col_i / 3);
    (row_i, col_i, box_i)
}

fn to_index(row: usize, col: usize) -> usize {
    (row * 9) + col
}

pub trait GenPosition {
    fn gen_position(&self, inc: usize) -> usize;
}

#[derive(Debug, Clone)]
pub struct RowGenPosition {
    start_row: usize,
    start_col: usize,
}
impl RowGenPosition {
    fn new(row: usize) -> RowGenPosition {
        RowGenPosition {
            start_row: row,
            start_col: 0,
        }
    }
}
impl GenPosition for RowGenPosition {
    fn gen_position(&self, inc: usize) -> usize {
        to_index(self.start_row, self.start_col + inc)
    }
}

#[derive(Debug, Clone)]
pub struct ColumnGenPosition {
    start_row: usize,
    start_col: usize,
}
impl ColumnGenPosition {
    fn new(col: usize) -> ColumnGenPosition {
        ColumnGenPosition {
            start_row: 0,
            start_col: col,
        }
    }
}
impl GenPosition for ColumnGenPosition {
    fn gen_position(&self, inc: usize) -> usize {
        to_index(self.start_row + inc, self.start_col)
    }
}

#[derive(Debug, Clone)]
pub struct BoxGenPosition {
    start_row: usize,
    start_col: usize,
}
impl BoxGenPosition {
    fn new(box_i: usize) -> BoxGenPosition {
        let start_row = (box_i / 3) * 3;
        let start_col = (box_i % 3) * 3;
        BoxGenPosition {
            start_row,
            start_col,
        }
    }
}
impl GenPosition for BoxGenPosition {
    fn gen_position(&self, inc: usize) -> usize {
        let row_inc = inc % 3;
        let col_inc = inc / 3;
        to_index(self.start_row + row_inc, self.start_col + col_inc)
    }
}

pub struct RelatedIndexIterator {
    incr: usize,
    gp: Box<GenPosition>,
}
impl Iterator for RelatedIndexIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.incr < 9 {
            let i = self.gp.gen_position(self.incr);
            self.incr += 1;
            Some(i)
        } else {
            None
        }
    }
}

pub struct MultiRelatedIndexIterator {
    idx: [usize; 4],
    num_idx: usize,
    gp: Box<GenPosition>,
}
impl MultiRelatedIndexIterator {
    pub fn new(num_idx: usize) -> Self {
        assert!(num_idx <= 4);
        assert!(num_idx >= 2);
        let mut idx = [0; 4];
        for (i, idx_item) in idx.iter_mut().enumerate().take(num_idx) {
            *idx_item = i;
        }
        idx[num_idx - 1] -= 1;
        MultiRelatedIndexIterator {
            num_idx,
            idx,
            gp: Box::new(RowGenPosition::new(0)),
        }
    }
    pub fn set_gen_position(self, gp: Box<GenPosition>) -> Self {
        MultiRelatedIndexIterator { gp, ..self }
    }
}

impl Iterator for MultiRelatedIndexIterator {
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
                .map(|i| self.gp.gen_position(i))
                .collect(),
        )
    }
}

pub enum Grouping {
    RowType,
    ColumnType,
    BoxType,
}

pub const ALL_GROUPINGS: [Grouping; 3] =
    [Grouping::RowType, Grouping::ColumnType, Grouping::BoxType];

impl Grouping {
    pub fn iter(&self, i: usize) -> RelatedIndexIterator {
        match *self {
            Grouping::RowType => RelatedIndexIterator {
                incr: 0,
                gp: Box::new(RowGenPosition::new(i)),
            },
            Grouping::ColumnType => RelatedIndexIterator {
                incr: 0,
                gp: Box::new(ColumnGenPosition::new(i)),
            },
            Grouping::BoxType => RelatedIndexIterator {
                incr: 0,
                gp: Box::new(BoxGenPosition::new(i)),
            },
        }
    }
    pub fn sub_iter(&self, i: usize, subset_size: usize) -> MultiRelatedIndexIterator {
        match *self {
            Grouping::RowType => MultiRelatedIndexIterator::new(subset_size)
                .set_gen_position(Box::new(RowGenPosition::new(i))),
            Grouping::ColumnType => MultiRelatedIndexIterator::new(subset_size)
                .set_gen_position(Box::new(ColumnGenPosition::new(i))),
            Grouping::BoxType => MultiRelatedIndexIterator::new(subset_size)
                .set_gen_position(Box::new(BoxGenPosition::new(i))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_row() {
        for (idx, item) in Grouping::RowType.iter(0).enumerate() {
            assert_eq!(idx, item);
        }
    }

    #[test]
    fn test_first_col() {
        let mut idx = 0;
        for i in Grouping::ColumnType.iter(0) {
            assert_eq!(idx, i);
            idx += 9;
        }
    }

    #[test]
    fn test_first_box() {
        for i in Grouping::BoxType.iter(0) {
            assert!(i < 27);
        }
    }

    #[test]
    fn test_all_row() {
        for row in 0..9 {
            for idx in Grouping::RowType.iter(row) {
                let (row_i, _col_i, _box_i) = get_index_tuple(idx);
                assert_eq!(row, row_i);
            }
        }
    }

    #[test]
    fn test_all_col() {
        for col in 0..9 {
            for idx in Grouping::ColumnType.iter(col) {
                let (_row_i, col_i, _box_i) = get_index_tuple(idx);
                assert_eq!(col, col_i);
            }
        }
    }

    #[test]
    fn test_all_box() {
        for b in 0..9 {
            for idx in Grouping::BoxType.iter(b) {
                let (_row_i, _col_i, box_i) = get_index_tuple(idx);
                assert_eq!(b, box_i);
            }
        }
    }

    #[test]
    fn test_box_quad_iter() {
        for i in 0..9 {
            // 9 choose 4 = 126
            assert_eq!(126, Grouping::BoxType.sub_iter(i, 4).count());
        }
    }

    #[test]
    fn test_col_quad_iter() {
        for i in 0..9 {
            // 9 choose 4 = 126
            assert_eq!(126, Grouping::ColumnType.sub_iter(i, 4).count());
        }
    }

    #[test]
    fn test_row_quad_iter() {
        for i in 0..9 {
            // 9 choose 4 = 126
            assert_eq!(126, Grouping::RowType.sub_iter(i, 4).count());
        }
    }
}
