use super::candidate_set::CandidateSet;
use super::index::ALL_GROUPINGS;

pub struct Sudoku {
    pub positions: [CandidateSet; 81],
}

const ALL_POSSIBLE: usize = (1 << 9) - 1;
fn valid_group<T: Iterator<Item = usize>>(sudoku: &Sudoku, mut iter: T) -> bool {
    // Set for solved
    let mut s = 0;
    // set for all
    let mut a = 0;
    iter.all(|i| {
        let m = sudoku.positions[i].get_candidates();
        a |= m;
        if sudoku.positions[i].is_solved() {
            if (s & m) != 0 {
                false
            } else {
                s |= m;
                true
            }
        } else {
            true
        }
    }) && a == ALL_POSSIBLE
}

impl Sudoku {
    pub fn num_solved(&self) -> usize {
        self.positions.iter().filter(|x| x.is_solved()).count()
    }

    pub fn is_solved(&self) -> bool {
        self.positions.iter().all(|x| x.is_solved())
    }

    pub fn oneline(&self) -> String {
        self.positions
            .iter()
            .map(|p| {
                let v = p.value().unwrap_or(0);
                match v {
                    0 => ".".to_string(),
                    _ => v.to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    /// Check to see if the puzzle is invalid. Where invalid means:
    ///
    /// This puzzle doesn't have the same digit twice in a row, col, box.
    /// Every digit is either in the candidate set or solved in every row, col, box.
    ///
    /// This doesn't 100% mean that the puzzle has a unique solution.
    pub fn is_valid(&self) -> bool {
        ALL_GROUPINGS
            .iter()
            .all(|g| (0..9).all(|idx| valid_group(self, g.iter(idx))))
    }
}
