use advent_of_code_2024::solutions;

fn main() {
    println!("Advent of Code 2024");

    if let Err(e) = solutions::day01::solve() {
        eprintln!("Error solving day 01: {}", e);
    }

    if let Err(e) = solutions::day02::solve() {
        eprintln!("Error solving day 02: {}", e);
    }
}