use musicode::pattern::*;

#[allow(unused_macros)]
macro_rules! search_asserts {
    ($scale:expr, $pattern:expr, $testname:expr, [$($func:ident),*], $result:expr) => {
        let mut searcher = $pattern.into_searcher($scale);
        let arr = [$( Step::from(searcher.$func()) ),*];
        assert_eq!(&arr[..], &$result, $testname);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Step<const N: usize> {
    Matches([usize; N]),
    Rejects([Option<usize>; N]),
    Done,
}

use self::Step::*;

impl<const N: usize> From<SearchStep<N>> for Step<N> {
    fn from(x: SearchStep<N>) -> Self {
        match x {
            SearchStep::Match(a) => Matches(a),
            SearchStep::Reject(a) => Rejects(a),
            SearchStep::Done => Done,
        }
    }
}

impl<const N: usize> From<Option<[usize; N]>> for Step<N> {
    fn from(x: Option<[usize; N]>) -> Self {
        match x {
            Some(a) => Matches(a),
            None => Done,
        }
    }
}

impl<const N: usize> From<Option<[Option<usize>; N]>> for Step<N> {
    fn from(x: Option<[Option<usize>; N]>) -> Self {
        match x {
            Some(a) => Rejects(a),
            None => Done,
        }
    }
}
#[test]
fn test_simple_interval_iteration() {
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "forward iteration for scale",
        [
            next, next, next, next, next, next, next, next, next, next, next, next, next, next,
            next, next, next, next, next, next, next, next, next, next, next, next
        ],
        [
            Rejects([Some(0), Some(0)]),
            Rejects([Some(0), Some(1)]),
            Matches([0, 2]),
            Rejects([Some(1), Some(1)]),
            Rejects([Some(1), Some(2)]),
            Rejects([Some(1), Some(3)]),
            Rejects([Some(1), Some(4)]),
            Rejects([Some(2), Some(2)]),
            Rejects([Some(2), Some(3)]),
            Rejects([Some(2), Some(4)]),
            Rejects([Some(2), Some(5)]),
            Rejects([Some(3), Some(3)]),
            Rejects([Some(3), Some(4)]),
            Matches([3, 5]),
            Rejects([Some(4), Some(4)]),
            Rejects([Some(4), Some(5)]),
            Matches([4, 6]),
            Rejects([Some(5), Some(5)]),
            Rejects([Some(5), Some(6)]),
            Rejects([Some(5), Some(0)]),
            Rejects([Some(5), Some(1)]),
            Rejects([Some(6), Some(6)]),
            Rejects([Some(6), Some(0)]),
            Rejects([Some(6), Some(1)]),
            Rejects([Some(6), Some(2)]),
            Done,
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "reverse iteration for scale",
        [
            next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back,
            next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back,
            next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back,
            next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back,
            next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back,
            next_back, next_back, next_back
        ],
        [
            Rejects([Some(6), Some(6)]),
            Rejects([Some(6), Some(5)]),
            Rejects([Some(6), Some(4)]),
            Rejects([Some(6), Some(3)]),
            Rejects([Some(6), Some(2)]),
            Rejects([Some(6), Some(1)]),
            Rejects([Some(5), Some(5)]),
            Rejects([Some(5), Some(4)]),
            Rejects([Some(5), Some(3)]),
            Rejects([Some(5), Some(2)]),
            Rejects([Some(5), Some(1)]),
            Rejects([Some(5), Some(0)]),
            Rejects([Some(4), Some(4)]),
            Rejects([Some(4), Some(3)]),
            Rejects([Some(4), Some(2)]),
            Rejects([Some(4), Some(1)]),
            Rejects([Some(4), Some(0)]),
            Matches([4, 6]),
            Rejects([Some(3), Some(3)]),
            Rejects([Some(3), Some(2)]),
            Rejects([Some(3), Some(1)]),
            Rejects([Some(3), Some(0)]),
            Rejects([Some(3), Some(6)]),
            Matches([3, 5]),
            Rejects([Some(2), Some(2)]),
            Rejects([Some(2), Some(1)]),
            Rejects([Some(2), Some(0)]),
            Rejects([Some(2), Some(6)]),
            Rejects([Some(2), Some(5)]),
            Rejects([Some(2), Some(4)]),
            Rejects([Some(1), Some(1)]),
            Rejects([Some(1), Some(0)]),
            Rejects([Some(1), Some(6)]),
            Rejects([Some(1), Some(5)]),
            Rejects([Some(1), Some(4)]),
            Rejects([Some(1), Some(3)]),
            Rejects([Some(0), Some(0)]),
            Rejects([Some(0), Some(6)]),
            Rejects([Some(0), Some(5)]),
            Rejects([Some(0), Some(4)]),
            Rejects([Some(0), Some(3)]),
            Matches([0, 2]),
            Done,
        ]
    );
}

#[test]
fn test_simple_interval_search() {
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "next_match for scale",
        [next_match, next_match, next_match, next_match],
        [Matches([0, 2]), Matches([3, 5]), Matches([4, 6]), Done]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "next_match_back for scale",
        [
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back
        ],
        [Matches([4, 6]), Matches([3, 5]), Matches([0, 2]), Done]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        12u8,
        "next_match_back for scale",
        [
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back
        ],
        [
            Matches([6, 6]),
            Matches([5, 5]),
            Matches([4, 4]),
            Matches([3, 3]),
            Matches([2, 2]),
            Matches([1, 1]),
            Matches([0, 0]),
            Done
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "next_reject for scale",
        [
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject,
            next_reject
        ],
        [
            Rejects([Some(0), Some(0)]),
            Rejects([Some(0), Some(1)]),
            Rejects([Some(1), Some(1)]),
            Rejects([Some(1), Some(2)]),
            Rejects([Some(1), Some(3)]),
            Rejects([Some(1), Some(4)]),
            Rejects([Some(2), Some(2)]),
            Rejects([Some(2), Some(3)]),
            Rejects([Some(2), Some(4)]),
            Rejects([Some(2), Some(5)]),
            Rejects([Some(3), Some(3)]),
            Rejects([Some(3), Some(4)]),
            Rejects([Some(4), Some(4)]),
            Rejects([Some(4), Some(5)]),
            Rejects([Some(5), Some(5)]),
            Rejects([Some(5), Some(6)]),
            Rejects([Some(5), Some(0)]),
            Rejects([Some(5), Some(1)]),
            Rejects([Some(6), Some(6)]),
            Rejects([Some(6), Some(0)]),
            Rejects([Some(6), Some(1)]),
            Rejects([Some(6), Some(2)]),
            Done,
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "next_reject_back for scale",
        [
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back,
            next_reject_back
        ],
        [
            Rejects([Some(6), Some(6)]),
            Rejects([Some(6), Some(5)]),
            Rejects([Some(6), Some(4)]),
            Rejects([Some(6), Some(3)]),
            Rejects([Some(6), Some(2)]),
            Rejects([Some(6), Some(1)]),
            Rejects([Some(5), Some(5)]),
            Rejects([Some(5), Some(4)]),
            Rejects([Some(5), Some(3)]),
            Rejects([Some(5), Some(2)]),
            Rejects([Some(5), Some(1)]),
            Rejects([Some(5), Some(0)]),
            Rejects([Some(4), Some(4)]),
            Rejects([Some(4), Some(3)]),
            Rejects([Some(4), Some(2)]),
            Rejects([Some(4), Some(1)]),
            Rejects([Some(4), Some(0)]),
            Rejects([Some(3), Some(3)]),
            Rejects([Some(3), Some(2)]),
            Rejects([Some(3), Some(1)]),
            Rejects([Some(3), Some(0)]),
            Rejects([Some(3), Some(6)]),
            Rejects([Some(2), Some(2)]),
            Rejects([Some(2), Some(1)]),
            Rejects([Some(2), Some(0)]),
            Rejects([Some(2), Some(6)]),
            Rejects([Some(2), Some(5)]),
            Rejects([Some(2), Some(4)]),
            Rejects([Some(1), Some(1)]),
            Rejects([Some(1), Some(0)]),
            Rejects([Some(1), Some(6)]),
            Rejects([Some(1), Some(5)]),
            Rejects([Some(1), Some(4)]),
            Rejects([Some(1), Some(3)]),
            Rejects([Some(0), Some(0)]),
            Rejects([Some(0), Some(6)]),
            Rejects([Some(0), Some(5)]),
            Rejects([Some(0), Some(4)]),
            Rejects([Some(0), Some(3)]),
            Done,
        ]
    );
}

#[test]
fn double_ended_regression() {
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "alternating double ended search",
        [next_match, next_match_back, next_match, next_match_back],
        [Matches([0, 2]), Matches([4, 6]), Matches([3, 5]), Done]
    );
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        4u8,
        "triple double ended search for 4",
        [
            next_match,
            next_match_back,
            next_match_back,
            next_match_back
        ],
        [Matches([0, 2]), Matches([4, 6]), Matches([3, 5]), Done]
    );
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        3u8,
        "triple double ended search for 3",
        [
            next_match,
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back
        ],
        [
            Matches([1, 3]),
            Matches([6, 1]),
            Matches([5, 0]),
            Matches([2, 4]),
            Done
        ]
    );
}

