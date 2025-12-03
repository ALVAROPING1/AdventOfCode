use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let banks = process_input(input);
    Ok(Solution::default()
        .part1(solve(&banks, 2))
        .part2(solve(&banks, 12)))
}

fn process_input(input: &str) -> Vec<&[u8]> {
    input.split_terminator('\n').map(str::as_bytes).collect()
}

#[must_use]
fn solve(banks: &[&[u8]], digits: u32) -> usize {
    let mut total: usize = 0;
    for bank in banks {
        let mut pos = 0;
        let mut result = 0;
        for digit in 0..digits {
            let (i, max) = bank[pos..=bank.len() - (digits - digit) as usize]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, x)| **x)
                .expect("The subrange shouldn't be empty");
            result += (max - b'0') as usize * 10usize.pow(digits - digit - 1);
            pos += i + 1;
        }
        total += result;
    }
    total
}
