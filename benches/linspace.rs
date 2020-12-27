use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::lin_space;

pub fn bench_lin_space(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinSpace");

    group.bench_function("linspace [0,1) x20", |b| {
        b.iter(|| lin_space(1.0..3.0, 20).collect::<Vec<f32>>())
    });

    group.bench_function("linspace [0,1) x20 std", |b| {
        b.iter(|| {
            (0..20)
                .map(|i| 1.0 + (i as f32) / 20.0 * 2.0)
                .collect::<Vec<f32>>()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lin_space);
criterion_main!(benches);

// n = (steps as F)
// x = (i as F) / n
// y0 + x * (y1 - y0)
