use advent_2022::day12::{find_start_end, neighbours, parse_input, solve_a, solve_b, Pos};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day12.txt";

#[test]
fn test_find_start_end() {
    let data = read_lines(TEST_FILE);
    assert_eq!(find_start_end(&data), (Pos(0, 0), Pos(2, 5)));
}

#[test]
fn test_parse_input() {
    let data = read_lines(TEST_FILE);
    let map = parse_input(&data);
    assert_eq!(map[0], vec![0, 0, 1, 16, 15, 14, 13, 12]);
    assert_eq!(map[2], vec![0, 2, 2, 18, 25, 25, 23, 10]);
}

#[test]
fn test_neighbours() {
    let data = read_lines(TEST_FILE);
    let map = parse_input(&data);
    assert_eq!(neighbours(&Pos(0, 0), &map), vec![Pos(0, 1), Pos(1, 0)]);
    assert_eq!(
        neighbours(&Pos(1, 2), &map),
        vec![Pos(1, 1), Pos(2, 2), Pos(0, 2)]
    );
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 31);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 29);
}
