use std::error::Error;

use regex::Regex;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("The regex should compile");
    Ok(Solution::default()
        .part1(part1(input, &mul_re))
        .part2(part2(input, &mul_re)))
}

fn parse(input: &str) -> u32 {
    input.parse().expect("The input should be a number")
}

fn solve(input: &str, mul_re: &Regex) -> u32 {
    mul_re
        .captures_iter(input)
        .map(|captures| {
            let (_, [a, b]) = captures.extract();
            parse(a) * parse(b)
        })
        .sum()
}

#[must_use]
pub fn part1(input: &str, mul_re: &Regex) -> u32 {
    solve(input, mul_re)
}

#[must_use]
pub fn part2(input: &str, mul_re: &Regex) -> u32 {
    let regions = Regex::new(r"(?s:^.*?don't\(\))|do\(\)(?s:.*?)don't|(?s:do\(\).*?$)")
        .expect("The regex should compile");
    regions
        .find_iter(input)
        .map(|matches| solve(matches.as_str(), mul_re))
        .sum()
}
