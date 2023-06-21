use criterion::*;
use rs_sudoku::fast_index;
use rs_sudoku::slow_index::*;

fn bench_old(c: &mut Criterion) {
    let mut old_group = c.benchmark_group("Old Slow");
    for size in [2, 3, 4] {
        let id = BenchmarkId::new("size", size);
        old_group.bench_with_input(id.clone(), &size, |b: &mut Bencher, size: &usize| {
            b.iter(|| {
                MultiRelatedIndexIterator::new(*size)
                    .set_gen_position(Box::new(RowGenPosition::new(4)))
                    .map(|v| v.into_iter().sum::<usize>())
                    .sum::<usize>()
            })
        });
    }
}

fn bench_new(c: &mut Criterion) {
    let mut new_group = c.benchmark_group("New Fast");
    for size in [2, 3, 4] {
        let id = BenchmarkId::new("size", size);
        new_group.bench_with_input(id, &size, |b: &mut Bencher, size: &usize| {
            b.iter(|| {
                fast_index::box_comb_iter(4, *size)
                    .map(|v| v.into_iter().sum::<usize>())
                    .sum::<usize>()
            })
        });
    }
}

criterion_group!(benches, bench_new, bench_old);
criterion_main!(benches);
