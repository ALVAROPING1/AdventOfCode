use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;

use crate::prelude::*;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut map, bytes) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&map))
        .part2(part2(&mut map, bytes)))
}

const MAP_SIZE: usize = 71;
type Map = [[bool; MAP_SIZE]; MAP_SIZE];

fn parse_input(input: &str) -> (Map, impl Iterator<Item = (usize, usize)> + '_) {
    let mut map = [[true; MAP_SIZE]; MAP_SIZE];
    let mut iter = input.lines().map(|line| {
        let parse = |x: &str| -> usize { x.parse().expect("This should only parse numbers") };
        let (x, y) = line.split_once(',').expect("There should be 2 coordinates");
        (parse(x), parse(y))
    });
    for _ in 0..1024 {
        let (x, y) = iter.next().expect("There should be at least 1024 values");
        map[x][y] = false;
    }
    (map, iter)
}

static STEPS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone, Copy)]
struct Cost<T> {
    cost: u32,
    state: T,
}

impl<T: Eq> PartialOrd for Cost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl<T: Eq> Ord for Cost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<T: Eq> PartialEq for Cost<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T: Eq> Eq for Cost<T> {}

fn solve(map: &Map) -> Option<u32> {
    let start = Vec2D(0, 0);
    let end = Vec2D(MAP_SIZE - 1, MAP_SIZE - 1);
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Cost {
        state: start,
        cost: 0,
    }));
    while let Some(Reverse(state)) = queue.pop() {
        if !seen.insert(state.state) {
            continue;
        }
        let pos = state.state;
        if pos == end {
            return Some(state.cost);
        }
        for step in STEPS {
            let pos = pos + step;
            if pos.0 < MAP_SIZE && pos.1 < MAP_SIZE && map[pos.0][pos.1] {
                queue.push(Reverse(Cost {
                    state: pos,
                    cost: state.cost + 1,
                }));
            }
        }
    }
    None
}

#[must_use]
fn part1(map: &Map) -> u32 {
    solve(map).expect("There should be a solution")
}

#[must_use]
fn part2(map: &mut Map, bytes: impl Iterator<Item = (usize, usize)>) -> String {
    for (x, y) in bytes {
        map[x][y] = false;
        if solve(map).is_none() {
            return format!("{x},{y}");
        }
    }
    panic!("No solution found")
}
