use core::ops::{Range, RangeInclusive};
use num_traits::{real::Real, FromPrimitive};

use crate::space::{Interpolate, Space};

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
pub fn log_space<R, T>(range: R, steps: usize) -> LogSpace<T>
where
    (R, usize): Into<LogarithmicInterpolation<T>>,
{
    LogSpace::new(steps, (range, steps).into())
}

#[derive(Clone, Copy, Debug)]
pub struct LogarithmicInterpolation<T> {
    pub start: T,
    pub step: T,
}

impl<T: Real> Interpolate for LogarithmicInterpolation<T> {
    type Item = T;
    fn interpolate(self, x: usize) -> T {
        let Self { start, step } = self;
        start * step.powi(x as i32)
    }
}

impl<T: Real + FromPrimitive> From<(Range<T>, usize)> for LogarithmicInterpolation<T> {
    fn from((range, steps): (Range<T>, usize)) -> Self {
        let Range { start, end } = range;
        let step = (end / start).powf(T::from_usize(steps).unwrap().recip());
        Self { start, step }
    }
}

impl<T: Real + FromPrimitive> From<(RangeInclusive<T>, usize)> for LogarithmicInterpolation<T> {
    fn from((range, steps): (RangeInclusive<T>, usize)) -> Self {
        let (start, end) = range.into_inner();
        let step = (end / start).powf(T::from_usize(steps - 1).unwrap().recip());
        Self { start, step }
    }
}

/// Iterator returned by [`log_space`]
pub type LogSpace<T> = Space<LogarithmicInterpolation<T>>;

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
