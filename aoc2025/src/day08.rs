use std::collections::BinaryHeap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let positions = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&positions))
        .part2(part2(&positions)))
}

#[must_use]
fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    input
        .split_terminator('\n')
        .map(|line| {
            line.split(',')
                .map(parse)
                .collect_tuple()
                .expect("There should be 3 values per line")
        })
        .collect()
}

#[must_use]
fn solve<F>(positions: &[(usize, usize, usize)], mut end: F) -> usize
where
    F: FnMut(&[usize], usize, usize, usize) -> Option<usize>,
{
    let mut sizes: Vec<_> = std::iter::repeat_n(1, positions.len()).collect();
    let mut ids: Vec<_> = (0..positions.len()).collect();
    let mut distances: BinaryHeap<_> = ids
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| {
            (
                std::cmp::Reverse(
                    (positions[a].0.abs_diff(positions[b].0).pow(2))
                        + (positions[a].1.abs_diff(positions[b].1).pow(2))
                        + (positions[a].2.abs_diff(positions[b].2).pow(2)),
                ),
                a,
                b,
            )
        })
        .collect();
    while let Some((_, a, b)) = distances.pop() {
        let min = ids[a].min(ids[b]);
        let max = ids[a].max(ids[b]);
        if min != max {
            sizes[min] += sizes[max];
            sizes[max] = 0;
            for id in &mut ids {
                if *id == max {
                    *id = min;
                }
            }
        }
        if let Some(res) = end(&sizes, min, a, b) {
            return res;
        }
    }
    unreachable!("There should always be a solution");
}

#[must_use]
fn part1(positions: &[(usize, usize, usize)]) -> usize {
    let mut total = 0;
    solve(positions, |sizes, _, _, _| {
        total += 1;
        (total == 1000).then(|| sizes.iter().k_largest(3).product())
    })
}

#[must_use]
fn part2(positions: &[(usize, usize, usize)]) -> usize {
    solve(positions, |sizes, id, a, b| {
        (sizes[id] == positions.len()).then(|| positions[a].0 * positions[b].0)
    })
}
