use crate::fast_index;

pub enum Grouping {
    Row,
    Column,
    Box,
}

pub const ALL_GROUPINGS: [Grouping; 3] = [Grouping::Row, Grouping::Column, Grouping::Box];

impl Grouping {
    pub fn iter(&self, i: usize) -> impl std::iter::Iterator<Item = usize> {
        match *self {
            Grouping::Row => fast_index::row_iter(i),
            Grouping::Column => fast_index::column_iter(i),
            Grouping::Box => fast_index::box_iter(i),
        }
    }
    pub fn sub_iter(
        &self,
        i: usize,
        subset_size: usize,
    ) -> impl std::iter::Iterator<Item = Vec<usize>> {
        match *self {
            Grouping::Row => fast_index::row_comb_iter(i, subset_size),
            Grouping::Column => fast_index::column_comb_iter(i, subset_size),
            Grouping::Box => fast_index::box_comb_iter(i, subset_size),
        }
    }
    pub fn start_row(&self, i: usize) -> usize {
        match *self {
            Grouping::Row => i,
            Grouping::Column => 0,
            Grouping::Box => (i / 3) * 3,
        }
    }
    pub fn start_column(&self, i: usize) -> usize {
        match *self {
            Grouping::Row => 0,
            Grouping::Column => i,
            Grouping::Box => (i % 3) * 3,
        }
    }
}
