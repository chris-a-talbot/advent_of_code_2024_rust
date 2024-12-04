use anyhow::Result;  // Import Result type for error handling
use crate::utils::input;  // Import custom input utilities
use regex::Regex;
use once_cell::sync::Lazy;

pub fn solve() -> Result<()> {
    let input = input::read_file(3)?;

    println!("Day 03 - Part 1: {}", solve_part1(&input)?);
    println!("Day 03 - Part 2: {}", solve_part2(&input)?);

    Ok(())
}

static MUL_PATTERN: Lazy<Regex> = Lazy::new(||
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap()
);

static STATE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap()
});

fn solve_part1(input: &str) -> Result<String> {
    Ok(MUL_PATTERN
        .captures_iter(input)
        .filter_map(|cap| {
            Some(
                cap.get(1)?.as_str().parse::<u64>().ok()?
                    * cap.get(2)?.as_str().parse::<u64>().ok()?
            )
        })
        .sum::<u64>()
        .to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut sum = 0u64;
    let mut allowed = true;  // Start allowing muls until first don't()

    for capture in STATE_PATTERN.find_iter(input) {
        match capture.as_str() {
            "don't()" => allowed = false,
            "do()" => allowed = true,
            mul_expr if allowed && mul_expr.starts_with("mul") => {
                if let Some(caps) = MUL_PATTERN.captures(mul_expr) {
                    if let (Ok(x), Ok(y)) = (
                        caps[1].parse::<u64>(),
                        caps[2].parse::<u64>()
                    ) {
                        sum += x * y;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(sum.to_string())
}