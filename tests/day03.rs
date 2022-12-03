use advent_2022::day03::{intersect_sets, priority, solve_a, solve_b, split_rucksack};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day03.txt";

#[test]
fn test_split_rucksack() {
    assert_eq!(split_rucksack("12"), ("1", "2"));
    assert_eq!(split_rucksack("1234"), ("12", "34"));
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('Z'), 52);
}

#[test]
fn test_intersect_sets() {
    let sets = vec![
        "abc".chars().collect(),
        "bcd".chars().collect(),
        "cde".chars().collect(),
    ];
    assert_eq!(intersect_sets(&sets), "c".chars().collect());
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 157);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 70);
}
