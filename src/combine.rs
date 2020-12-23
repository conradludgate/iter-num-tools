pub struct Combine<I>(I);

impl<I> Combine<I> {
    pub fn new(i: impl IntoIterator<IntoIter=I>) -> Self {
        Combine(i.into_iter())
    }
}

impl<I, A, B, C> Iterator for Combine<I>
where
    I: Iterator<Item = ((A, B), C)>,
{
    type Item = (A, B, C);
    fn next(&mut self) -> Option<Self::Item> {
        let ((a, b), c) = self.0.next()?;
        Some((a, b, c))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }
}
