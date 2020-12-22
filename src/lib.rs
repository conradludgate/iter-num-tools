#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use num_traits::{Float, FromPrimitive, ToPrimitive};
use std::ops::{Add, AddAssign, Div, Mul, Range, RangeInclusive, Sub};

pub fn lerp_fn<T>(from: RangeInclusive<T>, to: RangeInclusive<T>) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    let x0 = *from.start();
    let x1 = *from.end();
    let y0 = *to.start();
    let y1 = *to.end();

    move |x: T| ((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0)
}

pub fn lerp_usize_fn<T>(from: RangeInclusive<usize>, to: RangeInclusive<T>) -> impl Fn(usize) -> T
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    let x0 = T::from_usize(*from.start()).unwrap();
    let x1 = T::from_usize(*from.end()).unwrap();
    let f = lerp_fn(x0..=x1, to);

    move |x: usize| f(T::from_usize(x).unwrap())
}

pub fn lerp_usize_iter<T>(
    from: RangeInclusive<usize>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = usize>,
) -> impl Iterator<Item = T>
where
    T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    over.into_iter().map(lerp_usize_fn(from, to))
}

pub fn lerp_iter<T>(
    from: RangeInclusive<T>,
    to: RangeInclusive<T>,
    over: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T> + Copy,
{
    over.into_iter().map(lerp_fn(from, to))
}

pub struct Arange<T> {
    end: T,
    step: T,
    current: T,
}

impl<T> Arange<T> {
    pub fn new(range: Range<T>, step: T) -> Self {
        let Range {
            start: current,
            end,
        } = range;
        Arange { end, step, current }
    }
}

impl<F> Iterator for Arange<F>
where
    F: Float + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if current < self.end {
            self.current += self.step;
            Some(current)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let steps = (self.end - self.current) / self.step;
        let lower = steps.floor().to_usize().unwrap_or(0);
        let upper = steps.ceil().to_usize();
        (lower, upper)
    }
}
