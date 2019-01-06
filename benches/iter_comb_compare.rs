use criterion::*;
use rs_sudoku::fast_index;
use rs_sudoku::slow_index::*;

fn bench(c: &mut Criterion) {
    c.bench(
        "Combination Iters (box)",
        ParameterizedBenchmark::new(
            "old slow",
            |b, sz| {
                b.iter(|| {
                    MultiRelatedIndexIterator::new(*sz)
                        .set_gen_position(Box::new(RowGenPosition::new(4)))
                        .map(|v| v.into_iter().fold(0, |a, b| a + b))
                        .fold(0, |a, b| a + b)
                })
            },
            vec![2, 3, 4],
        )
        .with_function("new fast", |b, sz| {
            b.iter(|| {
                fast_index::box_comb_iter(4, *sz)
                    .map(|v| v.into_iter().fold(0, |a, b| a + b))
                    .fold(0, |a, b| a + b)
            })
        }),
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
