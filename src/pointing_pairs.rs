use crate::index::Grouping;
use crate::index_helpers::*;
use crate::remove_mask::*;
use crate::sudoku::Sudoku;

pub trait Pointing {
    fn handle_pointing(&mut self) -> (usize, usize);
}

impl Pointing for Sudoku {
    fn handle_pointing(&mut self) -> (usize, usize) {
        let g = Grouping::Box;
        (0..9)
            .find_map(|box_i| {
                let start_row = g.start_row(box_i);
                let start_col = g.start_column(box_i);
                let mut row_count = [0; 9];
                // Mask of the candidates that are in this row
                let mut row_mask: [usize; 3] = [0; 3];
                // Mask of candidates that are in other rows
                let mut row_mask_other: [usize; 3] = [0; 3];
                // Mask of the candidates that are in this column
                let mut col_mask: [usize; 3] = [0; 3];
                // Mask of candidates that are in other columns
                let mut col_mask_other: [usize; 3] = [0; 3];

                // Copy down the candidates that are
                // in each row or col of this box.
                for idx in g.iter(box_i) {
                    let (row, col, _bi) = get_index_tuple(idx);
                    // Skip the solved one. Those are
                    // handled by remove_candidates.
                    if self[idx].is_solved() {
                        continue;
                    }

                    let m = self[idx].get();
                    row_mask[row - start_row] |= m;
                    col_mask[col - start_col] |= m;
                    row_count[row] += 1;
                }

                let mut changed = 0;
                for idx in 0..3 {
                    for other_idx in 0..3 {
                        if idx == other_idx {
                            continue;
                        }
                        row_mask_other[other_idx] |= row_mask[idx];
                        col_mask_other[other_idx] |= col_mask[idx];
                    }
                }

                for idx in 0..3 {
                    let only_row = row_mask[idx] & !row_mask_other[idx];
                    let only_col = col_mask[idx] & !col_mask_other[idx];

                    if only_row != 0 {
                        let row = start_row + idx;
                        assert!(only_row.count_ones() <= 3);
                        let iter = Grouping::Row.iter(row).filter(|i| {
                            let (_, _, b) = get_index_tuple(*i);
                            b != box_i
                        });

                        changed += self.remove_mask(only_row, iter);
                    }
                    if only_col != 0 {
                        let col = start_col + idx;
                        assert!(only_col.count_ones() <= 3);
                        let iter = Grouping::Column.iter(col).filter(|i| {
                            let (_, _, b) = get_index_tuple(*i);
                            b != box_i
                        });

                        changed += self.remove_mask(only_col, iter);
                    }
                }
                if changed > 0 {
                    Some((changed, 0))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0))
    }
}
