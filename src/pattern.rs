use crate::consts::OCTAVE;
use num_integer::Integer;
use std::cmp::Ordering;

pub trait Pattern<'a, const N: usize>: Sized {
    /// Associated searcher for this pattern
    type Searcher: Searcher<'a, N>;

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
        matches!(self.into_searcher(scale).next(), SearchStep::Match(_))
    }

    /// Checks whether the pattern matches at the back of the scale
    #[inline]
    fn is_leading_of(self, scale: &'a [u8]) -> bool
    where
        Self::Searcher: ReverseSearcher<'a, N>,
    {
        matches!(self.into_searcher(scale).next_back(), SearchStep::Match(_))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SearchStep<const N: usize> {
    /// Expresses that a match of the interval sequence has been found at
    /// `[scale[a], scale[b], ..; N]`
    Match([usize; N]),
    /// Expresses that `[Some(scale[a]), Some(scale[b]), None, ..; N]` has been
    /// rejected as a
    /// possible match of the interval sequence.
    ///
    /// Note that there might be more than one `Reject` between two `Match`es,
    /// there is no requirement for them to be combined into one.
    Reject([Option<usize>; N]),
    /// Expresses that every root of the scale has been visited, ending the
    /// iteration.
    Done,
}

pub unsafe trait Searcher<'a, const N: usize> {
    fn scale(&self) -> &'a [u8];

    fn next(&mut self) -> SearchStep<N>;

    fn next_match(&mut self) -> Option<[usize; N]> {
        loop {
            match self.next() {
                SearchStep::Match(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject(&mut self) -> Option<[Option<usize>; N]> {
        loop {
            match self.next() {
                SearchStep::Reject(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub unsafe trait ReverseSearcher<'a, const N: usize>: Searcher<'a, N> {
    fn next_back(&mut self) -> SearchStep<N>;

    fn next_match_back(&mut self) -> Option<[usize; N]> {
        loop {
            match self.next_back() {
                SearchStep::Match(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject_back(&mut self) -> Option<[Option<usize>; N]> {
        loop {
            match self.next_back() {
                SearchStep::Reject(a) => return Some(a),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub trait DoubleEndedSearcher<'a, const N: usize>: ReverseSearcher<'a, N> {}

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

unsafe impl<'a> Searcher<'a, 2> for IntervalSearcher<'a> {
    #[inline]
    fn scale(&self) -> &'a [u8] {
        self.scale
    }
    fn next(&mut self) -> SearchStep<2> {
        let len = self.scale.len();
        let old_root = self.root;
        let root_slice = unsafe { self.scale.get_unchecked(old_root..self.root_back) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next() {
            let (octave, finger) = self.finger.div_rem(&len);
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
                    Ordering::Less => SearchStep::Reject([Some(old_root), Some(old_finger)]),
                    Ordering::Equal => {
                        self.root += old_root_len - root_iter.len();
                        self.finger = self.root;
                        SearchStep::Match([old_root, old_finger])
                    }
                    Ordering::Greater => {
                        self.root += old_root_len - root_iter.len();
                        self.finger = self.root;
                        SearchStep::Reject([Some(old_root), Some(old_finger)])
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

unsafe impl<'a> ReverseSearcher<'a, 2> for IntervalSearcher<'a> {
    fn next_back(&mut self) -> SearchStep<2> {
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
                        SearchStep::Reject([Some(self.root_back), Some(old_finger)])
                    }
                    Ordering::Equal => {
                        self.root_back -= old_root_len - root_iter.len();
                        self.finger_back = self.root_back + len;
                        SearchStep::Match([self.root_back, old_finger])
                    }
                    Ordering::Greater => {
                        SearchStep::Reject([Some(self.root_back - 1), Some(old_finger)])
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

#[derive(Clone, Debug)]
pub struct ChordSearcher<'a, 'b, const N: usize> {
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
    /// The chord being searched for.
    chord: &'b [u8; N],
}

impl<'a, 'b, const N: usize> ChordSearcher<'a, 'b, N> {
    pub fn new(scale: &'a [u8], chord: &'b [u8; N]) -> Self {
        let len = scale.len();
        let root_back = len;
        ChordSearcher {
            scale,
            root: 0,
            root_back,
            finger: 0,
            finger_back: root_back + len,
            chord,
        }
    }
}

unsafe impl<'a, 'b, const N: usize> Searcher<'a, N> for ChordSearcher<'a, 'b, N> {
    #[inline]
    fn scale(&self) -> &'a [u8] {
        self.scale
    }
    fn next(&mut self) -> SearchStep<N> {
        let len = self.scale.len();
        let old_root = self.root;
        let root_slice = unsafe { self.scale.get_unchecked(old_root..self.root_back) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next() {
            let mut scale_indices: [Option<usize>; N] = [None; N];
            let mut chord_iter = self.chord.iter().enumerate();
            'chord: loop {
                if let Some((i, interval)) = chord_iter.next() {
                    let interval = interval % OCTAVE;
                    'interval: loop {
                        let (octave, finger) = self.finger.div_rem(&len);
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
                            match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&interval) {
                                Ordering::Less => continue 'interval,
                                Ordering::Equal => {
                                    scale_indices[i] = Some(old_finger);
                                    continue 'chord;
                                }
                                Ordering::Greater => {
                                    self.root += old_root_len - root_iter.len();
                                    self.finger = self.root;
                                    scale_indices[i] = Some(old_finger);
                                    return SearchStep::Reject(scale_indices);
                                }
                            }
                        } else {
                            self.root += old_root_len - root_iter.len();
                            self.finger = self.root;
                            return SearchStep::Reject(scale_indices);
                        }
                    }
                } else {
                    self.root += old_root_len - root_iter.len();
                    self.finger = self.root;
                    let mut match_indices: [usize; N] = [0; N];
                    for (i, interval) in scale_indices.iter().enumerate() {
                        match_indices[i] = interval.unwrap();
                    }
                    return SearchStep::Match(match_indices);
                }
            }
        } else {
            SearchStep::Done
        }
    }
}

unsafe impl<'a, 'b, const N: usize> ReverseSearcher<'a, N> for ChordSearcher<'a, 'b, N> {
    fn next_back(&mut self) -> SearchStep<N> {
        let len = self.scale.len();
        let old_root = self.root_back;
        let root_slice = unsafe { self.scale.get_unchecked(self.root..old_root) };
        let mut root_iter = root_slice.iter();
        let old_root_len = root_iter.len();
        if let Some(&r) = root_iter.next_back() {
            let mut scale_indices: [Option<usize>; N] = [None; N];
            let mut chord_iter = self.chord.iter().enumerate();
            'chord: loop {
                if let Some((i, interval)) = chord_iter.next_back() {
                    let interval = interval % OCTAVE;
                    'interval: loop {
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
                            match (f + (octave * OCTAVE as usize) as u8 - r).cmp(&interval) {
                                Ordering::Less => {
                                    self.root_back -= old_root_len - root_iter.len();
                                    self.finger_back = self.root_back + len;
                                    scale_indices[i] = Some(old_finger);
                                    return SearchStep::Reject(scale_indices);
                                }
                                Ordering::Equal => {
                                    scale_indices[i] = Some(old_finger);
                                    continue 'chord;
                                }
                                Ordering::Greater => continue 'interval,
                            }
                        } else {
                            self.root_back -= old_root_len - root_iter.len();
                            self.finger_back = self.root_back + len;
                            return SearchStep::Reject(scale_indices);
                        }
                    }
                } else {
                    self.root_back -= old_root_len - root_iter.len();
                    self.finger_back = self.root_back + len;
                    let mut match_indices: [usize; N] = [0; N];
                    for (i, interval) in scale_indices.iter().enumerate() {
                        match_indices[i] = interval.unwrap();
                    }
                    return SearchStep::Match(match_indices);
                }
            }
        } else {
            SearchStep::Done
        }
    }
}

impl<'a> Pattern<'a, 2> for u8 {
    type Searcher = IntervalSearcher<'a>;

    #[inline]
    fn into_searcher(self, scale: &'a [u8]) -> IntervalSearcher<'a> {
        IntervalSearcher::new(scale, self)
    }
}

impl<'a, 'b, const N: usize> Pattern<'a, N> for &'b [u8; N] {
    type Searcher = ChordSearcher<'a, 'b, N>;

    #[inline]
    fn into_searcher(self, scale: &'a [u8]) -> ChordSearcher<'a, 'b, N> {
        ChordSearcher::new(scale, self)
    }
}
