use anyhow::Result;
use crate::utils::input;
use std::str::FromStr;
use itertools::Itertools;
pub fn solve() -> Result<()> {
    let input = input::read_file(2)?;

    println!("Day 02 - Part 1: {}", solve_part1(&input)?);
    println!("Day 02 - Part 2: {}", solve_part2(&input)?);

    Ok(())
}

fn read_reports(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| i32::from_str(num).unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_report(report: &Vec<i32>) -> bool {
    (report.is_sorted() || report.is_sorted_by(|a, b| a > b))
        && report.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .all(|diff| diff >= 1 && diff <= 3)
}

fn could_be_valid_with_removal(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|i| {
        let mut modified = report.clone();
        modified.remove(i);
        is_valid_report(&modified)
    })
}

fn solve_part1(input: &str) -> Result<String> {
    let reports = read_reports(input);
    let valid_count = reports
        .iter()
        .filter(|report| is_valid_report(report))
        .count();

    Ok(valid_count.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let reports = read_reports(input);
    let valid_count = reports
        .iter()
        .filter(|report| is_valid_report(report) || could_be_valid_with_removal(report))
        .count();

    Ok(valid_count.to_string())
}