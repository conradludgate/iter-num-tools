use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::arange;

fn bench(i: impl Iterator<Item = f64>) -> f64 {
    black_box(i.sum())
}

pub fn bench_arange(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arange");

    group.bench_function("arange [0,200) steps 1.0", |b| {
        b.iter(|| bench(arange(black_box(0.0..200.0), black_box(1.0))))
    });

    group.bench_function("arange [0,200) steps 1.0 std", |b| {
        b.iter(|| {
            let mut start = 0.0;
            let step = black_box(1.0);
            bench(black_box(0..200).map(|_| {
                let result = start;
                start += step;
                result
            }))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_arange);
criterion_main!(benches);
