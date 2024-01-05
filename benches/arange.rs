use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::arange;

fn bench(i: impl Iterator<Item = f64>) -> f64 {
    black_box(black_box(i).sum())
}

pub fn bench_arange(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arange");

    group.bench_function("arange [0,200) steps 1.0", |b| {
        b.iter(|| bench(arange(0.0..200.0, 1.0)))
    });

    group.bench_function("arange [0,200) steps 1.0 std", |b| {
        b.iter(|| {
            let mut start = 0.0;
            bench((0..200).map(|_| {
                let result = start;
                start += 1.0;
                result
            }))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_arange);
criterion_main!(benches);
