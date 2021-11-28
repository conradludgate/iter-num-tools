use crate::accum::{Product2, Sum2};

/// Adds a few extra methods to iterators
pub trait IterAdapter: Iterator + Sized {
    /// Sums the values in the iterator
    fn sum2(self) -> <Self::Item as Sum2>::Output
    where
        Self::Item: Sum2,
    {
        <Self::Item as Sum2>::sum2(self)
    }

    /// Multiplies the values in the iterator
    fn product2(self) -> <Self::Item as Product2>::Output
    where
        Self::Item: Product2,
    {
        <Self::Item as Product2>::product2(self)
    }
}

impl<I> IterAdapter for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::IterAdapter;

    #[test]
    fn sum_num() {
        let x = vec![1, 2, 3, 4];
        assert_eq!(x.iter().sum2(), 10); // by ref
        assert_eq!(x.into_iter().sum2(), 10); // by value
    }

    #[test]
    fn sum_opt() {
        let x = vec![Some(1), Some(2), Some(3), Some(4)];
        assert_eq!(x.into_iter().sum2(), Some(10)); // complete

        let y = vec![None, Some(2), Some(3), Some(4)];
        assert_eq!(y.into_iter().sum2(), None); // short-circuit
    }

    #[test]
    fn prod_num() {
        let x = vec![1, 2, 3, 4];
        assert_eq!(x.iter().product2(), 24); // by ref
        assert_eq!(x.into_iter().product2(), 24); // by value
    }

    #[test]
    fn prod_res() {
        let x: Vec<Result<_, ()>> = vec![Ok(1), Ok(2), Ok(3), Ok(4)];
        assert_eq!(x.into_iter().product2(), Ok(24)); // complete

        let y = vec![Err(()), Ok(2), Ok(3), Ok(4)];
        assert_eq!(y.into_iter().product2(), Err(())); // short-circuit
    }
}
