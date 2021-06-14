use crate::{Lerp, LinSpace, Linear};
use core::ops::Range;
use num_traits::real::Real;

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
/// use itertools::Itertools;
///
/// let it = arange(0.0..2.0, 0.5);
/// println!("{:?}", it);
/// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
/// ```
pub fn arange<F>(range: Range<F>, step: F) -> LinSpace<F>
where
    F: Real + Linear,
{
    let (util, steps) = arange_lerp(range, step);
    LinSpace { x: 0, steps, util }
}

pub(crate) fn arange_lerp<F>(range: Range<F>, step: F) -> (Lerp<F>, usize)
where
    F: Real + Linear,
{
    let Range { start, end } = range;

    (
        Lerp { start, step },
        ((end - start) / step).ceil().to_usize().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let it = arange(0.0..2.0, 0.5);
        assert_eq_iter!(it, [0.0, 0.5, 1.0, 1.5]);
    }
}
