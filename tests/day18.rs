use advent_2022::day18::{find_interior, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day18.txt";

#[test]
fn test_find_interior() {
    let data = read_lines(TEST_FILE);
    let cubes = advent_2022::day18::parse_input(&data);
    assert_eq!(find_interior(&cubes), vec![(2, 2, 5)]);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 64);
}

#[ignore]
#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 58);
}
