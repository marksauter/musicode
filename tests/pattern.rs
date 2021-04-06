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
enum Step {
    Matches(Vec<usize>),
    Rejects(Vec<usize>),
    Indices(Vec<usize>),
    Done,
}

use self::Step::*;

impl From<SearchStep> for Step {
    fn from(x: SearchStep) -> Self {
        match x {
            SearchStep::Match(a) => Matches(a),
            SearchStep::Reject(a) => Rejects(a),
            SearchStep::Done => Done,
        }
    }
}

impl From<Option<Vec<usize>>> for Step {
    fn from(x: Option<Vec<usize>>) -> Self {
        match x {
            Some(a) => Indices(a),
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
            Rejects(vec![0, 0]),
            Rejects(vec![0, 1]),
            Matches(vec![0, 2]),
            Rejects(vec![1, 1]),
            Rejects(vec![1, 2]),
            Rejects(vec![1, 3]),
            Rejects(vec![1, 4]),
            Rejects(vec![2, 2,]),
            Rejects(vec![2, 3]),
            Rejects(vec![2, 4]),
            Rejects(vec![2, 5]),
            Rejects(vec![3, 3]),
            Rejects(vec![3, 4]),
            Matches(vec![3, 5]),
            Rejects(vec![4, 4]),
            Rejects(vec![4, 5]),
            Matches(vec![4, 6]),
            Rejects(vec![5, 5]),
            Rejects(vec![5, 6]),
            Rejects(vec![5, 0]),
            Rejects(vec![5, 1]),
            Rejects(vec![6, 6]),
            Rejects(vec![6, 0]),
            Rejects(vec![6, 1]),
            Rejects(vec![6, 2]),
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
            Rejects(vec![6, 6]),
            Rejects(vec![6, 5]),
            Rejects(vec![6, 4]),
            Rejects(vec![6, 3]),
            Rejects(vec![6, 2]),
            Rejects(vec![6, 1]),
            Rejects(vec![5, 5]),
            Rejects(vec![5, 4]),
            Rejects(vec![5, 3]),
            Rejects(vec![5, 2]),
            Rejects(vec![5, 1]),
            Rejects(vec![5, 0]),
            Rejects(vec![4, 4]),
            Rejects(vec![4, 3]),
            Rejects(vec![4, 2]),
            Rejects(vec![4, 1]),
            Rejects(vec![4, 0]),
            Matches(vec![4, 6]),
            Rejects(vec![3, 3]),
            Rejects(vec![3, 2]),
            Rejects(vec![3, 1]),
            Rejects(vec![3, 0]),
            Rejects(vec![3, 6]),
            Matches(vec![3, 5]),
            Rejects(vec![2, 2]),
            Rejects(vec![2, 1]),
            Rejects(vec![2, 0]),
            Rejects(vec![2, 6]),
            Rejects(vec![2, 5]),
            Rejects(vec![2, 4]),
            Rejects(vec![1, 1]),
            Rejects(vec![1, 0]),
            Rejects(vec![1, 6]),
            Rejects(vec![1, 5]),
            Rejects(vec![1, 4]),
            Rejects(vec![1, 3]),
            Rejects(vec![0, 0]),
            Rejects(vec![0, 6]),
            Rejects(vec![0, 5]),
            Rejects(vec![0, 4]),
            Rejects(vec![0, 3]),
            Matches(vec![0, 2]),
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
        [
            Indices(vec![0, 2]),
            Indices(vec![3, 5]),
            Indices(vec![4, 6]),
            Done
        ]
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
        [
            Indices(vec![4, 6]),
            Indices(vec![3, 5]),
            Indices(vec![0, 2]),
            Done
        ]
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
            Indices(vec![6, 6]),
            Indices(vec![5, 5]),
            Indices(vec![4, 4]),
            Indices(vec![3, 3]),
            Indices(vec![2, 2]),
            Indices(vec![1, 1]),
            Indices(vec![0, 0]),
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
            Indices(vec![0, 0]),
            Indices(vec![0, 1]),
            Indices(vec![1, 1]),
            Indices(vec![1, 2]),
            Indices(vec![1, 3]),
            Indices(vec![1, 4]),
            Indices(vec![2, 2]),
            Indices(vec![2, 3]),
            Indices(vec![2, 4]),
            Indices(vec![2, 5]),
            Indices(vec![3, 3]),
            Indices(vec![3, 4]),
            Indices(vec![4, 4]),
            Indices(vec![4, 5]),
            Indices(vec![5, 5]),
            Indices(vec![5, 6]),
            Indices(vec![5, 0]),
            Indices(vec![5, 1]),
            Indices(vec![6, 6]),
            Indices(vec![6, 0]),
            Indices(vec![6, 1]),
            Indices(vec![6, 2]),
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
            Indices(vec![6, 6]),
            Indices(vec![6, 5]),
            Indices(vec![6, 4]),
            Indices(vec![6, 3]),
            Indices(vec![6, 2]),
            Indices(vec![6, 1]),
            Indices(vec![5, 5]),
            Indices(vec![5, 4]),
            Indices(vec![5, 3]),
            Indices(vec![5, 2]),
            Indices(vec![5, 1]),
            Indices(vec![5, 0]),
            Indices(vec![4, 4]),
            Indices(vec![4, 3]),
            Indices(vec![4, 2]),
            Indices(vec![4, 1]),
            Indices(vec![4, 0]),
            Indices(vec![3, 3]),
            Indices(vec![3, 2]),
            Indices(vec![3, 1]),
            Indices(vec![3, 0]),
            Indices(vec![3, 6]),
            Indices(vec![2, 2]),
            Indices(vec![2, 1]),
            Indices(vec![2, 0]),
            Indices(vec![2, 6]),
            Indices(vec![2, 5]),
            Indices(vec![2, 4]),
            Indices(vec![1, 1]),
            Indices(vec![1, 0]),
            Indices(vec![1, 6]),
            Indices(vec![1, 5]),
            Indices(vec![1, 4]),
            Indices(vec![1, 3]),
            Indices(vec![0, 0]),
            Indices(vec![0, 6]),
            Indices(vec![0, 5]),
            Indices(vec![0, 4]),
            Indices(vec![0, 3]),
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
        [
            Indices(vec![0, 2]),
            Indices(vec![4, 6]),
            Indices(vec![3, 5]),
            Done
        ]
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
        [
            Indices(vec![0, 2]),
            Indices(vec![4, 6]),
            Indices(vec![3, 5]),
            Done
        ]
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
            Indices(vec![1, 3]),
            Indices(vec![6, 1]),
            Indices(vec![5, 0]),
            Indices(vec![2, 4]),
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
            Matches(vec![0, 2, 4]),
            Rejects(vec![1, 4]),
            Rejects(vec![2, 5]),
            Matches(vec![3, 5, 0]),
            Matches(vec![4, 6, 1]),
            Rejects(vec![5, 1]),
            Rejects(vec![6, 2]),
            Done
        ]
    );

    search_asserts!(
        &[0, 2, 4, 5, 7, 9, 11],
        &[0, 4, 7],
        "reverse iteration for scale",
        [next_back, next_back, next_back, next_back, next_back, next_back, next_back, next_back],
        [
            Rejects(vec![6, 2]),
            Rejects(vec![5, 1]),
            Matches(vec![4, 6, 1]),
            Matches(vec![3, 5, 0]),
            Rejects(vec![2, 5]),
            Rejects(vec![1, 4]),
            Matches(vec![0, 2, 4]),
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
            Indices(vec![0, 2, 4]),
            Indices(vec![3, 5, 0]),
            Indices(vec![4, 6, 1]),
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
            Indices(vec![4, 6, 1]),
            Indices(vec![3, 5, 0]),
            Indices(vec![0, 2, 4]),
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
