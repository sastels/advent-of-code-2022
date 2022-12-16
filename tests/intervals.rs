use advent_2022::intervals::merge_intervals;

#[test]
fn test_merge_intervals_disjoint() {
    assert_eq!(
        merge_intervals(vec![(1, 2), (6, 10), (3, 4)]),
        vec![(1, 2), (3, 4), (6, 10)]
    );
}

#[test]
fn test_merge_intervals_overlap() {
    assert_eq!(merge_intervals(vec![(1, 2), (2, 3), (3, 4)]), vec![(1, 4)]);
}

#[test]
fn test_merge_intervals_general() {
    assert_eq!(
        merge_intervals(vec![(1, 5), (2, 3), (3, 7), (8, 10), (9, 11)]),
        vec![(1, 7), (8, 11)]
    );
}

#[test]
fn test_merge_intervals_adjacent() {
    assert_eq!(merge_intervals(vec![(1, 2), (3, 4)]), vec![(1, 4)]);
}
