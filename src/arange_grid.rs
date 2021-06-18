use crate::{
    arange::ArangeImpl,
    gridspace::{GridSpace, GridSpaceInterpolation},
};
use array_iter_tools::{ArrayIterator, IntoArrayIterator};
use core::ops::Range;

/// Iterator returned by `arange_grid`
pub type ArangeGrid<T, const N: usize> = GridSpace<T, N>;

/// Creates a grid space over the range made up of fixed step intervals
///
/// ```
/// use iter_num_tools::arange_grid;
///
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
/// assert!(it.eq(vec![
///     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
///     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
/// ]));
///
/// // different step count in each direction
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
/// assert!(it.eq(vec![
///     [0.0, 0.0], [0.0, 1.0],
///     [0.5, 0.0], [0.5, 1.0],
/// ]));
///
/// // even 3d spaces
/// let it = arange_grid([0.0, 0.0, 0.0]..[2.0, 2.0, 2.0], 1.0);
/// assert!(it.eq(vec![
///     [0.0, 0.0, 0.0], [0.0, 0.0, 1.0],
///     [0.0, 1.0, 0.0], [0.0, 1.0, 1.0],
///
///     [1.0, 0.0, 0.0], [1.0, 0.0, 1.0],
///     [1.0, 1.0, 0.0], [1.0, 1.0, 1.0],
/// ]));
/// ```
pub fn arange_grid<F, R, S, const N: usize>(range: R, step: S) -> ArangeGrid<F, N>
where
    (R, S): Into<ArangeGridImpl<F, N>>,
{
    let ArangeGridImpl { len, interpolate } = (range, step).into();
    ArangeGrid::new(len, interpolate)
}

pub struct ArangeGridImpl<F, const N: usize> {
    len: usize,
    interpolate: GridSpaceInterpolation<F, N>,
}

impl<F: Copy, const N: usize> From<(Range<[F; N]>, [F; N])> for ArangeGridImpl<F, N>
where
    (Range<F>, F): Into<ArangeImpl<F>>,
{
    fn from((range, step): (Range<[F; N]>, [F; N])) -> Self {
        let Range { start, end } = range;

        let (lerps, steps) = start
            .into_array_iter()
            .zip(end)
            .zip(step)
            .map(|((start, end), step)| {
                let ArangeImpl { interpolate, steps } = (start..end, step).into();
                (interpolate, steps)
            })
            .unzip();

        Self {
            len: steps.iter().product(),
            interpolate: GridSpaceInterpolation(lerps, steps),
        }
    }
}

impl<F: Copy, const N: usize> From<(Range<[F; N]>, F)> for ArangeGridImpl<F, N>
where
    (Range<F>, F): Into<ArangeImpl<F>>,
{
    fn from((range, step): (Range<[F; N]>, F)) -> Self {
        let Range { start, end } = range;

        let (lerps, steps) = start
            .into_array_iter()
            .zip(end)
            .map(|(start, end)| {
                let ArangeImpl { interpolate, steps } = (start..end, step).into();
                (interpolate, steps)
            })
            .unzip();

        Self {
            len: steps.iter().product(),
            interpolate: GridSpaceInterpolation(lerps, steps),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange_grid_exclusive() {
        let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
        assert!(it.eq(vec![[0.0, 0.0], [0.0, 1.0], [0.5, 0.0], [0.5, 1.0]]));
    }

    #[test]
    fn test_arange_grid_exclusive_rev() {
        let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
        assert!(it.rev().eq(vec![
            [0.5, 1.5],
            [0.5, 1.0],
            [0.5, 0.5],
            [0.5, 0.0],
            [0.0, 1.5],
            [0.0, 1.0],
            [0.0, 0.5],
            [0.0, 0.0],
        ]));
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
}
