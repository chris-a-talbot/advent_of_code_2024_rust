use std::fs;
use anyhow::Result;

pub fn read_file(day: u8) -> Result<String> {
    let filename = format!("inputs/day{:02}.txt", day);
    Ok(fs::read_to_string(filename)?)
}
