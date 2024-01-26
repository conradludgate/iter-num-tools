use crate::linspace::{IntoLinSpace, LinSpace, LinearInterpolation};
use core::ops::Range;
use num_traits::real::Real;

/// [`Iterator`] returned by [`arange`]
pub type Arange<T, R> = LinSpace<T, R>;

/// [`IntoIterator`] returned by [`ToArange::into_arange`]
pub type IntoArange<T, R> = IntoLinSpace<T, R>;

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
///
/// let it = arange(0.0..2.0, 0.5);
/// assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
/// ```
pub fn arange<R, F>(range: R, step: F) -> Arange<R::Item, <R::Range as IntoIterator>::IntoIter>
where
    R: ToArange<F>,
{
    range.into_arange(step).into_space()
}

/// Helper trait for [`arange`]
pub trait ToArange<S> {
    /// The item that this is a arange space over
    type Item;

    /// The type of range this space spans - eg inclusive or exclusive
    type Range: IntoIterator<Item = usize>;

    /// Create the arange space
    fn into_arange(self, step: S) -> IntoArange<Self::Item, Self::Range>;
}

impl<F: Real> ToArange<F> for Range<F> {
    type Item = F;

    type Range = Range<usize>;

    fn into_arange(self, step: F) -> IntoArange<Self::Item, Self::Range> {
        let Range { start, end } = self;

        IntoArange::new(
            LinearInterpolation { start, step },
            0..((end - start) / step).ceil().to_usize().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Bound;

    use super::*;

    #[test]
    fn test_arange() {
        let it = arange(0.0..2.0, 0.5);
        assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
    }

    #[test]
    fn test_arange_bounds() {
        assert_eq!(
            arange(0.0..2.0, 0.5).bounds(),
            (Bound::Included(0.0), Bound::Excluded(2.0))
        );
    }
}
