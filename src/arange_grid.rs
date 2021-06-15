use crate::{arange::arange_lerp, gridspace::GridSpace, linspace::Linear};
use array_init::array_init;
use core::ops::Range;
use num_traits::real::Real;

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
    R: IntoArangeGrid<F, S, N>,
{
    range.into_arange_grid(step)
}

/// Used by [`arange_grid`]
pub trait IntoArangeGrid<F, S, const N: usize> {
    /// Convert self into an [`ArangeGrid`]
    fn into_arange_grid(self, step: S) -> ArangeGrid<F, N>;
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<F, [F; N], N> for Range<[F; N]> {
    fn into_arange_grid(self, step: [F; N]) -> ArangeGrid<F, N> {
        let Self { start, end } = self;

        let mut steps = [0; N];
        let mut y = 1;
        let utils = array_init(|i| {
            let (util, s) = arange_lerp(start[i]..end[i], step[i]);
            steps[i] = s;
            y *= s;
            util
        });

        ArangeGrid {
            lerps: utils,
            steps,
            x: 0,
            y,
        }
    }
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<F, F, N> for Range<[F; N]> {
    fn into_arange_grid(self, step: F) -> ArangeGrid<F, N> {
        let Self { start, end } = self;

        let mut steps = [0; N];
        let mut y = 1;
        let utils = array_init(|i| {
            let (util, s) = arange_lerp(start[i]..end[i], step);
            steps[i] = s;
            y *= s;
            util
        });

        ArangeGrid {
            lerps: utils,
            steps,
            x: 0,
            y,
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
