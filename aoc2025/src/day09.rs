use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let positions = parse_input(input);
    Ok(Solution::default().part1(part1(&positions)))
    // .part2(part2(&positions, &mut distances, &mut sets)))
}

#[must_use]
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    input
        .split_terminator('\n')
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect("There should be 2 values per line");
            (parse(x), parse(y))
        })
        .collect()
}

#[must_use]
fn part1(positions: &[(usize, usize)]) -> usize {
    positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .expect("There should be at least 2 points")
}
