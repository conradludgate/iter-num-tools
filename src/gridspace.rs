use array_iter_tools::{ArrayIterator, IntoArrayIterator};
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
///     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
///     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
/// ]));
///
/// // inclusive and with a single step count
/// let it = grid_space([0.0, 0.0]..=[1.0, 2.0], 3);
/// assert!(it.eq(vec![
///     [0.0, 0.0], [0.0, 1.0], [0.0, 2.0],
///     [0.5, 0.0], [0.5, 1.0], [0.5, 2.0],
///     [1.0, 0.0], [1.0, 1.0], [1.0, 2.0],
/// ]));
///
/// // even 3d spaces
/// let it = grid_space([0, 0, 0]..=[1, 1, 1], 2);
/// assert!(it.eq(vec![
///     [0, 0, 0], [0, 0, 1],
///     [0, 1, 0], [0, 1, 1],
///
///     [1, 0, 0], [1, 0, 1],
///     [1, 1, 0], [1, 1, 1],
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

        let lerps = start
            .into_array_iter()
            .zip(end)
            .zip(steps)
            .map(|((start, end), step)| (start..end, step).into())
            .collect();

        let y = steps.iter().product();

        Self::new(y, GridSpaceInterpolation(lerps, steps))
    }
}

impl<T: Num + Copy, const N: usize> From<(RangeInclusive<[T; N]>, [usize; N])> for GridSpace<T, N>
where
    (RangeInclusive<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (RangeInclusive<[T; N]>, [usize; N])) -> Self {
        let (start, end) = range.into_inner();

        let lerps = start
            .into_array_iter()
            .zip(end)
            .zip(steps)
            .map(|((start, end), step)| (start..=end, step).into())
            .collect();

        let y = steps.iter().product();

        Self::new(y, GridSpaceInterpolation(lerps, steps))
    }
}

impl<T: Num + Copy, const N: usize> From<(Range<[T; N]>, usize)> for GridSpace<T, N>
where
    (Range<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (Range<[T; N]>, usize)) -> Self {
        let Range { start, end } = range;

        let lerps = start
            .into_array_iter()
            .zip(end)
            .map(|(start, end)| (start..end, steps).into())
            .collect();

        Self::new(
            steps.pow(N as u32),
            GridSpaceInterpolation(lerps, [steps; N]),
        )
    }
}

impl<T: Num + Copy, const N: usize> From<(RangeInclusive<[T; N]>, usize)> for GridSpace<T, N>
where
    (RangeInclusive<T>, usize): Into<LinearInterpolation<T>>,
{
    fn from((range, steps): (RangeInclusive<[T; N]>, usize)) -> Self {
        let (start, end) = range.into_inner();

        let lerps = start
            .into_array_iter()
            .zip(end)
            .map(|(start, end)| (start..=end, steps).into())
            .collect();

        Self::new(
            steps.pow(N as u32),
            GridSpaceInterpolation(lerps, [steps; N]),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSpaceInterpolation<T, const N: usize>(
    pub [LinearInterpolation<T>; N],
    pub [usize; N],
);

impl<T, const N: usize> Interpolate for GridSpaceInterpolation<T, N>
where
    LinearInterpolation<T>: Interpolate<Item = T>,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        let mut indices = [0; N];
        for j in (0..N).rev() {
            indices[j] = x % self.1[j];
            x /= self.1[j]
        }
        self.0
            .into_array_iter()
            .zip(indices)
            .map(|(lerp, i)| lerp.interpolate(i))
            .collect()
    }
}

/// Iterator returned by [`grid_space`]
pub type GridSpace<T, const N: usize> = Space<GridSpaceInterpolation<T, N>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_space_exclusive() {
        let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
        assert!(it.eq(vec![
            [0.0, 0.0],
            [0.0, 0.5],
            [0.0, 1.0],
            [0.0, 1.5],
            [0.5, 0.0],
            [0.5, 0.5],
            [0.5, 1.0],
            [0.5, 1.5],
        ]));
    }

    #[test]
    fn test_grid_space_exclusive_rev() {
        let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
        assert!(it.rev().eq(vec![
            [0.5, 1.5],
            [0.5, 1.0],
            [0.5, 0.5],
            [0.5, 0.0],
            [0.0, 1.5],
            [0.0, 1.0],
            [0.0, 0.5],
            [0.0, 0.0],
        ],));
    }

    #[test]
    fn test_grid_space_inclusive() {
        let it = grid_space([0.0, 0.0]..=[1.0, 2.0], [3, 5]);
        assert!(it.eq(vec![
            [0.0, 0.0],
            [0.0, 0.5],
            [0.0, 1.0],
            [0.0, 1.5],
            [0.0, 2.0],
            [0.5, 0.0],
            [0.5, 0.5],
            [0.5, 1.0],
            [0.5, 1.5],
            [0.5, 2.0],
            [1.0, 0.0],
            [1.0, 0.5],
            [1.0, 1.0],
            [1.0, 1.5],
            [1.0, 2.0],
        ],));
    }

    #[test]
    fn test_grid_space_inclusive_rev() {
        let it = grid_space([0.0, 0.0]..=[1.0, 2.0], [3, 5]);
        assert!(it.rev().eq(vec![
            [1.0, 2.0],
            [1.0, 1.5],
            [1.0, 1.0],
            [1.0, 0.5],
            [1.0, 0.0],
            [0.5, 2.0],
            [0.5, 1.5],
            [0.5, 1.0],
            [0.5, 0.5],
            [0.5, 0.0],
            [0.0, 2.0],
            [0.0, 1.5],
            [0.0, 1.0],
            [0.0, 0.5],
            [0.0, 0.0],
        ],));
    }

    #[test]
    fn test_grid_space_exclusive_single() {
        let it = grid_space([0.0, 0.0]..[1.0, 1.0], 2);
        assert!(it.eq(vec![[0.0, 0.0], [0.0, 0.5], [0.5, 0.0], [0.5, 0.5]]));
    }

    #[test]
    fn test_grid_space_inclusive_single() {
        let it = grid_space([0.0, 0.0]..=[1.0, 1.0], 3);
        assert!(it.eq(vec![
            [0.0, 0.0],
            [0.0, 0.5],
            [0.0, 1.0],
            [0.5, 0.0],
            [0.5, 0.5],
            [0.5, 1.0],
            [1.0, 0.0],
            [1.0, 0.5],
            [1.0, 1.0],
        ],));
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
