mod lerp;
mod arange;

pub use lerp::*;
pub use arange::*;

use itertools::Itertools;
pub fn grid((w, h): (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
    (0..w).cartesian_product(0..h)
}
