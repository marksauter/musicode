use crate::iter::{MatchIndices, MatchIndicesInternal, PitchIndices, Pitches};
use crate::pattern::Pattern;
use crate::pitch::Pitch;
use arrayvec::{ArrayVec, Drain};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::iter::{Extend, FromIterator};
use std::ops::{Deref, DerefMut, RangeBounds};

type CapacityError = arrayvec::CapacityError<u8>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct IntervalSet<const CAP: usize> {
    set: ArrayVec<u8, CAP>,
}

#[allow(dead_code)]
impl<const CAP: usize> IntervalSet<CAP> {
    /// Create a new, empty `IntervalSet`.
    pub fn new() -> IntervalSet<CAP> {
        IntervalSet {
            set: ArrayVec::new(),
        }
    }

    /// Create a new, empty `IntervalSet` (const fn).
    pub fn new_const() -> IntervalSet<CAP> {
        IntervalSet {
            set: ArrayVec::new_const(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        CAP
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.set.is_full()
    }

    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        self.set.remaining_capacity()
    }

    /// Insert an interval into sorted position.
    ///
    /// If the set did not have this interval, a tuple of the order index at which it
    /// was placed and true is returned.
    ///
    /// If the set did have this interval, a tuple of the index at which it was found
    /// and false is returned.
    ///
    /// It is an error if the set is full.
    ///
    /// ***Panics*** if the set is full.
    pub fn insert(&mut self, interval: u8) -> (usize, bool) {
        match self.find_or_insert(interval) {
            Ok(i) => (i, false),
            Err(i) => (i, true),
        }
    }

    /// Insert an interval into sorted position, returning the order index with `Ok`
    /// at which it was placed.
    ///
    /// Returns an error if the set is already at full capacity.
    pub fn try_insert(&mut self, interval: u8) -> Result<usize, CapacityError> {
        let insert_at = match self.binary_search(&interval) {
            Ok(insert_at) | Err(insert_at) => insert_at,
        };
        self.set.try_insert(insert_at, interval)?;
        Ok(insert_at)
    }

    /// Find the interval and return the index with `Ok`, otherwise insert them
    /// interval and return the new interval index with `Err`.
    ///
    /// It is an error if the set is full.
    ///
    /// ***Panics*** if the set is full.
    pub fn find_or_insert(&mut self, interval: u8) -> Result<usize, usize> {
        self.binary_search(&interval).map_err(|insert_at| {
            self.set.try_insert(insert_at, interval).unwrap();
            insert_at
        })
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.set.truncate(new_len)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.set.clear()
    }

    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        self.set.pop()
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        self.set.remove(index)
    }

    pub fn remove_interval(&mut self, interval: &u8) -> Option<u8> {
        match self.binary_search(interval) {
            Ok(remove_at) => Some(self.remove(remove_at)),
            Err(_) => None,
        }
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&mut u8) -> bool,
    {
        self.set.retain(f)
    }

    #[inline]
    pub fn drain<R>(&mut self, range: R) -> Drain<u8, CAP>
    where
        R: RangeBounds<usize>,
    {
        self.set.drain(range)
    }

    pub fn into_inner(self) -> Result<[u8; CAP], ArrayVec<u8, CAP>> {
        self.set.into_inner()
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

    pub fn pitch_indices(&self, root: Pitch) -> PitchIndices<'_> {
        PitchIndices {
            front_offset: 0,
            iter: self.pitches(root),
        }
    }

    pub fn match_indices<'a, P, const N: usize>(&'a self, pat: P) -> MatchIndices<'a, P, N>
    where
        P: Pattern<'a, N>,
    {
        MatchIndices(MatchIndicesInternal(pat.into_searcher(&self)))
    }
}

impl<const CAP: usize> Deref for IntervalSet<CAP> {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<const CAP: usize> DerefMut for IntervalSet<CAP> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<const CAP: usize> From<[u8; CAP]> for IntervalSet<CAP> {
    fn from(array: [u8; CAP]) -> Self {
        IntervalSet { set: array.into() }
    }
}

impl<const CAP: usize> TryFrom<&[u8]> for IntervalSet<CAP> {
    type Error = arrayvec::CapacityError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let set: ArrayVec<u8, CAP> = slice.try_into()?;
        Ok(IntervalSet { set })
    }
}

impl<const CAP: usize> Extend<u8> for IntervalSet<CAP> {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.set.extend(iter)
    }
}

impl<'a, const CAP: usize> Extend<&'a u8> for IntervalSet<CAP> {
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

impl<const CAP: usize> FromIterator<u8> for IntervalSet<CAP> {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter);
        is
    }
}

impl<'a, const CAP: usize> FromIterator<&'a u8> for IntervalSet<CAP> {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter);
        is
    }
}

impl<const CAP: usize> FromIterator<Pitch> for IntervalSet<CAP> {
    fn from_iter<I: IntoIterator<Item = Pitch>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter.into_iter().map(|p| p.as_interval()));
        is
    }
}

impl<'a, const CAP: usize> FromIterator<&'a Pitch> for IntervalSet<CAP> {
    fn from_iter<I: IntoIterator<Item = &'a Pitch>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter.into_iter().map(|p| p.as_interval()));
        is
    }
}

impl<'a, const CAP: usize> IntoIterator for &'a IntervalSet<CAP> {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const CAP: usize> IntoIterator for &'a mut IntervalSet<CAP> {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const CAP: usize> IntoIterator for IntervalSet<CAP> {
    type Item = u8;
    type IntoIter = arrayvec::IntoIter<u8, CAP>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl<const CAP: usize> std::borrow::Borrow<[u8]> for IntervalSet<CAP> {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl<const CAP: usize> std::borrow::BorrowMut<[u8]> for IntervalSet<CAP> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl<const CAP: usize> AsRef<[u8]> for IntervalSet<CAP> {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl<const CAP: usize> AsMut<[u8]> for IntervalSet<CAP> {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}
