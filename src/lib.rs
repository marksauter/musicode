#![feature(associated_type_bounds)]
// #![feature(test)]

// #[cfg(test)]
// extern crate test;

mod consts;

mod interval_set;
mod iter;

pub mod pattern;
mod pitch;

pub use crate::interval_set::IntervalSet;
pub use crate::pitch::{Accidental, Pitch};
