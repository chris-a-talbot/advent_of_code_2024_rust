use anyhow::Result;  // Import Result type for error handling
use crate::utils::input;  // Import custom input utilities
use regex::Regex;
use once_cell::unsync::Lazy;

pub fn solve() -> Result<()> {
    let input = input::read_file(3)?;

    println!("Day 03 - Part 1: {}", solve_part1(&input)?);
    println!("Day 03 - Part 2: {}", solve_part2(&input)?);

    Ok(())
}

static MUL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern")
});

static MUL_PATTERN_STATES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern")
});


/// Finds multiplication expressions in the form mul(X,Y) where X,Y are numbers 0-999,
/// returning the operands and position of each match.
fn find_mul_patterns(input: &str) -> Vec<((u64, u64), usize)> {
    MUL_PATTERN
        .captures_iter(input)
        .filter_map(|cap| {
            let position = cap.get(0)?.start();
            let x = cap[1].parse().ok()?;
            let y = cap[2].parse().ok()?;

            Some(((x, y), position))
        })
        .collect()
}

/// Returns the sum of all number pairs found in mul(X,Y) expressions.
fn calculate_mul_sum(input: &str) -> u64 {
    find_mul_patterns(input)
        .into_iter()
        .map(|((x, y), _)| x * y)
        .sum()
}

fn solve_part1(input: &str) -> Result<String> {
    Ok(calculate_mul_sum(input).to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    Ok("Not implemented".to_string())

}