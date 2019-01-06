use criterion::*;
use rs_sudoku::fast_index;
use rs_sudoku::slow_index::*;

fn bench(c: &mut Criterion) {
    let slow_box = Fun::new("slow_old", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0 as usize,
                    gp: Box::new(BoxGenPosition::new(idx)),
                })
                .fold(0, |a, b| a + b)
        })
    });
    let fast_box = Fun::new("fast_new", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| fast_index::box_iter(idx))
                .fold(0, |a, b| a + b)
        })
    });

    let slow_col = Fun::new("slow_old", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0 as usize,
                    gp: Box::new(ColumnGenPosition::new(idx)),
                })
                .fold(0, |a, b| a + b)
        })
    });
    let fast_col = Fun::new("fast_new", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| fast_index::column_iter(idx))
                .fold(0, |a, b| a + b)
        })
    });
    let slow_row = Fun::new("slow_old", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0 as usize,
                    gp: Box::new(RowGenPosition::new(idx)),
                })
                .fold(0, |a, b| a + b)
        })
    });
    let fast_row = Fun::new("fast_new", |b, _usused| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| fast_index::row_iter(idx))
                .fold(0, |a, b| a + b)
        })
    });
    let box_functions = vec![slow_box, fast_box];
    let col_functions = vec![slow_col, fast_col];
    let row_functions = vec![slow_row, fast_row];

    c.bench_functions("Iters (box)", box_functions, &100);
    c.bench_functions("Iters (col)", col_functions, &100);
    c.bench_functions("Iters (row)", row_functions, &100);
}

criterion_group!(benches, bench);
criterion_main!(benches);
