use core::iter::FusedIterator;
use core::ops::Range;

pub trait Interpolate {
    type Item;
    fn interpolate(&self, x: usize) -> Self::Item;
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

impl<I: Interpolate> Iterator for Space<I> {
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

    #[cfg(feature = "advanced_by")]
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        self.range.advanced_by(n)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.range.nth(n).map(|x| self.interpolate.interpolate(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<I: Interpolate> DoubleEndedIterator for Space<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|x| self.interpolate.interpolate(x))
    }
}

impl<I: Interpolate> ExactSizeIterator for Space<I> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<I: Interpolate> FusedIterator for Space<I> {}

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<I: Interpolate> TrustedLen for Space<I> {}
