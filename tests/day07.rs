use advent_2022::day07::{parse_chdir, parse_dir_listing, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day07.txt";

#[test]
fn test_parse_chdir() {
    assert_eq!(parse_chdir("$ cd /home"), Some("/home".to_string()));
    assert_eq!(parse_chdir("$ ls"), None);
}

#[test]
fn test_parse_dir_listing() {
    assert_eq!(parse_dir_listing("123 abc"), Some((123, "abc".to_string())));
    assert_eq!(parse_dir_listing("abc"), None);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 95437);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 24933642);
}
