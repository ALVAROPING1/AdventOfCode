use std::{error::Error, iter::zip};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut left, mut right) = process_lines(input);
    Ok(Solution::default()
        .part1(part1(&mut left, &mut right))
        .part2(part2(&left, &right)))
}

fn process_lines(input: &str) -> (Vec<usize>, Vec<usize>) {
    fn parse(x: &str) -> usize {
        x.parse().expect("All values should be integers")
    }

    input
        .split_terminator('\n')
        .map(|line| (parse(&line[..5]), parse(&line[8..])))
        .unzip()
}

#[must_use]
fn part1(left: &mut Vec<usize>, right: &mut Vec<usize>) -> usize {
    left.sort_unstable();
    right.sort_unstable();
    zip(left, right).map(|(l, r)| r.abs_diff(*l)).sum()
}

#[must_use]
fn part2(left: &[usize], right: &[usize]) -> usize {
    let mut counts: [u8; 100_000] = [0; 100_000];
    for x in right {
        counts[*x] += 1;
    }
    left.iter().map(|l| l * usize::from(counts[*l])).sum()
}
