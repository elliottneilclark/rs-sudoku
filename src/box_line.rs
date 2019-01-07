use crate::index::Grouping;
use crate::index_helpers::*;
use crate::remove_mask::*;
use crate::sudoku::Sudoku;

pub trait LineBoxReduction {
    fn box_line_reduce(&mut self) -> (usize, usize);
}

impl LineBoxReduction for Sudoku {
    fn box_line_reduce(&mut self) -> (usize, usize) {
        [Grouping::RowType, Grouping::ColumnType]
            .into_iter()
            .find_map(|g| {
                (0..9).find_map(|g_idx| {
                    let mut val_pos: [usize; 9] = [0; 9];
                    for idx in g.iter(g_idx) {
                        let (_, _, box_i) = get_index_tuple(idx);
                        for c in self[idx] {
                            val_pos[c.trailing_zeros() as usize] |= 1 << box_i;
                        }
                    }
                    let removed: usize = val_pos
                        .into_iter()
                        .enumerate()
                        .filter(|(_val_idx, val_pos_mask)| val_pos_mask.count_ones() == 1)
                        .map(|(val_idx, val_pos_mask)| {
                            let box_i = val_pos_mask.trailing_zeros() as usize;
                            let iter = Grouping::BoxType.iter(box_i).filter(|i| {
                                let (r, c, _b) = get_index_tuple(*i);
                                match g {
                                    Grouping::RowType => r != g_idx,
                                    Grouping::ColumnType => c != g_idx,
                                    _ => true,
                                }
                            });
                            self.remove_mask(1 << val_idx, iter)
                        })
                        .sum();
                    if removed > 0 {
                        Some((removed, 0))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| (0, 0))
    }
}
