use crate::iter::Pitches;
use crate::Pitch;
use serde::{Deserialize, Serialize};
use std::iter::{Extend, FromIterator};
use std::ops::{Deref, DerefMut, RangeBounds};

/// Forward sorted set of unique intervals, backed by an Vec.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct IntervalSet {
    set: Vec<u8>,
}

#[allow(dead_code)]
impl IntervalSet {
    /// Create a new, empty `IntervalSet`.
    pub fn new() -> IntervalSet {
        IntervalSet { set: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> IntervalSet {
        IntervalSet {
            set: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(mut vec: Vec<u8>) -> Self {
        vec.sort_unstable();
        vec.dedup();
        IntervalSet { set: vec }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Insert an interval into sorted position.
    ///
    /// If the set did not have this interval, a tuple of the order index at which it
    /// was placed and true is returned.
    ///
    /// If the set did have this interval, a tuple of the index at which it was found
    /// and false is returned.
    pub fn insert(&mut self, interval: u8) -> (usize, bool) {
        match self.find_or_insert(interval) {
            Ok(i) => (i, false),
            Err(i) => (i, true),
        }
    }

    /// Find the interval and return the index with `Ok`, otherwise insert the
    /// interval and return the new interval index with `Err`.
    pub fn find_or_insert(&mut self, interval: u8) -> Result<usize, usize> {
        self.binary_search(&interval).map_err(|insert_at| {
            self.set.insert(insert_at, interval);
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
        F: FnMut(&u8) -> bool,
    {
        self.set.retain(f)
    }

    #[inline]
    pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<u8>
    where
        R: RangeBounds<usize>,
    {
        self.set.drain(range)
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
}

impl Deref for IntervalSet {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for IntervalSet {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<const N: usize> From<[u8; N]> for IntervalSet {
    fn from(array: [u8; N]) -> Self {
        let mut is = IntervalSet::new();
        is.extend(array.iter());
        is
    }
}

impl From<&[u8]> for IntervalSet {
    fn from(slice: &[u8]) -> Self {
        let mut is = IntervalSet::new();
        is.extend(slice.into_iter().copied());
        is
    }
}

impl From<&mut [u8]> for IntervalSet {
    fn from(slice: &mut [u8]) -> Self {
        let mut is = IntervalSet::new();
        is.extend(slice.iter());
        is
    }
}

impl From<Vec<u8>> for IntervalSet {
    fn from(vec: Vec<u8>) -> Self {
        IntervalSet::from_vec(vec)
    }
}

impl Extend<u8> for IntervalSet {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}

impl<'a> Extend<&'a u8> for IntervalSet {
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

impl FromIterator<u8> for IntervalSet {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter);
        is
    }
}

impl<'a> FromIterator<&'a u8> for IntervalSet {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter);
        is
    }
}

impl FromIterator<Pitch> for IntervalSet {
    fn from_iter<I: IntoIterator<Item = Pitch>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter.into_iter().map(|p| p.as_interval()));
        is
    }
}

impl<'a> FromIterator<&'a Pitch> for IntervalSet {
    fn from_iter<I: IntoIterator<Item = &'a Pitch>>(iter: I) -> Self {
        let mut is = IntervalSet::new();
        is.extend(iter.into_iter().map(|p| p.as_interval()));
        is
    }
}

impl<'a> IntoIterator for &'a IntervalSet {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut IntervalSet {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for IntervalSet {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl std::borrow::Borrow<[u8]> for IntervalSet {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl std::borrow::BorrowMut<[u8]> for IntervalSet {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl AsRef<[u8]> for IntervalSet {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl AsMut<[u8]> for IntervalSet {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}
