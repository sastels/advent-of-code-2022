use advent_2022::utils::read_lines;
use advent_2022::{day01, day02, day03, day04, day05, day06, day07};
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
        "4" => {
            let data = read_lines("./data/day04.txt");
            println!("Day 4 A: {}", day04::solve_a(&data));
            println!("Day 4 B: {}", day04::solve_b(&data));
        }
        "5" => {
            let data = read_lines("./data/day05.txt");
            println!("Day 5 A: {}", day05::solve_a(&data));
            println!("Day 5 B: {}", day05::solve_b(&data));
        }
        "6" => {
            let data = &read_lines("./data/day06.txt")[0];
            println!("Day {} A: {}", day.as_str(), day06::solve_a(data));
            println!("Day {} B: {}", day.as_str(), day06::solve_b(data));
        }
        "7" => {
            let data = read_lines("./data/day07.txt");
            println!("Day {} A: {}", day.as_str(), day07::solve_a(&data));
            println!("Day {} B: {}", day.as_str(), day07::solve_b(&data));
        }

        _ => panic!("usage: cargo run n for n in 1..=24"),
    }
}
