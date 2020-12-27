//! Combines iterators over `((A, B), C)` items into `(A, B, C)`.
//! Used by [Grid](crate::grid)

use crate::map::{Map, Function};

/// Iterator over combined tuples
pub type Combine<I> = Map<I, CombineFn>;

/// Converts an iterator over nested tuples into an iterator over a single tuple
pub fn combine<I>(iter: I) -> Combine<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    Map::new(iter, CombineFn)
}

/// Combines nested tuples into a single tuple
pub struct CombineFn;

/// Converts ((A, B), C) into (A, B, C)
impl<A, B, C> Function<((A, B), C)> for CombineFn {
    type Output = (A, B, C);
    #[inline]
    fn call(&self, ((a, b), c): ((A, B), C)) -> Self::Output {
        (a, b, c)
    }
}

/// Converts ((A, B, C), D) into (A, B, C, D)
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
        // iterator that returns [((0, 1), 2), ((3, 4), 5)]
        let i = (0..6).step_by(3).map(|i| ((i, i+1), i+2));

        let it = combine(i);
        assert_eq_iter!(it, [
            (0, 1, 2),
            (3, 4, 5)
        ]);
    }
}
