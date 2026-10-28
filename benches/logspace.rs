use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::{lin_space, log_space};

fn bench(i: impl Iterator<Item = f32>) -> f32 {
    black_box(i.sum())
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("logspace [1,1000) x200", |b| {
        b.iter(|| bench(log_space(black_box(1.0..1000.0), black_box(200))))
    });

    c.bench_function("logspace [1,1000) x200 linspace + exp2", |b| {
        b.iter(|| {
            let start = black_box(1.0f32);
            let end = black_box(1000.0f32);
            bench(lin_space(start.log2()..end.log2(), black_box(200)).map(f32::exp2))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
