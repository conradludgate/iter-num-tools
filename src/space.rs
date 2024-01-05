use core::iter::FusedIterator;
use core::ops::Range;

pub trait Interpolate {
    type Item;
    fn interpolate(self, x: usize) -> Self::Item;
}

#[derive(Clone, Copy, Debug)]
pub struct IntoSpace<I> {
    pub interpolate: I,
    pub len: usize,
}

impl<I> IntoSpace<I> {
    pub fn new(len: usize, interpolate: I) -> Self {
        IntoSpace { interpolate, len }
    }
    pub fn into_space(self) -> Space<I> {
        Space::new(self.len, self.interpolate)
    }
}

impl<I: Interpolate + Copy> IntoIterator for IntoSpace<I> {
    type Item = I::Item;
    type IntoIter = Space<I>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_space()
    }
}

#[derive(Clone, Debug)]
pub struct Space<I> {
    interpolate: I,
    range: Range<usize>,
}

impl<I> Space<I> {
    pub fn new(len: usize, interpolate: I) -> Self {
        Space {
            interpolate,
            range: 0..len,
        }
    }
}

impl<I: Interpolate + Copy> Iterator for Space<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|x| self.interpolate.interpolate(x))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }

    #[cfg(feature = "iter_advance_by")]
    fn advance_by(&mut self, n: usize) -> Result<(), core::num::NonZeroUsize> {
        self.range.advance_by(n)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.range.nth(n).map(|x| self.interpolate.interpolate(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<I: Interpolate + Copy> DoubleEndedIterator for Space<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|x| self.interpolate.interpolate(x))
    }

    #[cfg(feature = "iter_advance_by")]
    fn advance_back_by(&mut self, n: usize) -> Result<(), core::num::NonZeroUsize> {
        self.range.advance_back_by(n)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.range
            .nth_back(n)
            .map(|x| self.interpolate.interpolate(x))
    }
}

impl<I: Interpolate + Copy> ExactSizeIterator for Space<I> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<I: Interpolate + Copy> FusedIterator for Space<I> {}

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<I: Interpolate + Copy> TrustedLen for Space<I> {}
