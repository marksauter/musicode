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

pub(super) struct MatchIndicesInternal<'a, P: Pattern<'a>>(pub(super) P::Searcher);

impl<'a, P> fmt::Debug for MatchIndicesInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchIndicesInternal")
            .field(&self.0)
            .finish()
    }
}

impl<'a, P: Pattern<'a>> MatchIndicesInternal<'a, P> {
    #[inline]
    fn next(&mut self) -> Option<(usize, Vec<u8>)> {
        self.0.next_match().map(|scale_indices| {
            (
                scale_indices[0],
                scale_indices.iter().map(|i| self.0.scale()[*i]).collect(),
            )
        })
    }

    #[inline]
    fn next_back(&mut self) -> Option<(usize, Vec<u8>)>
    where
        P::Searcher: ReverseSearcher<'a>,
    {
        self.0.next_match_back().map(|scale_indices| {
            (
                scale_indices[0],
                scale_indices.iter().map(|i| self.0.scale()[*i]).collect(),
            )
        })
    }
}

impl<'a, P> Clone for MatchIndicesInternal<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        let s = self;
        MatchIndicesInternal(s.0.clone())
    }
}

pub struct MatchIndices<'a, P: Pattern<'a>>(pub(super) MatchIndicesInternal<'a, P>);

impl<'a, P> fmt::Debug for MatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(MatchIndices))
            .field(&self.0)
            .finish()
    }
}

impl<'a, P: Pattern<'a>> Iterator for MatchIndices<'a, P> {
    type Item = (usize, Vec<u8>);

    #[inline]
    fn next(&mut self) -> Option<(usize, Vec<u8>)> {
        self.0.next()
    }
}

impl<'a, P> Clone for MatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        MatchIndices(self.0.clone())
    }
}

pub struct RMatchIndices<'a, P: Pattern<'a>>(pub(super) MatchIndicesInternal<'a, P>);

impl<'a, P> fmt::Debug for RMatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(RMatchIndices))
            .field(&self.0)
            .finish()
    }
}

impl<'a, P> Iterator for RMatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: ReverseSearcher<'a>>,
{
    type Item = (usize, Vec<u8>);

    #[inline]
    fn next(&mut self) -> Option<(usize, Vec<u8>)> {
        self.0.next_back()
    }
}

impl<'a, P> Clone for RMatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        RMatchIndices(self.0.clone())
    }
}

impl<'a, P: Pattern<'a>> FusedIterator for MatchIndices<'a, P> {}

impl<'a, P> FusedIterator for RMatchIndices<'a, P> where
    P: Pattern<'a, Searcher: ReverseSearcher<'a>>
{
}

impl<'a, P> DoubleEndedIterator for MatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: DoubleEndedSearcher<'a>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, Vec<u8>)> {
        self.0.next_back()
    }
}

impl<'a, P> DoubleEndedIterator for RMatchIndices<'a, P>
where
    P: Pattern<'a, Searcher: DoubleEndedSearcher<'a>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, Vec<u8>)> {
        self.0.next()
    }
}

pub(super) struct MatchesInternal<'a, P: Pattern<'a>>(pub(super) P::Searcher);

impl<'a, P> fmt::Debug for MatchesInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MatchInternal").field(&self.0).finish()
    }
}

impl<'a, P: Pattern<'a>> MatchesInternal<'a, P> {
    #[inline]
    fn next(&mut self) -> Option<Vec<u8>> {
        self.0
            .next_match()
            .map(|scale_indices| scale_indices.iter().map(|i| self.0.scale()[*i]).collect())
    }

    #[inline]
    fn next_back(&mut self) -> Option<Vec<u8>>
    where
        P::Searcher: ReverseSearcher<'a>,
    {
        self.0
            .next_match_back()
            .map(|scale_indices| scale_indices.iter().map(|i| self.0.scale()[*i]).collect())
    }
}

impl<'a, P> Clone for MatchesInternal<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        let s = self;
        MatchesInternal(s.0.clone())
    }
}

pub struct Matches<'a, P: Pattern<'a>>(pub(super) MatchesInternal<'a, P>);

impl<'a, P> fmt::Debug for Matches<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Match)).field(&self.0).finish()
    }
}

impl<'a, P: Pattern<'a>> Iterator for Matches<'a, P> {
    type Item = Vec<u8>;

    #[inline]
    fn next(&mut self) -> Option<Vec<u8>> {
        self.0.next()
    }
}

impl<'a, P> Clone for Matches<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        Matches(self.0.clone())
    }
}

pub struct RMatches<'a, P: Pattern<'a>>(pub(super) MatchesInternal<'a, P>);

impl<'a, P> fmt::Debug for RMatches<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(RMatch)).field(&self.0).finish()
    }
}

impl<'a, P> Iterator for RMatches<'a, P>
where
    P: Pattern<'a, Searcher: ReverseSearcher<'a>>,
{
    type Item = Vec<u8>;

    #[inline]
    fn next(&mut self) -> Option<Vec<u8>> {
        self.0.next_back()
    }
}

impl<'a, P> Clone for RMatches<'a, P>
where
    P: Pattern<'a, Searcher: Clone>,
{
    fn clone(&self) -> Self {
        RMatches(self.0.clone())
    }
}

impl<'a, P: Pattern<'a>> FusedIterator for Matches<'a, P> {}

impl<'a, P> FusedIterator for RMatches<'a, P> where P: Pattern<'a, Searcher: ReverseSearcher<'a>> {}

impl<'a, P> DoubleEndedIterator for Matches<'a, P>
where
    P: Pattern<'a, Searcher: DoubleEndedSearcher<'a>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Vec<u8>> {
        self.0.next_back()
    }
}

impl<'a, P> DoubleEndedIterator for RMatches<'a, P>
where
    P: Pattern<'a, Searcher: DoubleEndedSearcher<'a>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Vec<u8>> {
        self.0.next()
    }
}
