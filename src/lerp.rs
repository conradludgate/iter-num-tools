//! Lerp implements a linear interpolation function.
//!
//! Used by [LinSpace](crate::lin_space).

use crate::map::Function;
use core::ops::{Add, Div, Mul, RangeInclusive, Sub};
use num_traits::FromPrimitive;

/// Lerp represents the range over the linear interpolation
#[derive(Copy, Clone)]
pub struct LinSpaceFn<T> {
    start: T,
    len: T,
    steps: T,
}

impl<T> Function<usize> for LinSpaceFn<T>
where
    T: FromPrimitive + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    type Output = T;
    #[inline]
    fn call(&self, x: usize) -> Self::Output {
        let LinSpaceFn { start, len, steps } = *self;
        start + T::from_usize(x).unwrap() / steps * len
    }
}

impl<T> LinSpaceFn<T>
where
    T: FromPrimitive + Sub<Output = T> + Copy,
{
    /// Create a new linear interpolator over the provided ranges
    #[inline]
    pub fn new(range: RangeInclusive<T>, steps: usize) -> Self {
        let (start, end) = range.into_inner();
        let steps = T::from_usize(steps).unwrap();
        LinSpaceFn {
            start,
            len: end - start,
            steps,
        }
    }
}

