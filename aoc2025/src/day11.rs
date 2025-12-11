use std::{collections::HashMap, error::Error};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let graph = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&graph))
        .part2(part2(&graph)))
}

struct Graph<'a> {
    ids: HashMap<&'a str, usize>,
    connections: Vec<Vec<usize>>,
}

#[must_use]
fn parse_input(input: &str) -> Graph<'_> {
    let mut total = 0;
    let mut ids = HashMap::new();
    let mut connections: Vec<Vec<usize>> = Vec::new();
    for line in input.split_terminator('\n') {
        let (origin, targets) = line.split_once(':').expect("There should be a `:`");
        let o = *ids.entry(origin).or_insert_with(|| {
            let id = total;
            total += 1;
            id
        });
        for target in targets[1..].split_whitespace() {
            let t = *ids.entry(target).or_insert_with(|| {
                let id = total;
                total += 1;
                id
            });
            if t >= connections.len() {
                connections.resize_with(t + 1, Default::default);
            }
            connections[t].push(o);
        }
    }
    Graph { ids, connections }
}

fn solve(g: &Graph, paths: &mut [Option<usize>], start: usize, node: usize) -> usize {
    let count: usize = g.connections[node]
        .iter()
        .map(|&o| {
            paths[o].map_or_else(
                || {
                    let res = solve(g, paths, start, o);
                    paths[o] = Some(res);
                    res
                },
                |x| x,
            )
        })
        .sum();
    count + usize::from(node == start)
}

#[must_use]
fn part1(g: &Graph) -> usize {
    solve(g, &mut vec![None; g.ids.len()], g.ids["you"], g.ids["out"])
}

fn solve2(
    g: &Graph,
    paths: &mut (&mut [Option<[usize; 4]>], [usize; 2]),
    start: usize,
    node: usize,
) -> [usize; 4] {
    let count: [usize; 4] = g.connections[node]
        .iter()
        .map(|&o| {
            paths.0[o].map_or_else(
                || {
                    let mut res = solve2(g, paths, start, o);
                    if o == paths.1[0] {
                        res[1] = res[0];
                        res[3] = res[2];
                    } else if o == paths.1[1] {
                        res[2] = res[0];
                        res[3] = res[1];
                    }
                    paths.0[o] = Some(res);
                    res
                },
                |x| x,
            )
        })
        .reduce(|mut acc, x| {
            for (acc, x) in std::iter::zip(&mut acc, x) {
                *acc += x;
            }
            acc
        })
        .unwrap_or([0, 0, 0, 0]);
    [
        count[0] + usize::from(node == start),
        count[1],
        count[2],
        count[3],
    ]
}

#[must_use]
fn part2(g: &Graph) -> usize {
    let mut cache = vec![None; g.ids.len()];
    let mut paths = (&mut cache[..], [g.ids["dac"], g.ids["fft"]]);
    solve2(g, &mut paths, g.ids["svr"], g.ids["out"])[3]
}
