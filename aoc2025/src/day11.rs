use std::collections::HashMap;
use std::error::Error;

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
    let mut total = 0..;
    let mut ids = HashMap::new();
    macro_rules! get_id {
        ($x:ident) => {
            *ids.entry($x).or_insert_with(|| total.next().unwrap())
        };
    }
    let mut connections: Vec<Vec<usize>> = Vec::new();
    for line in input.split_terminator('\n') {
        let (origin, targets) = line.split_once(':').expect("There should be a `:`");
        let o = get_id!(origin);
        for target in targets[1..].split_whitespace() {
            let t = get_id!(target);
            if t >= connections.len() {
                connections.resize_with(t + 1, Default::default);
            }
            connections[t].push(o);
        }
    }
    Graph { ids, connections }
}

fn solve<F, const N: usize>(
    g: &Graph,
    paths: &mut [Option<[usize; N]>],
    merge: F,
    start: usize,
    node: usize,
) -> [usize; N]
where
    F: Fn(&mut [usize; N], usize) + Copy,
{
    if node == start {
        let mut res = [0; N];
        res[0] = 1;
        return res;
    }
    g.connections[node]
        .iter()
        .map(|&o| {
            paths[o].map_or_else(
                || {
                    let mut res = solve(g, paths, merge, start, o);
                    merge(&mut res, o);
                    paths[o] = Some(res);
                    res
                },
                |x| x,
            )
        })
        .fold([0; N], |mut acc, x| {
            for (acc, x) in std::iter::zip(&mut acc, x) {
                *acc += x;
            }
            acc
        })
}

#[must_use]
fn part1(g: &Graph) -> usize {
    let mut paths = vec![None; g.ids.len()];
    solve::<_, 1>(g, &mut paths, |_, _| {}, g.ids["you"], g.ids["out"])[0]
}

#[must_use]
fn part2(g: &Graph) -> usize {
    let mut paths = vec![None; g.ids.len()];
    let midpoints = [g.ids["dac"], g.ids["fft"]];
    let merge = |paths: &mut [_; _], o| {
        if o == midpoints[0] {
            paths[1] = paths[0];
            paths[3] = paths[2];
        } else if o == midpoints[1] {
            paths[2] = paths[0];
            paths[3] = paths[1];
        }
    };
    solve::<_, 4>(g, &mut paths, merge, g.ids["svr"], g.ids["out"])[3]
}
