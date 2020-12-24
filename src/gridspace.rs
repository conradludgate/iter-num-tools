/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
/// use itertools::Itertools;
///
/// let it = grid_space((0.0, 0.0)..(1.0, 2.0), (2, 4));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5),
/// ]);
///
/// // inclusive and with a single step count
/// let it = grid_space((0.0, 0.0)..=(1.0, 2.0), 3);
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 1.0), (0.0, 2.0),
///     (0.5, 0.0), (0.5, 1.0), (0.5, 2.0),
///     (1.0, 0.0), (1.0, 1.0), (1.0, 2.0),
/// ]);
///
/// // even 3d spaces
/// let it = grid_space((0, 0, 0)..=(1, 1, 1), 2);
/// itertools::assert_equal(it, vec![
///     (0, 0, 0), (0, 0, 1),
///     (0, 1, 0), (0, 1, 1),
///
///     (1, 0, 0), (1, 0, 1),
///     (1, 1, 0), (1, 1, 1),
/// ]);
/// ```
pub fn grid_space<R, S>(range: R, size: S) -> <R as IntoGridSpace<S>>::GridSpace
where
    R: IntoGridSpace<S>,
{
    range.into_grid_space(size)
}

pub trait IntoGridSpace<S> {
    type GridSpace;
    fn into_grid_space(self, size: S) -> Self::GridSpace;
}

use crate::{LinSpace, Linear, grid::{Grid2, Grid3, Grid4, Transpose, grid}, lin_space};
use std::ops::{Range, RangeInclusive};

macro_rules! impl_grid_space {
    ($Grid:ident: $($t:ident;$u:ty: $r:ident;$s:ident),*) => {

impl<$($t),*> IntoGridSpace<($($u),*)> for Range<($($t),*)> where $($t: Linear),* {
    type GridSpace = $Grid<$(LinSpace<$t>),*>;

    fn into_grid_space(self, ($($s),*): ($($u),*)) -> Self::GridSpace {
        let ($($r),*) = self.transpose();
        grid(($(
            lin_space($r, $s),
        )*))
    }
}

impl<$($t),*> IntoGridSpace<usize> for Range<($($t),*)> where $($t: Linear),* {
    type GridSpace = $Grid<$(LinSpace<$t>),*>;

    fn into_grid_space(self, s: usize) -> Self::GridSpace {
        $(
            let $s = s;
        )*
        self.into_grid_space(($($s),*))
    }
}
impl<$($t),*> IntoGridSpace<($($u),*)> for RangeInclusive<($($t),*)> where $($t: Linear),* {
    type GridSpace = $Grid<$(LinSpace<$t>),*>;

    fn into_grid_space(self, ($($s),*): ($($u),*)) -> Self::GridSpace {
        let ($($r),*) = self.transpose();
        grid(($(
            lin_space($r, $s),
        )*))
    }
}

impl<$($t),*> IntoGridSpace<usize> for RangeInclusive<($($t),*)> where $($t: Linear),* {
    type GridSpace = $Grid<$(LinSpace<$t>),*>;

    fn into_grid_space(self, s: usize) -> Self::GridSpace {
        $(
            let $s = s;
        )*
        self.into_grid_space(($($s),*))
    }
}

    };
}

impl_grid_space!(Grid2: T0;usize: r0;s0, T1;usize: r1;s1); // 2d grid
impl_grid_space!(Grid3: T0;usize: r0;s0, T1;usize: r1;s1, T2;usize: r2;s2); // 3d grid
impl_grid_space!(Grid4: T0;usize: r0;s0, T1;usize: r1;s1, T2;usize: r2;s2, T3;usize: r3;s3); // 4d grid
