use crate::{lerp::LerpPrim, map::Map};
use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

/// Iterator over a linear number space
pub type LinSpace<T> = Map<Range<usize>, LerpPrim<T>>;

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
        LinSpace::new(0..steps, LerpPrim::new_usize(0..=steps - 1, self))
    }
}

impl<T> IntoLinSpace<T> for Range<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        let Range { start, end } = self;
        LinSpace::new(0..steps, LerpPrim::new_usize(0..=steps, start..=end))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lin_space_inclusive() {
        let it = lin_space(1.0..=5.0, 5);
        assert!(it.eq(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
    }

    #[test]
    fn test_lin_space_exclusive() {
        let it = lin_space(0.0..5.0, 5);
        assert!(it.eq(vec![0.0, 1.0, 2.0, 3.0, 4.0]));
    }
}
