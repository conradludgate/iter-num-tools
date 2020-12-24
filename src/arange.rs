use num_traits::{real::Real, One, ToPrimitive, Zero};
use std::ops::{AddAssign, Div, Range, Sub};

/// Iterator over a range, stepping by a fixed amount each time
#[derive(Clone, Copy)]
pub struct Arange<F> {
    start: F,
    end: F,
    step_size: F,
    step: F,
}

impl<F> Arange<F>
where
    F: Zero,
{
    /// Create a new iterator over the range, stepping by `step` each time
    /// This allows you to create simple float iterators
    ///
    /// ```
    /// use iter_num_tools::Arange;
    /// use itertools::Itertools;
    ///
    /// let it = Arange::new(0.0..2.0, 0.5);
    /// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
    /// ```
    ///
    /// Arange isn't perfect, you might want [lin_space](crate::lin_space) if
    /// `step` isn't 'whole' float
    ///
    /// ```
    /// use iter_num_tools::{Arange, lin_space};
    /// use itertools::Itertools;
    ///
    /// // With Arange, you get some accuracy loss
    /// let it = Arange::new(0.0..0.5, 0.1);
    /// itertools::assert_equal(it, vec![0.0, 0.1, 0.2, 0.30000000000000004, 0.4]);
    ///
    /// let it = lin_space(0.0..0.5, 5);
    /// itertools::assert_equal(it, vec![0.0, 0.1, 0.2, 0.3, 0.4]);
    /// ```
    pub fn new(range: Range<F>, step: F) -> Self {
        let Range { start, end } = range;
        Arange {
            end,
            step_size: step,
            start,
            step: F::zero(),
        }
    }
}

impl<F> Iterator for Arange<F>
where
    F: CanArange,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.start + self.step * self.step_size;
        if x < self.end {
            self.step += F::one();
            Some(x)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.end - (self.start + self.step * self.step_size);
        match (length / self.step_size).ceil().to_usize() {
            Some(steps_left) => (steps_left, Some(steps_left)),
            None => (usize::MAX, None),
        }
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        match self.size_hint() {
            (_, Some(x)) => x,
            (_, None) => panic!("iterator is infinite"),
        }
    }
}

pub trait CanArange:
    Real + AddAssign + Sub<Output = Self> + Div<Output = Self> + ToPrimitive + One
{
}
impl<F> CanArange for F where
    F: Real + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive + One
{
}

#[test]
fn test_size_hint() {
    let it = Arange::new(0.0..0.55, 0.1);
    assert_eq!(it.size_hint(), (6, Some(6)));

    let it = Arange::new(0.0..0.5, 0.1);
    assert_eq!(it.size_hint(), (5, Some(5)));

    let it = Arange::new(0.0..0.5, 0.0);
    assert_eq!(it.size_hint(), (usize::MAX, None));
}
