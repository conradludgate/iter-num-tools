use array_init::array_init;

use crate::linspace::{Lerp, Linear};
use core::{
    iter::FusedIterator,
    ops::{Range, RangeInclusive},
};

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

        let utils = array_init(|i| (start[i]..end[i], steps[i]).into());

        let mut y = steps[0];
        for i in 1..N {
            y *= steps[i];
        }

        GridSpace {
            lerps: utils,
            steps,
            x: 0,
            y,
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, [usize; N], N> for RangeInclusive<[T; N]> {
    fn into_grid_space(self, steps: [usize; N]) -> GridSpace<T, N> {
        let (start, end) = self.into_inner();

        let utils = array_init(|i| (start[i]..=end[i], steps[i]).into());

        let mut y = steps[0];
        for i in 1..N {
            y *= steps[i];
        }

        GridSpace {
            lerps: utils,
            steps,
            x: 0,
            y,
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, usize, N> for Range<[T; N]> {
    fn into_grid_space(self, steps: usize) -> GridSpace<T, N> {
        let Self { start, end } = self;

        let utils = array_init(|i| (start[i]..end[i], steps).into());

        GridSpace {
            lerps: utils,
            steps: [steps; N],
            x: 0,
            y: steps.pow(N as u32),
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<T, usize, N> for RangeInclusive<[T; N]> {
    fn into_grid_space(self, steps: usize) -> GridSpace<T, N> {
        let (start, end) = self.into_inner();

        let utils = array_init(|i| (start[i]..=end[i], steps).into());

        GridSpace {
            lerps: utils,
            steps: [steps; N],
            x: 0,
            y: steps.pow(N as u32),
        }
    }
}

/// Iterator returned by [`grid_space`]
#[derive(Clone, Debug)]
pub struct GridSpace<T, const N: usize> {
    pub(crate) lerps: [Lerp<T>; N],
    pub(crate) steps: [usize; N],
    pub(crate) x: usize,
    pub(crate) y: usize,
}

fn get_indices<const N: usize>(mut i: usize, max: &[usize; N]) -> [usize; N] {
    let mut output = [0; N];
    for j in (0..N).rev() {
        output[j] = i % max[j];
        i /= max[j]
    }
    output
}

impl<T: Linear, const N: usize> Iterator for GridSpace<T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let Self { lerps, steps, x, y } = self;

        if x < y {
            let n = *x + 1;
            let n = get_indices(core::mem::replace(x, n), steps);
            Some(array_init(|i| lerps[i].lerp(n[i])))
        } else {
            None
        }
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }

    #[cfg(feature = "advanced_by")]
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        let diff = self.y - self.x;
        if diff < n {
            self.x = self.y;
            Err(diff)
        } else {
            self.x += n;
            Ok(())
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.y - self.x < n {
            self.x = self.y;
            None
        } else {
            let indices = get_indices(self.x + n, &self.steps);
            self.x += n + 1;
            Some(array_init(|i| self.lerps[i].lerp(indices[i])))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Linear, const N: usize> DoubleEndedIterator for GridSpace<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self { lerps, steps, x, y } = self;

        if x < y {
            *y -= 1;
            let n = get_indices(*y, steps);
            Some(array_init(|i| lerps[i].lerp(n[i])))
        } else {
            None
        }
    }
}

impl<T: Linear, const N: usize> ExactSizeIterator for GridSpace<T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.y - self.x
    }
}

impl<T: Linear, const N: usize> FusedIterator for GridSpace<T, N> {}

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<T: Linear, const N: usize> TrustedLen for GridSpace<T, N> {}

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
