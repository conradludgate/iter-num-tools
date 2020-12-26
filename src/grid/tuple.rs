use super::{Grid2, Grid3, Grid4, IntoGrid};
use crate::combine::combine;
use itertools::Itertools;

impl<I1, I2> IntoGrid for (I1, I2)
where
    I1: Iterator + Itertools,
    <I1 as Iterator>::Item: Clone,
    I2: Iterator + Clone,
{
    type Grid = Grid2<I1, I2>;
    fn into_grid(self) -> Self::Grid {
        self.0.cartesian_product(self.1)
    }
}

impl<I1, I2, I3> IntoGrid for (I1, I2, I3)
where
    I1: Iterator + Itertools,
    <I1 as Iterator>::Item: Clone,
    I2: Iterator + Clone,
    <I2 as Iterator>::Item: Clone,
    I3: Iterator + Clone,
{
    type Grid = Grid3<I1, I2, I3>;
    fn into_grid(self) -> Self::Grid {
        combine((self.0, self.1).into_grid().cartesian_product(self.2))
    }
}

impl<I1, I2, I3, I4> IntoGrid for (I1, I2, I3, I4)
where
    I1: Iterator + Itertools,
    <I1 as Iterator>::Item: Clone,
    I2: Iterator + Clone,
    <I2 as Iterator>::Item: Clone,
    I3: Iterator + Clone,
    <I3 as Iterator>::Item: Clone,
    I4: Iterator + Clone,
{
    type Grid = Grid4<I1, I2, I3, I4>;
    fn into_grid(self) -> Self::Grid {
        combine(
            combine((self.0, self.1).into_grid().cartesian_product(self.2))
                .cartesian_product(self.3),
        )
    }
}
