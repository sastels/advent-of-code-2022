use advent_2022::day01::{solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day01.txt";

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 24000);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 45000);
}
