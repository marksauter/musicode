use std::error::Error;
use std::fmt;

/// Error value indicating insufficient capacity
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct OctaveError {
    interval: u8,
}

impl OctaveError {
    /// Create a new `OctaveError` from `interval`.
    pub fn new(interval: u8) -> OctaveError {
        OctaveError { interval }
    }

    /// Extract the overflowing interval
    pub fn interval(self) -> u8 {
        self.interval
    }
}

const OCTERROR: &'static str = "outside octave range";

impl Error for OctaveError {}

impl fmt::Display for OctaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", OCTERROR)
    }
}

impl fmt::Debug for OctaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", "OctaveError", OCTERROR)
    }
}
