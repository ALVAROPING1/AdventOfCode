use itertools::Itertools;
use std::{error::Error, fmt::Debug, mem::swap};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u64 {
    process_input(
        input,
        |input| {
            input
                .split_whitespace()
                .map(|x| x.parse().expect("Should only try to parse numbers"))
                .collect()
        },
        |map, &val| {
            vec![map
                .iter()
                .find(|&&entry| val >= entry[1] && val < entry[1] + entry[2])
                .map_or(val, |&entry| entry[0] + val - entry[1])]
        },
        |vals| *vals.iter().min().expect("There should be at least 1 value"),
    )
}

fn part2(input: &str) -> u64 {
    process_input(
        input,
        |input| -> Vec<(u64, u64)> {
            input
                .split_whitespace()
                .map(|x| x.parse().expect("Should only try to parse numbers"))
                .tuples()
                .collect()
        },
        |map, &val| {
            let mut min = val.0;
            let max = val.0 + val.1;
            let mut res: Vec<_> = map
                .iter()
                .skip_while(|&&entry| entry[1] + entry[2] < val.0)
                .take_while(|&&entry| entry[1] < max)
                .flat_map(|&entry| {
                    let _ = 0;
                    let mut res = vec![];
                    if min < entry[1] {
                        res.push((min, entry[1] - min));
                        min = entry[1];
                    }
                    let next = u64::min(entry[1] + entry[2], max);
                    res.push((entry[0] + min - entry[1], next - min));
                    min = next;

                    res.into_iter()
                })
                .collect();
            if min < max {
                res.push((min, max - min));
            }
            res
        },
        |vals| {
            vals.iter()
                .map(|val| val.0)
                .min()
                .expect("There should be at least 1 value")
        },
    )
}

fn process_input<T, S, F, M>(input: &str, get_seeds: S, map_val: F, min_val: M) -> u64
where
    T: Debug,
    S: Fn(&str) -> Vec<T>,
    F: Fn(&[[u64; 3]], &T) -> Vec<T>,
    M: Fn(&[T]) -> u64,
{
    let mut iter = input.split("\n\n");
    let mut values = get_seeds(&iter.next().expect("There should be a list of seeds")[7..]);
    let mut next_values = vec![];
    for map in iter {
        let map = parse_map(map);
        for val in &values {
            next_values.extend(map_val(&map, val));
        }
        swap(&mut values, &mut next_values);
        next_values.clear();
    }
    min_val(&values)
}

fn parse_map(map: &str) -> Vec<[u64; 3]> {
    let mut res: Vec<_> = map
        .lines()
        .skip(1)
        .map(|line| {
            utils_rust::collect_array(
                line.split_whitespace()
                    .map(|x| x.parse().expect("Should only try to parse numbers")),
            )
        })
        .collect();
    res.sort_unstable_by_key(|x| x[1]);
    res
}
