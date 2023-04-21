use array_bin_ops::Array;

use crate::{
    linspace::{LinearInterpolation, ToLinSpace},
    space::{Interpolate, IntoSpace, Space},
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
pub fn grid_space<R, S, const N: usize>(range: R, steps: S) -> GridSpace<R::Item, N>
where
    R: ToGridSpace<S, N>,
{
    range.into_grid_space(steps).into_space()
}

/// Helper trait for [`grid_space`]
pub trait ToGridSpace<S, const N: usize> {
    /// The item that this is a grid space over
    type Item;
    /// Create the grid space
    fn into_grid_space(self, step: S) -> IntoGridSpace<Self::Item, N>;
}

impl<T, const N: usize> ToGridSpace<[usize; N], N> for Range<[T; N]>
where
    Range<T>: ToLinSpace,
{
    type Item = <Range<T> as ToLinSpace>::Item;

    fn into_grid_space(self, steps: [usize; N]) -> IntoGridSpace<Self::Item, N> {
        let Range { start, end } = self;

        let mut len = 1;
        let ranges = Array(start).zip_map(end, |start, end| start..end);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lin_space = range.into_lin_space(step);
            len *= lin_space.len;
            lin_space
        });

        IntoGridSpace::new(len, GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<[usize; N], N> for RangeInclusive<[T; N]>
where
    RangeInclusive<T>: ToLinSpace,
{
    type Item = <RangeInclusive<T> as ToLinSpace>::Item;

    fn into_grid_space(self, steps: [usize; N]) -> IntoGridSpace<Self::Item, N> {
        let (start, end) = self.into_inner();

        let mut len = 1;
        let ranges = Array(start).zip_map(end, RangeInclusive::new);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lin_space = range.into_lin_space(step);
            len *= lin_space.len;
            lin_space
        });

        IntoGridSpace::new(len, GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<usize, N> for Range<[T; N]>
where
    Range<T>: ToLinSpace,
{
    type Item = <Range<T> as ToLinSpace>::Item;

    fn into_grid_space(self, steps: usize) -> IntoGridSpace<Self::Item, N> {
        let Range { start, end } = self;

        let lerps = Array(start).zip_map(end, |start, end| (start..end).into_lin_space(steps));

        IntoGridSpace::new(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

impl<T, const N: usize> ToGridSpace<usize, N> for RangeInclusive<[T; N]>
where
    RangeInclusive<T>: ToLinSpace,
{
    type Item = <RangeInclusive<T> as ToLinSpace>::Item;

    fn into_grid_space(self, steps: usize) -> IntoGridSpace<Self::Item, N> {
        let (start, end) = self.into_inner();

        let lerps = Array(start).zip_map(end, |start, end| (start..=end).into_lin_space(steps));

        IntoGridSpace::new(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSpaceInterpolation<T, const N: usize>(pub [IntoSpace<LinearInterpolation<T>>; N]);

impl<T, const N: usize> Interpolate for GridSpaceInterpolation<T, N>
where
    LinearInterpolation<T>: Interpolate<Item = T>,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        self.0.map(|space| {
            let z = x % space.len;
            x /= space.len;
            space.interpolate.interpolate(z)
        })
    }
}

/// [`Iterator`] returned by [`grid_space`]
pub type GridSpace<T, const N: usize> = Space<GridSpaceInterpolation<T, N>>;

/// [`IntoIterator`] returned by [`ToGridSpace::into_grid_space`]
pub type IntoGridSpace<T, const N: usize> = IntoSpace<GridSpaceInterpolation<T, N>>;

#[cfg(test)]
mod tests {
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
}
