use advent_2022::day13::{compare, get_pairs, pop_first, solve_a, solve_b};
use advent_2022::utils::read_lines;
use std::cmp::Ordering::{Equal, Greater, Less};

const TEST_FILE: &str = "./test_data/day13.txt";

#[test]
fn test_pop_first() {
    assert_eq!(
        pop_first("[a,b,c]"),
        (String::from("a"), String::from("[b,c]"))
    );
    assert_eq!(
        pop_first("[[a,b],c]"),
        (String::from("[a,b]"), String::from("[c]"))
    );
    assert_eq!(
        pop_first("[[a,[b],c],d,e]"),
        (String::from("[a,[b],c]"), String::from("[d,e]"))
    );
    assert_eq!(pop_first("[a]"), (String::from("a"), String::from("[]")));
    assert_eq!(pop_first("[]"), (String::from(""), String::from("[]")));
}

#[test]
fn test_get_pairs() {
    let data = read_lines(TEST_FILE);
    let pairs = get_pairs(&data);
    assert_eq!(pairs.len(), 8);
    assert_eq!(
        pairs[0],
        (String::from("[1,1,3,1,1]"), String::from("[1,1,5,1,1]"))
    );
    assert_eq!(pairs[2], (String::from("[9]"), String::from("[[8,7,6]]")));
    assert_eq!(pairs[5], (String::from("[]"), String::from("[3]")));
}

#[test]
fn test_compare() {
    assert_eq!(compare("1", "1"), Equal);
    assert_eq!(compare("3", "12"), Less);
    assert_eq!(compare("12", "3"), Greater);
    assert_eq!(compare("[]", "[]"), Equal);
    assert_eq!(compare("[]", "[1]"), Less);
    assert_eq!(compare("[[]]", "[]"), Greater);
    assert_eq!(compare("[1]", "[]"), Greater);
    assert_eq!(compare("[1]", "[1]"), Equal);
    assert_eq!(compare("[[[1]]]", "[1]"), Equal);
    assert_eq!(compare("[[1], 3]", "[2]"), Less);
    assert_eq!(compare("[1,1,3,1,1]", "[1,1,5,1,1]"), Less);
}

#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data), 13);
}

#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data), 140);
}
