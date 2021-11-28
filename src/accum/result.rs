use super::{Product2, Sum2};
use crate::adapter::IterAdapter;

impl<T, E> Sum2 for Result<T, E>
where
    T: Sum2,
{
    type Output = Result<<T as Sum2>::Output, E>;
    fn sum2(iter: impl IntoIterator<Item = Self>) -> Result<<T as Sum2>::Output, E> {
        process_results(iter.into_iter(), |i| T::sum2(i))
    }
}

impl<T, E> Product2 for Result<T, E>
where
    T: Product2,
{
    type Output = Result<<T as Product2>::Output, E>;
    fn product2(iter: impl IntoIterator<Item = Self>) -> Result<<T as Product2>::Output, E> {
        process_results(iter.into_iter(), |i| T::product2(i))
    }
}

impl<T> Sum2 for Option<T>
where
    T: Sum2,
{
    type Output = Option<T::Output>;
    fn sum2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
        iter.into_iter().map(|x| x.ok_or(())).sum2().ok()
    }
}

impl<T> Product2 for Option<T>
where
    T: Product2,
{
    type Output = Option<T::Output>;
    fn product2(iter: impl IntoIterator<Item = Self>) -> Self::Output {
        iter.into_iter().map(|x| x.ok_or(())).product2().ok()
    }
}

// the following code was borrowed very kindly from rustc
// https://github.com/rust-lang/rust/blob/2ee06e737208ce1bd1c18df5ea3aba733e6ac2a7/library/core/src/iter/adapters/mod.rs#L142

/// An iterator adapter that produces output as long as the underlying
/// iterator produces `Result::Ok` values.
///
/// If an error is encountered, the iterator stops and the error is
/// stored.
pub(crate) struct ResultShunt<'a, I, E> {
    iter: I,
    error: &'a mut Result<(), E>,
}

/// Process the given iterator as if it yielded a `T` instead of a
/// `Result<T, _>`. Any errors will stop the inner iterator and
/// the overall result will be an error.
pub(crate) fn process_results<I, T, E, F, U>(iter: I, mut f: F) -> Result<U, E>
where
    I: Iterator<Item = Result<T, E>>,
    for<'a> F: FnMut(ResultShunt<'a, I, E>) -> U,
{
    let mut error = Ok(());
    let shunt = ResultShunt {
        iter,
        error: &mut error,
    };
    let value = f(shunt);
    error.map(|()| value)
}

impl<I, T, E> Iterator for ResultShunt<'_, I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.error.is_err() {
            (0, Some(0))
        } else {
            let (_, upper) = self.iter.size_hint();
            (0, upper)
        }
    }

    fn fold<B, F>(self, init: B, mut fold: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let error = &mut *self.error;
        let mut accum = init;
        for x in self.iter {
            match x {
                Ok(x) => accum = fold(accum, x),
                Err(e) => {
                    *error = Err(e);
                    break;
                }
            }
        }
        accum
    }
}
