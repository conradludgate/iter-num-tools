use crate::lerp::LerpIterPrim;
use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

pub type LinSpace<T> = LerpIterPrim<T, Range<usize>, usize>;

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

pub trait IntoLinSpace<T> {
    fn into_lin_space(self, steps: usize) -> LinSpace<T>;
}

impl<T> IntoLinSpace<T> for RangeInclusive<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        LinSpace::new(0..=steps - 1, self, 0..steps)
    }
}

impl<T> IntoLinSpace<T> for Range<T>
where
    T: Linear,
{
    fn into_lin_space(self, steps: usize) -> LinSpace<T> {
        let Range { start, end } = self;
        LinSpace::new(0..=steps, start..=end, 0..steps)
    }
}

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