#[test]
fn test_simple_chord_iteration() {
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        &[0, 4, 7],
        "forward iteration for scale",
        [next, next, next, next, next, next, next, next],
        [
            Matches([0, 2, 4]),
            Rejects([Some(1), Some(4), None]),
            Rejects([Some(2), Some(5), None]),
            Matches([3, 5, 0]),
            Matches([4, 6, 1]),
            Rejects([Some(5), Some(1), None]),
            Rejects([Some(6), Some(2), None]),
            Done
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        &[0, 4, 7],
        "reverse iteration for scale",
        [next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back],
        [
            Rejects([None, None, Some(3)]),
            Rejects([None, Some(0), Some(2)]),
            Matches([4, 6, 1]),
            Matches([3, 5, 0]),
            Rejects([None, Some(4), Some(6)]),
            Rejects([None, Some(3), Some(5)]),
            Matches([0, 2, 4]),
            Done
        ]
    );
}

#[test]
fn test_simple_chord_search() {
    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        &[0, 4, 7],
        "next_match for scale",
        [next_match, next_match, next_match, next_match],
        [
            Matches([0, 2, 4]),
            Matches([3, 5, 0]),
            Matches([4, 6, 1]),
            Done
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        &[0, 4, 7],
        "next_match_back for scale",
        [
            next_match_back,
            next_match_back,
            next_match_back,
            next_match_back
        ],
        [
            Matches([4, 6, 1]),
            Matches([3, 5, 0]),
            Matches([0, 2, 4]),
            Done
        ]
    );
}

// #[bench]
// fn bench_chord_next_match(b: &mut Bencher) {
//     let mut searcher = [0, 4, 7].into_searcher(&[0, 2, 4, 5, 7, 9, 11]);
//     b.iter(|| searcher.next_match());
// }

// #[bench]
// fn bench_chord_next_match_back(b: &mut Bencher) {
//     let mut searcher = [0, 4, 7].into_searcher(&[0, 2, 4, 5, 7, 9, 11]);
//     b.iter(|| searcher.next_match_back());
// }
