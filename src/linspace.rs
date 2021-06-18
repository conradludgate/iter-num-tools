use crate::space::{Interpolate, Space};
use core::ops::{Range, RangeInclusive};
use num_traits::{FromPrimitive, Num};

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
    (R, usize): Into<LinearInterpolation<T>>,
{
    LinSpace::new(steps, (range, steps).into())
}

#[derive(Clone, Copy, Debug)]
pub struct LinearInterpolation<T> {
    pub start: T,
    pub step: T,
}

impl<T: Num + FromPrimitive> Interpolate for LinearInterpolation<T> {
    type Item = T;
    fn interpolate(self, x: usize) -> T {
        let Self { start, step } = self;
        start + T::from_usize(x).unwrap() * step
    }
}

impl<T: Num + FromPrimitive + Copy> From<(Range<T>, usize)> for LinearInterpolation<T> {
    fn from((range, steps): (Range<T>, usize)) -> Self {
        let Range { start, end } = range;
        let step = (end - start) / T::from_usize(steps).unwrap();
        Self { start, step }
    }
}

impl<T: Num + FromPrimitive + Copy> From<(RangeInclusive<T>, usize)> for LinearInterpolation<T> {
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

    #[test]
    fn test_lin_space_extras() {
        // count
        assert_eq!(lin_space(0.0..=5.0, 6).count(), 6);

        // nth
        let mut it = lin_space(0.0..=5.0, 6);
        assert_eq!(it.nth(2), Some(2.0));
        assert_eq!(it.nth_back(2), Some(3.0));

        assert_eq!(lin_space(0.0..=5.0, 6).last(), Some(5.0));
    }

    #[test]
    #[cfg(feature = "iter_advance_by")]
    fn test_lin_space_advance_by() {
        let mut it = lin_space(0.0..=5.0, 6);
        it.advance_by(2).unwrap();
        assert_eq!(it.next(), Some(2.0));

        it.advance_back_by(2).unwrap();
        assert_eq!(it.next_back(), Some(3.0));
    }
}
