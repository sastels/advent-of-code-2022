use advent_2022::day01;
use advent_2022::utils::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("usage: cargo run n")
    }
    let day = &args[1];
    match day.as_str() {
        "1" => {
            let data = read_lines("./data/day01.txt");
            println!("Day 1 A: {}", day01::solve_a(&data));
            println!("Day 1 B: {}", day01::solve_b(&data));
        }
        _ => panic!("usage: cargo run n for n in 1..=24"),
    }
}
