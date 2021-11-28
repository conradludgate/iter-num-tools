mod num;
mod result;

pub trait Sum2 {
    type Output;
    fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}

pub trait Product2 {
    type Output;
    fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}
