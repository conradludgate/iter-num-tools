use crate::space::{Interpolate, IntoSpace, Space};
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
pub fn lin_space<R>(range: R, steps: usize) -> LinSpace<R::Item>
where
    R: ToLinSpace,
{
    range.into_lin_space(steps).into_space()
}

#[derive(Clone, Copy, Debug)]
pub struct LinearInterpolation<T> {
    pub start: T,
    pub step: T,
}

/// A helper trait for [`lin_space`]
pub trait ToLinSpace {
    /// The item that this is a linear space over
    type Item;
    /// Create the lin space
    fn into_lin_space(self, step: usize) -> IntoLinSpace<Self::Item>;
}

impl<T: Num + FromPrimitive + Copy> ToLinSpace for Range<T> {
    type Item = T;

    fn into_lin_space(self, steps: usize) -> IntoLinSpace<Self::Item> {
        let Range { start, end } = self;
        let step = (end - start) / T::from_usize(steps).unwrap();
        IntoLinSpace::new(steps, LinearInterpolation { start, step })
    }
}

impl<T: Num + FromPrimitive + Copy> ToLinSpace for RangeInclusive<T> {
    type Item = T;

    fn into_lin_space(self, steps: usize) -> IntoLinSpace<Self::Item> {
        let (start, end) = self.into_inner();
        let step = (end - start) / T::from_usize(steps - 1).unwrap();
        IntoLinSpace::new(steps, LinearInterpolation { start, step })
    }
}

impl<T: Num + FromPrimitive> Interpolate for LinearInterpolation<T> {
    type Item = T;
    fn interpolate(self, x: usize) -> T {
        let Self { start, step } = self;
        start + T::from_usize(x).unwrap() * step
    }
}

/// [`Iterator`] returned by [`lin_space`]
pub type LinSpace<T> = Space<LinearInterpolation<T>>;

/// [`IntoIterator`] returned by [`ToLinSpace::into_lin_space`]
pub type IntoLinSpace<T> = IntoSpace<LinearInterpolation<T>>;

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
