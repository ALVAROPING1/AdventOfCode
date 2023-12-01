use std::error::Error;

use crate::prelude::*;
use utils_rust::parse;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u32 {
    process_input(input, |line| {
        find_ascii_digit(line.bytes()) * 10 + find_ascii_digit(line.bytes().rev())
    })
}

fn part2(input: &str) -> u32 {
    process_input(input, |line| {
        find_digit(line, 0..line.len()) * 10 + find_digit(line, (0..line.len()).rev())
    })
}

fn process_input(input: &str, fun: impl Fn(&str) -> u32) -> u32 {
    input
        .lines()
        .map(fun)
        .reduce(|acc, x| acc + x)
        .expect("There should be a result")
}

fn find_ascii_digit(mut input: impl Iterator<Item = u8>) -> u32 {
    parse::from_ascii_digit(
        input
            .find(u8::is_ascii_digit)
            .expect("There should be at least 1 digit"),
    )
}

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn check_digit(input: &[u8]) -> Option<u32> {
    let mut n = 1;
    for num in NUMBERS {
        if input.starts_with(num) {
            return Some(n);
        }
        n += 1;
    }
    if input[0].is_ascii_digit() {
        return Some(parse::from_ascii_digit(input[0]));
    }
    None
}

fn find_digit(input: &str, range: impl Iterator<Item = usize>) -> u32 {
    for i in range {
        if let Some(n) = check_digit(&input.as_bytes()[i..]) {
            return n;
        }
    }
    panic!("There should be a value")
}
