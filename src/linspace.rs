use crate::lerp::LerpIterPrim;
use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

/// Creates a linear space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::lin_space;
/// use itertools::Itertools;
///
/// let it = lin_space(20.0..=21.0, 3);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0]);
/// ```
pub fn lin_space<T>(range: RangeInclusive<T>, steps: usize) -> LerpIterPrim<usize, T, Range<usize>>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    LerpIterPrim::<usize, T, Range<usize>>::new(0..=steps - 1, range, 0..steps)
}

/// Creates a linear space over range with a fixed number of steps, excluding the end value
///
/// Similar to [lin_space]
///
/// ```
/// use iter_num_tools::lin_space_ex;
/// use itertools::Itertools;
///
/// let it = lin_space_ex(20.0..21.0, 2);
/// itertools::assert_equal(it, vec![20.0, 20.5]);
/// ```
pub fn lin_space_ex<T>(range: Range<T>, steps: usize) -> LerpIterPrim<usize, T, Range<usize>>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    let Range { start, end } = range;
    LerpIterPrim::<usize, T, Range<usize>>::new(0..=steps, start..=end, 0..steps)
}

use itertools::Itertools;
/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
/// use itertools::Itertools;
///
/// let it = grid_space((0.0, 0.0)..=(1.0, 2.0), (3, 5));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5), (0.0, 2.0),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5), (0.5, 2.0),
///     (1.0, 0.0), (1.0, 0.5), (1.0, 1.0), (1.0, 1.5), (1.0, 2.0),
/// ]);
/// ```
pub fn grid_space<T>(
    range: RangeInclusive<(T, T)>,
    (w, h): (usize, usize),
) -> impl Iterator<Item = (T, T)>
where
    T: FromPrimitive
        + Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Copy
        + Clone,
{
    let ((w0, h0), (w1, h1)) = range.into_inner();

    let wl = lin_space(w0..=w1, w);
    let hl = lin_space(h0..=h1, h);
    wl.cartesian_product(hl)
}

/// Creates a linear grid space over range with a fixed number of width and height steps, excluding the end values
///
/// Similar to [grid_space]
///
/// ```
/// use iter_num_tools::grid_space_ex;
/// use itertools::Itertools;
///
/// let it = grid_space_ex((0.0, 0.0)..(1.0, 2.0), (2, 4));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5),
/// ]);
/// ```
pub fn grid_space_ex<T>(
    range: Range<(T, T)>,
    (w, h): (usize, usize),
) -> impl Iterator<Item = (T, T)>
where
    T: FromPrimitive
        + Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Copy
        + Clone,
{
    let Range {
        start: (w0, h0),
        end: (w1, h1),
    } = range;

    let wl = lin_space_ex(w0..w1, w);
    let hl = lin_space_ex(h0..h1, h);
    wl.cartesian_product(hl)
}
