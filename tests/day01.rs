use advent_2022::day01;
use advent_2022::utils::read_lines;

#[test]
fn test_day1_solve_a() {
    let data = read_lines("./test_data/day01.txt");
    assert_eq!(day01::solve_a(&data), 24000);
}

#[test]
fn test_day1_solve_b() {
    let data = read_lines("./test_data/day01.txt");
    assert_eq!(day01::solve_b(&data), 45000);
}
