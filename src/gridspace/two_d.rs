use super::IntoGridSpace;
use crate::{lin_space, LinSpace, Linear};
use itertools::{Itertools, Product};
use std::ops::{Range, RangeInclusive};

pub(crate) type Grid2<T1, T2> = Product<LinSpace<T1>, LinSpace<T2>>;

// Implements IntoGridSpace for (w0, h0)..=(w1, h1) with control over both width and height step counts
impl<T1, T2> IntoGridSpace<(usize, usize)> for RangeInclusive<(T1, T2)>
where
    T1: Linear,
    T2: Linear,
{
    type GridSpace = Grid2<T1, T2>;
    fn into_grid_space(self, (w, h): (usize, usize)) -> Self::GridSpace {
        let ((w0, h0), (w1, h1)) = self.into_inner();

        let wl = lin_space(w0..=w1, w);
        let hl = lin_space(h0..=h1, h);
        wl.cartesian_product(hl)
    }
}

// Implements IntoGridSpace for (w0, h0)..=(w1, h1) with the same width and height step count
impl<T1, T2> IntoGridSpace<usize> for RangeInclusive<(T1, T2)>
where
    T1: Linear,
    T2: Linear,
{
    type GridSpace = Grid2<T1, T2>;
    fn into_grid_space(self, steps: usize) -> Self::GridSpace {
        self.into_grid_space((steps, steps))
    }
}

// Implements IntoGridSpace for (w0, h1)..(w1, h1) with control over both width and height step counts
impl<T1, T2> IntoGridSpace<(usize, usize)> for Range<(T1, T2)>
where
    T1: Linear,
    T2: Linear,
{
    type GridSpace = Grid2<T1, T2>;
    fn into_grid_space(self, (w, h): (usize, usize)) -> Self::GridSpace {
        let Range {
            start: (w0, h0),
            end: (w1, h1),
        } = self;

        let wl = lin_space(w0..w1, w);
        let hl = lin_space(h0..h1, h);
        wl.cartesian_product(hl)
    }
}

// Implements IntoGridSpace for (w0, h0)..(w1, h1) with the same width and height step count
impl<T1, T2> IntoGridSpace<usize> for Range<(T1, T2)>
where
    T1: Linear,
    T2: Linear,
{
    type GridSpace = Grid2<T1, T2>;
    fn into_grid_space(self, steps: usize) -> Self::GridSpace {
        self.into_grid_space((steps, steps))
    }
}
