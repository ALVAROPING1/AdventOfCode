use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::Str2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (antennas, size) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&antennas, size))
        .part2(part2(&antennas, size)))
}

type Vec2D = (usize, usize);

fn parse_input(input: &str) -> ([Vec<Vec2D>; 128], Vec2D) {
    const VEC: Vec<Vec2D> = Vec::new();
    let map = Str2D::new(input);
    let mut antennas = [VEC; 128];
    for y in 0..map.rows() {
        for x in 0..map.cols() {
            let char = map.char(&(x, y));
            if char == '.' {
                continue;
            }
            antennas[char as usize].push((x, y));
        }
    }
    (antennas, (map.cols(), map.rows()))
}

fn mark_point(pos: Vec2D, size: Vec2D, antinodes: &mut [bool], total: &mut u32) -> bool {
    let get_idx = |(x, y): Vec2D| y * size.0 + x;
    if pos.0 >= size.0 || pos.1 >= size.1 {
        return false;
    }
    let idx = get_idx(pos);
    *total += u32::from(antinodes[idx]);
    antinodes[idx] = false;
    true
}

fn solve<F>(antennas: &[Vec<Vec2D>], size: Vec2D, f: F) -> u32
where
    F: Fn(Vec2D, Vec2D, Vec2D, &mut [bool], &mut u32),
{
    let frequencies = ('0'..='9').chain('A'..='Z').chain('a'..='z');
    let mut total = 0;
    let mut antinodes = vec![true; size.0 * size.1];
    for freq in frequencies {
        for (i, a) in antennas[freq as usize].iter().enumerate() {
            for b in &antennas[freq as usize][i + 1..] {
                let d = (b.0.overflowing_sub(a.0).0, b.1.overflowing_sub(a.1).0);
                f(*a, *b, d, &mut antinodes, &mut total);
            }
        }
    }
    total
}

#[must_use]
fn part1(antennas: &[Vec<Vec2D>], size: Vec2D) -> u32 {
    solve(antennas, size, |a, b, d, antinodes, total| {
        let pos = (b.0.overflowing_add(d.0).0, b.1.overflowing_add(d.1).0);
        mark_point(pos, size, antinodes, total);
        let pos = (a.0.overflowing_sub(d.0).0, a.1.overflowing_sub(d.1).0);
        mark_point(pos, size, antinodes, total);
    })
}

#[must_use]
fn part2(antennas: &[Vec<Vec2D>], size: Vec2D) -> u32 {
    solve(antennas, size, |a, b, d, antinodes, total| {
        // Assume diff.0 and diff.1 are coprime: this ensures the `diff` vector is the
        // shortest vector that ends in a grid position
        let mut pos = b;
        while mark_point(pos, size, antinodes, total) {
            pos = (pos.0.overflowing_add(d.0).0, pos.1.overflowing_add(d.1).0);
        }
        let mut pos = a;
        while mark_point(pos, size, antinodes, total) {
            pos = (pos.0.overflowing_sub(d.0).0, pos.1.overflowing_sub(d.1).0);
        }
    })
}
