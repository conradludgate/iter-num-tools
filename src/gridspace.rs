use array_bin_ops::Array;
use strength_reduce::StrengthReducedUsize;

use crate::{
    linspace::{LinearInterpolation, ToLinSpace},
    space::{Interpolate, IntoSpace, Space},
    IntoLinSpace,
};
use core::ops::{Range, RangeInclusive};

/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
///
/// let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
/// assert!(it.eq(vec![
///     [0.0, 0.0], [0.5, 0.0],
///     [0.0, 0.5], [0.5, 0.5],
///     [0.0, 1.0], [0.5, 1.0],
///     [0.0, 1.5], [0.5, 1.5],
/// ]));
///
/// // inclusive and with a single step count
/// let it = grid_space([0.0, 0.0]..=[1.0, 2.0], 3);
/// assert!(it.eq(vec![
///     [0.0, 0.0], [0.5, 0.0], [1.0, 0.0],
///     [0.0, 1.0], [0.5, 1.0], [1.0, 1.0],
///     [0.0, 2.0], [0.5, 2.0], [1.0, 2.0],
/// ]));
///
/// // even nd spaces
/// let it = grid_space([0, 0, 0]..=[1, 1, 1], 2);
/// assert!(it.eq(vec![
///     [0, 0, 0], [1, 0, 0],
///     [0, 1, 0], [1, 1, 0],
///
///     [0, 0, 1], [1, 0, 1],
///     [0, 1, 1], [1, 1, 1],
/// ]));
/// ```
pub fn grid_space<R, S, const N: usize>(
    range: R,
    steps: S,
) -> GridSpace<R::Item, <R::Range as IntoIterator>::IntoIter, N>
where
    R: ToGridSpace<S, N>,
{
    range.into_grid_space(steps).into_space()
}

/// Helper trait for [`grid_space`]
pub trait ToGridSpace<S, const N: usize> {
    /// The item that this is a grid space over
    type Item;

    /// The type of range this space spans - eg inclusive or exclusive
    type Range: IntoIterator<Item = usize>;

    /// Create the grid space
    fn into_grid_space(self, step: S) -> IntoGridSpace<Self::Item, Self::Range, N>;
}

