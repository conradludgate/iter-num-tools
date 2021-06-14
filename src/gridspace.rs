use crate::{Lerp, Linear};
use core::{
    iter::{FusedIterator, InPlaceIterable, TrustedLen},
    ops::{Range, RangeInclusive},
};

/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
///
/// let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
/// itertools::assert_equal(it, vec![
///     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
///     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
/// ]);
///
/// // inclusive and with a single step count
/// let it = grid_space([0.0, 0.0]..=[1.0, 2.0], 3);
/// itertools::assert_equal(it, vec![
///     [0.0, 0.0], [0.0, 1.0], [0.0, 2.0],
///     [0.5, 0.0], [0.5, 1.0], [0.5, 2.0],
///     [1.0, 0.0], [1.0, 1.0], [1.0, 2.0],
/// ]);
///
/// // even 3d spaces
/// let it = grid_space([0, 0, 0]..=[1, 1, 1], 2);
/// itertools::assert_equal(it, vec![
///     [0, 0, 0], [0, 0, 1],
///     [0, 1, 0], [0, 1, 1],
///
///     [1, 0, 0], [1, 0, 1],
///     [1, 1, 0], [1, 1, 1],
/// ]);
/// ```
pub fn grid_space<R, S>(range: R, size: S) -> <R as IntoGridSpace<S>>::GridSpace
where
    R: IntoGridSpace<S>,
{
    range.into_grid_space(size)
}

/// Used by [grid_space]
pub trait IntoGridSpace<S> {
    type GridSpace;
    fn into_grid_space(self, size: S) -> Self::GridSpace;
}

impl<T: Linear, const N: usize> IntoGridSpace<[usize; N]> for Range<[T; N]> {
    type GridSpace = GridSpace<T, N>;
    fn into_grid_space(self, size: [usize; N]) -> Self::GridSpace {
        let Self { start, end } = self;
        let mut utils = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            utils[i].write((start[i]..end[i], size[i]).into());
        }
        let mut y = [0; N];
        y[0] = size[0];
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps: size,
            x: [0; N],
            y,
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<[usize; N]> for RangeInclusive<[T; N]> {
    type GridSpace = GridSpace<T, N>;
    fn into_grid_space(self, size: [usize; N]) -> Self::GridSpace {
        let (start, end) = self.into_inner();
        let mut utils = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            utils[i].write((start[i]..=end[i], size[i]).into());
        }
        let mut y = [0; N];
        y[0] = size[0];
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps: size,
            x: [0; N],
            y,
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<usize> for Range<[T; N]> {
    type GridSpace = GridSpace<T, N>;
    fn into_grid_space(self, size: usize) -> Self::GridSpace {
        let Self { start, end } = self;
        let mut utils = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            utils[i].write((start[i]..end[i], size).into());
        }
        let mut y = [0; N];
        y[0] = size;
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps: [size; N],
            x: [0; N],
            y,
        }
    }
}

impl<T: Linear, const N: usize> IntoGridSpace<usize> for RangeInclusive<[T; N]> {
    type GridSpace = GridSpace<T, N>;
    fn into_grid_space(self, size: usize) -> Self::GridSpace {
        let (start, end) = self.into_inner();
        let mut utils = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            utils[i].write((start[i]..=end[i], size).into());
        }
        let mut y = [0; N];
        y[0] = size;
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps: [size; N],
            x: [0; N],
            y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSpace<T, const N: usize> {
    pub(crate) utils: [Lerp<T>; N],
    pub(crate) steps: [usize; N],
    pub(crate) x: [usize; N],
    pub(crate) y: [usize; N],
}

fn grid_lerp<T: Linear, const N: usize>(utils: [Lerp<T>; N], x: [usize; N]) -> [T; N] {
    let mut output = core::mem::MaybeUninit::uninit_array();
    for i in 0..N {
        output[i].write(utils[i].lerp(x[i]));
    }
    unsafe { core::mem::MaybeUninit::array_assume_init(output) }
}

fn inc<const N: usize>(mut n: [usize; N], max: &[usize; N]) -> [usize; N] {
    n[N - 1] += 1;
    let mut i = N - 1;
    while i > 0 && n[i] == max[i] {
        n[i] = 0;
        i -= 1;
        n[i] += 1;
    }
    n
}

impl<T: Linear, const N: usize> Iterator for GridSpace<T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let Self { utils, steps, x, y } = self;

        if (*x).lt(y) {
            let n = inc(*x, steps);
            Some(grid_lerp(*utils, core::mem::replace(x, n)))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

fn dec<const N: usize>(n: &mut [usize; N], max: &[usize; N]) {
    let mut i = N - 1;
    while i > 0 && n[i] == 0 {
        n[i] = max[i] - 1;
        i -= 1;
    }
    n[i] -= 1;
}

impl<T: Linear, const N: usize> DoubleEndedIterator for GridSpace<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self { utils, steps, x, y } = self;

        if (*x).lt(y) {
            dec(y, steps);
            Some(grid_lerp(*utils, *y))
        } else {
            None
        }
    }
}

impl<T: Linear, const N: usize> ExactSizeIterator for GridSpace<T, N> {
    #[inline]
    fn len(&self) -> usize {
        let mut x = self.x[0];
        let mut y = self.y[0];
        for i in 1..N {
            x = x * self.steps[i] + self.x[i];
            y = y * self.steps[i] + self.y[i];
        }
        y - x
    }
}

impl<T: Linear, const N: usize> FusedIterator for GridSpace<T, N> {}
unsafe impl<T: Linear, const N: usize> TrustedLen for GridSpace<T, N> {}
unsafe impl<T: Linear, const N: usize> InPlaceIterable for GridSpace<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_space_exclusive() {
        let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
        assert_eq_iter!(
            it,
            [
                [0.0, 0.0],
                [0.0, 0.5],
                [0.0, 1.0],
                [0.0, 1.5],
                [0.5, 0.0],
                [0.5, 0.5],
                [0.5, 1.0],
                [0.5, 1.5]
            ]
        );
    }

    #[test]
    fn test_grid_space_exclusive_rev() {
        let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
        assert_eq_iter!(
            it.rev(),
            [
                [0.5, 1.5],
                [0.5, 1.0],
                [0.5, 0.5],
                [0.5, 0.0],
                [0.0, 1.5],
                [0.0, 1.0],
                [0.0, 0.5],
                [0.0, 0.0]
            ]
        );
    }

    #[test]
    fn test_grid_space_exclusive_len() {
        let mut it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
        assert_eq!(it.len(), 8);
        it.next();
        assert_eq!(it.len(), 7);
        it.next_back();
        assert_eq!(it.len(), 6);
        it.next();
        assert_eq!(it.len(), 5);
        it.next_back();
        assert_eq!(it.len(), 4);
        it.next();
        assert_eq!(it.len(), 3);
        it.next_back();
        assert_eq!(it.len(), 2);
        it.next();
        assert_eq!(it.len(), 1);
        it.next_back();
        assert_eq!(it.len(), 0);
    }
}
