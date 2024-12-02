use std::collections::{HashMap, HashSet};
use anyhow::Result;  // Import Result type for error handling
use crate::utils::input;  // Import custom input utilities

pub fn solve() -> Result<()> {
    let input = input::read_file(1)?;

    println!("Day 01 - Part 1: {}", solve_part1(&input)?);
    println!("Day 01 - Part 2: {}", solve_part2(&input)?);

    Ok(())
}

fn parse_sort_lists(content: &str) -> (Vec<i32>, Vec<i32>) {
    // Convert input lines into pairs of numbers and unzip them into separate vectors
    let mut lists: (Vec<i32>, Vec<i32>) = content
        .lines()  // Split input into lines
        .map(|line| {
            let mut nums = line.split_whitespace();  // Split each line into number strings
            (
                // Parse first number, unwrap assumes valid input
                nums.next().unwrap().parse::<i32>().unwrap(),
                // Parse second number, unwrap assumes valid input
                nums.next().unwrap().parse::<i32>().unwrap()
            )
        })
        .unzip();  // Separate pairs into two vectors

    // Sort both lists using partial_cmp for floating-point comparison
    lists.0.sort_unstable();
    lists.1.sort_unstable();

    lists
}

fn solve_part1(input: &str) -> Result<String> {
    // Parse input into two sorted lists
    let (left, right) = parse_sort_lists(input);

    let mut sum = 0;

    // Calculate sum of absolute differences between corresponding elements
    for (i, x) in left.iter().enumerate() {
        let y = right[i];
        let diff = (x - y).abs();
        sum += diff;
    }

    // Convert result to string and wrap in Ok
    Ok(sum.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    // Parse input into two sorted lists
    let (left, right) = parse_sort_lists(input);
    let left_unique: Vec<i32> = left.into_iter().collect::<HashSet<_>>().into_iter().collect();
    let mut frequencies = HashMap::new();
    for num in right {
        *frequencies.entry(num).or_insert(0) += 1;
    }

    let mut similarity = 0;

    // Calculate similarity score
    for x in left_unique.iter() {
        if frequencies.contains_key(x) {
            similarity += x * frequencies.get(x).unwrap();
        }
    }

    // Convert result to string
    Ok(similarity.to_string())
}