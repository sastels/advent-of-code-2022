use advent_2022::day05::{
    move_crate, move_crate_9001, parse_move_line, parse_stacks, solve_a, solve_b,
};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day05.txt";

#[test]
fn test_parse_stacks() {
    let data = read_lines(TEST_FILE);
    assert_eq!(
        parse_stacks(&data),
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
    );
}

#[test]
fn test_parse_move_line() {
    assert_eq!(parse_move_line("move 1 from 1 to 2"), (1, 1, 2));
    assert_eq!(parse_move_line("move 3 from 1 to 3"), (3, 1, 3));
}

#[test]
fn test_move_crate() {
    let data = read_lines(TEST_FILE);
    let mut stacks = parse_stacks(&data);
    stacks = move_crate(stacks, 1, 2, 1);
    assert!(stacks == vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]);
    stacks = move_crate(stacks, 3, 1, 3);
    assert!(stacks == vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']]);
}

#[test]
fn test_move_crate_9001() {
    let mut stacks = vec![vec!['Z', 'N'], vec![]];
    stacks = move_crate_9001(stacks, 2, 1, 2);
    assert!(stacks == vec![vec![], vec!['Z', 'N']]);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), "CMZ");
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), "MCD");
}