impl<T, const N: usize> ToGridSpace<[usize; N], N> for Range<[T; N]>
where
    Range<T>: ToLinSpace,
{
    type Item = <Range<T> as ToLinSpace>::Item;
    type Range = Range<usize>;

    fn into_grid_space(self, steps: [usize; N]) -> IntoGridSpace<Self::Item, Self::Range, N> {
        let Range { start, end } = self;

        let mut len = 1;
        let ranges = Array(start).zip_map(end, |start, end| start..end);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lin_space = Linear::new(range.into_lin_space(step));
            len *= lin_space.length.get();
            lin_space
        });

        IntoGridSpace::new_exclusive(len, GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<[usize; N], N> for RangeInclusive<[T; N]>
where
    RangeInclusive<T>: ToLinSpace,
{
    type Item = <RangeInclusive<T> as ToLinSpace>::Item;
    type Range = RangeInclusive<usize>;

    fn into_grid_space(self, steps: [usize; N]) -> IntoGridSpace<Self::Item, Self::Range, N> {
        let (start, end) = self.into_inner();

        let mut len = 1;
        let ranges = Array(start).zip_map(end, RangeInclusive::new);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lin_space = Linear::new(range.into_lin_space(step));
            len *= lin_space.length.get();
            lin_space
        });

        IntoGridSpace::new_inclusive(len, GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<usize, N> for Range<[T; N]>
where
    Range<T>: ToLinSpace,
{
    type Item = <Range<T> as ToLinSpace>::Item;
    type Range = Range<usize>;

    fn into_grid_space(self, steps: usize) -> IntoGridSpace<Self::Item, Self::Range, N> {
        let Range { start, end } = self;

        let lerps = Array(start).zip_map(end, |start, end| {
            Linear::new((start..end).into_lin_space(steps))
        });

        IntoGridSpace::new_exclusive(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<usize, N> for RangeInclusive<[T; N]>
where
    RangeInclusive<T>: ToLinSpace,
{
    type Item = <RangeInclusive<T> as ToLinSpace>::Item;
    type Range = RangeInclusive<usize>;

    fn into_grid_space(self, steps: usize) -> IntoGridSpace<Self::Item, Self::Range, N> {
        let (start, end) = self.into_inner();

        let lerps = Array(start).zip_map(end, |start, end| {
            Linear::new((start..=end).into_lin_space(steps))
        });

        IntoGridSpace::new_inclusive(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Linear<T> {
    interpolate: LinearInterpolation<T>,
    pub(crate) length: StrengthReducedUsize,
}

impl<T> Linear<T> {
    pub(crate) fn new<R: IntoIterator>(x: IntoLinSpace<T, R>) -> Self {
        let len = x.range.into_iter().size_hint().0;
        Linear {
            interpolate: x.interpolate,
            length: StrengthReducedUsize::new(len),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSpaceInterpolation<T, const N: usize>(pub(crate) [Linear<T>; N]);

impl<T, const N: usize> Interpolate for GridSpaceInterpolation<T, N>
where
    LinearInterpolation<T>: Interpolate<Item = T>,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        self.0.map(|space| {
            let z;
            (x, z) = StrengthReducedUsize::div_rem(x, space.length);
            space.interpolate.interpolate(z)
        })
    }

    fn interpolate_exclusive_end(self, mut x: usize) -> Self::Item {
        let res = self.0.map(|space| {
            x = x / space.length;
            space.interpolate.interpolate(space.length.get())
        });

        assert_eq!(x, 1);

        res
    }
}

/// [`Iterator`] returned by [`grid_space`]
pub type GridSpace<T, R, const N: usize> = Space<GridSpaceInterpolation<T, N>, R>;

/// [`IntoIterator`] returned by [`ToGridSpace::into_grid_space`]
pub type IntoGridSpace<T, R, const N: usize> = IntoSpace<GridSpaceInterpolation<T, N>, R>;

#[cfg(test)]
mod tests {
    use core::ops::Bound;

    use crate::check_double_ended_iter;

    use super::*;

    #[test]
    fn test_grid_space_exclusive() {
        check_double_ended_iter(
            grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]),
            [
                [0.0, 0.0],
                [0.5, 0.0],
                [0.0, 0.5],
                [0.5, 0.5],
                [0.0, 1.0],
                [0.5, 1.0],
                [0.0, 1.5],
                [0.5, 1.5],
            ],
        );
    }

    #[test]
    fn test_grid_space_inclusive() {
        check_double_ended_iter(
            grid_space([0.0, 0.0]..=[1.0, 2.0], [3, 5]),
            [
                [0.0, 0.0],
                [0.5, 0.0],
                [1.0, 0.0],
                [0.0, 0.5],
                [0.5, 0.5],
                [1.0, 0.5],
                [0.0, 1.0],
                [0.5, 1.0],
                [1.0, 1.0],
                [0.0, 1.5],
                [0.5, 1.5],
                [1.0, 1.5],
                [0.0, 2.0],
                [0.5, 2.0],
                [1.0, 2.0],
            ],
        );
    }

    #[test]
    fn test_grid_space_exclusive_single() {
        check_double_ended_iter(
            grid_space([0.0, 0.0]..[1.0, 1.0], 2),
            [[0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]],
        );
    }

    #[test]
    fn test_grid_space_inclusive_single() {
        check_double_ended_iter(
            grid_space([0.0, 0.0]..=[1.0, 1.0], 3),
            [
                [0.0, 0.0],
                [0.5, 0.0],
                [1.0, 0.0],
                [0.0, 0.5],
                [0.5, 0.5],
                [1.0, 0.5],
                [0.0, 1.0],
                [0.5, 1.0],
                [1.0, 1.0],
            ],
        );
    }

    #[test]
    fn test_grid_space_exclusive_len() {
        let mut it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);

        let mut expected_len = 2 * 4;

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
    fn test_grid_inclusive_bounds() {
        assert_eq!(
            grid_space([0.0, 0.0]..=[1.0, 2.0], [3, 5]).bounds(),
            (Bound::Included([0.0, 0.0]), Bound::Included([1.0, 2.0]))
        );
    }

    #[test]
    fn test_grid_exclusive_bounds() {
        assert_eq!(
            grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]).bounds(),
            (Bound::Included([0.0, 0.0]), Bound::Excluded([1.0, 2.0]))
        );
    }
}
