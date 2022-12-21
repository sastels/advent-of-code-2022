use advent_2022::day16::{solve_a, solve_b};
use advent_2022::utils::read_lines;

fn main() {
    let data = read_lines("./data/day16.txt");
    println!("A: {}", solve_a(&data));
    println!("B: {}", solve_b(&data));
}
