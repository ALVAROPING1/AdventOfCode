use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;
use utils_rust::numbers::min_max;

type Point = (usize, usize);

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let points = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&points))
        .part2(part2(&points)))
}

#[must_use]
fn parse_input(input: &str) -> Vec<Point> {
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
const fn size((ax, bx): Point, (ay, by): Point) -> usize {
    (bx - ax + 1) * (by - ay + 1)
}

#[must_use]
fn part1(points: &[Point]) -> usize {
    points
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| size(min_max(x1, x2), min_max(y1, y2)))
        .max()
        .expect("There should be a solution")
}

/// Check if the rectangle doesn't intersect with any of the lines in the polygon
///
/// NOTE: this only guarantees none of the sides of the polygon intersects the rectangle, i.e. the
/// rectangle is fully inside/outside of the polygon. However, that's enough for the input cases
#[must_use]
fn is_rect_inside((x1, y1): Point, (x2, y2): Point, points: &[Point]) -> bool {
    !points
        .iter()
        .circular_tuple_windows()
        .any(|(&(px1, py1), &(px2, py2))| {
            let px = min_max(px1, px2);
            let py = min_max(py1, py2);
            px.0 < x2 && py.0 < y2 && px.1 > x1 && py.1 > y1
        })
}

fn part2(points: &[Point]) -> usize {
    points
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| (min_max(x1, x2), min_max(y1, y2)))
        .fold(0, |acc, ((x1, x2), (y1, y2))| {
            // Calculating the area is significantly cheaper than calculating whether the rectangle
            // is contained in the polygon, so calculate that first and only check if it's
            // contained in the polygon if it could give a bigger area
            let size = size((x1, x2), (y1, y2));
            if acc >= size || !is_rect_inside((x1, y1), (x2, y2), points) {
                return acc;
            }
            size
        })
}
