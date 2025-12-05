use std::error::Error;
use std::ops::RangeInclusive;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut ranges, food) = process_input(input);
    Ok(Solution::default()
        .part1(part1(&ranges, food))
        .part2(part2(&mut ranges)))
}

fn process_input(input: &str) -> (Vec<RangeInclusive<usize>>, impl Iterator<Item = usize> + '_) {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    let parse_range = |line: &str| {
        let (s, e) = line.split_once('-').expect("There should be single `-`");
        parse(s)..=parse(e)
    };
    let (ranges, food) = input
        .split_once("\n\n")
        .expect("There should be a single empty line");
    (
        ranges.split_terminator('\n').map(parse_range).collect(),
        food.split_terminator('\n').map(parse),
    )
}

#[must_use]
fn part1(ranges: &[RangeInclusive<usize>], food: impl Iterator<Item = usize>) -> usize {
    food.filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count()
}

#[must_use]
fn part2(ranges: &mut [RangeInclusive<usize>]) -> usize {
    ranges.sort_unstable_by_key(|range| *range.start());
    ranges
        .iter()
        .scan(0, |max, range| {
            Some(if *max < *range.end() + 1 {
                let min = (*max).max(*range.start());
                *max = *range.end() + 1;
                *max - min
            } else {
                0
            })
        })
        .sum()
}
