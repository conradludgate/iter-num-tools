# iter-num-tools

[![Build Status](https://img.shields.io/github/actions/workflow/status/conradludgate/iter-num-tools/test.yml?branch=main&style=flat-square)][actions]
[![Rust Documentation](https://img.shields.io/crates/v/iter-num-tools?color=blue&label=docs&style=flat-square)][docs.rs]
[![Latest Version](https://img.shields.io/crates/d/iter-num-tools?style=flat-square)][crates.io]
[![Code Coverage](https://img.shields.io/codecov/c/gh/conradludgate/iter-num-tools?style=flat-square)][codecov]

[actions]: https://github.com/conradludgate/iter-num-tools/actions?query=branch%3Amain
[crates.io]: https://crates.io/crates/iter_num_tools
[docs.rs]: https://docs.rs/iter_num_tools
[codecov]: https://codecov.io/gh/conradludgate/iter-num-tools

This is a collection of iterator extensions that make heavy use of number properties. Mostly extending on Range.

## LinSpace

LinSpace is an iterator over a range with a fixed number of values all evenly spaced.

```rust
use iter_num_tools::lin_space;

// Count from 1.0 up to and including 5.0, with 5 numbers counted in total
let it = lin_space(1.0..=5.0, 5);
assert!(it.eq([1.0, 2.0, 3.0, 4.0, 5.0]));

// Count from 0.0 up to and excluding 5.0, with 5 numbers counted in total
let it = lin_space(0.0..5.0, 5);
assert!(it.eq([0.0, 1.0, 2.0, 3.0, 4.0]));
```

## GridSpace

GridSpace extends on [LinSpace](#linspace).

```rust
use iter_num_tools::grid_space;

// count in 2 dimensions (excluding end points),
// from 0.0 up to 1.0 in the x direction with 2 even steps,
// and 0.0 up to 2.0 in the y direction with 4 even steps
let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
assert!(it.eq([
    [0.0, 0.0], [0.5, 0.0],
    [0.0, 0.5], [0.5, 0.5],
    [0.0, 1.0], [0.5, 1.0],
    [0.0, 1.5], [0.5, 1.5],
]));

// count in 2 dimensions (including end points),
// from 0.0 up to 1.0 in the x direction,
// and 0.0 up to 2.0 in the y direction with 3 even steps in all directions
let it = grid_space([0.0, 0.0]..=[1.0, 2.0], 3);
assert!(it.eq([
    [0.0, 0.0], [0.5, 0.0], [1.0, 0.0],
    [0.0, 1.0], [0.5, 1.0], [1.0, 1.0],
    [0.0, 2.0], [0.5, 2.0], [1.0, 2.0],
]));
```

## Arange

Arange is similar to [LinSpace](#linspace), but instead of a fixed amount of steps, it steps by a fixed amount.

```rust
use iter_num_tools::arange;

let it = arange(0.0..2.0, 0.5);
assert!(it.eq([0.0, 0.5, 1.0, 1.5]));
```

#### Note

There is no inclusive version of arange. Consider the following

```rust
use iter_num_tools::arange;

let it = arange(0.0..=2.1, 0.5);
```

We would not expect 2.1 to ever be a value that the iterator will ever meet, but the range suggests it should be included. Therefore, no RangeInclusive implementation is provided.

## ArangeGrid

ArangeGrid is the same as [GridSpace](#gridspace) but for [Arange](#arange) instead of [LinSpace](#linspace).

```rust
use iter_num_tools::arange_grid;

// count in 2 dimensions,
// from 0.0 up to 1.0 in the x direction,
// and 0.0 up to 2.0 in the y direction,
// stepping by 0.5 each time
let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
assert!(it.eq([
    [0.0, 0.0], [0.5, 0.0],
    [0.0, 0.5], [0.5, 0.5],
    [0.0, 1.0], [0.5, 1.0],
    [0.0, 1.5], [0.5, 1.5],
]));

// count in 2 dimensions,
// from 0.0 up to 1.0 in the x direction stepping by 0.5 each time,
// and 0.0 up to 2.0 in the y direction stepping by 1.0 each time
let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
assert!(it.eq([
    [0.0, 0.0], [0.5, 0.0],
    [0.0, 1.0], [0.5, 1.0],
]));
```

## LogSpace

LogSpace is similar to [LinSpace](#linspace), but instead of evenly spaced linear steps, it has evenly spaced logarithmic steps.

```rust
use iter_num_tools::log_space;
use itertools::zip_eq;

// From 1.0 up to and including 1000.0, taking 4 logarithmic steps
let it = log_space(1.0..=1000.0, 4);
let expected: [f64; 4] = [1.0, 10.0, 100.0, 1000.0];

assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));

// From 1.0 up to 1000.0, taking 3 logarithmic steps
let it = log_space(1.0..1000.0, 3);
let expected: [f64; 3] = [1.0, 10.0, 100.0];

assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
```

## Alternatives

[`itertools-num`](https://docs.rs/itertools-num/0.1.3/itertools_num/) also provides
inclusive `linspace` and a cumulative-sum iterator adaptor. Performance depends on
the workload and compiler; this crate also supports exclusive bounds and the
grid, step and logarithmic iterators described above.

### Benchmarks

The linspace comparison generates 200 `f64` values over the inclusive interval
`[1, 100]` for every implementation. It checks lengths and approximate values
before timing, then measures both summing and mapping `x * 2.0` into a `Vec`.
Inputs and outputs pass through `black_box`; construction and, for collection,
allocation/deallocation are included. The std baseline uses `(0..steps)` and
`(end - start) / (steps - 1)` to produce the same number of values.

Example results from 2026-10-28, Rust 1.99.1, default features, Linux x86_64,
AMD Ryzen 9 7950X, pinned to one logical CPU. Criterion 0.5.1, 50 samples,
1-second warm-up and 3-second measurement per case; point estimates per iteration:

| Implementation | Sum | Map and collect |
| --- | ---: | ---: |
| iter-num-tools | 132.43 ns | 141.18 ns |
| std range and map | 116.50 ns | 84.46 ns |
| itertools-num 0.1.3 | 120.91 ns | 139.61 ns |

The earlier README timings used a different workload and an unequal-count std
baseline, so they cannot establish a performance trend. These results do not
show a general speed advantage for iter-num-tools. Benchmark your own workload;
CPU, compiler and optional nightly features can change the result.

On nightly-2026-10-24, enabling the existing `trusted_len` feature reduced this
crate's map-and-collect estimate from 138.12 ns to 84.65 ns; the std baseline was
85.39 ns and 84.37 ns respectively. That closes the collection gap in this
workload, rather than establishing a general speed advantage.

Run the comparison on stable Rust with:

```sh
cargo bench --bench linspace -- --noplot --sample-size 50 --warm-up-time 1 --measurement-time 3
```

The `arange`, `gridspace` and `logspace` benchmark targets cover other operations.
The grid case visits 200 × 200 points. The logspace alternative uses this crate's
`lin_space` followed by `exp2`, rather than an independent std implementation;
floating-point results need not be bit-identical.
