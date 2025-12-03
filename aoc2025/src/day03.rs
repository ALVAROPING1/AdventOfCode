use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let banks = process_input(input);
    Ok(Solution::default()
        .part1(part1(banks.clone()))
        .part2(part2(banks)))
}

fn process_input(input: &str) -> impl Iterator<Item = &[u8]> + Clone {
    input.split_terminator('\n').map(str::as_bytes)
}

#[must_use]
fn part1<'a>(banks: impl Iterator<Item = &'a [u8]>) -> usize {
    let mut total: usize = 0;
    for bank in banks {
        let mut maxs = std::iter::repeat_n(0, bank.len()).collect::<Vec<_>>();
        let max_pos = bank.len() - 1;
        let mut max = 0;
        maxs[max_pos] = (bank[max_pos] - b'0') as usize;
        for (i, val) in bank.iter().enumerate().rev().skip(1) {
            let val = (*val - b'0') as usize;
            max = max.max(val * 10 + maxs[i + 1]);
            maxs[i] = maxs[i + 1].max(val);
        }
        total += max;
    }
    total
}

#[must_use]
fn part2<'a>(banks: impl Iterator<Item = &'a [u8]>) -> usize {
    const DIGITS: usize = 12;
    let mut total: usize = 0;
    let mut num = Vec::<u8>::with_capacity(DIGITS);
    let mut pos = Vec::<usize>::with_capacity(DIGITS);
    for bank in banks {
        num.clear();
        pos.clear();
        num.extend(bank[..DIGITS].iter());
        pos.extend(0..DIGITS);
        for (i, new) in bank.iter().enumerate().skip(1) {
            let right = DIGITS.min(i + 1);
            let left = DIGITS.saturating_sub(bank.len() - i);
            for j in left..right {
                let curr = num[j];
                let curr_pos = pos[j];
                if curr_pos >= i {
                    break;
                } else if *new > curr {
                    let new_digits = i..i + DIGITS - j;
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
