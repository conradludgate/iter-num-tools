use array_bin_ops::Array;

use crate::{
    space::{Interpolate, IntoSpace, Space},
    step::Step,
};
use core::ops::{Range, RangeInclusive};

/// Creates a iterator over a range of arrays
///
/// ```
/// use iter_num_tools::grid_step;
///
/// // exclusive range
/// let it = grid_step([0, 0]..[2, 4]);
/// assert!(it.eq(vec![
///     [0, 0], [1, 0],
///     [0, 1], [1, 1],
///     [0, 2], [1, 2],
///     [0, 3], [1, 3],
/// ]));
///
/// // inclusive range
/// let it = grid_step([0, 0]..=[1, 3]);
/// assert!(it.eq(vec![
///     [0, 0], [1, 0],
///     [0, 1], [1, 1],
///     [0, 2], [1, 2],
///     [0, 3], [1, 3],
/// ]));
///
/// // even nd spaces
/// let it = grid_step([0, 0, 0]..=[1, 1, 1]);
/// assert!(it.eq(vec![
///     [0, 0, 0], [1, 0, 0],
///     [0, 1, 0], [1, 1, 0],
///
///     [0, 0, 1], [1, 0, 1],
///     [0, 1, 1], [1, 1, 1],
/// ]));
/// ```
pub fn grid_step<R, const N: usize>(range: R) -> GridStep<R::Item, N>
where
    R: ToGridStep<N>,
{
    range.into_grid_step().into_space()
}

/// Helper trait for [`grid_step`]
pub trait ToGridStep<const N: usize> {
    /// The item that this is a grid space over
    type Item;
    /// Create the grid space
    fn into_grid_step(self) -> IntoGridStep<Self::Item, N>;
}

impl<T: Step, const N: usize> ToGridStep<N> for Range<[T; N]> {
    type Item = T;

    fn into_grid_step(self) -> IntoGridStep<Self::Item, N> {
        let mut len = 1;
        let steps = Array(self.start).zip_map(self.end, |start, end| {
            let steps = T::steps_between(&start, &end).expect("grid size cannot be infinite");
            len *= steps;
            (start, steps)
        });
        IntoGridStep {
            interpolate: GridStepInterpolation(steps),
            len,
        }
    }
}

impl<T: Step, const N: usize> ToGridStep<N> for RangeInclusive<[T; N]> {
    type Item = T;

    fn into_grid_step(self) -> IntoGridStep<Self::Item, N> {
        let mut len = 1;
        let (start, end) = self.into_inner();
        let steps = Array(start).zip_map(end, |start, end| {
            let steps = T::steps_between(&start, &end).expect("grid size cannot be infinite") + 1;
            len *= steps;
            (start, steps)
        });
        IntoGridStep {
            interpolate: GridStepInterpolation(steps),
            len,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridStepInterpolation<T, const N: usize>(pub [(T, usize); N]);

impl<T, const N: usize> Interpolate for GridStepInterpolation<T, N>
where
    T: Step,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        self.0.map(|space| {
            let z = x % space.1;
            x /= space.1;
            T::forward(space.0, z).unwrap()
        })
    }
}

/// [`Iterator`] returned by [`grid_space`]
pub type GridStep<T, const N: usize> = Space<GridStepInterpolation<T, N>>;

/// [`IntoIterator`] returned by [`ToGridSpace::into_grid_space`]
pub type IntoGridStep<T, const N: usize> = IntoSpace<GridStepInterpolation<T, N>>;

#[cfg(test)]
mod tests {
    use crate::check_double_ended_iter;

    use super::*;

    #[test]
    fn test_grid_space_exclusive() {
        check_double_ended_iter(
            grid_step([0, 0]..[2, 4]),
            [
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
            ],
        );
    }

    #[test]
    fn test_grid_space_inclusive() {
        check_double_ended_iter(
            grid_step([0, 0]..=[1, 3]),
            [
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
            ],
        );
    }

    #[test]
    fn test_grid_space_exclusive_len() {
        let mut it = grid_step([0, 0]..[2, 4]);

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
