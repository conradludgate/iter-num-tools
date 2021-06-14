#![allow(incomplete_features)]
#![feature(impl_trait_in_bindings)]
#![feature(min_type_alias_impl_trait)]
#![feature(trusted_len)]
//! `iter_num_tools` is a collection if iterator extensions that
//! make heavy use of number properties.
//! Mostly extending on [Range](std::ops::Range).
//!
//! ## `LinSpace`
//! [`LinSpace`](lin_space) is an iterator over a range with a fixed number of values all evenly spaced.
//!
//! ```
//! use iter_num_tools::lin_space;
//!
//! // Count from 1.0 up to and including 5.0, with 5 numbers counted in total
//! let it = lin_space(1.0..=5.0, 5);
//! assert!(it.eq([1.0, 2.0, 3.0, 4.0, 5.0]));
//!
//! // Count from 0.0 up to and excluding 5.0, with 5 numbers counted in total
//! let it = lin_space(0.0..5.0, 5);
//! assert!(it.eq([0.0, 1.0, 2.0, 3.0, 4.0]));
//! ```
//!
//!
//! ## `GridSpace`
//! [`GridSpace`](grid_space) extends on [`LinSpace`](#linspace).
//!
//! ```
//! use iter_num_tools::grid_space;
//!
//! // count in 2 dimensions (excluding end points),
//! // from 0.0 up to 1.0 in the x direction with 2 even steps,
//! // and 0.0 up to 2.0 in the y direction with 4 even steps
//! let it = grid_space([0.0, 0.0]..[1.0, 2.0], [2, 4]);
//! assert!(it.eq([
//!     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
//!     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
//! ]));
//!
//! // count in 2 dimensions (including end points),
//! // from 0.0 up to 1.0 in the x direction,
//! // and 0.0 up to 2.0 in the y direction with 3 even steps in all directions
//! let it = grid_space([0.0, 0.0]..=[1.0, 2.0], 3);
//! assert!(it.eq([
//!     [0.0, 0.0], [0.0, 1.0], [0.0, 2.0],
//!     [0.5, 0.0], [0.5, 1.0], [0.5, 2.0],
//!     [1.0, 0.0], [1.0, 1.0], [1.0, 2.0],
//! ]));
//! ```
//!
//! ## `Arange`
//! [`Arange`](arange()) is similar to [`LinSpace`](#linspace), but instead of a fixed amount of steps,
//! it steps by a fixed amount.
//!
//! ```
//! use iter_num_tools::arange;
//!
//! let it = arange(0.0..2.0, 0.5);
//! assert!(it.eq([0.0, 0.5, 1.0, 1.5]));
//! ```
//!
//! #### Note
//! There is no inclusive version of arange. Consider the following
//! ```compile_fail
//! use iter_num_tools::arange;
//!
//! let it = arange(0.0..=2.1, 0.5);
//! ```
//! We would not expect 2.1 to ever be a value that the iterator will ever meet, but the range suggests it should be included.
//! Therefore, no [RangeInclusive](std::ops::RangeInclusive) implementation is provided.
//!
//! ## ArangeGrid
//! [`ArangeGrid`](arange_grid()) is the same as [`GridSpace`](#gridspace) but for [`Arange`](#arange) instead of [`LinSpace`](#linspace).
//!
//!
//! ```
//! use iter_num_tools::arange_grid;
//!
//! // count in 2 dimensions,
//! // from 0.0 up to 1.0 in the x direction,
//! // and 0.0 up to 2.0 in the y direction,
//! // stepping by 0.5 each time
//! let it = arange_grid([0.0, 0.0]..[1.0, 2.0], 0.5);
//! assert!(it.eq([
//!     [0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [0.0, 1.5],
//!     [0.5, 0.0], [0.5, 0.5], [0.5, 1.0], [0.5, 1.5],
//! ]));
//!
//! // count in 2 dimensions,
//! // from 0.0 up to 1.0 in the x direction stepping by 0.5 each time,
//! // and 0.0 up to 2.0 in the y direction stepping by 1.0 each time
//! let it = arange_grid([0.0, 0.0]..[1.0, 2.0], [0.5, 1.0]);
//! assert!(it.eq([
//!     [0.0, 0.0], [0.0, 1.0],
//!     [0.5, 0.0], [0.5, 1.0],
//! ]));
//! ```
//!
//! ## `LogSpace`
//! [`LogSpace`](log_space()) is similar to [`LinSpace`](#linspace), but instead of evenly spaced linear steps, it has evenly spaced logarithmic steps.
//!
//! ```
//! use iter_num_tools::log_space;
//! use itertools::zip_eq;
//!
//! // From 1.0 up to and including 1000.0, taking 4 logarithmic steps
//! let it = log_space(1.0..=1000.0, 4);
//! let expected: [f64; 4] = [1.0, 10.0, 100.0, 1000.0];
//!
//! assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
//!
//! // From 1.0 up to 1000.0, taking 3 logarithmic steps
//! let it = log_space(1.0..1000.0, 3);
//! let expected: [f64; 3] = [1.0, 10.0, 100.0];
//!
//! assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
//! ```

#![no_std]

#[cfg(test)]
#[macro_use]
mod test_util;

mod arange;
mod arange_grid;
mod gridspace;
mod linspace;
mod logspace;

pub use arange::{arange, Arange, IntoArange};
pub use arange_grid::{arange_grid, ArangeGrid, IntoArangeGrid};
pub use gridspace::{grid_space, GridSpace, IntoGridSpace};
pub use linspace::{lin_space, LinSpace, IntoLinSpace};
pub use logspace::{log_space, LogSpace, IntoLogSpace};
