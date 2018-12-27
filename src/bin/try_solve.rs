use rs_sudoku::*;
use std::io;
use std::time::Instant;

fn solve_print(line: &str) {
    let now = Instant::now();
    if let Ok(p) = parse_sudoku(line) {
        let report = p.try_solve();
        println!("{}\t{}\t{:?}", line, report.state, now.elapsed());
    }
}

fn main() {
    let mut input = String::new();
    let mut cont = true;
    while cont {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                cont = n > 0;
                solve_print(input.trim());
            }
            Err(error) => {
                cont = false;
                println!("error: {}", error);
            }
        }
        if cont {
            input.clear();
        }
    }
}
