use advent_2022::day04::{elves_overlap, elves_redundant, elves_sections, solve_a, solve_b};
use advent_2022::utils::read_lines;

const TEST_FILE: &str = "./test_data/day04.txt";

#[test]
fn test_elves_sections() {
    assert_eq!(elves_sections("1-2,3-4"), vec![1, 2, 3, 4]);
    assert_eq!(elves_sections("11-22,9-444"), vec![11, 22, 9, 444]);
}

#[test]
fn test_elves_redundant() {
    assert!(elves_redundant(&[2, 3, 1, 4]));
    assert!(elves_redundant(&[12, 200, 33, 54]));
    assert!(!elves_redundant(&[1, 2, 3, 4]));
    assert!(!elves_redundant(&[1, 3, 2, 4]));
}

#[test]
fn test_elves_overlap() {
    assert!(elves_overlap(&elves_sections("5-7,7-9")));
    assert!(elves_overlap(&elves_sections("2-8,3-7")));
    assert!(elves_overlap(&elves_sections("6-6,4-6")));
    assert!(elves_overlap(&elves_sections("2-6,4-8")));
    assert!(!elves_overlap(&elves_sections("2-4,6-8")));
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 2);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 4);
}
