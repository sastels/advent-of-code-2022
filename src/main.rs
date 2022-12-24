use advent_2022::day16::{solve_a, solve_b};
use advent_2022::utils::read_lines;

fn main() {
    let data = read_lines("./data/day16.txt");
    println!("A: {}", solve_a(&data, 11, 10000000));
    println!("B: {}", solve_b(&data, 10000000));
}
