use std::collections::HashMap;
use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let mut input = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&mut input))
        .part2(part2(&mut input)))
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut buffer = HashMap::new();
    for x in utils_rust::parse::value_list(input) {
        buffer.entry(x).and_modify(|x| *x += 1).or_insert(1);
    }
    buffer
}

fn solve(input: &mut HashMap<u64, u64>, iters: usize) -> u64 {
    let mut back = HashMap::new();
    for n in 0..iters {
        println!("{n}");
        for (k, &v) in input.iter() {
            if *k == 0 {
                back.entry(1).and_modify(|x| *x += v).or_insert(v);
                continue;
            }
            let digits = k.ilog10() + 1;
            if digits % 2 == 0 {
                let split = 10u64.pow(digits / 2);
                back.entry(k % split).and_modify(|x| *x += v).or_insert(v);
                back.entry(k / split).and_modify(|x| *x += v).or_insert(v);
            } else {
                back.entry(k * 2024).and_modify(|x| *x += v).or_insert(v);
            }
        }
        std::mem::swap(input, &mut back);
        back.clear();
    }
    input.values().copied().sum()
}

#[must_use]
fn part1(input: &mut HashMap<u64, u64>) -> u64 {
    solve(input, 25)
}

#[must_use]
fn part2(input: &mut HashMap<u64, u64>) -> u64 {
    solve(input, 75 - 25)
}
