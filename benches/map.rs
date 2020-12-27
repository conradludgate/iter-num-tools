use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::{lin_space, map::Map};
use iter_num_tools::Exp2;

pub fn bench_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("Map");

    group.bench_function("map [0,20) exp2", |b| {
        b.iter(|| Map::new(lin_space(0.0..20.0, 20), Exp2).collect::<Vec<f32>>())
    });

    group.bench_function("map [0,20) exp2 std", |b| {
        b.iter(|| {
            lin_space(0.0..20.0, 20).map(f32::exp2).collect::<Vec<f32>>()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_map);
criterion_main!(benches);
