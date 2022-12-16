use advent_2022::day15::{parse_line, solve_a, solve_b};
use advent_2022::utils::{read_lines, Pos};

const TEST_FILE: &str = "./test_data/day15.txt";

#[test]
fn test_parse_line() {
    let s = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
    let sensor = parse_line(s);
    assert_eq!(sensor.pos, Pos(2, 18));
    assert_eq!(sensor.beacon, Pos(-2, 15));
    assert_eq!(sensor.beacon_dist, 7);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data, 10), 26);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data, 20), Some(56000011));
}
