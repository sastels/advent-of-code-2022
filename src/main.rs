use advent_2022::utils::read_lines;
use advent_2022::{day01, day02, day03};
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
        "2" => {
            let data = read_lines("./data/day02.txt");
            println!("Day 2 A: {}", day02::solve_a(&data));
            println!("Day 2 B: {}", day02::solve_b(&data));
        }
        "3" => {
            let data = read_lines("./data/day03.txt");
            println!("Day 3 A: {}", day03::solve_a(&data));
            println!("Day 3 B: {}", day03::solve_b(&data));
        }

        _ => panic!("usage: cargo run n for n in 1..=24"),
    }
}
