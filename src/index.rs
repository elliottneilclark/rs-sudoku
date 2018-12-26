use std::iter::Iterator;

pub type IndexTuple = (u8, u8, u8);

pub fn get_index_tuple(i: u8) -> IndexTuple {
    let row_i: u8 = i / 9;
    let col_i: u8 = i % 9;
    let box_i: u8 = ((row_i / 3) * 3) + (col_i / 3);
    (row_i, col_i, box_i)
}

fn to_index(row: u8, col: u8) -> u8 {
    (row * 9) + col
}

pub trait NextPosition {
    fn next_position(start_row: u8, start_col: u8, inc: u8) -> u8;
}

struct RowNextPosition;
impl NextPosition for RowNextPosition {
    fn next_position(start_row: u8, start_col: u8, inc: u8) -> u8 {
        to_index(start_row, start_col + inc)
    }
}

struct ColumnNextPosition;
impl NextPosition for ColumnNextPosition {
    fn next_position(start_row: u8, start_col: u8, inc: u8) -> u8 {
        to_index(start_row + inc, start_col)
    }
}

struct BoxNextPosition;
impl NextPosition for BoxNextPosition {
    fn next_position(start_row: u8, start_col: u8, inc: u8) -> u8 {
        let row_inc = inc / 3;
        let col_inc = inc % 3;
        to_index(start_row + row_inc, start_col + col_inc)
    }
}

#[derive(Debug)]
pub struct RelatedIndexIterator<T: NextPosition> {
    start_row: u8,
    start_col: u8,
    incr: u8,
    next_position: T,
}

impl<T: NextPosition> Iterator for RelatedIndexIterator<T> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.incr >= 9 {
            None
        } else {
            let i = T::next_position(self.start_row, self.start_col, self.incr);
            self.incr += 1;
            Some(i as u8)
        }
    }
}

pub fn row_iter(row: u8) -> impl Iterator<Item = u8> {
    RelatedIndexIterator {
        start_row: row,
        start_col: 0,
        incr: 0,
        next_position: RowNextPosition,
    }
}
pub fn col_iter(col: u8) -> impl Iterator<Item = u8> {
    RelatedIndexIterator {
        start_row: 0,
        start_col: col,
        incr: 0,
        next_position: ColumnNextPosition,
    }
}
pub fn box_iter(box_i: u8) -> impl Iterator<Item = u8> {
    let start_row = (box_i / 3) * 3;
    let start_col = (box_i % 3) * 3;
    RelatedIndexIterator {
        start_row: start_row,
        start_col: start_col,
        incr: 0,
        next_position: BoxNextPosition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_row() {
        let mut idx = 0;
        for i in row_iter(0) {
            assert_eq!(idx, i);
            idx += 1;
        }
    }

    #[test]
    fn test_first_col() {
        let mut idx = 0;
        for i in col_iter(0) {
            assert_eq!(idx, i);
            idx += 9;
        }
    }

    #[test]
    fn test_first_box() {
        for idx in box_iter(0) {
            assert!(idx < 27);
        }
    }

    #[test]
    fn test_all_row() {
        for row in 0..9 {
            for idx in row_iter(row) {
                let (row_i, _col_i, _box_i) = get_index_tuple(idx);
                assert_eq!(row, row_i);
            }
        }
    }

    #[test]
    fn test_all_col() {
        for col in 0..9 {
            for idx in col_iter(col) {
                let (_row_i, col_i, _box_i) = get_index_tuple(idx);
                assert_eq!(col, col_i);
            }
        }
    }

    #[test]
    fn test_all_box() {
        for b in 0..9 {
            for idx in box_iter(b) {
                let (_row_i, _col_i, box_i) = get_index_tuple(idx);
                assert_eq!(b, box_i);
            }
        }
    }
}
