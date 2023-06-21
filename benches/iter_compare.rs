use criterion::*;
use rs_sudoku::fast_index;
use rs_sudoku::slow_index::*;

fn bench(c: &mut Criterion) {
    c.bench_function("slow_old", |b| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0_usize,
                    gp: Box::new(BoxGenPosition::new(idx)),
                })
                .sum::<usize>()
        })
    });

    c.bench_function("fast_new", |b| {
        b.iter(|| (0..9).flat_map(fast_index::box_iter).sum::<usize>())
    });

    c.bench_function("slow_old", |b| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0_usize,
                    gp: Box::new(ColumnGenPosition::new(idx)),
                })
                .sum::<usize>()
        })
    });
    c.bench_function("fast_new", |b| {
        b.iter(|| (0..9).flat_map(fast_index::column_iter).sum::<usize>())
    });
    c.bench_function("slow_old", |b| {
        b.iter(|| {
            (0..9)
                .flat_map(|idx| RelatedIndexIterator {
                    incr: 0_usize,
                    gp: Box::new(RowGenPosition::new(idx)),
                })
                .sum::<usize>()
        })
    });
    c.bench_function("fast_new", |b| {
        b.iter(|| (0..9).flat_map(fast_index::row_iter).sum::<usize>())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
