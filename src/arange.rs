use num_traits::{Float, ToPrimitive};
use std::ops::{AddAssign, Div, Range, Sub};

/// Create a new iterator over the range, stepping by step each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
/// use itertools::Itertools;
///
/// let it = arange(0.0..2.0, 0.5);
/// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
/// ```
pub fn arange<T>(range: Range<T>, step: T) -> Arange<T> {
    Arange::new(range, step)
}

/// Iterator over a range, stepping by a fixed amount each time
pub struct Arange<T> {
    end: T,
    step: T,
    current: T,
}

impl<T> Arange<T> {
    /// Create a new iterator over the range, stepping by step each time
    /// This allows you to create simple float iterators
    ///
    /// ```
    /// use iter_num_tools::Arange;
    /// use itertools::Itertools;
    ///
    /// let it = Arange::new(0.0..2.0, 0.5);
    /// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
    /// ```
    pub fn new(range: Range<T>, step: T) -> Self {
        let Range {
            start: current,
            end,
        } = range;
        Arange { end, step, current }
    }
}

impl<F> Iterator for Arange<F>
where
    F: Float + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if current < self.end {
            self.current += self.step;
            Some(current)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let steps = (self.end - self.current) / self.step;
        let lower = steps.floor().to_usize().unwrap_or(0);
        let upper = steps.ceil().to_usize();
        (lower, upper)
    }
}
