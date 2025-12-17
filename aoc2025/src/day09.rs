use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;
use utils_rust::numbers::min_max;

type Point = (usize, usize);

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let points = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&points))
        .part2(solve_areas(&points)))
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
#[allow(dead_code)]
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

// Solve part 2 using intersections between candidate rectangles and the polygon
#[allow(dead_code)]
fn solve_intersections(points: &[Point]) -> usize {
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

// Solve part 2 using coordinate compression and checking the area within the polygon of the
// rectangles
//
// NOTE: idea from <https://reddit.com/r/adventofcode/comments/1phywvn/2025_day_9_solutions/nt2lbxc/?context=3#nt2lbxc>
fn solve_areas(points: &[Point]) -> usize {
    fn compress(points: &[Point], get: impl Fn(Point) -> usize) -> Vec<usize> {
        let p = points.iter().copied().map(get).unique();
        p.sorted_unstable().collect_vec()
    }

    // Compress the coordinates into a smaller grid, replacing each value with its index in the
    // sorted list of unique values
    let x_ids = compress(points, |(x, _)| x);
    let y_ids = compress(points, |(_, y)| y);
    let transform = |ids: &[_], x| ids.binary_search(x).expect("All values should be found");
    let points = points
        .iter()
        .map(|(x, y)| (transform(&x_ids, x), transform(&y_ids, y)))
        .collect_vec();
    // Create grid with the state of each position
    let mut map = vec![0u8; x_ids.len() * y_ids.len()];
    let idx = |y: usize, x: usize| y * x_ids.len() + x;
    // Iterate through vertical segments of the polygon
    for (&(ax, ay), &(bx, by)) in points.iter().circular_tuple_windows() {
        if ax == bx {
            let (miny, maxy) = min_max(ay, by);
            // For each cell in the segment, create a bitmask where the 1st bit indicates whether
            // the cell is connected to the one below, and the 2nd bit indicates whether the cell
            // is connected to the one above
            map[idx(maxy, ax)] = 0b01;
            map[idx(miny, ax)] = 0b10;
            for y in miny + 1..maxy {
                map[idx(y, ax)] = 0b11;
            }
        }
    }
    // Iterate through each cell of the grid row by row and determine whether it is inside or
    // outside of the polygon. The position is inside of the polygon if the prefix XOR of the
    // bitmasks of the row up to the cell is positive (has any bits set). This works because cells
    // are inside the polygon iff the cell is connected to the cell above/below, and whenever we
    // pass through a vertical segment of the polygon each of those connections gets toggled
    for y in 0..y_ids.len() {
        let mut prefix_xor = 0;
        for x in 0..x_ids.len() {
            let i = idx(y, x);
            prefix_xor ^= map[i];
            map[i] = u8::from(prefix_xor > 0);
        }
    }

    // Use 2D prefix sums to be able to quickly query the area within a given rectangle inside the
    // polygon
    let submatrix = SubMatrix::new(&map, y_ids.len(), x_ids.len());
    // Calculate the maximum area of rectangles fully inside the polygon
    points
        .iter()
        .tuple_combinations()
        .map(|(&(ax, ay), &(bx, by))| (min_max(ax, bx), min_max(ay, by)))
        .filter(|&(x, y)| size(x, y) == submatrix.sum(y.0, x.0, y.1 + 1, x.1 + 1))
        .map(|(x, y)| size((x_ids[x.0], x_ids[x.1]), (y_ids[y.0], y_ids[y.1])))
        .max()
        .expect("There should be a solution")
}

/// Calculate submatrix sums quickly, given upper-left and lower-right corners (half-open)
struct SubMatrix {
    p: Vec<isize>,
    cols: usize,
}

impl SubMatrix {
    /// Create a new submatrix from a `RxC` matrix
    fn new(values: &[u8], r: usize, c: usize) -> Self {
        assert_eq!(values.len(), r * c);
        let mut p = vec![0; (c + 1) * (r + 1)];
        let idx = |ri, ci| ri * (c + 1) + ci;
        // For each position (x, y) calculate the sum of the rectangle between (0, 0) and (x-1, y-1)
        // using bottom-up dynamic programming, as the sum of the value at the given position and
        // the values of the rectangles above and to the left, minus their intersection
        for ri in 0..r {
            for ci in 0..c {
                let v = isize::from(values[ri * c + ci]);
                p[idx(ri + 1, ci + 1)] =
                    v + p[idx(ri, ci + 1)] + p[idx(ri + 1, ci)] - p[idx(ri, ci)];
            }
        }
        Self { p, cols: c + 1 }
    }

    /// Get the sum of the values within a given rectangle
    fn sum(&self, u: usize, l: usize, d: usize, r: usize) -> usize {
        let idx = |ri, ci| ri * self.cols + ci;
        let res = self.p[idx(d, r)] - self.p[idx(d, l)] - self.p[idx(u, r)] + self.p[idx(u, l)];
        res.try_into().expect("The areas should be positive")
    }
}
