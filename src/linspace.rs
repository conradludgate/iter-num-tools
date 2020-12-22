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

use itertools::{Itertools, Product};
/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
/// use itertools::Itertools;
///
/// let it = grid_space((0.0, 0.0)..(1.0, 2.0), (2, 4));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5),
/// ]);
///
/// // inclusive
/// let it = grid_space((0.0, 0.0)..=(1.0, 2.0), (3, 5));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5), (0.0, 2.0),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5), (0.5, 2.0),
///     (1.0, 0.0), (1.0, 0.5), (1.0, 1.0), (1.0, 1.5), (1.0, 2.0),
/// ]);
///
/// // even 3d spaces
/// let it = grid_space((0, 0, 0)..=(1, 1, 1), (2, 2, 2));
/// itertools::assert_equal(it, vec![
///     ((0, 0), 0), ((0, 0), 1),
///     ((0, 1), 0), ((0, 1), 1),
///
///     ((1, 0), 0), ((1, 0), 1),
///     ((1, 1), 0), ((1, 1), 1),
/// ]);
/// ```
pub fn grid_space<R, I, S, T>(
    range: R,
    size: S,
) -> I
where
    R: IntoGridSpace<I, S, T>,
{
    range.into_grid_space(size)
}

pub trait IntoGridSpace<I, S, T> {
    fn into_grid_space(self, size: S) -> I;
}

impl<T>
    IntoGridSpace<
        Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,
        (usize, usize),
        (T, T),
    > for RangeInclusive<(T, T)>
where
T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    fn into_grid_space(self, (w, h): (usize, usize)) -> Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>> {
        let ((w0, h0), (w1, h1)) = self.into_inner();

        let wl = lin_space(w0..=w1, w);
        let hl = lin_space(h0..=h1, h);
        wl.cartesian_product(hl)
    }
}

impl<T>
    IntoGridSpace<
        Product<Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,LerpIterPrim<usize, T, Range<usize>>>,
        (usize, usize, usize),
        (T, T, T),
    > for RangeInclusive<(T, T, T)>
where
T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    fn into_grid_space(self, (w, h, d): (usize, usize, usize)) -> Product<Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,LerpIterPrim<usize, T, Range<usize>>> {
        let ((w0, h0, d0), (w1, h1, d1)) = self.into_inner();

        let wl = lin_space(w0..=w1, w);
        let hl = lin_space(h0..=h1, h);
        let dl = lin_space(d0..=d1, d);
        wl.cartesian_product(hl).cartesian_product(dl)
    }
}

impl<T>
    IntoGridSpace<
        Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,
        (usize, usize),
        (T, T),
    > for Range<(T, T)>
where
T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    fn into_grid_space(self, (w, h): (usize, usize)) -> Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>> {
        let Range {
            start: (w0, h0),
            end: (w1, h1),
        } = self;

        let wl = lin_space_ex(w0..w1, w);
        let hl = lin_space_ex(h0..h1, h);
        wl.cartesian_product(hl)
    }
}

impl<T>
    IntoGridSpace<
        Product<Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,LerpIterPrim<usize, T, Range<usize>>>,
        (usize, usize, usize),
        (T, T, T),
    > for Range<(T, T, T)>
where
T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    fn into_grid_space(self, (w, h, d): (usize, usize, usize)) -> Product<Product<LerpIterPrim<usize, T, Range<usize>>, LerpIterPrim<usize, T, Range<usize>>>,LerpIterPrim<usize, T, Range<usize>>> {
        let Range {
            start: (w0, h0, d0),
            end: (w1, h1, d1),
        } = self;

        let wl = lin_space_ex(w0..w1, w);
        let hl = lin_space_ex(h0..h1, h);
        let dl = lin_space_ex(d0..d1, d);
        wl.cartesian_product(hl).cartesian_product(dl)
    }
}
