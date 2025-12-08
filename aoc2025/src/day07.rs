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
    let mut curr: Vec<usize> = std::iter::repeat_n(0, first.len()).collect();
    curr[start] = 1;
    let mut splitters = 0;
    let mut range = (start, start);
    for row in rows.split_terminator('\n').skip(1).step_by(2) {
        for i in (range.0..=range.1).step_by(2) {
            if row.as_bytes()[i] == b'^' && curr[i] > 0 {
                splitters += 1;
                curr[i - 1] += curr[i];
                curr[i + 1] += curr[i];
                curr[i] = 0;
            }
        }
        range = (range.0 - 1, range.1 + 1);
    }
    (splitters, curr.iter().sum())
}
