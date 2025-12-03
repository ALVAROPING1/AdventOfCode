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
fn solve(banks: &[&[u8]], digits: usize) -> usize {
    let mut total: usize = 0;
    let mut num: Vec<u8> = std::iter::repeat_n(0, digits).collect();
    let mut pos = Vec::<usize>::with_capacity(digits);
    for bank in banks {
        pos.clear();
        num.copy_from_slice(&bank[..digits]);
        pos.extend(0..digits);
        for (i, new) in bank.iter().enumerate().skip(1) {
            let right = digits.min(i + 1);
            let left = digits.saturating_sub(bank.len() - i);
            for j in left..right {
                if pos[j] >= i {
                    break;
                } else if *new > num[j] {
                    let new_digits = i..i + digits - j;
                    num[j..].copy_from_slice(&bank[new_digits.clone()]);
                    pos.truncate(j);
                    pos.extend(new_digits);
                    break;
                }
            }
        }
        let max = num.iter().fold(0, |acc, x| acc * 10 + (*x - b'0') as usize);
        total += max;
    }
    total
}
