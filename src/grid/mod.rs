//! Some convenient traits and functions for process grid based iterators

use crate::combine::Combine;
use itertools::Product;
mod transpose;
mod tuple;

/// Create a grid over a tuple of iterators
pub fn grid<G: IntoGrid>(g: G) -> G::Grid {
    g.into_grid()
}

/// Trait used by [grid]
pub trait IntoGrid {
    type Grid;
    fn into_grid(self) -> Self::Grid;
}

/// 2D Grid Iterator
pub type Grid2<I1, I2> = Product<I1, I2>;
/// 3D Grid Iterator
pub type Grid3<I1, I2, I3> = Combine<Product<Grid2<I1, I2>, I3>>;
/// 4D Grid Iterator
pub type Grid4<I1, I2, I3, I4> = Combine<Product<Grid3<I1, I2, I3>, I4>>;

/// Trait for Transpose
/// Used by [Range](std::ops::Range) to convert `Range<(A, B)>` into `(Range<A>, Range<B>)` for example
pub trait Transpose {
    type Output;
    fn transpose(self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid2() {
        let it = grid((0..2, 2..4));

        assert_eq_iter!(it, [(0, 2), (0, 3), (1, 2), (1, 3)]);
    }

    #[test]
    fn test_grid3() {
        let it = grid((0..2, 2..4, 4..6));

        assert_eq_iter!(
            it,
            [
                (0, 2, 4),
                (0, 2, 5),
                (0, 3, 4),
                (0, 3, 5),
                (1, 2, 4),
                (1, 2, 5),
                (1, 3, 4),
                (1, 3, 5)
            ]
        );
    }

    #[test]
    fn test_grid4() {
        let it = grid((0..2, 2..4, 4..6, 6..8));

        assert_eq_iter!(
            it,
            [
                (0, 2, 4, 6),
                (0, 2, 4, 7),
                (0, 2, 5, 6),
                (0, 2, 5, 7),
                (0, 3, 4, 6),
                (0, 3, 4, 7),
                (0, 3, 5, 6),
                (0, 3, 5, 7),
                (1, 2, 4, 6),
                (1, 2, 4, 7),
                (1, 2, 5, 6),
                (1, 2, 5, 7),
                (1, 3, 4, 6),
                (1, 3, 4, 7),
                (1, 3, 5, 6),
                (1, 3, 5, 7)
            ]
        );
    }
}
