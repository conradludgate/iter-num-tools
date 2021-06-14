//! Provides a generic Map iterator that is similar but easier to create type signatures for than [std::iter::Map].

use core::iter::{FusedIterator, InPlaceIterable, SourceIter, TrustedLen};

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
    fn next(&mut self) -> Option<F::Output> {
        // self.f.call(self.i.next()?)
        match self.i.next() {
            Some(x) => Some(self.f.call(x)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.i.size_hint()
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
    #[inline]
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

unsafe impl<I, F> TrustedLen for Map<I, F>
where
    I: TrustedLen,
    F: Function<I::Item>,
{
}

unsafe impl<S: Iterator, I: Iterator, F> SourceIter for Map<I, F>
where
    F: Function<I::Item>,
    I: SourceIter<Source = S>,
{
    type Source = S;

    #[inline]
    unsafe fn as_inner(&mut self) -> &mut S {
        // SAFETY: unsafe function forwarding to unsafe function with the same requirements
        SourceIter::as_inner(&mut self.i)
    }
}

unsafe impl<I: InPlaceIterable, F> InPlaceIterable for Map<I, F> where F: Function<I::Item> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq_iter!(it, [0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_size_hint() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq!(it.size_hint(), (5, Some(5)));
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
        let it = Map::new(0..5, |x| 2 * x).rev();
        assert_eq_iter!(it, [8, 6, 4, 2, 0]);
    }

    #[test]
    fn test_len() {
        let it = Map::new(0..5, |x| 2 * x);
        assert_eq!(it.len(), 5);
    }
}
