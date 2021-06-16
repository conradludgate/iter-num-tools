use crate::space::{Interpolate, Space};
use core::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};
use num_traits::FromPrimitive;

/// Creates a linear space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::lin_space;
///
/// // Inclusive
/// let it = lin_space(20.0..=21.0, 3);
/// assert!(it.eq(vec![20.0, 20.5, 21.0]));
///
/// // Exclusive
/// let it = lin_space(20.0..21.0, 2);
/// assert!(it.eq(vec![20.0, 20.5]));
/// ```
#[inline]
pub fn lin_space<R, T>(range: R, steps: usize) -> LinSpace<T>
where
    R: IntoLinSpace<T>,
{
    range.into_lin_space(steps)
}

/// Used by [`lin_space`]
pub trait IntoLinSpace<T> {
    /// Convert self into a [`LinSpace`]
    fn into_lin_space(self, steps: usize) -> LinSpace<T>;
}

impl<T, R> IntoLinSpace<T> for R
where
    T: Linear,
    (R, usize): Into<LinearInterpolation<T>>,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        LinSpace::new(steps, (self, steps).into())
    }
}

/// Trait required for [`lin_space`] implementations.
pub trait Linear:
    FromPrimitive
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Add<Output = Self>
    + Div<Output = Self>
    + Copy
{
}
impl<T> Linear for T where
    T: FromPrimitive
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Add<Output = Self>
        + Div<Output = Self>
        + Copy
{
}

#[derive(Clone, Copy, Debug)]
pub struct LinearInterpolation<T> {
    pub start: T,
    pub step: T,
}

impl<T: Linear> Interpolate for LinearInterpolation<T> {
    type Item = T;
    fn interpolate(&self, x: usize) -> T {
        let Self { start, step } = *self;
        start + T::from_usize(x).unwrap() * step
    }
}

impl<T: Linear> From<(Range<T>, usize)> for LinearInterpolation<T> {
    fn from((range, steps): (Range<T>, usize)) -> Self {
        let Range { start, end } = range;
        let step = (end - start) / T::from_usize(steps).unwrap();
        Self { start, step }
    }
}

impl<T: Linear> From<(RangeInclusive<T>, usize)> for LinearInterpolation<T> {
    fn from((range, steps): (RangeInclusive<T>, usize)) -> Self {
        let (start, end) = range.into_inner();
        let step = (end - start) / T::from_usize(steps - 1).unwrap();
        Self { start, step }
    }
}

/// Iterator returned by [`lin_space`]
pub type LinSpace<T> = Space<LinearInterpolation<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lin_space_inclusive() {
        let it = lin_space(1.0..=5.0, 5);
        assert!(it.eq(vec![1.0, 2.0, 3.0, 4.0, 5.0]))
    }

    #[test]
    fn test_lin_space_exclusive() {
        let it = lin_space(0.0..5.0, 5);
        assert!(it.eq(vec![0.0, 1.0, 2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_lin_space_exclusive_rev() {
        let it = lin_space(0.0..5.0, 5).rev();
        assert!(it.eq(vec![4.0, 3.0, 2.0, 1.0, 0.0]));
    }

    #[test]
    fn test_lin_space_exclusive_len() {
        let mut it = lin_space(0.0..=5.0, 6);
        let mut expected_len = 6;

        assert_eq!(it.size_hint(), (expected_len, Some(expected_len)));

        while expected_len > 0 {
            assert_eq!(it.len(), expected_len);
            it.next();
            expected_len -= 1;

            assert_eq!(it.len(), expected_len);
            it.next_back();
            expected_len -= 1;
        }

        assert_eq!(it.len(), expected_len);
    }
}
