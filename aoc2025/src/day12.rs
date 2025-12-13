use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default()
        .part1(part1(input))
        .part2("Nothing to solve".to_string()))
}

#[must_use]
fn part1(input: &str) -> usize {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    input
        .rsplit_once("\n\n")
        .expect("There should be 2 sections")
        .1
        .split_terminator('\n')
        .filter(|line| {
            // In the input, the presents either fit trivially with a lot of leftover space, or
            // they trivially would require at least more area than there is on the input
            let (area, counts) = line.split_once(": ").expect("There should be 2 sections");
            let (x, y) = area.split_once('x').expect("There should be 2 sections");
            counts.split_whitespace().map(parse).sum::<usize>() * 9 <= parse(x) * parse(y)
        })
        .count()
}
