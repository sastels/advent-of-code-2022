use advent_2022::day15::{solve_a, solve_b};
use advent_2022::utils::read_lines;

fn main() {
    let data = read_lines("./data/day15.txt");
    println!("A: {}", solve_a(&data, 2000000));
    println!("B: {}", solve_b(&data));
}
