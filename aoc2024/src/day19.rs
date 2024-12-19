use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (combinations, patterns) = parse_input(input);
    let (possible, combinations) = solve(&combinations, patterns);
    Ok(Solution::default().part1(possible).part2(combinations))
}

fn parse_input(input: &str) -> (Vec<&str>, &str) {
    let (combinations, patterns) = input
        .split_once("\n\n")
        .expect("There should always be 2 sections");
    let combinations = combinations.split(", ").collect();
    (combinations, patterns)
}

fn count_combinations(combinations: &[&str], pattern: &str) -> u64 {
    let mut amounts = vec![0; pattern.len() + 1];
    amounts[0] = 1;
    for len in 0..=pattern.len() {
        let pattern = &pattern[len..];
        for combination in combinations {
            let comb_len = combination.len();
            if pattern.len() >= comb_len && &pattern[..comb_len] == *combination {
                amounts[len + comb_len] += amounts[len];
            }
        }
    }
    amounts
        .pop()
        .expect("There should always be at least one value")
}

#[must_use]
fn solve(combinations: &[&str], patterns: &str) -> (u64, u64) {
    patterns
        .lines()
        .map(|pattern| count_combinations(combinations, pattern))
        .fold((0, 0), |mut acc, x| {
            acc.0 += u64::from(x > 0);
            acc.1 += x;
            acc
        })
}
