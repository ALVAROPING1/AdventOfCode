use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::Str2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let input = Str2D::new(input);
    Ok(Solution::default()
        .part1(part1(&input))
        .part2(part2(&input)))
}

static OFFSETS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const fn same_cell(map: &Str2D, pos: Vec2D, adj: Vec2D) -> bool {
    adj.0 < map.cols()
        && adj.1 < map.rows()
        && map.char(&pos.as_tuple()) == map.char(&adj.as_tuple())
}

fn expand<F>(map: &Str2D, visited: &mut [bool], pos: Vec2D, else_inc: bool, f: F) -> (u32, u32)
where
    F: Copy + Fn(&Str2D, usize, Vec2D, Vec2D) -> bool,
{
    let idx = pos.1 * map.cols() + pos.0;
    if visited[idx] {
        return (0, 0);
    }
    visited[idx] = true;
    let (mut area, mut perimeter) = (1, 0);
    for (i, offset) in OFFSETS.iter().enumerate() {
        let adj = pos + *offset;
        if same_cell(map, pos, adj) {
            let (adj_area, adj_perimeter) = expand(map, visited, adj, else_inc, f);
            area += adj_area;
            perimeter += adj_perimeter;
        } else {
            perimeter += u32::from(else_inc);
        }
        perimeter += u32::from(f(map, i, pos, adj));
    }
    (area, perimeter)
}

fn solve<F>(input: &Str2D, else_inc: bool, f: F) -> u32
where
    F: Copy + Fn(&Str2D, usize, Vec2D, Vec2D) -> bool,
{
    let mut visited = vec![false; input.rows() * input.cols()];
    let mut total = 0;
    for y in 0..input.rows() {
        for x in 0..input.cols() {
            if visited[y * input.cols() + x] {
                continue;
            }
            let (area, perimeter) = expand(input, &mut visited, Vec2D(x, y), else_inc, f);
            total += area * perimeter;
        }
    }
    total
}

#[must_use]
fn part1(input: &Str2D) -> u32 {
    solve(input, true, |_, _, _, _| false)
}

#[must_use]
fn part2(input: &Str2D) -> u32 {
    solve(input, false, |map, i, pos, adj| {
        // Use the fact that the amount of sides is equal to the amount of corners: add 1 side for
        // each corner found. A corner can be made by 2 adjacent, different type cells (convex
        // corner), or by 2 adjacent, same type cells with a different type cell in the diagonal in
        // between (concave corner)
        let same = |adj: Vec2D| same_cell(map, pos, adj);
        let offset2 = OFFSETS[(i + 1) % OFFSETS.len()];
        let adj2 = pos + offset2;
        (!same(adj) && !same(adj2))
            || (same(adj) && same(adj2) && !same(pos + OFFSETS[i] + offset2))
    })
}
