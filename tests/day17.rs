use advent_2022::day17::{find_cycle, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day17.txt";

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data, 2022), 3068);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    find_cycle(&data, 200, 24);

    let prefix = [
        1, 3, 2, 1, 2, 1, 3, 2, 2, 0, 1, 3, 2, 0, 2, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0,
        0, 2, 3, 4,
    ];
    let cycle = [
        0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2,
        0, 0, 2, 3, 4,
    ];
    assert_eq!(solve_b(1000000000000, &prefix, &cycle), 1514285714288);
}
