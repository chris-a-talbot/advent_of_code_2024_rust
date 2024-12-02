use anyhow::Result;  // Import Result type for error handling
use crate::utils::input;  // Import custom input utilities

pub fn solve() -> Result<()> {
    let input = input::read_file(1)?;

    println!("Day XX - Part 1: {}", solve_part1(&input)?);
    println!("Day XX - Part 2: {}", solve_part2(&input)?);

    Ok(())
}

fn solve_part1(input: &str) -> Result<String> {
    Ok("Not implemented".to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    Ok("Not implemented".to_string())

}