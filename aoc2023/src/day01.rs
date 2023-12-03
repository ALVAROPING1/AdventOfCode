use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

#[must_use]
pub fn part1(input: &str) -> u32 {
    process_lines(input, str::bytes, |c| (c as char).to_digit(10))
}

#[must_use]
pub fn part2(input: &str) -> u32 {
    #[allow(clippy::cast_possible_truncation)]
    fn check_digit(input: &str) -> Option<u32> {
        static NUMBERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for (n, num) in NUMBERS.iter().enumerate() {
            if input.starts_with(num) {
                return Some(1 + n as u32);
            }
        }
        (input.as_bytes()[0] as char).to_digit(10)
    }

    process_lines(
        input,
        |line| (0..line.len()).map(|i| &line[i..]),
        check_digit,
    )
}

fn process_lines<'a, M, I, F, T>(input: &'a str, make_iter: M, find_map: F) -> u32
where
    M: Fn(&'a str) -> I,
    I: DoubleEndedIterator<Item = T>,
    F: Copy + Fn(T) -> Option<u32>,
{
    input
        .lines()
        .map(make_iter)
        .filter_map(|mut iter| {
            let first = iter.find_map(find_map)?;
            let last = iter.rev().find_map(find_map).unwrap_or(first);
            Some(first * 10 + last)
        })
        .sum()
}
