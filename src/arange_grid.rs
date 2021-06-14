use crate::{arange::arange_lerp, gridspace::GridSpace, linspace::Linear};
use core::ops::Range;
use num_traits::real::Real;


pub type ArangeGrid<T, const N: usize> = GridSpace<T, N>;

/// Creates a grid space over the range made up of fixed step intervals
///
/// ```
/// use iter_num_tools::arange_grid;
/// use itertools::Itertools;
///
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
/// itertools::assert_equal(it, vec![
///     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
///     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
/// ]);
///
/// // different step count in each direction
/// let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
/// itertools::assert_equal(it, vec![
///     [0.0, 0.0], [0.0, 1.0],
///     [0.5, 0.0], [0.5, 1.0],
/// ]);
///
/// // even 3d spaces
/// let it = arange_grid([0.0, 0.0, 0.0]..[2.0, 2.0, 2.0], 1.0);
/// itertools::assert_equal(it, vec![
///     [0.0, 0.0, 0.0], [0.0, 0.0, 1.0],
///     [0.0, 1.0, 0.0], [0.0, 1.0, 1.0],
///
///     [1.0, 0.0, 0.0], [1.0, 0.0, 1.0],
///     [1.0, 1.0, 0.0], [1.0, 1.0, 1.0],
/// ]);
/// ```
pub fn arange_grid<R, S>(range: R, size: S) -> <R as IntoArangeGrid<S>>::ArangeGrid
where
    R: IntoArangeGrid<S>,
{
    range.into_arange_grid(size)
}

/// Used by [`arange_grid`]
pub trait IntoArangeGrid<S> {
    type ArangeGrid;
    fn into_arange_grid(self, size: S) -> Self::ArangeGrid;
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<[F; N]> for Range<[F; N]> {
    type ArangeGrid = ArangeGrid<F, N>;

    fn into_arange_grid(self, size: [F; N]) -> Self::ArangeGrid {
        let Self { start, end } = self;
        let mut utils = core::mem::MaybeUninit::uninit_array();
        let mut steps = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            let (util, s) = arange_lerp(start[i]..end[i], size[i]);
            utils[i].write(util);
            steps[i].write(s);
        }
        let steps = unsafe { core::mem::MaybeUninit::array_assume_init(steps) };
        let mut y = [0; N];
        y[0] = steps[0];
        ArangeGrid {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps,
            x: [0; N],
            y,
        }
    }
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<F> for Range<[F; N]> {
    type ArangeGrid = ArangeGrid<F, N>;

    fn into_arange_grid(self, size: F) -> Self::ArangeGrid {
        let Self { start, end } = self;
        let mut utils = core::mem::MaybeUninit::uninit_array();
        let mut steps = core::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            let (util, s) = arange_lerp(start[i]..end[i], size);
            utils[i].write(util);
            steps[i].write(s);
        }
        let steps = unsafe { core::mem::MaybeUninit::array_assume_init(steps) };
        let mut y = [0; N];
        y[0] = steps[0];
        ArangeGrid {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps,
            x: [0; N],
            y,
        }
    }
}
