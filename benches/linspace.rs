use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::{lerp::LinSpaceFn, lin_space, map::{Map, Function}};

fn lin_space_std(start: f64, end: f64, steps: usize) -> impl Iterator<Item = f64> {
    let f = LinSpaceFn::new(start..=end, steps);
    (0..steps).map(move |i| f.call(i))
}
fn lin_space_std2(start: f64, end: f64, steps: usize) -> Map<std::ops::Range<usize>, LinSpaceFn<f64>> {
    let f = LinSpaceFn::new(start..=end, steps);
    Map::new(0..steps, f)
}

pub fn bench_lin_space(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinSpace");

    group.bench_function("linspace [1.0, 3.0) x100", |b| {
        b.iter(|| lin_space(black_box(1.0..3.0), black_box(100)).map(|x| x * 2.0).collect::<Vec<f64>>())
    });

    group.bench_function("linspace [1.0, 3.0) x100 std", |b| {
        b.iter(|| lin_space_std(black_box(1.0), black_box(3.0), black_box(100)).map(|x| x * 2.0).collect::<Vec<f64>>())
    });
    group.bench_function("linspace [1.0, 3.0) x100 std2", |b| {
        b.iter(|| lin_space_std2(black_box(1.0), black_box(3.0), black_box(100)).map(|x| x * 2.0).collect::<Vec<f64>>())
    });

    group.finish();
}

criterion_group!(benches, bench_lin_space);
criterion_main!(benches);

// n = (steps as F)
// x = (i as F) / n
// y0 + x * (y1 - y0)
