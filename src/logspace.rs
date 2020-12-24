use num_traits::{real::Real};
use std::ops::{Range, RangeInclusive};

use crate::{LinSpace, Linear, lin_space};

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

pub trait IntoLogSpace<T> {
    fn into_log_space(self, steps: usize) -> LogSpace<T>;
}

impl<T> IntoLogSpace<T> for RangeInclusive<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let (a, b) = self.into_inner();
        LogSpace::new(lin_space(a.ln()..=b.ln(), steps))
    }
}

impl<T> IntoLogSpace<T> for Range<T>
where
    T: Linear + Real,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        let Range { start: a, end: b } = self;
        LogSpace::new(lin_space(a.ln()..b.ln(), steps))
    }
}

pub struct LogSpace<T>(LinSpace<T>);

impl<T> LogSpace<T> {
    fn new(linspace: LinSpace<T>) -> Self {
        LogSpace(linspace)
    }
}

impl<T> Iterator for LogSpace<T>
where
    LinSpace<T>: Iterator<Item = T>,
    T: Real,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(<T as Real>::exp)
    }
}
