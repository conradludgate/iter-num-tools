use num_traits::{real::Real};
use core::ops::{Range, RangeInclusive};

use crate::{LinSpace, Linear, lin_space, map::{Map, Function}};

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
    R: IntoLogSpace<T>,
{
    range.into_log_space(steps)
}

/// Used by [log_space]
pub trait IntoLogSpace<T> {
    fn into_log_space(self, steps: usize) -> LogSpace<T>;
}

impl<T> IntoLogSpace<T> for RangeInclusive<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let (a, b) = self.into_inner();
        Map::new(lin_space(a.log2()..=b.log2(), steps), Exp2)
    }
}

impl<T> IntoLogSpace<T> for Range<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let Range { start: a, end: b } = self;
        Map::new(lin_space(a.log2()..b.log2(), steps), Exp2)
    }
}

/// Implements [Function](crate::map::Function) to perform [exp2](num_traits::real::Real::exp2())
pub struct Exp2;

impl<T> Function<T> for Exp2 where T: Real {
    type Output = T;

    #[inline]
    fn call(&self, x: T) -> Self::Output {
        x.exp2()
    }
}

/// Iterator over a logarithmic number space
pub type LogSpace<T> = Map<LinSpace<T>, Exp2>;

#[cfg(test)]
mod tests {
    use super::*;

    use approx::*;

    #[test]
    fn test_log_space_inclusive() {
        let it = log_space(1.0..=1000.0, 4);
        assert_relative_eq_iter!(it, [1.0, 10.0, 100.0, 1000.0]);
    }

    #[test]
    fn test_log_space_exclusive() {
        let it = log_space(1.0..1000.0, 3);
        assert_relative_eq_iter!(it, [1.0, 10.0, 100.0]);
    }
}
