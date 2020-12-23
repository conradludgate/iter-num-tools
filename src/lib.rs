mod arange;
pub mod lerp;
mod linspace;
pub mod combine;

use std::ops::Range;

pub use arange::*;
pub use linspace::*;

use itertools::Itertools;

/// Creates a grid of 2-tuples over the range of 2-tuples
///
/// ```
/// use iter_num_tools::grid;
/// use itertools::Itertools;
///
/// let it = grid((0, 0)..(2, 3));
/// itertools::assert_equal(it, vec![
///     (0, 0), (0, 1), (0, 2),
///     (1, 0), (1, 1), (1, 2),
/// ]);
/// ```
pub fn grid<T>(range: Range<(T, T)>) -> impl Iterator<Item = (T, T)>
where
    T: Clone,
    Range<T>: Itertools + Iterator<Item = T>,
{
    let Range { start, end } = range;
    let (w0, h0) = start;
    let (w1, h1) = end;
    let wr = w0..w1;
    let hr = h0..h1;
    wr.cartesian_product(hr)
}
