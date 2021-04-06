use musicode::IntervalSet;

#[test]
fn test_simple() {
    let mut set: IntervalSet = IntervalSet::new();

    assert_eq!(set.insert(5), (0, true));
    assert_eq!(set.insert(3), (0, true));
    assert_eq!(set.insert(4), (1, true));
    assert_eq!(set.insert(4), (1, false));
    assert_eq!(set.find_or_insert(4), Ok(1));
    assert_eq!(set.len(), 3);
    assert_eq!(set.binary_search(&3), Ok(0));
}
