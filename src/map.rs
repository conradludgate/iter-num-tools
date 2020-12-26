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

impl<I, F> Map<I, F> {
    pub fn new(i: impl IntoIterator<IntoIter = I>, f: F) -> Self {
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
