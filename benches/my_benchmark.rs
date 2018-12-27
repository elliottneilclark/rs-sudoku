use criterion::*;
use rs_sudoku::examples::*;
use rs_sudoku::{parse_sudoku, SolveReport, Solveable};

fn parse_solve(p_str: &str) -> Option<SolveReport> {
    if let Ok(p) = parse_sudoku(p_str) {
        Some(p.try_solve())
    } else {
        None
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("easy parse", |b| b.iter(|| parse_solve(ONE_LINE)));
    c.bench_function("multiline parse", |b| b.iter(|| parse_solve(MULTI_LINE)));
    c.bench_function("multiline hard", |b| {
        b.iter(|| {
            parse_solve(
                "....5..6.9.5..82...4..97.5..64......5.2.8.7.1......63..8.52..1...39..4.2.2..6....",
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
