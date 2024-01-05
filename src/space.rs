use core::iter::FusedIterator;
use core::ops::{Bound, Range, RangeBounds, RangeInclusive};

pub trait Interpolate: Sized {
    type Item;
    fn interpolate(self, x: usize) -> Self::Item;

    /// Some interpolations don't have correct interpolation points for the end bounds
    /// calculation. Notably the grid-based interpolations. This is a hack to provide a fix for them.
    fn interpolate_exclusive_end(self, x: usize) -> Self::Item {
        self.interpolate(x)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IntoSpace<I, R> {
    pub(crate) interpolate: I,
    pub(crate) range: R,
}

impl<I, R> IntoSpace<I, R> {
    pub fn new(interpolate: I, range: R) -> Self {
        IntoSpace { interpolate, range }
    }

    pub(crate) fn map<J>(self, f: impl FnOnce(I) -> J) -> IntoSpace<J, R> {
        IntoSpace {
            interpolate: f(self.interpolate),
            range: self.range,
        }
    }
}

impl<I> IntoSpace<I, Range<usize>> {
    pub(crate) fn new_exclusive(steps: usize, interpolate: I) -> Self {
        let range = 0..steps;
        IntoSpace { interpolate, range }
    }
}

impl<I> IntoSpace<I, RangeInclusive<usize>> {
    pub(crate) fn new_inclusive(steps: usize, interpolate: I) -> Self {
        let mut range = 0..=steps;
        // trim the end
        let _ = range.next_back();

        IntoSpace { interpolate, range }
    }
}

impl<I, R: IntoIterator<Item = usize>> IntoSpace<I, R> {
    pub fn into_space(self) -> Space<I, R::IntoIter> {
        Space::new(self.interpolate, self.range.into_iter())
    }
}

impl<I: Interpolate + Copy, R: IntoIterator<Item = usize>> IntoIterator for IntoSpace<I, R> {
    type Item = I::Item;
    type IntoIter = Space<I, R::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_space()
    }
}

#[derive(Clone, Debug)]
pub struct Space<I, R> {
    interpolate: I,
    range: R,
}

impl<I, R> Space<I, R> {
    pub fn new(interpolate: I, range: R) -> Self {
        Space { interpolate, range }
    }
}

impl<I: Interpolate + Copy, R: RangeBounds<usize>> Space<I, R> {
    pub fn start_bound(&self) -> Bound<I::Item> {
        match self.range.start_bound() {
            Bound::Included(i) => Bound::Included(self.interpolate.interpolate(*i)),
            Bound::Excluded(i) => Bound::Excluded(self.interpolate.interpolate(*i)),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    pub fn end_bound(&self) -> Bound<I::Item> {
        match self.range.end_bound() {
            Bound::Included(i) => Bound::Included(self.interpolate.interpolate(*i)),
            Bound::Excluded(i) => Bound::Excluded(self.interpolate.interpolate_exclusive_end(*i)),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    pub fn bounds(&self) -> (Bound<I::Item>, Bound<I::Item>) {
        (self.start_bound(), self.end_bound())
    }
}

impl<I: Interpolate + Copy, R: Iterator<Item = usize>> Iterator for Space<I, R> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|x| self.interpolate.interpolate(x))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.range.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.range.last().map(|x| self.interpolate.interpolate(x))
    }

    #[cfg(feature = "iter_advance_by")]
    fn advance_by(&mut self, n: usize) -> Result<(), core::num::NonZeroUsize> {
        self.range.advance_by(n)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.range.nth(n).map(|x| self.interpolate.interpolate(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<I: Interpolate + Copy, R: DoubleEndedIterator<Item = usize>> DoubleEndedIterator
    for Space<I, R>
{
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

impl<I: Interpolate + Copy, R: ExactSizeIterator<Item = usize>> ExactSizeIterator for Space<I, R> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<I: Interpolate + Copy, R: FusedIterator<Item = usize>> FusedIterator for Space<I, R> {}

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<I: Interpolate + Copy, R: TrustedLen<Item = usize>> TrustedLen for Space<I, R> {}
