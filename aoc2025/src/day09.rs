use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

type Point = (isize, isize);

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let points = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&points))
        .part2(part2(&points)))
}

#[must_use]
fn parse_input(input: &str) -> Vec<Point> {
    let parse = |x: &str| -> isize { x.parse().expect("All values should be integers") };
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
const fn size((a, b): (&Point, &Point)) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[must_use]
fn part1(points: &[Point]) -> usize {
    points
        .iter()
        .tuple_combinations()
        .map(size)
        .max()
        .expect("There should be a solution")
}

fn is_point_on_line((px, py): Point, (x1, y1): Point, (x2, y2): Point) -> bool {
    // NOTE: lines can only be vertical or horizontal from problem statement
    (px >= x1.min(x2) && px <= x1.max(x2)) && (py >= y1.min(y2) && py <= y1.max(y2))
}

fn is_point_inside(px: isize, py: isize, points: &[Point]) -> bool {
    let mut inside = false;
    for (&(x1, y1), &(x2, y2)) in points.iter().circular_tuple_windows() {
        if is_point_on_line((px, py), (x1, y1), (x2, y2)) {
            return true;
        }
        // Count the parity of vertical lines a ray sent from the point to the right would cross.
        // Any point within the polygon will result in an odd amount of line crosses
        if (y1 > py) != (y2 > py) && px < x1 {
            inside = !inside;
        }
    }
    inside
}

fn do_lines_intersect(a1: Point, a2: Point, b1: Point, b2: Point) -> bool {
    // Calculate the side c is at from the line (a, b)
    let get_orient = |(ax, ay), (bx, by), (cx, cy)| {
        let v: isize = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax);
        v.signum()
    };
    let o1 = get_orient(a1, a2, b1) * get_orient(a1, a2, b2);
    let o2 = get_orient(b1, b2, a1) * get_orient(b1, b2, a2);
    o1 < 0 && o2 < 0
}

fn is_rect_inside((x1, y1): Point, (x2, y2): Point, points: &[Point]) -> bool {
    // All corners have to be inside the polygon
    let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    let corners_inside = corners
        .iter()
        .all(|&(cx, cy)| is_point_inside(cx, cy, points));
    if !corners_inside {
        return false;
    }
    // None of the polygon's edges must intersect the edges of the rectangle
    let rect_edges = [
        ((x1, y1), (x2, y1)),
        ((x2, y1), (x2, y2)),
        ((x2, y2), (x1, y2)),
        ((x1, y2), (x1, y1)),
    ];
    points.iter().circular_tuple_windows().all(|(&p1, &p2)| {
        rect_edges
            .iter()
            .all(|&(start, end)| !do_lines_intersect(start, end, p1, p2))
    })
}

fn part2(points: &[Point]) -> usize {
    points
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| is_rect_inside(a, b, points))
        .map(size)
        .max()
        .expect("There should be a solution")
}
