mod num;
mod result;

/// Similar to [`std::iter::Sum`] but doesn't need turbofish to specify the output
pub trait Sum2 {
    /// Output for the summation
    type Output;
    /// compute the sum over the iterator
    fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}

/// Similar to [`std::iter::Product`] but doesn't need turbofish to specify the output
pub trait Product2 {
    /// Output for the product
    type Output;
    /// compute the product over the iterator
    fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output;
}
