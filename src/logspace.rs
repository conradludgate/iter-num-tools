use core::{
    marker::PhantomData,
    ops::{Range, RangeInclusive},
};
use num_traits::real::Real;

use crate::linspace::{lin_space, LinSpace, Linear};

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

/// Used by [`log_space`]
pub trait IntoLogSpace<T> {
    /// Convert self into a [`LogSpace`]
    fn into_log_space(self, steps: usize) -> LogSpace<T>;
}

impl<T> IntoLogSpace<T> for RangeInclusive<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let (a, b) = self.into_inner();
        lin_space(a.log2()..=b.log2(), steps).map(Exp2Impl::FUN)
    }
}

impl<T> IntoLogSpace<T> for Range<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let Range { start: a, end: b } = self;
        lin_space(a.log2()..b.log2(), steps).map(Exp2Impl::FUN)
    }
}

type Exp2<T> = impl Fn(T) -> T;
struct Exp2Impl<T>(PhantomData<T>);
impl<T: Real> Exp2Impl<T> {
    const FUN: Exp2<T> = {
        fn exp2<T: Real>(x: T) -> T {
            x.exp2()
        }
        exp2::<T>
    };
}

/// Iterator returned by [`log_space`]
pub type LogSpace<T> = core::iter::Map<LinSpace<T>, Exp2<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::zip_eq;

    #[test]
    fn test_log_space_inclusive() {
        let it = log_space(1.0..=1000.0, 4);
        assert!(zip_eq(it, [1.0, 10.0, 100.0, 1000.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive() {
        let it = log_space(1.0..1000.0, 3);
        assert!(zip_eq(it, [1.0, 10.0, 100.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }
}
