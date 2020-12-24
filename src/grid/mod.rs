use crate::combine::Combine;
use itertools::Product;
mod tuple;
mod transpose;

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
