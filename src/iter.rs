use super::pattern::Pattern;
use super::pattern::{DoubleEndedSearcher, ReverseSearcher, Searcher};
use super::pitch::Pitch;
use std::fmt;
use std::iter::FusedIterator;

#[derive(Debug)]
pub struct Pitches<'a> {
    pub(super) root: Pitch,
    pub(super) iter: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for Pitches<'a> {
    type Item = Pitch;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|&i| self.root.add_interval(i))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).and_then(|&i| self.root.add_interval(i))
    }
}

impl<'a> DoubleEndedIterator for Pitches<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .and_then(|&i| self.root.add_interval(i))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth_back(n)
            .and_then(|&i| self.root.sub_interval(i))
    }
}

impl FusedIterator for Pitches<'_> {}

pub struct PitchIndices<'a> {
    pub(super) front_offset: usize,
    pub(super) iter: Pitches<'a>,
}

impl<'a> Iterator for PitchIndices<'a> {
    type Item = (usize, Pitch);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pre_len = self.iter.iter.len();
        match self.iter.next() {
            None => None,
            Some(p) => {
                let index = self.front_offset;
                let len = self.iter.iter.len();
                self.front_offset += pre_len - len;
                Some((index, p))
            }
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a> DoubleEndedIterator for PitchIndices<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|p| {
            let index = self.front_offset + self.iter.iter.len();
            (index, p)
        })
    }
}

impl FusedIterator for PitchIndices<'_> {}

pub(super) struct MatchIndicesInternal<'a, P: Pattern<'a, N>, const N: usize>(
    pub(super) P::Searcher,
);

impl<'a, P, const N: usize> fmt::Debug for MatchIndicesInternal<'a, P, N>
where
    P: Pattern<'a, N, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchIndicesInternal")
            .field(&self.0)
            .finish()
    }
}

impl<'a, P: Pattern<'a, N>, const N: usize> MatchIndicesInternal<'a, P, N> {
    #[inline]
    fn next(&mut self) -> Option<(usize, [u8; N])> {
        self.0.next_match().map(|scale_indices| {
            let mut match_indices: [u8; N] = [0; N];
            for (i, &index) in scale_indices.iter().enumerate() {
                unsafe { match_indices[i] = *self.0.scale().get_unchecked(index) };
            }
            (scale_indices[0], match_indices)
        })
    }

    #[inline]
    fn next_back(&mut self) -> Option<(usize, [u8; N])>
    where
        P::Searcher: ReverseSearcher<'a, N>,
    {
        self.0.next_match_back().map(|scale_indices| {
            let mut match_indices: [u8; N] = [0; N];
            for (i, &index) in scale_indices.iter().enumerate() {
                unsafe { match_indices[i] = *self.0.scale().get_unchecked(index) };
            }
            (scale_indices[0], match_indices)
        })
    }
}

impl<'a, P, const N: usize> Clone for MatchIndicesInternal<'a, P, N>
where
    P: Pattern<'a, N, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        let s = self;
        MatchIndicesInternal(s.0.clone())
    }
}

pub struct MatchIndices<'a, P: Pattern<'a, N>, const N: usize>(
    pub(super) MatchIndicesInternal<'a, P, N>,
);

impl<'a, P, const N: usize> fmt::Debug for MatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(MatchIndices))
            .field(&self.0)
            .finish()
    }
}

impl<'a, P: Pattern<'a, N>, const N: usize> Iterator for MatchIndices<'a, P, N> {
    type Item = (usize, [u8; N]);

    #[inline]
    fn next(&mut self) -> Option<(usize, [u8; N])> {
        self.0.next()
    }
}

impl<'a, P, const N: usize> Clone for MatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        MatchIndices(self.0.clone())
    }
}

pub struct RMatchIndices<'a, P: Pattern<'a, N>, const N: usize>(
    pub(super) MatchIndicesInternal<'a, P, N>,
);

impl<'a, P, const N: usize> fmt::Debug for RMatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(RMatchIndices))
            .field(&self.0)
            .finish()
    }
}

impl<'a, P, const N: usize> Iterator for RMatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: ReverseSearcher<'a, N>>,
{
    type Item = (usize, [u8; N]);

    #[inline]
    fn next(&mut self) -> Option<(usize, [u8; N])> {
        self.0.next_back()
    }
}

impl<'a, P, const N: usize> Clone for RMatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        RMatchIndices(self.0.clone())
    }
}

impl<'a, P: Pattern<'a, N>, const N: usize> FusedIterator for MatchIndices<'a, P, N> {}

impl<'a, P, const N: usize> FusedIterator for RMatchIndices<'a, P, N> where
    P: Pattern<'a, N, Searcher: ReverseSearcher<'a, N>>
{
}

impl<'a, P, const N: usize> DoubleEndedIterator for MatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: DoubleEndedSearcher<'a, N>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, [u8; N])> {
        self.0.next_back()
    }
}

impl<'a, P, const N: usize> DoubleEndedIterator for RMatchIndices<'a, P, N>
where
    P: Pattern<'a, N, Searcher: DoubleEndedSearcher<'a, N>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, [u8; N])> {
        self.0.next()
    }
}
