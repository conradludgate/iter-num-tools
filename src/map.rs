//! Provides a generic Map iterator that is similar but easier to create type signatures for than [std::iter::Map].

use std::iter::FusedIterator;

/// Function is a generic trait for a Fn(T)->O
pub trait Function<T> {
    type Output;
    fn call(&self, x: T) -> Self::Output;
}

impl<F, T, O> Function<T> for F
where
    F: Fn(T) -> O,
{
    type Output = O;

    #[inline]
    fn call(&self, x: T) -> Self::Output {
        self(x)
    }
}

#[derive(Clone, Copy)]
/// Works a lot like [Map](std::iter::Map)
/// but accepts a non Fn(T)->O type,
/// Instead, opting to use the trait [Function]
/// for the mapping function. This makes creating types
/// using map a lot easier
pub struct Map<I, F> {
    i: I,
    f: F,
}

impl<I, F> Map<I, F>
where
    I: Iterator,
{
    pub fn new(i: impl IntoIterator<Item = I::Item, IntoIter = I>, f: F) -> Self {
        Map {
            i: i.into_iter(),
            f,
        }
    }
}

impl<I, F> Iterator for Map<I, F>
where
    I: Iterator,
    F: Function<I::Item>,
{
    type Item = F::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.f.call(self.i.next()?))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.i.size_hint()
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.i.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        Some(self.f.call(self.i.last()?))
    }
}

impl<I, F> DoubleEndedIterator for Map<I, F>
where
    I: DoubleEndedIterator,
    F: Function<I::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.f.call(self.i.next_back()?))
    }
}

impl<I, F> ExactSizeIterator for Map<I, F>
where
    I: ExactSizeIterator,
    F: Function<I::Item>,
{
    fn len(&self) -> usize {
        self.i.len()
    }
}

impl<I, F> FusedIterator for Map<I, F>
where
    I: FusedIterator,
    F: Function<I::Item>,
{
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_iter() {
        let c: Vec<_> = Map::new(0..5, |x| 2 * x).collect();
        assert_eq!(c, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_count() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq!(it.count(), 5);
    }

    #[test]
    fn test_last() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq!(it.last(), Some(8));
    }

    #[test]
    fn test_reverse() {
        let it = Map::new(0..5, |x| 2 * x);
        assert!(it.rev().eq(vec![8, 6, 4, 2, 0]));
    }

    #[test]
    fn test_len() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq!(it.len(), 5);
    }
}
