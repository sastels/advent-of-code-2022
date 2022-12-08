use advent_2022::day08::{is_visible, parse_forest, scene_score, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day08.txt";

#[test]
fn test_parse_forest() {
    let data = vec!["123".to_string()];
    assert_eq!(parse_forest(&data), vec![vec![1, 2, 3]]);
    let data = vec!["123".to_string(), "456".to_string()];
    assert_eq!(parse_forest(&data), vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_is_visible() {
    let data = read_lines(TEST_FILE);
    let forest = parse_forest(&data);
    assert!(is_visible(0, 2, &forest));
    assert!(is_visible(3, 0, &forest));
    assert!(is_visible(4, 3, &forest));
    assert!(is_visible(2, 4, &forest));

    assert!(is_visible(1, 1, &forest));
    assert!(is_visible(2, 1, &forest));
    assert!(!is_visible(3, 1, &forest));
    assert!(is_visible(1, 2, &forest));
    assert!(!is_visible(2, 2, &forest));
    assert!(is_visible(3, 2, &forest));
    assert!(is_visible(2, 3, &forest));
    assert!(!is_visible(1, 3, &forest));
    assert!(!is_visible(3, 3, &forest));
}

#[test]
fn test_scene_score() {
    let data = read_lines(TEST_FILE);
    let forest = parse_forest(&data);
    assert_eq!(scene_score(2, 1, &forest), 4);
    assert_eq!(scene_score(2, 3, &forest), 8);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 21);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 8);
}
