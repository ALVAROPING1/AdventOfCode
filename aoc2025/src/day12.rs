use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (shapes, regions) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&shapes, regions))
        .part2("Nothing to solve".to_string()))
}

fn parse_input(input: &str) -> (Vec<usize>, &str) {
    let (shapes, regions) = input
        .rsplit_once("\n\n")
        .expect("There should be 2 sections");
    let shapes = shapes
        .split("\n\n")
        .map(|shape| shape.chars().filter(|c| *c == '#').count())
        .collect();
    (shapes, regions)
}

#[must_use]
fn part1(shapes: &[usize], regions: &str) -> usize {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    let regions = regions.split_terminator('\n').map(|line| {
        let (area, counts) = line.split_once(": ").expect("There should be 2 sections");
        let (x, y) = area.split_once('x').expect("There should be 2 sections");
        let area = parse(x) * parse(y);
        let counts = counts.split_whitespace().map(parse);
        let total: usize = counts.zip(shapes).map(|(count, area)| count * area).sum();
        (area, total)
    });
    regions.filter(|(area, total)| total < area).count()
}
