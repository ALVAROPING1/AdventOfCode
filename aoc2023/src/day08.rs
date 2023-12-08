use std::{collections::HashMap, error::Error};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    let (instructions, map) = process_input(input);
    find_distance(&map, instructions, "AAA", |x| x == "ZZZ")
}

fn part2(input: &str) -> usize {
    let (instructions, map) = process_input(input);
    map.keys()
        .copied()
        .filter(|&x| x.as_bytes()[2] == b'A')
        .map(|node| find_distance(&map, instructions, node, |x| x.as_bytes()[2] == b'Z'))
        .reduce(utils_rust::numbers::lcm)
        .expect("There should be a result")
}

fn process_input(input: &str) -> (&str, HashMap<&str, [&str; 2]>) {
    let mut iter = input.split("\n\n");
    let instructions = iter.next().expect("There should be a list of instructions");
    let map = iter
        .next()
        .expect("There should be a map of connected nodes")
        .lines()
        .map(|line| (&line[..3], [&line[7..10], &line[12..15]]))
        .collect();
    (instructions, map)
}

fn find_distance<'a>(
    map: &HashMap<&'a str, [&'a str; 2]>,
    instructions: &str,
    mut node: &'a str,
    is_goal: impl Fn(&str) -> bool,
) -> usize {
    let mut step = 0;
    while !is_goal(node) {
        node = map[node][usize::from(instructions.as_bytes()[step % instructions.len()] != b'L')];
        step += 1;
    }
    step
}
