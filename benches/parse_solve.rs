use criterion::*;
use rs_sudoku::examples::*;
use rs_sudoku::{parse_sudoku, SolveReport, Solveable};

fn parse_solve(p_str: &str) -> Option<SolveReport> {
    if let Ok(p) = parse_sudoku(p_str) {
        let sr = p.try_solve();
        assert!(sr.is_valid);
        Some(sr)
    } else {
        None
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("easy", |b| b.iter(|| parse_solve(ONE_LINE)));

    c.bench_function("multi_line", |b| b.iter(|| parse_solve(MULTI_LINE)));

    c.bench_function("lots", |b| {
        b.iter(|| PUZZLES.lines().filter_map(parse_solve).count())
    });
    c.bench_function("hard", |b| b.iter(|| parse_solve(HARD)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
