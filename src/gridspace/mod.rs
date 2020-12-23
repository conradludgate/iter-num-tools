mod three_d;
mod two_d;
/// Creates a linear grid space over range with a fixed number of width and height steps
///
/// ```
/// use iter_num_tools::grid_space;
/// use itertools::Itertools;
///
/// let it = grid_space((0.0, 0.0)..(1.0, 2.0), (2, 4));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5),
/// ]);
///
/// // inclusive and with a single step count
/// let it = grid_space((0.0, 0.0)..=(1.0, 2.0), 3);
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 1.0), (0.0, 2.0),
///     (0.5, 0.0), (0.5, 1.0), (0.5, 2.0),
///     (1.0, 0.0), (1.0, 1.0), (1.0, 2.0),
/// ]);
///
/// // even 3d spaces
/// let it = grid_space((0, 0, 0)..=(1, 1, 1), 2);
/// itertools::assert_equal(it, vec![
///     (0, 0, 0), (0, 0, 1),
///     (0, 1, 0), (0, 1, 1),
///
///     (1, 0, 0), (1, 0, 1),
///     (1, 1, 0), (1, 1, 1),
/// ]);
/// ```
pub fn grid_space<R, S>(range: R, size: S) -> <R as IntoGridSpace<S>>::GridSpace
where
    R: IntoGridSpace<S>,
{
    range.into_grid_space(size)
}

pub trait IntoGridSpace<S> {
    type GridSpace;
    fn into_grid_space(self, size: S) -> Self::GridSpace;
}
