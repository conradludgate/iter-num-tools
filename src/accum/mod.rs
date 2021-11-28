mod num;
mod result;

/// Similar to [`std::iter::Sum`] but doesn't need turbofish to specify the output
pub trait Sum2 {
    type Output;
    fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}

/// Similar to [`std::iter::Product`] but doesn't need turbofish to specify the output
pub trait Product2 {
    type Output;
    fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}
