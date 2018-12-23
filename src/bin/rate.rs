extern crate rs_sudoku;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hey {:?}", args);
}
