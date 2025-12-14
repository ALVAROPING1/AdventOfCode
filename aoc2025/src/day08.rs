use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;
use utils_rust::UnionFind;

type Queue = BinaryHeap<Reverse<(usize, usize, usize)>>;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let positions = parse_input(input);
    let mut sets = UnionFind::new(positions.len());
    let mut distances: BinaryHeap<_> = (0..positions.len())
        .tuple_combinations()
        .map(|(a, b)| {
            Reverse((
                std::iter::zip(positions[a], positions[b])
                    .map(|(a, b)| a.abs_diff(b).pow(2))
                    .sum(),
                a,
                b,
            ))
        })
        .collect();
    Ok(Solution::default()
        .part1(part1(&mut distances, &mut sets))
        .part2(part2(&positions, &mut distances, &mut sets)))
}

#[must_use]
fn parse_input(input: &str) -> Vec<[usize; 3]> {
    input
        .split_terminator('\n')
        .map(|line| {
            utils_rust::parse::value_list_comma(line)
                .collect_array()
                .expect("There should be 3 values per line")
        })
        .collect()
}

#[must_use]
fn solve<F>(distances: &mut Queue, sets: &mut UnionFind, mut end: F) -> usize
where
    F: FnMut(&[usize], usize, usize, usize) -> Option<usize>,
{
    while let Some(Reverse((_, a, b))) = distances.pop() {
        let id = sets.union(a, b);
        if let Some(res) = end(sets.sizes(), id, a, b) {
            return res;
        }
    }
    unreachable!("There should always be a solution");
}

#[must_use]
fn part1(distances: &mut Queue, sets: &mut UnionFind) -> usize {
    let mut total = 0;
    solve(distances, sets, |sizes, _, _, _| {
        total += 1;
        (total == 1000).then(|| sizes.iter().k_largest(3).product())
    })
}

#[must_use]
fn part2(positions: &[[usize; 3]], distances: &mut Queue, sets: &mut UnionFind) -> usize {
    solve(distances, sets, |sizes, id, a, b| {
        (sizes[id] == positions.len()).then(|| positions[a][0] * positions[b][0])
    })
}
