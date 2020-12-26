//! Provides a generic Map iterator that is similar but easier to create type signatures for than [Map](std::iter::Map).

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
    F: Function<<I as Iterator>::Item>,
{
    type Item = <F as Function<<I as Iterator>::Item>>::Output;

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
}
