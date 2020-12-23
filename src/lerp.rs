use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

#[derive(Copy, Clone)]
pub struct Lerp<T> {
    x0: T,
    x1: T,
    y0: T,
    y1: T,
}

impl<T> Lerp<T>
where
{
    pub fn new(from: RangeInclusive<T>, to: RangeInclusive<T>) -> Self {
        let (x0, x1) = from.into_inner();
        let (y0, y1) = to.into_inner();
        Lerp { x0, x1, y0, y1 }
    }
}

macro_rules! LerpPrimitive {
    ($($name:ident; $t:ty; $from:ident),*) => {

impl<T> Lerp<T>
where
    T: FromPrimitive,
{
    $(

    pub fn $name(from: RangeInclusive<$t>, to: RangeInclusive<T>) -> Self {
        let (x0, x1) = from.into_inner();
        let (y0, y1) = to.into_inner();
        Lerp {
            x0: T::$from(x0).unwrap(),
            x1: T::$from(x1).unwrap(),
            y0,
            y1,
        }
    }

    )*
}
    };
}

LerpPrimitive![
    new_usize; usize; from_usize,
    new_u128; u128; from_u128,
    new_u64; u64; from_u64,
    new_u32; u32; from_u32,
    new_u8; u8; from_u8,

    new_isize; isize; from_isize,
    new_i128; i128; from_i128,
    new_i64; i64; from_i64,
    new_i32; i32; from_i32,
    new_i8; i8; from_i8,

    new_f32; f32; from_f32,
    new_f64; f64; from_f64
];

impl<T> Lerp<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    fn lerp(&self, x: T) -> T {
        let Lerp { x0, x1, y0, y1 } = *self;

        ((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0)
    }
}

/// Wraps a given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// ```
/// use iter_num_tools::lerp::LerpIter;
/// use itertools::Itertools;
///
/// let it = LerpIter::new(0.0..=2.0, 20.0..=21.0, vec![-1.0, 1.0]);
/// itertools::assert_equal(it, vec![19.5, 20.5]);
/// ```
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

/// Wraps a given iterator and maps each value through a
/// linear interpolation from the first range to the second
///
/// Similar to [LerpIter] but handles conversions from primitive types
///
/// You probably want to look at [crate::lin_space] instead for a more
/// convenient usage
///
/// ```
/// use std::ops::Range;
/// use iter_num_tools::lerp::LerpIterPrim;
/// use itertools::Itertools;
///
/// let it = LerpIterPrim::<f64, Range<usize>, _>::new(0..=2, 20.0..=21.0, 0..5);
/// itertools::assert_equal(it, vec![20.0, 20.5, 21.0, 21.5, 22.0]);
/// ```
#[derive(Clone)]
pub struct LerpIterPrim<T, I, P>
where
    I: Iterator<Item = P>,
{
    lerp: Lerp<T>,
    over: I,
}

macro_rules! LerpIterPrimitive {
    ($($new_lerp:ident; $t:ty; $from:ident),*) => {

$(

impl<T, I> LerpIterPrim<T, I, $t>
where
    T: FromPrimitive + Copy,
    I: Iterator<Item = $t>
{
    pub fn new(
        from: RangeInclusive<$t>,
        to: RangeInclusive<T>,
        over: impl IntoIterator<Item = usize, IntoIter = I>,
    ) -> Self {
        LerpIterPrim {
            lerp: Lerp::$new_lerp(from, to),
            over: over.into_iter(),
        }
    }
}

impl<T, I> Iterator for LerpIterPrim<T, I, $t>
where
    I: Iterator<Item = $t>,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = T::$from(self.over.next()?).unwrap();
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
        let LerpIterPrim { lerp, over } = self;
        let x = T::$from(over.last()?).unwrap();
        Some(lerp.lerp(x))
    }
}

)*

    }
}

LerpIterPrimitive![
    new_usize; usize; from_usize,
    new_u128; u128; from_u128,
    new_u64; u64; from_u64,
    new_u32; u32; from_u32,
    new_u8; u8; from_u8,

    new_isize; isize; from_isize,
    new_i128; i128; from_i128,
    new_i64; i64; from_i64,
    new_i32; i32; from_i32,
    new_i8; i8; from_i8,

    new_f32; f32; from_f32,
    new_f64; f64; from_f64
];
