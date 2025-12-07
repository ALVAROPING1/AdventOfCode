use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (splitters, timelines) = solve(input);
    Ok(Solution::default().part1(splitters).part2(timelines))
}

#[must_use]
fn solve(map: &str) -> (usize, usize) {
    let (first, rows) = map
        .split_once('\n')
        .expect("There should be at least 2 lines");
    let start = first
        .find('S')
        .expect("There should be a starting position");
    let mut curr: Vec<usize> = std::iter::repeat_n(0, first.len() + 2).collect();
    curr[start + 1] = 1;
    let mut next = curr.clone();
    let mut splitters = 0;
    for row in rows.split_terminator('\n') {
        for (i, c) in row.chars().enumerate() {
            let i = i + 1;
            if c == '^' && curr[i] > 0 {
                splitters += 1;
                next[i - 1] += curr[i];
                next[i + 1] += curr[i];
                next[i] = 0;
            }
        }
        curr.copy_from_slice(&next);
    }
    (splitters, curr.iter().sum())
}
