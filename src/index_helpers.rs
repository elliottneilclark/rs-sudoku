pub fn to_index(row: usize, col: usize) -> usize {
    (row * 9) + col
}

pub type IndexTuple = (usize, usize, usize);
pub fn get_index_tuple(i: usize) -> IndexTuple {
    let row_i = i / 9;
    let col_i = i % 9;
    let box_i = ((row_i / 3) * 3) + (col_i / 3);
    (row_i, col_i, box_i)
}
