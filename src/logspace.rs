use core::ops::{Range, RangeInclusive};
use num_traits::{real::Real, FromPrimitive};

use crate::space::{Interpolate, IntoSpace, Space};

/// Creates a logarithmic space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::log_space;
/// use itertools::zip_eq;
///
/// // Inclusive
/// let it = log_space(1.0..=1000.0, 4);
/// let expected: Vec<f64> = vec![1.0, 10.0, 100.0, 1000.0];
///
/// // all approx equal
/// assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
///
/// // Exclusive
/// let it = log_space(1.0..1000.0, 3);
/// let expected: Vec<f64> = vec![1.0, 10.0, 100.0];
///
/// // all approx equal
/// assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
/// ```
pub fn log_space<R>(range: R, steps: usize) -> LogSpace<R::Item>
where
    R: ToLogSpace,
{
    range.into_log_space(steps).into_space()
}

#[derive(Clone, Copy, Debug)]
pub struct LogarithmicInterpolation<T> {
    pub start: T,
    pub step: T,
}

/// A helper trait for [`log_space`]
pub trait ToLogSpace {
    /// The item that this is a logarithmic space over
    type Item;
    /// Create the log space
    fn into_log_space(self, step: usize) -> IntoLogSpace<Self::Item>;
}

impl<T: Real> Interpolate for LogarithmicInterpolation<T> {
    type Item = T;
    fn interpolate(self, x: usize) -> T {
        let Self { start, step } = self;
        start * step.powi(x as i32)
    }
}

impl<T: Real + FromPrimitive> ToLogSpace for Range<T> {
    type Item = T;

    fn into_log_space(self, steps: usize) -> IntoLogSpace<Self::Item> {
        let Range { start, end } = self;
        let step = (end / start).powf(T::from_usize(steps).unwrap().recip());
        IntoLogSpace::new(steps, LogarithmicInterpolation { start, step })
    }
}

impl<T: Real + FromPrimitive> ToLogSpace for RangeInclusive<T> {
    type Item = T;

    fn into_log_space(self, steps: usize) -> IntoLogSpace<Self::Item> {
        let (start, end) = self.into_inner();
        let step = (end / start).powf(T::from_usize(steps - 1).unwrap().recip());
        IntoLogSpace::new(steps, LogarithmicInterpolation { start, step })
    }
}

/// [`Iterator`] returned by [`log_space`]
pub type LogSpace<T> = Space<LogarithmicInterpolation<T>>;
/// [`IntoIterator`] returned by [`ToLogSpace::into_log_space`]
pub type IntoLogSpace<T> = IntoSpace<LogarithmicInterpolation<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::zip_eq;

    #[test]
    fn test_log_space_inclusive() {
        let it = log_space(1.0..=1000.0, 4);
        assert!(zip_eq(it, vec![1.0, 10.0, 100.0, 1000.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive() {
        let it = log_space(1.0..1000.0, 3);
        assert!(zip_eq(it, vec![1.0, 10.0, 100.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_inclusive_rev() {
        let it = log_space(1.0..=1000.0, 4);
        assert!(zip_eq(it.rev(), vec![1000.0, 100.0, 10.0, 1.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive_rev() {
        let it = log_space(1.0..1000.0, 3);
        assert!(zip_eq(it.rev(), vec![100.0, 10.0, 1.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive_len() {
        let mut it = log_space(1.0..=1000.0, 4);
        let mut expected_len = 4;

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
