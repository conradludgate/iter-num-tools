#![deny(missing_docs)]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
#![cfg_attr(feature = "iter_advance_by", feature(iter_advance_by))]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

mod accum;
mod adapter;
mod arange;
mod arange_grid;
mod gridspace;
mod linspace;
mod logspace;
mod space;

pub use arange::{arange, Arange};
pub use arange_grid::{arange_grid, ArangeGrid};
pub use gridspace::{grid_space, GridSpace};
pub use linspace::{lin_space, LinSpace};
pub use logspace::{log_space, LogSpace};
