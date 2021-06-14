use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::lin_space;

fn lin_space_std(start: f64, end: f64, steps: usize) -> impl Iterator<Item = f64> {
    let step = steps as f64;
    let len = end - start;
    (0..steps).map(move |i| start + i as f64 / step * len)
}

pub fn bench_lin_space(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinSpace");

    group.bench_function("linspace [1.0, 3.0) x100", |b| {
        b.iter(|| {
            black_box(
                lin_space(1.0..3.0, 100)
                    .map(|x| x * 2.0)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    group.bench_function("linspace [1.0, 3.0) x100 std", |b| {
        b.iter(|| {
            black_box(
                lin_space_std(1.0, 3.0, 100)
                    .map(|x| x * 2.0)
                    .collect::<Vec<f64>>(),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lin_space);
criterion_main!(benches);

// n = (steps as F)
// x = (i as F) / n
// y0 + x * (y1 - y0)
