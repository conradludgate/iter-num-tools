use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

/// Wraps the given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// ```
/// use iter_num_tools::lerp_iter;
/// use itertools::Itertools;
///
/// let it = lerp_iter(0.0..=2.0, 20.0..=21.0, vec![-1.0, 1.0]);
/// itertools::assert_equal(it, vec![19.5, 20.5]);
/// ```
pub fn lerp_iter<T, I>(
    from: RangeInclusive<T>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = T, IntoIter = I>,
) -> LerpIter<T, I>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    LerpIter::new(from, to, over)
}

/// Wraps the given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// Similar to [lerp_iter] but handles the conversion from usize
///
/// ```
/// use iter_num_tools::lerp_usize_iter;
/// use itertools::Itertools;
///
/// let it = lerp_usize_iter(0..=2, 20.0..=21.0, 0..5);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0, 21.5, 22.0]);
/// ```
pub fn lerp_usize_iter<T, I>(
    from: RangeInclusive<usize>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = usize, IntoIter = I>,
) -> LerpIterUsize<T, I>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    LerpIterUsize::new(from, to, over)
}

#[derive(Clone)]
pub struct LerpIter<T, I> {
    from: RangeInclusive<T>,
    to: RangeInclusive<T>,
    over: I,
}

impl<T, I> LerpIter<T, I> {
    pub fn new(
        from: RangeInclusive<T>,
        to: RangeInclusive<T>,
        over: impl IntoIterator<Item = T, IntoIter = I>,
    ) -> Self {
        LerpIter {
            from,
            to,
            over: over.into_iter(),
        }
    }
}

impl<T, I> Iterator for LerpIter<T, I>
where
    I: Iterator<Item = T>,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.over.next()?;

        let x0 = *self.from.start();
        let x1 = *self.from.end();
        let y0 = *self.to.start();
        let y1 = *self.to.end();

        Some(((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0))
    }
}

#[derive(Clone)]
pub struct LerpIterUsize<T, I> {
    x0: T,
    x1: T,
    y0: T,
    y1: T,
    over: I,
}

impl<T, I> LerpIterUsize<T, I>
where
    T: FromPrimitive + Copy,
{
    pub fn new(
        from: RangeInclusive<usize>,
        to: RangeInclusive<T>,
        over: impl IntoIterator<Item = usize, IntoIter = I>,
    ) -> Self {
        LerpIterUsize {
            x0: T::from_usize(*from.start()).unwrap(),
            x1: T::from_usize(*from.end()).unwrap(),
            y0: *to.start(),
            y1: *to.end(),
            over: over.into_iter(),
        }
    }
}

impl<T, I> Iterator for LerpIterUsize<T, I>
where
    I: Iterator<Item = usize>,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = T::from_usize(self.over.next()?).unwrap();

        let x0 = self.x0;
        let x1 = self.x1;
        let y0 = self.y0;
        let y1 = self.y1;

        Some(((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0))
    }
}

/// Creates a linear space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::lin_space;
/// use itertools::Itertools;
///
/// let it = lin_space(20.0..=21.0, 3);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0]);
/// ```
pub fn lin_space<T>(range: RangeInclusive<T>, steps: usize) -> LerpIterUsize<T, Range<usize>>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    // lerp_usize_iter(0..=steps - 1, range, 0..steps)
    LerpIterUsize::new(0..=steps - 1, range, 0..steps)
}

use itertools::Itertools;
/// Creates a linear grid space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::grid_space;
/// use itertools::Itertools;
///
/// let it = grid_space((0.0, 0.0)..=(10.0, 20.0), (3, 5));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 5.0), (0.0, 10.0), (0.0, 15.0), (0.0, 20.0),
///     (5.0, 0.0), (5.0, 5.0), (5.0, 10.0), (5.0, 15.0), (5.0, 20.0),
///     (10.0, 0.0), (10.0, 5.0), (10.0, 10.0), (10.0, 15.0), (10.0, 20.0),
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
    let (w0, h0) = *range.start();
    let (w1, h1) = *range.end();

    let wl = lin_space(w0..=w1, w);
    let hl = lin_space(h0..=h1, h);
    wl.cartesian_product(hl)
}
