use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let rotations = process_lines(input);
    Ok(Solution::default()
        .part1(part1(&rotations))
        .part2(part2(&rotations)))
}

fn process_lines(input: &str) -> Vec<i16> {
    let parse = |x: &str| -> i16 { x.parse().expect("All values should be integers") };
    input
        .split_terminator('\n')
        .map(|line| if line.as_bytes()[0] == b'L' { -1 } else { 1 } * parse(&line[1..]))
        .collect()
}

#[must_use]
fn solve(rotations: &[i16], f: impl Fn(i16, i16, i16) -> usize) -> usize {
    let mut state = 50;
    let mut total = 0;
    for rot in rotations {
        let next = ((state + rot) % 100 + 100) % 100;
        total += f(state, next, *rot);
        state = next;
    }
    total
}

#[must_use]
fn part1(rotations: &[i16]) -> usize {
    solve(rotations, |_, state, _| (state == 0).into())
}

#[must_use]
fn part2(rotations: &[i16]) -> usize {
    solve(rotations, |start, end, rot| {
        let loops = usize::try_from((rot / 100).abs()).expect("Absolute value should be positive");
        let passed = start != 0 && ((rot < 0 && end > start) || (rot > 0 && end < start));
        loops + usize::from(end == 0 || passed)
    })
}
