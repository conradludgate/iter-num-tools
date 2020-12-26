//! Some convenient traits and functions for process grid based iterators

use crate::combine::Combine;
use itertools::Product;
mod tuple;
mod transpose;

/// Create a grid over a tuple of iterators
pub fn grid<G: IntoGrid>(g: G) -> G::Grid {
    g.into_grid()
}

pub trait IntoGrid {
    type Grid;
    fn into_grid(self) -> Self::Grid;
}

pub type Grid2<I1, I2> = Product<I1, I2>;
pub type Grid3<I1, I2, I3> = Combine<Product<Grid2<I1, I2>, I3>>;
pub type Grid4<I1, I2, I3, I4> = Combine<Product<Grid3<I1, I2, I3>, I4>>;

pub trait Transpose {
    type Output;
    fn transpose(self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid2() {
        let it = grid((
            vec![0, 1].into_iter(),
            vec![2, 3].into_iter(),
        ));

        assert!(it.eq(vec![
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
        ]));
    }

    #[test]
    fn test_grid3() {
        let it = grid((
            vec![0, 1].into_iter(),
            vec![2, 3].into_iter(),
            vec![4, 5].into_iter(),
        ));

        assert!(it.eq(vec![
            (0, 2, 4),
            (0, 2, 5),
            (0, 3, 4),
            (0, 3, 5),
            (1, 2, 4),
            (1, 2, 5),
            (1, 3, 4),
            (1, 3, 5),
        ]));
    }

    #[test]
    fn test_grid4() {
        let it = grid((
            vec![0, 1].into_iter(),
            vec![2, 3].into_iter(),
            vec![4, 5].into_iter(),
            vec![6, 7].into_iter(),
        ));

        assert!(it.eq(vec![
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
            (1, 3, 5, 7),
        ]));
    }
}
