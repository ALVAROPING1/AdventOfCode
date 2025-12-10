use std::collections::{HashSet, VecDeque};
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let problems = parse_input(input);
    Ok(Solution::default().part1(part1(&problems)))
    // .part2(part2(&points)))
}

#[derive(Debug)]
struct Problem {
    pub goal: usize,
    pub buttons: Vec<usize>,
    pub energy: Vec<usize>,
}

#[must_use]
fn parse_input(input: &str) -> Vec<Problem> {
    fn parse(x: &str) -> usize {
        x.parse().expect("All values should be integers")
    }
    fn num_list(x: &str) -> impl Iterator<Item = usize> + '_ {
        x[1..x.len() - 1].split(',').map(parse)
    }
    input
        .split_terminator('\n')
        .map(|line| {
            let (lights, rest) = line
                .split_once(' ')
                .expect("There should be at least 2 spaces ");
            let (buttons, energy) = rest
                .rsplit_once(' ')
                .expect("There should be at least 2 spaces ");
            let goal = lights[1..lights.len() - 1]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|x| x.0)
                .fold(0, |acc, i| acc | (1 << i));
            let buttons = buttons
                .split(' ')
                .map(|pos| num_list(pos).fold(0, |acc, i| acc | (1 << i)))
                .collect();
            let energy = num_list(energy).collect();
            Problem {
                goal,
                buttons,
                energy,
            }
        })
        .collect()
}

fn solve(p: &Problem) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((0, 0));
    while let Some((cost, state)) = queue.pop_front() {
        if !seen.insert(state) {
            continue;
        }
        for &button in &p.buttons {
            let next = state ^ button;
            if next == p.goal {
                return cost + 1;
            }
            queue.push_back((cost + 1, next));
        }
    }
    unreachable!("There should always be a solution");
}

#[must_use]
fn part1(problems: &[Problem]) -> usize {
    problems.iter().map(solve).sum()
}
