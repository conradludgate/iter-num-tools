use crate::accum::{Sum2, Product2};

pub trait IterAdapter: Iterator + Sized {
    fn sum2(self) -> <Self::Item as Sum2>::Output
    where
        Self::Item: Sum2,
    {
        <Self::Item as Sum2>::sum2(self)
    }

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
    fn main() {
        let x: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        println!("{:?}", x.iter().sum2());
        println!("{:?}", x.into_iter().sum2());

        let x: Vec<Option<f32>> = vec![Some(1.0), Some(2.0), None, Some(4.0)];
        println!("{:?}", x.iter().map(Option::as_ref).sum2());
        println!("{:?}", x.into_iter().sum2());

        let x: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        println!("{:?}", x.iter().product2());
        println!("{:?}", x.into_iter().product2());
    }
}

