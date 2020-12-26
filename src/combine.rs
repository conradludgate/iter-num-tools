//! Combines iterators over `((A, B), C)` items into `(A, B, C)`.
//! Used by [Grid](crate::grid)

use crate::map::{Map, Function};

pub type Combine<I> = Map<I, CombineFn>;

pub fn combine<I>(iter: I) -> Combine<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    Map::new(iter, CombineFn)
}

pub struct CombineFn;

impl<A, B, C> Function<((A, B), C)> for CombineFn {
    type Output = (A, B, C);
    #[inline]
    fn call(&self, ((a, b), c): ((A, B), C)) -> Self::Output {
        (a, b, c)
    }
}

impl<A, B, C, D> Function<((A, B, C), D)> for CombineFn {
    type Output = (A, B, C, D);
    #[inline]
    fn call(&self, ((a, b, c), d): ((A, B, C), D)) -> Self::Output {
        (a, b, c, d)
    }
}

#[cfg(test)]
mod tests {
    use super::combine;

    #[test]
    fn test_combine() {
        let it = combine(vec![
            ((0, 1), 2),
            ((3, 4), 5),
        ]);

        assert!(it.eq(vec![
            (0, 1, 2),
            (3, 4, 5),
        ]));
    }
}
