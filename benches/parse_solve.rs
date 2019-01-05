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
    c.bench_function("easy parse", |b| b.iter(|| parse_solve(ONE_LINE)));
    c.bench_function("multiline parse", |b| b.iter(|| parse_solve(MULTI_LINE)));
    c.bench_function("hard", |b| {
        b.iter(|| {
            parse_solve(
                "....5..6.9.5..82...4..97.5..64......5.2.8.7.1......63..8.52..1...39..4.2.2..6....",
            )
        })
    });
    c.bench_function("parse/solve lots", |b| {
        b.iter(|| PUZZLES.lines().filter_map(parse_solve).count())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
