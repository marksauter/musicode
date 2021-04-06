use crate::OCTAVE;
use std::cmp::Ordering;

pub trait Pattern<'a>: Sized {
    /// Associated searcher for this pattern
    type Searcher: Searcher<'a>;

    /// Constructs the associated seracher from
    /// `self` and the `scale` to search in.
    fn into_searcher(self, scale: &'a [u8]) -> Self::Searcher;

    /// Checks whether the pattern matches anywhere in the scale
    #[inline]
    fn is_contained_in(self, scale: &'a [u8]) -> bool {
        self.into_searcher(scale).next_match().is_some()
    }

    /// Checks whether the pattern matches at the front of the scale
    #[inline]
    fn is_tonic_of(self, scale: &'a [u8]) -> bool {
        matches!(
            self.into_searcher(scale).next_match().as_deref(),
            Some(&[0, ..])
        )
    }

    /// Checks whether the pattern matches at the back of the scale
    #[inline]
    fn is_leading_of(self, scale: &'a [u8]) -> bool
    where
        Self::Searcher: ReverseSearcher<'a>,
    {
        matches!(
            self.into_searcher(scale).next_match_back().as_deref(),
            Some([j, ..]) if scale.len() == *j
        )
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SearchStep {
    /// Expresses that a match of the interval sequence has been found at
    /// `[scale[a], scale[b], ..]`
    Match(Vec<usize>),
    /// Expresses that `[scale[a], scale[b], .., scale[n]]` has been
    /// rejected as a possible match of the interval sequence. The last value
    /// in the returned vector indicates the scale position at which the pattern
    /// failed to match.
    ///
    /// Note that there might be more than one `Reject` between two `Match`es,
    /// there is no requirement for them to be combined into one.
    Reject(Vec<usize>),
    /// Expresses that every root of the scale has been visited, ending the
    /// iteration.
    Done,
}

pub unsafe trait Searcher<'a> {
    fn scale(&self) -> &'a [u8];

    fn next(&mut self) -> SearchStep;

    fn next_match(&mut self) -> Option<Vec<usize>> {
        loop {
            match self.next() {
                SearchStep::Match(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject(&mut self) -> Option<Vec<usize>> {
        loop {
            match self.next() {
                SearchStep::Reject(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub unsafe trait ReverseSearcher<'a>: Searcher<'a> {
    fn next_back(&mut self) -> SearchStep;

    fn next_match_back(&mut self) -> Option<Vec<usize>> {
        loop {
            match self.next_back() {
                SearchStep::Match(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject_back(&mut self) -> Option<Vec<usize>> {
        loop {
            match self.next_back() {
                SearchStep::Reject(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub trait DoubleEndedSearcher<'a>: ReverseSearcher<'a> {}

#[derive(Clone, Debug)]
pub struct IntervalSearcher<'a> {
    /// The scale in which we are searching.
    scale: &'a [u8],
    /// `root` is the current index being used as the root of
    /// the interval for the forward search.
    root: usize,
    /// `root_back` is the current index being used as the root of
    /// the interval for the reverse search.
    root_back: usize,
    /// `finger` is the current index of the forward search.
    finger: usize,
    /// `finger_back` is the current index of the reverse search.
    finger_back: usize,
    /// The interval being searched for.
    interval: u8,
}

impl<'a> IntervalSearcher<'a> {
    pub fn new(scale: &'a [u8], interval: u8) -> Self {
        let len = scale.len();
        let root_back = len;
        IntervalSearcher {
            scale,
            root: 0,
            root_back,
            finger: 0,
            finger_back: root_back + len,
            interval: interval % OCTAVE,
        }
    }
}

unsafe impl<'a> Searcher<'a> for IntervalSearcher<'a> {
    #[inline]
    fn scale(&self) -> &'a [u8] {
        self.scale
    }
    fn next(&mut self) -> SearchStep {
        let len = self.scale.len();
        let old_root = self.root;
        let root_slice = unsafe { self.scale.get_unchecked(old_root..self.root_back) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next() {
            let finger = self.finger % len;
            let octave = self.finger / len;
            let finger_back = if self.root == self.root_back {
                self.finger_back
            } else {
                len
            };
            let finger_slice = unsafe { self.scale.get_unchecked(finger..finger_back) };
            let mut finger_iter = finger_slice.iter();
            let old_finger_len = finger_iter.len();
            if let Some(&f) = finger_iter.next() {
                // finger canNOT be >= to the `len`, because it represents a scale index
                let old_finger = if self.finger >= len {
                    self.finger - len
                } else {
                    self.finger
                };
                self.finger += old_finger_len - finger_iter.len();
                match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&self.interval) {
                    Ordering::Less => SearchStep::Reject(vec![old_root, old_finger]),
                    Ordering::Equal => {
                        self.root += old_root_len - root_iter.len();
                        self.finger = self.root;
                        SearchStep::Match(vec![old_root, old_finger])
                    }
                    Ordering::Greater => {
                        self.root += old_root_len - root_iter.len();
                        self.finger = self.root;
                        SearchStep::Reject(vec![old_root, old_finger])
                    }
                }
            } else {
                SearchStep::Done
            }
        } else {
            SearchStep::Done
        }
    }
}

unsafe impl<'a> ReverseSearcher<'a> for IntervalSearcher<'a> {
    fn next_back(&mut self) -> SearchStep {
        let len = self.scale.len();
        let old_root = self.root_back;
        let root_slice = unsafe { self.scale.get_unchecked(self.root..old_root) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next_back() {
            let octave = if self.finger_back > len { 1 } else { 0 };
            // finger_back must be <= to the `len`, because it is the `end` value of a Range
            let finger_back = if self.finger_back > len {
                self.finger_back - len
            } else {
                self.finger_back
            };
            let finger = if self.root_back == self.root {
                self.finger
            } else {
                0
            };
            let finger_slice = unsafe { self.scale.get_unchecked(finger..finger_back) };
            let mut finger_iter = finger_slice.iter();
            let old_finger_len = finger_iter.len();
            if let Some(&f) = finger_iter.next_back() {
                self.finger_back -= old_finger_len - finger_iter.len();
                // finger_back canNOT be >= to the `len`, because now it represents a scale index
                let old_finger = if self.finger_back >= len {
                    self.finger_back - len
                } else {
                    self.finger_back
                };
                match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&self.interval) {
                    Ordering::Less => {
                        self.root_back -= old_root_len - root_iter.len();
                        self.finger_back = self.root_back + len;
                        SearchStep::Reject(vec![self.root_back, old_finger])
                    }
                    Ordering::Equal => {
                        self.root_back -= old_root_len - root_iter.len();
                        self.finger_back = self.root_back + len;
                        SearchStep::Match(vec![self.root_back, old_finger])
                    }
                    Ordering::Greater => SearchStep::Reject(vec![self.root_back - 1, old_finger]),
                }
            } else {
                SearchStep::Done
            }
        } else {
            SearchStep::Done
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChordSearcher<'a, 'b> {
    /// The scale in which we are searching.
    scale: &'a [u8],
    /// `root` is the current index being used as the root of
    /// the interval for the forward search.
    root: usize,
    /// `root_back` is the current index being used as the root of
    /// the interval for the reverse search.
    root_back: usize,
    /// The chord being searched for.
    chord: &'b [u8],
}

impl<'a, 'b> ChordSearcher<'a, 'b> {
    pub fn new(scale: &'a [u8], chord: &'b [u8]) -> Self {
        let len = scale.len();
        ChordSearcher {
            scale,
            root: 0,
            root_back: len,
            chord,
        }
    }
}

unsafe impl<'a, 'b> Searcher<'a> for ChordSearcher<'a, 'b> {
    #[inline]
    fn scale(&self) -> &'a [u8] {
        self.scale
    }
    fn next(&mut self) -> SearchStep {
        let len = self.scale.len();
        let old_root = self.root;
        let root_slice = unsafe { self.scale.get_unchecked(old_root..self.root_back) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next() {
            let mut chord_iter = self.chord.iter();
            let mut scale_indices: Vec<usize> = Vec::new();
            let mut finger = old_root;
            'chord: loop {
                if let Some(interval) = chord_iter.next() {
                    let interval = interval % OCTAVE;
                    'interval: loop {
                        let finger_index = finger % len;
                        let octave = finger / len;
                        let finger_slice = unsafe { self.scale.get_unchecked(finger_index..len) };
                        let mut finger_iter = finger_slice.iter();
                        let old_finger_len = finger_iter.len();
                        if let Some(&f) = finger_iter.next() {
                            // finger canNOT be >= to the `len`, because it represents a scale index
                            let old_finger = if finger >= len { finger - len } else { finger };
                            finger += old_finger_len - finger_iter.len();
                            match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&interval) {
                                Ordering::Less => continue 'interval,
                                Ordering::Equal => {
                                    scale_indices.push(old_finger);
                                    continue 'chord;
                                }
                                Ordering::Greater => {
                                    self.root += old_root_len - root_iter.len();
                                    scale_indices.push(old_finger);
                                    return SearchStep::Reject(scale_indices);
                                }
                            }
                        } else {
                            self.root += old_root_len - root_iter.len();
                            return SearchStep::Reject(scale_indices);
                        }
                    }
                } else {
                    self.root += old_root_len - root_iter.len();
                    return SearchStep::Match(scale_indices);
                }
            }
        } else {
            SearchStep::Done
        }
    }
}

unsafe impl<'a, 'b> ReverseSearcher<'a> for ChordSearcher<'a, 'b> {
    fn next_back(&mut self) -> SearchStep {
        let len = self.scale.len();
        let old_root = self.root_back;
        let root_slice = unsafe { self.scale.get_unchecked(self.root..old_root) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next_back() {
            let mut chord_iter = self.chord.iter();
            let mut scale_indices: Vec<usize> = Vec::new();
            let mut finger = old_root - 1;
            'chord: loop {
                if let Some(interval) = chord_iter.next() {
                    let interval = interval % OCTAVE;
                    'interval: loop {
                        let finger_index = finger % len;
                        let octave = finger / len;
                        let finger_slice = unsafe { self.scale.get_unchecked(finger_index..len) };
                        let mut finger_iter = finger_slice.iter();
                        let old_finger_len = finger_iter.len();
                        if let Some(&f) = finger_iter.next() {
                            // finger canNOT be >= to the `len`, because it represents a scale index
                            let old_finger = if finger >= len { finger - len } else { finger };
                            finger += old_finger_len - finger_iter.len();
                            match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&interval) {
                                Ordering::Less => continue 'interval,
                                Ordering::Equal => {
                                    scale_indices.push(old_finger);
                                    continue 'chord;
                                }
                                Ordering::Greater => {
                                    self.root_back -= old_root_len - root_iter.len();
                                    scale_indices.push(old_finger);
                                    return SearchStep::Reject(scale_indices);
                                }
                            }
                        } else {
                            self.root_back -= old_root_len - root_iter.len();
                            return SearchStep::Reject(scale_indices);
                        }
                    }
                } else {
                    self.root_back -= old_root_len - root_iter.len();
                    return SearchStep::Match(scale_indices);
                }
            }
        } else {
            SearchStep::Done
        }
    }
}

impl<'a> Pattern<'a> for u8 {
    type Searcher = IntervalSearcher<'a>;

    #[inline]
    fn into_searcher(self, scale: &'a [u8]) -> IntervalSearcher<'a> {
        IntervalSearcher::new(scale, self)
    }
}

impl<'a, 'b> Pattern<'a> for &'b [u8] {
    type Searcher = ChordSearcher<'a, 'b>;

    #[inline]
    fn into_searcher(self, scale: &'a [u8]) -> ChordSearcher<'a, 'b> {
        ChordSearcher::new(scale, self)
    }
}

impl<'a, 'b, const N: usize> Pattern<'a> for &'b [u8; N] {
    type Searcher = ChordSearcher<'a, 'b>;

    #[inline]
    fn into_searcher(self, scale: &'a [u8]) -> ChordSearcher<'a, 'b> {
        ChordSearcher::new(scale, self)
    }
}
