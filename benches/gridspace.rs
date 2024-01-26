use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::grid_space;

fn bench(i: impl Iterator<Item = [f64; 2]>) -> f64 {
    black_box(i.map(|[a, b]| a + b).sum())
}

pub fn bench_grid_space(c: &mut Criterion) {
    let mut group = c.benchmark_group("GridSpace");

    group.bench_function("gridspace [1.0, 100.0] x200 (iter-num-tools)", |b| {
        b.iter(|| bench(grid_space(black_box([1.0, 1.0]..=[100.0, 100.0]), black_box(200))))
    });

    group.finish();
}

criterion_group!(benches, bench_grid_space);
criterion_main!(benches);
