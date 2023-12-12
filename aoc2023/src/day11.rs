use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    process_input(input, 2)
}

fn part2(input: &str) -> usize {
    process_input(input, 1_000_000)
}

fn process_input(input: &str, expand_factor: usize) -> usize {
    let mut positions: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(move |(j, char)| if char == '.' { None } else { Some((i, j)) })
        })
        .collect();
    expand(&mut positions, |(x, _)| x, expand_factor);
    positions.sort_unstable_by_key(|(_, y)| *y);
    expand(&mut positions, |(_, y)| y, expand_factor);
    positions
        .iter()
        .tuple_combinations()
        .map(|(pos1, pos2)| pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1))
        .sum()
}

fn expand(
    positions: &mut [(usize, usize)],
    get: impl Fn(&mut (usize, usize)) -> &mut usize,
    expand_factor: usize,
) {
    let mut prev = 0;
    let mut expanded = 0;
    for pos in positions {
        if *get(pos) > prev + 1 {
            expanded += (*get(pos) - (prev + 1)) * (expand_factor - 1);
        }
        prev = *get(pos);
        *get(pos) += expanded;
    }
}
