use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

/// Wraps the given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// ```
/// use iter_num_tools::lerp_iter;
/// use itertools::Itertools;
///
/// let it = lerp_iter(0.0..=2.0, 20.0..=21.0, vec![-1.0, 1.0]);
/// itertools::assert_equal(it, vec![19.5, 20.5]);
/// ```
pub fn lerp_iter<T, I>(
    from: RangeInclusive<T>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = T, IntoIter = I>,
) -> LerpIter<T, I>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    LerpIter::new(from, to, over)
}

/// Wraps the given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// Similar to [lerp_iter] but handles the conversion from usize
///
/// ```
/// use iter_num_tools::lerp_usize_iter;
/// use itertools::Itertools;
///
/// let it = lerp_usize_iter(0..=2, 20.0..=21.0, 0..5);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0, 21.5, 22.0]);
/// ```
pub fn lerp_usize_iter<T, I>(
    from: RangeInclusive<usize>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = usize, IntoIter = I>,
) -> LerpIterUsize<T, I>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    LerpIterUsize::new(from, to, over)
}
#[derive(Clone)]
pub(crate) struct Lerp<T> {
    x0: T,
    x1: T,
    y0: T,
    y1: T,
}

impl<T> Lerp<T>
where
    T: FromPrimitive + Copy,
{
    pub(crate) fn new_usize(from: RangeInclusive<usize>, to: RangeInclusive<T>) -> Self {
        Lerp {
            x0: T::from_usize(*from.start()).unwrap(),
            x1: T::from_usize(*from.end()).unwrap(),
            y0: *to.start(),
            y1: *to.end(),
        }
    }
}

impl<T> Lerp<T>
where
    T: Copy,
{
    pub(crate) fn new(from: RangeInclusive<T>, to: RangeInclusive<T>) -> Self {
        Lerp {
            x0: *from.start(),
            x1: *from.end(),
            y0: *to.start(),
            y1: *to.end(),
        }
    }
}

impl<T> Lerp<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    fn lerp(&self, x: T) -> T {
        let x0 = self.x0;
        let x1 = self.x1;
        let y0 = self.y0;
        let y1 = self.y1;

        ((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0)
    }
}

#[derive(Clone)]
pub struct LerpIter<T, I> {
    lerp: Lerp<T>,
    over: I,
}

impl<T, I> LerpIter<T, I> {
    pub fn new(
        from: RangeInclusive<T>,
        to: RangeInclusive<T>,
        over: impl IntoIterator<Item = T, IntoIter = I>,
    ) -> Self
    where
        T: Copy,
    {
        LerpIter {
            lerp: Lerp::new(from, to),
            over: over.into_iter(),
        }
    }
}

impl<T, I> Iterator for LerpIter<T, I>
where
    I: Iterator<Item = T>,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.over.next()?;
        Some(self.lerp.lerp(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.over.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.over.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let LerpIter { lerp, over } = self;
        let x = over.last()?;
        Some(lerp.lerp(x))
    }
}

#[derive(Clone)]
pub struct LerpIterUsize<T, I> {
    lerp: Lerp<T>,
    over: I,
}

impl<T, I> LerpIterUsize<T, I>
where
    T: FromPrimitive + Copy,
{
    pub fn new(
        from: RangeInclusive<usize>,
        to: RangeInclusive<T>,
        over: impl IntoIterator<Item = usize, IntoIter = I>,
    ) -> Self {
        LerpIterUsize {
            lerp: Lerp::new_usize(from, to),
            over: over.into_iter(),
        }
    }
}

impl<T, I> Iterator for LerpIterUsize<T, I>
where
    I: Iterator<Item = usize>,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = T::from_usize(self.over.next()?).unwrap();
        Some(self.lerp.lerp(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.over.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.over.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let LerpIterUsize { lerp, over } = self;
        let x = T::from_usize(over.last()?).unwrap();
        Some(lerp.lerp(x))
    }
}
