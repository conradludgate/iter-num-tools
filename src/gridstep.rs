use array_bin_ops::Array;
use strength_reduce::StrengthReducedUsize;

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
pub fn grid_step<R, const N: usize>(
    range: R,
) -> GridStep<R::Item, <R::Range as IntoIterator>::IntoIter, N>
where
    R: ToGridStep<N>,
{
    range.into_grid_step().into_space()
}

/// Helper trait for [`grid_step`]
pub trait ToGridStep<const N: usize> {
    /// The item that this is a grid space over
    type Item;
    /// The type of range this space spans - eg inclusive or exclusive
    type Range: IntoIterator<Item = usize>;
    /// Create the grid space
    fn into_grid_step(self) -> IntoGridStep<Self::Item, Self::Range, N>;
}

impl<T: Step, const N: usize> ToGridStep<N> for Range<[T; N]> {
    type Item = T;
    type Range = Range<usize>;

    fn into_grid_step(self) -> IntoGridStep<Self::Item, Self::Range, N> {
        let mut len = 1;
        let steps = Array(self.start).zip_map(self.end, |start, end| {
            let steps = T::steps_between(&start, &end).expect("grid size cannot be infinite");
            len *= steps;
            (start, StrengthReducedUsize::new(steps))
        });
        IntoGridStep::new_exclusive(len, GridStepInterpolation(steps))
    }
}

impl<T: Step, const N: usize> ToGridStep<N> for RangeInclusive<[T; N]> {
    type Item = T;
    type Range = RangeInclusive<usize>;

    fn into_grid_step(self) -> IntoGridStep<Self::Item, Self::Range, N> {
        let mut len = 1;
        let (start, end) = self.into_inner();
        let steps = Array(start).zip_map(end, |start, end| {
            let steps = T::steps_between(&start, &end).expect("grid size cannot be infinite") + 1;
            len *= steps;
            (start, StrengthReducedUsize::new(steps))
        });
        IntoGridStep::new_inclusive(len, GridStepInterpolation(steps))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridStepInterpolation<T, const N: usize>(pub [(T, StrengthReducedUsize); N]);

impl<T, const N: usize> Interpolate for GridStepInterpolation<T, N>
where
    T: Step,
{
    type Item = [T; N];
    fn interpolate(self, mut x: usize) -> [T; N] {
        self.0.map(|space| {
            let z;
            (x, z) = StrengthReducedUsize::div_rem(x, space.1);
            T::forward(space.0, z).unwrap()
        })
    }

    fn interpolate_exclusive_end(self, mut x: usize) -> Self::Item {
        let res = self.0.map(|space| {
            x = x / space.1;
            T::forward(space.0, space.1.get()).unwrap()
        });

        assert_eq!(x, 1);

        res
    }
}

/// [`Iterator`] returned by [`grid_space`]
pub type GridStep<T, R, const N: usize> = Space<GridStepInterpolation<T, N>, R>;

/// [`IntoIterator`] returned by [`ToGridSpace::into_grid_space`]
pub type IntoGridStep<T, R, const N: usize> = IntoSpace<GridStepInterpolation<T, N>, R>;

#[cfg(test)]
mod tests {
    use core::ops::Bound;

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

    #[test]
    fn test_grid_inclusive_bounds() {
        assert_eq!(
            grid_step([0, 0]..=[1, 2]).bounds(),
            (Bound::Included([0, 0]), Bound::Included([1, 2]))
        );
    }

    #[test]
    fn test_grid_exclusive_bounds() {
        assert_eq!(
            grid_step([0, 0]..[1, 2]).bounds(),
            (Bound::Included([0, 0]), Bound::Excluded([1, 2]))
        );
    }
}
