use criterion::{criterion_group, criterion_main, Criterion};
use iter_num_tools::{map::{Map, Function}};

struct Mul2;

impl Function<i32> for Mul2 {
    type Output = i32;

    #[inline]
    fn call(&self, x: i32) -> Self::Output {
        x * 2
    }
}

pub fn bench_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("Map");

    group.bench_function("map [0,20) exp2", |b| {
        b.iter(|| Map::new(0..100, Mul2).collect::<Vec<i32>>())
    });

    group.bench_function("map [0,20) exp2 std", |b| {
        b.iter(|| {
            (0..100).map(|x| x * 2).collect::<Vec<i32>>()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_map);
criterion_main!(benches);
