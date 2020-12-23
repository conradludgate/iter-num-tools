use super::{IntoGridSpace, two_d::Grid2};
use crate::{combine::Combine, lin_space, LinSpace, Linear};
use itertools::{Itertools, Product};
use std::ops::{Range, RangeInclusive};

pub(crate) type Grid3<T1, T2, T3> = Combine<Product<Grid2<T1, T2>, LinSpace<T3>>>;

// Implements IntoGridSpace for (w0, h0, d0)..=(w1, h1, d1) with control over both width, height and depth step counts
impl<T1, T2, T3> IntoGridSpace<(usize, usize, usize)> for RangeInclusive<(T1, T2, T3)>
where
    T1: Linear,
    T2: Linear,
    T3: Linear,
{
    type GridSpace = Grid3<T1, T2, T3>;
    fn into_grid_space(self, (w, h, d): (usize, usize, usize)) -> Self::GridSpace {
        let ((w0, h0, d0), (w1, h1, d1)) = self.into_inner();

        let first = ((w0, h0)..=(w1, h1)).into_grid_space((w, h));
        let second = lin_space(d0..=d1, d);
        Combine::new(first.cartesian_product(second))
    }
}

// Implements IntoGridSpace for (w0, h0, d0)..=(w1, h1, d1) with the same width, height and depth step count
impl<T1, T2, T3> IntoGridSpace<usize> for RangeInclusive<(T1, T2, T3)>
where
    T1: Linear,
    T2: Linear,
    T3: Linear,
{
    type GridSpace = Grid3<T1, T2, T3>;
    fn into_grid_space(self, steps: usize) -> Self::GridSpace {
        self.into_grid_space((steps, steps, steps))
    }
}

// Implements IntoGridSpace for (w0, h0, d0)..(w1, h1, d1) with control over both width, height and depth step counts
impl<T1, T2, T3> IntoGridSpace<(usize, usize, usize)> for Range<(T1, T2, T3)>
where
    T1: Linear,
    T2: Linear,
    T3: Linear,
{
    type GridSpace = Grid3<T1, T2, T3>;
    fn into_grid_space(self, (w, h, d): (usize, usize, usize)) -> Self::GridSpace {
        let Range {
            start: (w0, h0, d0),
            end: (w1, h1, d1),
        } = self;

        let first = ((w0, h0)..(w1, h1)).into_grid_space((w, h));
        let second = lin_space(d0..d1, d);
        Combine::new(first.cartesian_product(second))
    }
}

// Implements IntoGridSpace for (w0, h0, d0)..(w1, h1, d1) with the same width, height and depth step count
impl<T1, T2, T3> IntoGridSpace<usize> for Range<(T1, T2, T3)>
where
    T1: Linear,
    T2: Linear,
    T3: Linear,
{
    type GridSpace = Grid3<T1, T2, T3>;
    fn into_grid_space(self, steps: usize) -> Self::GridSpace {
        self.into_grid_space((steps, steps, steps))
    }
}
