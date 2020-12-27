use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::log_space;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("logspace [1,1000) x20", |b| {
        b.iter(|| log_space(1.0..1000.0, 20).collect::<Vec<_>>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
