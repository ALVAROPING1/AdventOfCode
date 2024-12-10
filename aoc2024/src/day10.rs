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

trait Mark {
    fn new(size: usize) -> Self;
    fn mark(&mut self, pos: Vec2D, cols: usize) -> bool;
}

impl Mark for Vec<bool> {
    fn new(size: usize) -> Self {
        vec![false; size]
    }
    fn mark(&mut self, pos: Vec2D, cols: usize) -> bool {
        let idx = pos.1 * cols + pos.0;
        if self[idx] {
            return true;
        }
        self[idx] = true;
        false
    }
}

impl Mark for u8 {
    fn new(_: usize) -> Self {
        0
    }
    fn mark(&mut self, _: Vec2D, _: usize) -> bool {
        false
    }
}

fn expand(input: &Str2D, visited: &mut impl Mark, pos: Vec2D) -> u32 {
    if visited.mark(pos, input.cols()) {
        return 0;
    }
    let c = input.char(&pos.as_tuple());
    if c == '9' {
        return 1;
    }
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .filter_map(|offset| {
            let adj = pos + offset;
            if adj.0 >= input.cols() || adj.1 >= input.rows() {
                return None;
            }
            let c2 = input.char(&adj.as_tuple());
            if c2 as u32 == c as u32 + 1 {
                Some(expand(input, visited, adj))
            } else {
                None
            }
        })
        .sum()
}

fn solve<T: Mark>(input: &Str2D) -> u32 {
    let mut total = 0;
    let size = input.rows() * input.cols();
    for y in 0..input.rows() {
        for x in 0..input.cols() {
            if input.char(&(x, y)) != '0' {
                continue;
            }
            let mut visited = T::new(size);
            total += expand(input, &mut visited, Vec2D(x, y));
        }
    }
    total
}

#[must_use]
fn part1(input: &Str2D) -> u32 {
    solve::<Vec<_>>(input)
}

#[must_use]
fn part2(input: &Str2D) -> u32 {
    solve::<u8>(input)
}
