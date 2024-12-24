use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::Str2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (map, start, end) = parse_input(input);
    let (cost, seen) = solve(&map, start, end);
    Ok(Solution::default()
        .part1(cost)
        .part2(part2(&map, &seen, end)))
}

fn parse_input(input: &str) -> (Str2D, Vec2D, Vec2D) {
    let map = Str2D::new(input);
    let start = map.find('S').expect("There should be a start point");
    let end = map.find('E').expect("There should be an end point");
    (map, Vec2D(start.0, start.1), Vec2D(end.0, end.1))
}

static STEPS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone, Copy)]
struct OrderedState<T> {
    cost: u32,
    state: T,
    parent: Option<T>,
}

impl<T: Eq> PartialOrd for OrderedState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl<T: Eq> Ord for OrderedState<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<T: Eq> PartialEq for OrderedState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T: Eq> Eq for OrderedState<T> {}

type State = (Vec2D, usize);
type SeenStates = HashMap<State, (u32, Vec<State>)>;

fn solve(map: &Str2D, start: Vec2D, end: Vec2D) -> (u32, SeenStates) {
    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(OrderedState {
        state: (start, 0),
        parent: None,
        cost: 0,
    }));
    let mut min_cost = u32::MAX;
    loop {
        let Reverse(state) = queue
            .pop()
            .expect("There should always be elements until we find the solution");
        if state.cost > min_cost {
            return (min_cost, seen);
        }
        if let Some(x) = seen.get_mut(&state.state) {
            if state.cost == x.0 {
                if let Some(parent) = state.parent {
                    x.1.push(parent);
                }
            }
            continue;
        }
        let parent = state.parent.map_or(Vec::new(), |p| vec![p]);
        seen.insert(state.state, (state.cost, parent));
        let (pos, rot) = state.state;
        if pos == end {
            min_cost = state.cost;
            continue;
        }
        for rot_offset in [0, 1, 3] {
            let next_rot = (rot + rot_offset) % 4;
            let next_pos = pos + STEPS[next_rot];
            if map.char(&next_pos.as_tuple()) != '#' {
                queue.push(Reverse(OrderedState {
                    state: (next_pos, next_rot),
                    parent: Some(state.state),
                    cost: state.cost + 1 + 1000 * u32::from(rot_offset != 0),
                }));
            }
        }
    }
}

fn expand_path(map: &Str2D, visited: &mut [bool], seen: &SeenStates, state: State) {
    if let Some((_, parents)) = seen.get(&state) {
        let (pos, _) = state;
        visited[pos.1 * map.cols() + pos.0] = true;
        for parent in parents {
            expand_path(map, visited, seen, *parent);
        }
    }
}

#[must_use]
fn part2(map: &Str2D, seen: &SeenStates, end: Vec2D) -> u32 {
    let mut visited = vec![false; map.cols() * map.rows()];
    for i in 0..4 {
        expand_path(map, &mut visited, seen, (end, i));
    }
    visited.iter().copied().map(u32::from).sum()
}
