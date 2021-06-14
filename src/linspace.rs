// use crate::{lerp::LinSpaceFn, map::Map};
use core::iter::{FusedIterator, InPlaceIterable, TrustedLen};
use core::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};
use num_traits::FromPrimitive;

/// Creates a linear space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::lin_space;
/// use itertools::Itertools;
///
/// // Inclusive
/// let it = lin_space(20.0..=21.0, 3);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0]);
///
/// // Exclusive
/// let it = lin_space(20.0..21.0, 2);
/// itertools::assert_equal(it, vec![20.0, 20.5]);
/// ```
#[inline]
pub fn lin_space<R, T>(range: R, steps: usize) -> LinSpace<T>
where
    R: IntoLinSpace<T>,
{
    range.into_lin_space(steps)
}

/// Used by [lin_space]
pub trait IntoLinSpace<T> {
    fn into_lin_space(self, steps: usize) -> LinSpace<T>;
}

impl<T> IntoLinSpace<T> for RangeInclusive<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        let (start, end) = self.into_inner();
        let step = T::from_usize(steps - 1).unwrap();
        let len = end - start;
        LinSpace {
            start,
            step,
            len,
            x: 0,
            steps,
        }
    }
}

impl<T> IntoLinSpace<T> for Range<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        let Range { start, end } = self;
        let step = T::from_usize(steps).unwrap();
        let len = end - start;
        LinSpace {
            start,
            step,
            len,
            x: 0,
            steps,
        }
    }
}

/// Trait required for [lin_space] implementations.
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
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy
{
}

#[derive(Clone, Copy, Debug)]
pub struct LinSpace<T> {
    x: usize,
    steps: usize,
    start: T,
    len: T,
    step: T,
}

impl<T: Linear> Iterator for LinSpace<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            start,
            step,
            len,
            x,
            steps,
        } = self;
        if x < steps {
            Some(*start + T::from_usize(core::mem::replace(x, *x + 1)).unwrap() / *step * *len)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Linear> DoubleEndedIterator for LinSpace<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self {
            start,
            step,
            len,
            x,
            steps,
        } = self;
        if x < steps {
            *steps -= 1;
            Some(*start + T::from_usize(*steps).unwrap() / *step * *len)
        } else {
            None
        }
    }
}

impl<T: Linear> ExactSizeIterator for LinSpace<T> {
    #[inline]
    fn len(&self) -> usize {
        self.steps - self.x
    }
}

impl<T: Linear> FusedIterator for LinSpace<T> {}

unsafe impl<T: Linear> TrustedLen for LinSpace<T> {}

unsafe impl<T: Linear> InPlaceIterable for LinSpace<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lin_space_inclusive() {
        let it = lin_space(1.0..=5.0, 5);
        assert_eq_iter!(it, [1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_lin_space_exclusive() {
        let it = lin_space(0.0..5.0, 5);
        assert_eq_iter!(it, [0.0, 1.0, 2.0, 3.0, 4.0]);
    }
}
