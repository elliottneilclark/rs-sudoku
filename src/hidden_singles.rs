use crate::index::{box_iter, col_iter, get_index_tuple, row_iter};
use crate::sudoku::Sudoku;
use std::iter::Iterator;

pub trait HiddenSingles {
    fn handle_hidden_singles(&mut self) -> usize;
}

fn find_hidden_single<T: Iterator<Item = usize>>(
    sudoku: &Sudoku,
    i: usize,
    iter: T,
) -> Option<usize> {
    let set = iter
        .filter_map(|idx| {
            if idx == i {
                None
            } else {
                Some(sudoku.positions[idx].get_candidates())
            }
        })
        .fold(0, |acc, s| acc | s);
    let hidden_value = sudoku.positions[i].get_candidates() & !set;
    if hidden_value == 0 {
        None
    } else {
        Some(hidden_value)
    }
}

fn find_hidden(sudoku: &Sudoku) -> Vec<(usize, usize)> {
    (0..sudoku.positions.len())
        .filter(|i| sudoku.positions[*i].num_candidates() != 1)
        .filter_map(|i| {
            let (row_i, col_i, box_i) = get_index_tuple(i);
            find_hidden_single(sudoku, i, row_iter(row_i))
                .or_else(|| find_hidden_single(sudoku, i, col_iter(col_i)))
                .or_else(|| find_hidden_single(sudoku, i, box_iter(box_i)))
                .map(|h| (i, h))
        })
        .collect()
}

impl HiddenSingles for Sudoku {
    fn handle_hidden_singles(&mut self) -> usize {
        let mut found = 0;
        let hidden = find_hidden(self);

        for (i, h) in hidden {
            found += 1;
            assert_eq!(1, h.count_ones());
            self.positions[i].candidates = h;
            self.positions[i].set_solved();
        }
        found
    }
}
