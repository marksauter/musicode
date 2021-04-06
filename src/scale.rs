use std::ops::{Deref, DerefMut};

use crate::iter::Pitches;
use crate::iter::{MatchIndices, MatchIndicesInternal, RMatchIndices};
use crate::iter::{Matches, MatchesInternal, RMatches};
use crate::pattern::Pattern;
use crate::OCTAVE;
use crate::{IntervalSet, OctaveError, Pitch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Scale {
    set: IntervalSet,
}

macro_rules! panic_oob {
    ($method_name:expr, $interval:expr) => {
        panic!(
            concat!(
                "Scale::",
                $method_name,
                ": interval {} is out of octave bounds 11"
            ),
            $interval
        )
    };
}

#[allow(dead_code)]
impl Scale {
    /// Creates a new `Scale`.
    ///
    /// `Scale`'s are initialized with a unison interval.
    pub fn new() -> Scale {
        let mut set: IntervalSet = IntervalSet::new();
        set.insert(0);
        Scale { set }
    }

    pub fn chromatic() -> Scale {
        Scale {
            set: (0..OCTAVE).collect(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Insert `interval` into sorted position.
    ///
    /// If the set did not have this `interval`, a tuple of the order index at which it
    /// was placed and true is returned.
    ///
    /// If the set did have this `interval`, a tuple of the index at which it was found
    /// and false is returned.
    ///
    /// It is an error if the `interval` is outside octave range.
    ///
    /// ***Panics*** if the `interval` is outside octave range. See `try_insert` for
    /// fallible version.
    pub fn insert(&mut self, interval: u8) -> (usize, bool) {
        if interval >= OCTAVE {
            panic_oob!("insert", interval)
        }
        match self.set.find_or_insert(interval) {
            Ok(i) => (i, false),
            Err(i) => (i, true),
        }
    }

    /// Insert `interval` into sorted position.
    ///
    /// Returns an error if `interval` is outside octave range.
    pub fn try_insert(&mut self, interval: u8) -> Result<usize, OctaveError> {
        if interval >= OCTAVE {
            Err(OctaveError::new(interval))
        } else {
            match self.set.find_or_insert(interval) {
                Ok(i) | Err(i) => Ok(i),
            }
        }
    }

    /// Find `interval` and return the index with `Ok`, otherwise insert the
    /// interval and return the new interval index with `Err`.
    ///
    /// It is an error if the `interval` outside octave range.
    ///
    /// ***Panics*** if the `interval` is outside octave range.
    pub fn find_or_insert(&mut self, interval: u8) -> Result<usize, usize> {
        if interval >= OCTAVE {
            panic_oob!("find_or_insert", interval)
        }
        self.set.find_or_insert(interval)
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.set.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.set.as_mut_slice()
    }

    pub fn pitches(&self, root: Pitch) -> Pitches<'_> {
        Pitches {
            root,
            iter: self.iter(),
        }
    }

    pub fn matches<'a, P>(&'a self, pat: P) -> Matches<'a, P>
    where
        P: Pattern<'a>,
    {
        Matches(MatchesInternal(pat.into_searcher(&self)))
    }

    pub fn rmatches<'a, P>(&'a self, pat: P) -> RMatches<'a, P>
    where
        P: Pattern<'a>,
    {
        RMatches(MatchesInternal(pat.into_searcher(&self)))
    }

    pub fn match_indices<'a, P>(&'a self, pat: P) -> MatchIndices<'a, P>
    where
        P: Pattern<'a>,
    {
        MatchIndices(MatchIndicesInternal(pat.into_searcher(&self)))
    }

    pub fn rmatch_indices<'a, P>(&'a self, pat: P) -> RMatchIndices<'a, P>
    where
        P: Pattern<'a>,
    {
        RMatchIndices(MatchIndicesInternal(pat.into_searcher(&self)))
    }
}

impl Deref for Scale {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Scale {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
