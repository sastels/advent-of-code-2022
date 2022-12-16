use advent_2022::day14::{drop_sand, make_cave, parse_line, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day14.txt";

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("1,2 -> 3,4 -> 44,5"),
        vec![(1, 2), (3, 4), (44, 5)]
    );
}

#[test]
fn test_make_cave() {
    let data = read_lines(TEST_FILE);
    let rocks = data.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    let cave = make_cave(&rocks);
    assert_eq!(cave.len(), 1000);
    assert_eq!(cave[0].len(), 1000);
    assert_eq!(cave[0][0], '.');
    assert_eq!(cave[498][5], '#');
    assert_eq!(cave[498][6], '#');
    assert_eq!(cave[499][6], '.');
    assert_eq!(cave[495][9], '#');
    assert_eq!(cave[494][9], '#');
    assert_eq!(cave[493][9], '.');
}

#[test]
fn test_drop_sand() {
    let data = read_lines(TEST_FILE);
    let rocks = data.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    let mut cave = make_cave(&rocks);
    assert!(drop_sand(&mut cave, 500, 0));
    assert_eq!(cave[500][0], '.');
    assert_eq!(cave[500][8], 'o');
    assert_eq!(cave[500][9], '#');
    assert!(!drop_sand(&mut cave, 100, 0));
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 24);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 93);
}
