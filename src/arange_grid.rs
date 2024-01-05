use crate::{
    arange::ToArange,
    gridspace::{GridSpace, GridSpaceInterpolation, Linear},
    IntoGridSpace,
};
use array_bin_ops::Array;
use core::ops::Range;

/// [`Iterator`] returned by [`arange_grid`]
pub type ArangeGrid<T, R, const N: usize> = GridSpace<T, R, N>;

/// [`IntoIterator`] returned by [`ToGridSpace::into_grid_space`]
pub type IntoArangeGrid<T, R, const N: usize> = IntoGridSpace<T, R, N>;

/// Creates a grid space over the range made up of fixed step intervals
///
/// ```
/// use iter_num_tools::arange_grid;
///
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
/// assert!(it.eq([
///     [0.0, 0.0], [0.5, 0.0],
///     [0.0, 0.5], [0.5, 0.5],
///     [0.0, 1.0], [0.5, 1.0],
///     [0.0, 1.5], [0.5, 1.5],
/// ]));
///
/// // different step count in each direction
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
/// assert!(it.eq([
///     [0.0, 0.0], [0.5, 0.0],
///     [0.0, 1.0], [0.5, 1.0],
/// ]));
///
/// // even nd spaces
/// let it = arange_grid([0.0, 0.0, 0.0]..[2.0, 2.0, 2.0], 1.0);
/// assert!(it.eq([
///     [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
///
///     [0.0, 0.0, 1.0], [1.0, 0.0, 1.0],
///     [0.0, 1.0, 1.0], [1.0, 1.0, 1.0],
/// ]));
/// ```
pub fn arange_grid<R, S, const N: usize>(
    range: R,
    step: S,
) -> ArangeGrid<R::Item, <R::Range as IntoIterator>::IntoIter, N>
where
    R: ToArangeGrid<S, N>,
{
    range.into_arange_grid(step).into_space()
}

/// Helper trait for [`arange_grid`]
pub trait ToArangeGrid<S, const N: usize> {
    /// The item that this is a arange grid over
    type Item;
    /// The type of range this space spans - eg inclusive or exclusive
    type Range: IntoIterator<Item = usize>;
    /// Create the arange grid
    fn into_arange_grid(self, step: S) -> IntoArangeGrid<Self::Item, Self::Range, N>;
}

impl<F: Copy, const N: usize> ToArangeGrid<[F; N], N> for Range<[F; N]>
where
    Range<F>: ToArange<F>,
{
    type Item = <Range<F> as ToArange<F>>::Item;
    type Range = Range<usize>;

    fn into_arange_grid(self, step: [F; N]) -> IntoArangeGrid<Self::Item, Self::Range, N> {
        let Range { start, end } = self;

        let mut len = 1;
        let ranges = Array(start).zip_map(end, |start, end| start..end);
        let lerps = Array(ranges).zip_map(step, |range, step| {
            let space = Linear::new(range.into_arange(step));
            len *= space.length.get();
            space
        });

        IntoArangeGrid::new_exclusive(len, GridSpaceInterpolation(lerps))
    }
}
impl<F: Copy, const N: usize> ToArangeGrid<F, N> for Range<[F; N]>
where
    Range<F>: ToArange<F>,
{
    type Item = <Range<F> as ToArange<F>>::Item;
    type Range = Range<usize>;

    fn into_arange_grid(self, step: F) -> IntoArangeGrid<Self::Item, Self::Range, N> {
        let Range { start, end } = self;

        let mut len = 1;
        let lerps = Array(start).zip_map(end, |start, end| {
            let space = Linear::new((start..end).into_arange(step));
            len *= space.length.get();
            space
        });

        IntoArangeGrid::new_exclusive(len, GridSpaceInterpolation(lerps))
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Bound;

    use crate::check_double_ended_iter;

    use super::*;

    #[test]
    fn test_arange_grid_exclusive() {
        check_double_ended_iter(
            arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]),
            [[0.0, 0.0], [0.5, 0.0], [0.0, 1.0], [0.5, 1.0]],
        );
    }

    #[test]
    fn test_arange_grid_exclusive_len() {
        let mut it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);

        let mut expected_len = 8;

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

    #[test]
    fn test_arange_grid_bounds() {
        assert_eq!(
            arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]).bounds(),
            (Bound::Included([0.0, 0.0]), Bound::Excluded([1.0, 2.0]))
        );
    }

    #[test]
    fn test_arange_grid_single_bounds() {
        assert_eq!(
            arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5).bounds(),
            (Bound::Included([0.0, 0.0]), Bound::Excluded([1.0, 2.0]))
        );
    }
}
