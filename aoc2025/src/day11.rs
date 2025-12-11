use std::{collections::HashMap, error::Error};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let graph = parse_input(input);
    Ok(Solution::default().part1(part1(&graph)))
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
