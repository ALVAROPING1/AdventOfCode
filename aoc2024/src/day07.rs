use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let input = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&input))
        .part2(part2(&input)))
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .split_terminator('\n')
        .map(|line| {
            let (value, numbers) = line
                .split_once(':')
                .expect("There should always be exactly 1 `:`");
            let numbers = utils_rust::parse::value_list(numbers).collect();
            (
                value
                    .parse()
                    .expect("The first value should always be a number"),
                numbers,
            )
        })
        .collect()
}

fn solve_case_2(target: u64, first: u64, numbers: &[u64]) -> bool {
    if first > target {
        return false;
    }
    if numbers.is_empty() {
        return target == first;
    }
    solve_case_2(target, first * numbers[0], &numbers[1..])
        || solve_case_2(target, first + numbers[0], &numbers[1..])
}

fn solve_case_3(target: u64, first: u64, numbers: &[u64]) -> bool {
    if first > target {
        return false;
    }
    if numbers.is_empty() {
        return target == first;
    }
    let concat = first * 10u64.pow(numbers[0].ilog10() + 1) + numbers[0];
    solve_case_3(target, concat, &numbers[1..])
        || solve_case_3(target, first * numbers[0], &numbers[1..])
        || solve_case_3(target, first + numbers[0], &numbers[1..])
}

fn solve(input: &[(u64, Vec<u64>)], solver: impl Fn(u64, u64, &[u64]) -> bool) -> u64 {
    input
        .iter()
        .filter(|(target, numbers)| solver(*target, numbers[0], &numbers[1..]))
        .map(|(target, _)| target)
        .sum()
}

#[must_use]
fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    solve(input, solve_case_2)
}

#[must_use]
fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    solve(input, solve_case_3)
}
