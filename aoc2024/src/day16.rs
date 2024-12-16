use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::hash::Hash;

use crate::prelude::*;
use utils_rust::parse::Str2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (map, start, end) = parse_input(input);
    Ok(Solution::default().part1(part1(&map, start, end)))
    // .part2(part2(&map, start, end)))
}

fn parse_input(input: &str) -> (Str2D, Vec2D, Vec2D) {
    let map = Str2D::new(input);
    let start = map.find('S').expect("There should be a start point");
    let end = map.find('E').expect("There should be an end point");
    (map, Vec2D(start.0, start.1), Vec2D(end.0, end.1))
}

static STEPS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone, Copy)]
struct Cost<T> {
    cost: u32,
    state: T,
}

impl<T: Hash> Hash for Cost<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
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

fn solve(map: &Str2D, start: Vec2D, end: Vec2D) -> u32 {
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Cost {
        state: (start, 0),
        cost: 0,
    }));
    loop {
        let Reverse(state) = queue
            .pop()
            .expect("There should always be elements until we find the solution");
        if !seen.insert(state.state) {
            continue;
        }
        let (pos, rot) = state.state;
        if pos == end {
            return state.cost;
        }
        let forwards = pos + STEPS[rot];
        if map.char(&forwards.as_tuple()) != '#' {
            queue.push(Reverse(Cost {
                state: (forwards, rot),
                cost: state.cost + 1,
            }));
        }
        let left_orientation = (rot + 3) % 4;
        let left = pos + STEPS[left_orientation];
        if map.char(&left.as_tuple()) != '#' {
            queue.push(Reverse(Cost {
                state: (left, left_orientation),
                cost: state.cost + 1001,
            }));
        }
        let right_orientation = (rot + 1) % 4;
        let right = pos + STEPS[right_orientation];
        if map.char(&right.as_tuple()) != '#' {
            queue.push(Reverse(Cost {
                state: (right, right_orientation),
                cost: state.cost + 1001,
            }));
        }
    }
}

#[must_use]
fn part1(map: &Str2D, start: Vec2D, end: Vec2D) -> u32 {
    solve(map, start, end)
}

// #[must_use]
// fn part2(map: &Str2D, start: Vec2D, end: Vec2D) -> u32 {
//     todo!()
// }
