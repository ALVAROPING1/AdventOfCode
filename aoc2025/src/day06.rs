use itertools::Itertools;
use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (numbers, ops) = process_input(input);
    Ok(Solution::default()
        .part1(part1(numbers, &ops))
        .part2(part2(numbers, &ops)))
}

fn process_input(input: &str) -> (&str, Vec<&str>) {
    let (numbers, ops) = input[..input.len() - 1]
        .rsplit_once('\n')
        .expect("There should be at least 2 lines");
    let ops = ops.split_whitespace().collect();
    (numbers, ops)
}

#[must_use]
fn part1(numbers: &str, ops: &[&str]) -> usize {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    let acc: Vec<usize> = ops.iter().map(|&op| usize::from(op != "+")).collect();
    numbers
        .split_terminator('\n')
        .fold(acc, |mut acc, line| {
            for ((x, acc), op) in line.split_whitespace().map(parse).zip(&mut acc).zip(ops) {
                *acc = if *op == "+" { *acc + x } else { *acc * x }
            }
            acc
        })
        .iter()
        .sum()
}

#[must_use]
fn part2(numbers: &str, ops: &[&str]) -> usize {
    let lines: Vec<&str> = numbers.split_terminator('\n').collect();
    let mut i = 0;
    (0..lines[0].len())
        .map(|col| {
            lines
                .iter()
                .map(|line| line.as_bytes()[col])
                .fold(0, |acc, x| {
                    if x == b' ' {
                        return acc;
                    }
                    acc * 10 + (x - b'0') as usize
                })
        })
        .coalesce(|acc, x| {
            if x == 0 {
                i += 1;
                return Err((acc, usize::from(ops[i] != "+")));
            }
            Ok(if ops[i] == "+" { acc + x } else { acc * x })
        })
        .sum()
}
