use crate::{lin_space, IntoLinSpace, LinSpace};
use num_traits::{real::Real, ToPrimitive};
use std::ops::{Div, Range, Sub};

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
/// use itertools::Itertools;
///
/// let it = arange(0.0..2.0, 0.5);
/// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
/// ```
pub fn arange<F>(range: Range<F>, step: F) -> LinSpace<F>
where
    F: Real + Sub<Output = F> + Div<Output = F> + ToPrimitive,
    Range<F>: IntoLinSpace<F>,
{
    let Range { start: a, end: b } = range;
    let l = ((b - a) / step).ceil();
    let len = l.to_usize().unwrap();

    let b = a + l * step;
    lin_space(a..b, len)
}
