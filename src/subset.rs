use crate::index::{RelatedIndexIterator, ALL_GROUPINGS};
use crate::sudoku::Sudoku;

pub trait FindSubset {
    fn find_subset(&mut self) -> (usize, usize);
}

/// Enum for the result of gen_subset
/// The usize is the mask of candidates to keep in the affected cells.
///
/// The effected cells are:
/// Naked: The cells that don't have the double,triple,quad
/// Hidden: The cells that make up the double,triple,quad
///
/// The tuple is all the indexes used to create the subset.
#[derive(Debug)]
enum Subset {
    Naked(usize, Vec<usize>),
    Hidden(usize, Vec<usize>),
}

fn gen_subset(sudoku: &Sudoku, subset: Vec<usize>, g_iter: RelatedIndexIterator) -> Option<Subset> {
    let expected_count = subset.len();
    let mask: usize = subset
        .iter()
        .map(|i| sudoku.positions[*i].get_candidates())
        .fold(0, |a, b| a | b);
    let other: usize = g_iter
        .filter(|i| !subset.iter().any(|s| *i == *s))
        .map(|i| sudoku.positions[i].get_candidates())
        .fold(0, |a, b| a | b);
    let m_count = mask.count_ones() as usize;
    let m_only_count = (mask & !other).count_ones() as usize;
    // Ok now we have a mask of candidates that are in the double or triple,
    // and a different mask for candidates that are ther for other locations.
    if m_count == expected_count && (other & mask) != 0 {
        // If the number of candidates in two unsolved positions is 2
        // and there's at least one occurance in other locations then
        // we know that a NakedDouble can result in some progress.
        Some(Subset::Naked(!mask, subset))
    } else if m_only_count == expected_count && m_count > expected_count {
        // If the number of candidates that are only in this set of tw
        // locations is equal to 2 then they are the only candidat
        // for these locations
        Some(Subset::Hidden(mask & !other, subset))
    } else {
        None
    }
}

fn remove_candidate(sudoku: &mut Sudoku, mask: usize, p: usize) -> (usize, usize) {
    let c = sudoku.positions[p].candidates & mask;
    if c != sudoku.positions[p].candidates {
        sudoku.positions[p].candidates = c;
        if c.count_ones() == 1 {
            sudoku.positions[p].set_solved();
            (1, 1)
        } else {
            (1, 0)
        }
    } else {
        (0, 0)
    }
}

fn remove_candidates<I: std::iter::Iterator<Item = usize>>(
    sudoku: &mut Sudoku,
    mask: usize,
    g_iter: I,
) -> Option<(usize, usize)> {
    let mut changed = 0;
    let mut solved = 0;
    for p in g_iter {
        let (c, s) = remove_candidate(sudoku, mask, p);
        changed += c;
        solved += s;
    }
    if changed > 0 || solved > 0 {
        Some((changed, solved))
    } else {
        None
    }
}
fn handle_subset(
    sudoku: &mut Sudoku,
    m: Subset,
    i: RelatedIndexIterator,
) -> Option<(usize, usize)> {
    match m {
        Subset::Naked(mask, v) => {
            // // For all positions other than the positions in the double remove using the mask
            remove_candidates(sudoku, mask, i.filter(|p| !v.iter().any(|s| *s == *p)))
        }
        Subset::Hidden(mask, v) => {
            // // Remove anything other than the double in the positions in the double
            remove_candidates(sudoku, mask, v.into_iter())
        }
    }
}

impl FindSubset for Sudoku {
    fn find_subset(&mut self) -> (usize, usize) {
        // For subset sizes 2,3,4
        (2..5)
            .find_map(|sub_size| {
                // Go throught all the groupings
                ALL_GROUPINGS.iter().find_map(|g| {
                    (0..9).find_map(|idx| {
                        // Then for a grouping get combination iter.
                        g.sub_iter(idx, sub_size).find_map(|sub| {
                            // See if this combination of index's has a subset
                            let found = gen_subset(self, sub, g.iter(idx));
                            // If it did then remove them and return the results
                            found.and_then(|s| handle_subset(self, s, g.iter(idx)))
                        })
                    })
                })
            })
            // If nothing else then return 0
            .unwrap_or_else(|| (0, 0))
    }
}
