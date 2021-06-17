use array_iter_tools::ArrayIterator;

use crate::{
    linspace::{Linear, LinearInterpolation},
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
    R: IntoGridSpace<T, S, N>,
{
    range.into_grid_space(steps)
}

/// Used by [`grid_space`]
pub trait IntoGridSpace<T, S, const N: usize> {
    /// Convert self into a [`GridSpace`]
    fn into_grid_space(self, steps: S) -> GridSpace<T, N>;
}

impl<T: Linear, const N: usize> IntoGridSpace<T, [usize; N], N> for Range<[T; N]> {
    fn into_grid_space(self, steps: [usize; N]) -> GridSpace<T, N> {
        let Self { start, end } = self;

        let lerps = start.zip_array(end).zip_array(steps).map_array(|((start, end), step)| {
            (start..end, step).into()
        }).collect_array();

        let y = steps.iter().product();

        GridSpace::new(y, GridSpaceInterpolation(lerps, steps))
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, [usize; N], N> for RangeInclusive<[T; N]> {
    fn into_grid_space(self, steps: [usize; N]) -> GridSpace<T, N> {
        let (start, end) = self.into_inner();

        let lerps = start.zip_array(end).zip_array(steps).map_array(|((start, end), step)| {
            (start..=end, step).into()
        }).collect_array();

        let y = steps.iter().product();

        GridSpace::new(y, GridSpaceInterpolation(lerps, steps))
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, usize, N> for Range<[T; N]> {
    fn into_grid_space(self, steps: usize) -> GridSpace<T, N> {
        let Self { start, end } = self;

        let lerps = start.zip_array(end).map_array(|(start, end)| {
            (start..end, steps).into()
        }).collect_array();

        GridSpace::new(
            steps.pow(N as u32),
            GridSpaceInterpolation(lerps, [steps; N]),
        )
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, usize, N> for RangeInclusive<[T; N]> {
    fn into_grid_space(self, steps: usize) -> GridSpace<T, N> {
        let (start, end) = self.into_inner();

        let lerps = start.zip_array(end).map_array(|(start, end)| {
            (start..=end, steps).into()
        }).collect_array();

        GridSpace::new(
            steps.pow(N as u32),
            GridSpaceInterpolation(lerps, [steps; N]),
        )
    }
}

pub struct GridSpaceInterpolation<T, const N: usize>(pub [LinearInterpolation<T>; N], pub [usize; N]);

impl<T, const N: usize> Interpolate for GridSpaceInterpolation<T, N>
where
    T: Copy,
    LinearInterpolation<T>: Interpolate<Item = T>,
{
    type Item = [T; N];
    fn interpolate(&self, mut x: usize) -> [T; N] {
        let mut indices = [0; N];
        for j in (0..N).rev() {
            indices[j] = x % self.1[j];
            x /= self.1[j]
        }
        self.0.zip_array(indices).map_array(|(lerp, i)| lerp.interpolate(i)).collect_array()
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
