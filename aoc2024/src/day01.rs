use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

#[must_use]
pub fn part1(input: &str) -> u32 {
    todo!()
}

#[must_use]
pub fn part2(input: &str) -> u32 {
    todo!()
}
