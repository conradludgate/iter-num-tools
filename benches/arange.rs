use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::arange;

pub fn bench_arange(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arange");

    group.bench_function("arange [0,20) steps 1.0", |b| {
        b.iter(|| arange(0.0..20.0, 1.0).collect::<Vec<f32>>())
    });

    group.bench_function("arange [0,20) steps 1.0 std", |b| {
        b.iter(|| {
            let mut start = 0.0;
            (0..20)
                .map(|_| {
                    let result = start;
                    start += 1.0;
                    result
                })
                .collect::<Vec<f32>>()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_arange);
criterion_main!(benches);
