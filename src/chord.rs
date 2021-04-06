use crate::iter::Pitches;
use crate::iter::{MatchIndices, MatchIndicesInternal, RMatchIndices};
use crate::iter::{Matches, MatchesInternal, RMatches};
use crate::pattern::Pattern;
use crate::{IntervalSet, Pitch};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Chord {
    set: IntervalSet,
}

#[allow(dead_code)]
impl Chord {
    /// Creates a new `Chord`.
    ///
    /// `Chord`'s are initialized with a unison interval.
    pub fn new() -> Chord {
        let mut set: IntervalSet = IntervalSet::new();
        set.insert(0);
        Chord { set }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn insert(&mut self, interval: u8) -> (usize, bool) {
        self.set.insert(interval)
    }

    pub fn find_or_insert(&mut self, interval: u8) -> Result<usize, usize> {
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

impl Deref for Chord {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Chord {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
