use advent_2022::day02::{decode_move, score_round_a, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day02.txt";

#[test]
fn test_decode_move() {
    assert_eq!(decode_move('X'), 0);
    assert_eq!(decode_move('Y'), 1);
    assert_eq!(decode_move('Z'), 2);
    assert_eq!(decode_move('A'), 0);
    assert_eq!(decode_move('B'), 1);
    assert_eq!(decode_move('C'), 2);
}

#[test]
fn test_score_round_a() {
    assert_eq!(score_round_a("A Y"), 8);
    assert_eq!(score_round_a("B X"), 1);
    assert_eq!(score_round_a("C Z"), 6);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 15);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 12);
}
