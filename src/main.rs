use advent_2022::day17::{solve_a, solve_b};
use advent_2022::utils::read_lines;

fn main() {
    let data = read_lines("./data/day17.txt");
    println!("A: {}", solve_a(&data, 2022));
    println!("B: {}", solve_b(&data));
}
