use crate::{arange_lerp, GridSpace, Linear};
use core::ops::Range;
use num_traits::real::Real;

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

/// Used by [arange_grid]
pub trait IntoArangeGrid<S> {
    type ArangeGrid;
    fn into_arange_grid(self, size: S) -> Self::ArangeGrid;
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<[F; N]> for Range<[F; N]> {
    type ArangeGrid = GridSpace<F, N>;

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
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps,
            x: [0; N],
            y,
        }
    }
}

impl<F: Real + Linear, const N: usize> IntoArangeGrid<F> for Range<[F; N]> {
    type ArangeGrid = GridSpace<F, N>;

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
        GridSpace {
            utils: unsafe { core::mem::MaybeUninit::array_assume_init(utils) },
            steps,
            x: [0; N],
            y,
        }
    }
}

// macro_rules! impl_arange_grid {
//     ($Grid:ident: $($f:ident: $r:ident;$s:ident),*) => {

// impl<$($f),*> IntoArangeGrid<($($f),*)> for Range<($($f),*)>
// where $(
//     $f: Real + Sub<Output = $f> + Div<Output = $f> + ToPrimitive + FromPrimitive,
//     Range<$f>: IntoLinSpace<$f>,
// )*
// {
//     type ArangeGrid = $Grid<$(LinSpace<$f>),*>;

//     fn into_arange_grid(self, ($($s),*): ($($f),*)) -> Self::ArangeGrid {
//         let ($($r),*) = self.transpose();
//         grid(($(
//             arange($r, $s),
//         )*))
//     }
// }

//     };
// }

// impl_arange_grid!(Grid2: F0: r0;s0, F1: r1;s1); // 2d grid
// impl_arange_grid!(Grid3: F0: r0;s0, F1: r1;s1, F2: r2;s2); // 3d grid
// impl_arange_grid!(Grid4: F0: r0;s0, F1: r1;s1, F2: r2;s2, F3: r3;s3); // 4d grid

// macro_rules! impl_arange_grid_simple {
//     ($Grid:ident;$F:ident: $($f:ident;$s:ident),*) => {

// impl<$F> IntoArangeGrid<$F> for Range<($($f),*)>
// where
//     $F: Real + Sub<Output = $F> + Div<Output = $F> + ToPrimitive + FromPrimitive,
//     Range<$F>: IntoLinSpace<$F>,
// {
//     type ArangeGrid = $Grid<$(LinSpace<$f>),*>;

//     fn into_arange_grid(self, s: $F) -> Self::ArangeGrid {
//         $( let $s = s; )*
//         self.into_arange_grid(($($s),*))
//     }
// }

//     };
// }

// impl_arange_grid_simple!(Grid2;F: F;s0, F;s1); // 2d grid
// impl_arange_grid_simple!(Grid3;F: F;s0, F;s1, F;s2); // 3d grid
// impl_arange_grid_simple!(Grid4;F: F;s0, F;s1, F;s2, F;s3); // 4d grid
