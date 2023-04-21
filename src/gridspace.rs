use array_bin_ops::Array;
use num_traits::Num;

use crate::{
    linspace::LinearInterpolation,
    space::{Interpolate, Space},
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
/// // even 3d spaces
/// let it = grid_space([0, 0, 0]..=[1, 1, 1], 2);
/// assert!(it.eq(vec![
///     [0, 0, 0], [1, 0, 0],
///     [0, 1, 0], [1, 1, 0],
///
///     [0, 0, 1], [1, 0, 1],
///     [0, 1, 1], [1, 1, 1],
/// ]));
/// ```
pub fn grid_space<T, R, S, const N: usize>(range: R, steps: S) -> GridSpace<T, N>
where
    (R, S): Into<GridSpace<T, N>>,
{
    (range, steps).into()
}

impl<T: Num + Copy, const N: usize> From<(Range<[T; N]>, [usize; N])> for GridSpace<T, N>
where
    (Range<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (Range<[T; N]>, [usize; N])) -> Self {
        let Range { start, end } = range;

        let mut len = 1;
        let ranges = Array(start).zip_map(end, |start, end| start..end);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lerp: LinearInterpolation<T> = (range, step).into();
            len *= step;
            (lerp, step)
        });

        Self::new(len, GridSpaceInterpolation(lerps))
    }
}

impl<T: Num + Copy, const N: usize> From<(RangeInclusive<[T; N]>, [usize; N])> for GridSpace<T, N>
where
    (RangeInclusive<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (RangeInclusive<[T; N]>, [usize; N])) -> Self {
        let (start, end) = range.into_inner();

        let mut len = 1;
        let ranges = Array(start).zip_map(end, RangeInclusive::new);
        let lerps = Array(ranges).zip_map(steps, |range, step| {
            let lerp: LinearInterpolation<T> = (range, step).into();
            len *= step;
            (lerp, step)
        });

        Self::new(len, GridSpaceInterpolation(lerps))
    }
}

impl<T: Num + Copy, const N: usize> From<(Range<[T; N]>, usize)> for GridSpace<T, N>
where
    (Range<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (Range<[T; N]>, usize)) -> Self {
        let Range { start, end } = range;

        let lerps = Array(start).zip_map(end, |start, end| ((start..end, steps).into(), steps));

        Self::new(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

impl<T: Num + Copy, const N: usize> From<(RangeInclusive<[T; N]>, usize)> for GridSpace<T, N>
where
    (RangeInclusive<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (RangeInclusive<[T; N]>, usize)) -> Self {
        let (start, end) = range.into_inner();

        let lerps = Array(start).zip_map(end, |start, end| ((start..=end, steps).into(), steps));

        Self::new(steps.pow(N as u32), GridSpaceInterpolation(lerps))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSpaceInterpolation<T, const N: usize>(pub [(LinearInterpolation<T>, usize); N]);

impl<T, const N: usize> Interpolate for GridSpaceInterpolation<T, N>
where
    LinearInterpolation<T>: Interpolate<Item = T>,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        self.0.map(|(lerp, step)| {
            let z = x % step;
            x /= step;
            lerp.interpolate(z)
        })
    }
}

/// Iterator returned by [`grid_space`]
pub type GridSpace<T, const N: usize> = Space<GridSpaceInterpolation<T, N>>;

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
