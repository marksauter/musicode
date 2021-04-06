#![feature(associated_type_bounds)]
// #![feature(test)]

// #[cfg(test)]
// extern crate test;

mod consts;

#[macro_use]
pub mod macros;

mod chord;
mod errors;
mod interval_set;
mod iter;
mod scale;

pub mod pattern;
mod pitch;

pub const OCTAVE: u8 = 12;

pub use crate::chord::Chord;
pub use crate::errors::OctaveError;
pub use crate::interval_set::IntervalSet;
pub use crate::pitch::{Accidental, Pitch};
pub use crate::scale::Scale;
