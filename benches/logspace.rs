use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::{lin_space, log_space};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("logspace [1,1000) x20", |b| {
        b.iter(|| log_space(1.0..1000.0, 20).collect::<Vec<f32>>())
    });

    c.bench_function("logspace [1,1000) x20 std", |b| {
        b.iter(|| lin_space(1.0f32.log2()..1000.0f32.log2(), 20).map(f32::exp2).collect::<Vec<f32>>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
