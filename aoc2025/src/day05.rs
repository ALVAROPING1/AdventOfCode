use itertools::Itertools;
use std::error::Error;

use crate::prelude::*;

type Range = std::ops::RangeInclusive<usize>;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut ranges, food) = process_input(input);
    let (fresh, total) = solve(&mut ranges, food);
    Ok(Solution::default().part1(fresh).part2(total))
}

fn process_input(input: &str) -> (impl Iterator<Item = Range>, impl Iterator<Item = usize>) {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    let parse_range = |line: &str| {
        let (s, e) = line.split_once('-').expect("There should be single `-`");
        parse(s)..=parse(e)
    };
    let (ranges, food) = input
        .split_once("\n\n")
        .expect("There should be a single empty line");
    let ranges = ranges
        .split_terminator('\n')
        .map(parse_range)
        .sorted_unstable_by_key(|range| *range.start())
        .coalesce(|acc, x| {
            if x.start() > acc.end() {
                return Err((acc, x));
            }
            Ok(*acc.start()..=std::cmp::max(*acc.end(), *x.end()))
        });
    let food = food.split_terminator('\n').map(parse).sorted_unstable();
    (ranges, food)
}

fn solve(ranges: impl Iterator<Item = Range>, food: impl Iterator<Item = usize>) -> (usize, usize) {
    let (mut ranges, mut food) = (ranges.peekable(), food.peekable());
    let (mut total, mut fresh) = (0, 0);
    while let Some((range, x)) = ranges.peek().zip(food.peek()) {
        if range.contains(x) {
            fresh += 1;
            food.next();
        } else if x < range.start() {
            food.next();
        } else {
            // if x > range.end()
            total += range.end() - range.start() + 1;
            ranges.next();
        }
    }
    total += ranges.map(|r| r.end() - r.start() + 1).sum::<usize>();
    (fresh, total)
}
