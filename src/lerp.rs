
use num_traits::{FromPrimitive};
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

/// Creates a function which calculates a linear interpolation from the first range to the second
/// Input to the produced function can be outside of the first range
///
/// ```
/// use iter_num_tools::lerp_fn;
/// let f = lerp_fn(0.0..=2.0, 20.0..=21.0);
/// assert_eq!(f(1.0), 20.5);
/// assert_eq!(f(-1.0), 19.5);
/// ```
pub fn lerp_fn<T>(from: RangeInclusive<T>, to: RangeInclusive<T>) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    let x0 = *from.start();
    let x1 = *from.end();
    let y0 = *to.start();
    let y1 = *to.end();

    move |x: T| ((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0)
}

/// Creates a function which calculates a linear interpolation from the first range to the second
/// Input to the produced function can be outside of the first range.
///
/// This is a convenient wrapper of [lerp_fn] for usize inputs
///
/// **Panics** if either end of the first range cannot be cast to T
///
/// ```
/// use iter_num_tools::lerp_usize_fn;
/// let f = lerp_usize_fn(0..=2, 20.0..=21.0);
/// assert_eq!(f(1), 20.5);
/// assert_eq!(f(3), 21.5);
/// ```
pub fn lerp_usize_fn<T>(from: RangeInclusive<usize>, to: RangeInclusive<T>) -> impl Fn(usize) -> T
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    let x0 = T::from_usize(*from.start()).unwrap();
    let x1 = T::from_usize(*from.end()).unwrap();
    let f = lerp_fn(x0..=x1, to);

    move |x: usize| f(T::from_usize(x).unwrap())
}

/// A convenient wrapper of `over.into_iter().map(lerp_usize_fn(from, to))`.
/// See [lerp_usize_fn]
///
/// ```
/// use iter_num_tools::lerp_usize_iter;
/// use itertools::Itertools;
///
/// let it = lerp_usize_iter(0..=2, 20.0..=21.0, 0..5);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0, 21.5, 22.0]);
/// ```
pub fn lerp_usize_iter<T>(
    from: RangeInclusive<usize>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = usize>,
) -> impl Iterator<Item = T>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    over.into_iter().map(lerp_usize_fn(from, to))
}

/// A convenient wrapper of `over.into_iter().map(lerp_fn(from, to))`.
/// See [lerp_fn]
///
/// ```
/// use iter_num_tools::lerp_iter;
/// use itertools::Itertools;
///
/// let it = lerp_iter(0.0..=2.0, 20.0..=21.0, vec![-1.0, 1.0]);
/// itertools::assert_equal(it, vec![19.5, 20.5]);
/// ```
pub fn lerp_iter<T>(
    from: RangeInclusive<T>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    over.into_iter().map(lerp_fn(from, to))
}
