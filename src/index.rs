pub type IndexTuple = (usize, usize, usize);

pub fn get_index_tuple(i: usize) -> IndexTuple {
    let row_i: usize = i / 9;
    let col_i: usize = i % 9;
    let box_i: usize = ((row_i / 3) * 3) + (col_i / 3);
    (row_i, col_i, box_i)
}
