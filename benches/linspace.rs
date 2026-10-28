use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::lin_space;

fn bench(i: impl Iterator<Item = f64>) -> f64 {
    black_box(i.sum())
}

fn lin_space_std(start: f64, end: f64, steps: usize) -> impl Iterator<Item = f64> {
    let len = end - start;
    let step = len / (steps - 1) as f64;
    (0..steps).map(move |i| start + i as f64 * step)
}

pub fn bench_lin_space(c: &mut Criterion) {
    // Check equal work before timing; the inclusive baseline used to emit 201 values.
    let expected: Vec<_> = lin_space(1.0..=100.0, 200).collect();
    assert_eq!(expected.len(), 200);
    assert_eq!(expected.first(), Some(&1.0));
    assert_eq!(expected.last(), Some(&100.0));
    for actual in [
        lin_space_std(1.0, 100.0, 200).collect::<Vec<_>>(),
        itertools_num::linspace(1.0, 100.0, 200).collect(),
    ] {
        assert_eq!(actual.len(), expected.len());
        assert!(actual
            .iter()
            .zip(&expected)
            .all(|(a, b)| (a - b).abs() < 1e-10));
    }

    let mut group = c.benchmark_group("LinSpace sum [1,100] x200");

    group.bench_function("iter-num-tools", |b| {
        b.iter(|| bench(lin_space(black_box(1.0..=100.0), black_box(200))))
    });

    group.bench_function("std", |b| {
        b.iter(|| {
            bench(lin_space_std(
                black_box(1.0),
                black_box(100.0),
                black_box(200),
            ))
        })
    });

    group.bench_function("itertools-num", |b| {
        b.iter(|| {
            bench(itertools_num::linspace(
                black_box(1.0),
                black_box(100.0),
                black_box(200),
            ))
        })
    });

    group.finish();

    let mut group = c.benchmark_group("LinSpace map-collect [1,100] x200");

    group.bench_function("iter-num-tools", |b| {
        b.iter(|| {
            black_box(
                lin_space(black_box(1.0..=100.0), black_box(200))
                    .map(|x| x * 2.0)
                    .collect::<Vec<_>>(),
            )
        })
    });

    group.bench_function("std", |b| {
        b.iter(|| {
            black_box(
                lin_space_std(black_box(1.0), black_box(100.0), black_box(200))
                    .map(|x| x * 2.0)
                    .collect::<Vec<_>>(),
            )
        })
    });

    group.bench_function("itertools-num", |b| {
        b.iter(|| {
            black_box(
                itertools_num::linspace(black_box(1.0), black_box(100.0), black_box(200))
                    .map(|x| x * 2.0)
                    .collect::<Vec<_>>(),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lin_space);
criterion_main!(benches);
