use std::{error::Error, iter::Peekable};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let reports = parse(input);
    Ok(Solution::default()
        .part1(part1(&reports))
        .part2(part2(&reports)))
}

fn check_report(
    mut report: Peekable<impl Iterator<Item = u16>>,
    comp: impl Fn(u16, u16) -> bool,
    diff: impl Fn(u16, u16) -> u16,
) -> bool {
    while let (Some(x), Some(next)) = (report.next(), report.peek()) {
        if comp(x, *next) || diff(x, *next) > 3 {
            return false;
        }
    }
    true
}

fn check(mut report: Peekable<impl Iterator<Item = u16>>) -> bool {
    let first = report
        .next()
        .expect("All reports should have at least 2 elements");
    let second = *report
        .peek()
        .expect("All reports should have at least 2 elements");
    match first.cmp(&second) {
        std::cmp::Ordering::Less if second - first <= 3 => {
            check_report(report, |a, b| a >= b, |a, b| b - a)
        }
        std::cmp::Ordering::Greater if first - second <= 3 => {
            check_report(report, |a, b| a <= b, |a, b| a - b)
        }
        _ => false,
    }
}

fn parse(input: &str) -> Vec<Vec<u16>> {
    input
        .split_terminator('\n')
        .map(|line| utils_rust::parse::value_list(line).collect())
        .collect()
}

fn solve(input: &[Vec<u16>], check: impl Fn(&[u16]) -> bool) -> u16 {
    input.iter().map(|x| check(x)).map(u16::from).sum()
}

#[must_use]
pub fn part1(input: &[Vec<u16>]) -> u16 {
    solve(input, |report| check(report.iter().copied().peekable()))
}

#[must_use]
pub fn part2(input: &[Vec<u16>]) -> u16 {
    solve(input, |report| {
        for i in 0..report.len() {
            let res = check(
                report
                    .iter()
                    .enumerate()
                    .filter_map(|(pos, x)| if pos == i { None } else { Some(*x) })
                    .peekable(),
            );
            if res {
                return true;
            }
        }
        false
    })
}
