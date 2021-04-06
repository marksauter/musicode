use musicode::Scale;

#[should_panic]
#[test]
fn test_interval_panic() {
    let mut set = Scale::new();
    set.insert(12);
}
