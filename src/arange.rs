use crate::linspace::{LinSpace, Linear, LinearInterpolation};
use core::ops::Range;
use num_traits::real::Real;

/// Iterator returned by [`arange`]
pub type Arange<T> = LinSpace<T>;

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
///
/// let it = arange(0.0..2.0, 0.5);
/// assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
/// ```
pub fn arange<R, F>(range: R, step: F) -> Arange<F>
where
    F: Real + Linear,
    R: IntoArange<F>,
{
    range.into_arange(step)
}

/// Used by [`arange`]
pub trait IntoArange<F: Real + Linear> {
    /// Convert self into an [`Arange`]
    fn into_arange(self, step: F) -> Arange<F>;
}

impl<F: Real + Linear> IntoArange<F> for Range<F> {
    fn into_arange(self, step: F) -> Arange<F> {
        let (interpolate, steps) = arange_lerp(self, step);
        LinSpace::new(steps, interpolate)
    }
}

pub fn arange_lerp<F: Real + Linear>(range: Range<F>, step: F) -> (LinearInterpolation<F>, usize) {
    let Range { start, end } = range;

    (
        LinearInterpolation { start, step },
        ((end - start) / step).ceil().to_usize().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let it = arange(0.0..2.0, 0.5);
        assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
    }
}
