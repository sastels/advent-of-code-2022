use advent_2022::day09::{move_tail, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day09.txt";

#[test]
fn test_move_tail() {
    assert_eq!(move_tail((1, 1), (1, 1)), (1, 1));
    assert_eq!(move_tail((1, 1), (0, 0)), (0, 0));
    assert_eq!(move_tail((1, 0), (1, 1)), (1, 1));
    assert_eq!(move_tail((2, 1), (1, 1)), (1, 1));

    assert_eq!(move_tail((3, 2), (1, 1)), (2, 2));
    assert_eq!(move_tail((2, 3), (2, 1)), (2, 2));
    assert_eq!(move_tail((3, 4), (1, 4)), (2, 4));
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 13);
}

#[test]
fn test_solve_b() {
    let data = read_lines("./test_data/day09-2.txt");
    assert_eq!(solve_b(&data), 36);
}
