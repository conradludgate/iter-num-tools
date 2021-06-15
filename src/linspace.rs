// use crate::{lerp::LinSpaceFn, map::Map};
use core::iter::FusedIterator;
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

impl<T> IntoLinSpace<T> for RangeInclusive<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        LinSpace {
            x: 0,
            steps,
            util: (self, steps).into(),
        }
    }
}

impl<T> IntoLinSpace<T> for Range<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        LinSpace {
            x: 0,
            steps,
            util: (self, steps).into(),
        }
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
pub struct Lerp<T> {
    pub start: T,
    pub step: T,
}

impl<T: Linear> Lerp<T> {
    #[inline]
    pub fn lerp(self, x: usize) -> T {
        let Self { start, step } = self;
        start + T::from_usize(x).unwrap() * step
    }
}

impl<T: Linear> From<(Range<T>, usize)> for Lerp<T> {
    fn from((range, steps): (Range<T>, usize)) -> Self {
        let Range { start, end } = range;
        let step = (end - start) / T::from_usize(steps).unwrap();
        Self { start, step }
    }
}

impl<T: Linear> From<(RangeInclusive<T>, usize)> for Lerp<T> {
    fn from((range, steps): (RangeInclusive<T>, usize)) -> Self {
        let (start, end) = range.into_inner();
        let step = (end - start) / T::from_usize(steps - 1).unwrap();
        Self { start, step }
    }
}

/// Iterator returned by [`lin_space`]
#[derive(Clone, Debug)]
pub struct LinSpace<T> {
    pub(crate) x: usize,
    pub(crate) steps: usize,
    pub(crate) util: Lerp<T>,
}

impl<T: Linear> Iterator for LinSpace<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.steps {
            let n = self.x + 1;
            Some(self.util.lerp(core::mem::replace(&mut self.x, n)))
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
        if self.x < self.steps {
            self.steps -= 1;
            Some(self.util.lerp(self.steps))
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

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<T: Linear> TrustedLen for LinSpace<T> {}

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
