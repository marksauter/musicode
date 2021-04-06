use musicode::scale;

#[test]
fn test_pitches() {
    use musicode::Pitch::*;
    let scale = scale![0, 2, 4, 5, 7, 9, 11];
    let pitches = [C(0), D(0), E(0), F(0), G(0), A(0), B(0)];

    let mut pos = 0;
    let it = scale.pitches(C(0));

    for p in it {
        assert_eq!(p, pitches[pos]);
        pos += 1;
    }
    assert_eq!(pos, pitches.len());
    assert_eq!(scale.pitches(C(0)).count(), pitches.len());
}

#[test]
fn test_rev_pitches() {
    use musicode::Pitch::*;
    let scale = scale![0, 2, 4, 5, 7, 9, 11];
    let pitches = [B(0), A(0), G(0), F(0), E(0), D(0), C(0)];

    let mut pos = 0;
    let it = scale.pitches(C(0)).rev();

    for p in it {
        assert_eq!(p, pitches[pos]);
        pos += 1;
    }
    assert_eq!(pos, pitches.len());
}

#[test]
fn double_ended_matches() {
    let res = [vec![0, 4, 7], vec![5, 9, 0], vec![7, 11, 2]];
    let scale = scale![0, 2, 4, 5, 7, 9, 11];
    let fwd_vec: Vec<_> = scale.matches(&[0, 4, 7]).collect();
    assert_eq!(fwd_vec, res);

    let mut bwd_vec: Vec<_> = scale.rmatches(&[0, 4, 7]).collect();
    bwd_vec.reverse();
    assert_eq!(bwd_vec, res);
}

#[test]
fn double_ended_match_indices() {
    let res = [(0, vec![0, 4, 7]), (3, vec![5, 9, 0]), (4, vec![7, 11, 2])];
    let scale = scale![0, 2, 4, 5, 7, 9, 11];
    let fwd_vec: Vec<_> = scale.match_indices(&[0, 4, 7]).collect();
    assert_eq!(fwd_vec, res);

    let mut bwd_vec: Vec<_> = scale.rmatch_indices(&[0, 4, 7]).collect();
    bwd_vec.reverse();
    assert_eq!(bwd_vec, res);
}
