use advent_2022::day10::{parse_op, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day10.txt";

#[test]
fn test_parse_op() {
    assert_eq!(parse_op("noop"), ("noop".to_string(), 0));
    assert_eq!(parse_op("addx 11"), ("addx".to_string(), 11));
    assert_eq!(parse_op("addx -12"), ("addx".to_string(), -12));
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 13140);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 0);
}
