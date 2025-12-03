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
    let mut total: usize = 0;
    for bank in banks {
        let maxs = std::iter::repeat_n((0, 0), bank.len()).collect::<Vec<_>>();
        let mut maxs = std::iter::repeat_n(maxs, 13).collect::<Vec<_>>();
        for digits in 1..=13 {
            let max_pos = bank.len() - digits;
            maxs[digits - 1][max_pos] = ((bank[max_pos] - b'0') as usize, max_pos);
            for (i, val) in bank.iter().enumerate().rev().skip(digits) {
                let val = (*val - b'0') as usize;
                maxs[digits - 1][i] = if val >= maxs[digits - 1][i + 1].0 {
                    (val, i)
                } else {
                    maxs[digits - 1][i + 1]
                };
            }
        }
        let mut max = 0;
        let mut left = 0;
        for digit in (0..12).rev() {
            max = max * 10 + maxs[digit][left].0;
            left = maxs[digit][left].1 + 1;
        }
        total += max;
    }
    total
}
