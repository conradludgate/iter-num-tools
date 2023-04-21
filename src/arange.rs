use crate::linspace::{IntoLinSpace, LinSpace, LinearInterpolation};
use core::ops::Range;
use num_traits::real::Real;

/// [`Iterator`] returned by [`arange`]
pub type Arange<T> = LinSpace<T>;

/// [`IntoIterator`] returned by [`ToArange::into_arange`]
pub type IntoArange<T> = IntoLinSpace<T>;

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
///
/// let it = arange(0.0..2.0, 0.5);
/// assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
/// ```
pub fn arange<R, F>(range: R, step: F) -> Arange<R::Item>
where
    R: ToArange<F>,
{
    range.into_arange(step).into_space()
}

/// Helper trait for [`arange`]
pub trait ToArange<S> {
    /// The item that this is a arange space over
    type Item;
    /// Create the arange space
    fn into_arange(self, step: S) -> IntoArange<Self::Item>;
}

impl<F: Real> ToArange<F> for Range<F> {
    type Item = F;

    fn into_arange(self, step: F) -> IntoArange<Self::Item> {
        let Range { start, end } = self;

        IntoArange::new(
            ((end - start) / step).ceil().to_usize().unwrap(),
            LinearInterpolation { start, step },
        )
    }
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
