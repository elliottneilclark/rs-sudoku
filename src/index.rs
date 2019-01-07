use crate::fast_index;

pub enum Grouping {
    RowType,
    ColumnType,
    BoxType,
}

pub const ALL_GROUPINGS: [Grouping; 3] =
    [Grouping::RowType, Grouping::ColumnType, Grouping::BoxType];

impl Grouping {
    pub fn iter(&self, i: usize) -> impl std::iter::Iterator<Item = usize> {
        match *self {
            Grouping::RowType => fast_index::row_iter(i),
            Grouping::ColumnType => fast_index::column_iter(i),
            Grouping::BoxType => fast_index::box_iter(i),
        }
    }
    pub fn sub_iter(
        &self,
        i: usize,
        subset_size: usize,
    ) -> impl std::iter::Iterator<Item = Vec<usize>> {
        match *self {
            Grouping::RowType => fast_index::row_comb_iter(i, subset_size),
            Grouping::ColumnType => fast_index::column_comb_iter(i, subset_size),
            Grouping::BoxType => fast_index::box_comb_iter(i, subset_size),
        }
    }
    pub fn start_row(&self, i: usize) -> usize {
        match *self {
            Grouping::RowType => i,
            Grouping::ColumnType => 0,
            Grouping::BoxType => (i / 3) * 3,
        }
    }
    pub fn start_column(&self, i: usize) -> usize {
        match *self {
            Grouping::RowType => 0,
            Grouping::ColumnType => i,
            Grouping::BoxType => (i % 3) * 3,
        }
    }
}
