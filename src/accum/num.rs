use super::{Sum2, Product2};
use core::num::Wrapping;

macro_rules! impl_sum_product {
    ($zero:expr, $one:expr, $($a:ty)*) => ($(
        impl Sum2 for $a {
            type Output = $a;
            fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
                iter.into_iter().fold($zero, |a, b| a + b)
            }
        }

        impl Product2 for $a {
            type Output = $a;
            fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
                iter.into_iter().fold($one, |a, b| a * b)
            }
        }

        impl<'a> Sum2 for &'a $a {
            type Output = $a;
            fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
                iter.into_iter().fold($zero, |a, b| a + b)
            }
        }

        impl<'a> Product2 for &'a $a {
            type Output = $a;
            fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
                iter.into_iter().fold($one, |a, b| a * b)
            }
        }
    )*);
}

macro_rules! integer_sum_product {
    ($($a:ty)*) => (
        impl_sum_product!(0, 1, $($a)*);
        impl_sum_product!(Wrapping(0), Wrapping(1), $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum_product {
    ($($a:ty)*) => (
        impl_sum_product!(0.0, 1.0, $($a)*);
    );
}

integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum_product! { f32 f64 }
