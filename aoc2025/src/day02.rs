use std::collections::HashSet;
use std::error::Error;

use crate::prelude::*;

type Num<'a> = (usize, &'a str);

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let ranges = process_input(input);
    Ok(Solution::default()
        .part1(part1(&ranges))
        .part2(part2(&ranges)))
}

fn process_input(input: &str) -> Vec<(Num<'_>, Num<'_>)> {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    input[..input.len() - 1]
        .split(',')
        .map(|x| {
            let (s, e) = x.split_once('-').expect("There should be a `-`");
            ((parse(s), s), (parse(e), e))
        })
        .collect()
}

#[must_use]
fn solve<'a, F, G, const SET: bool>(ranges: &[(Num<'a>, Num<'a>)], gen: F) -> usize
where
    F: Fn(&str, &str, usize, usize) -> G,
    G: Iterator<Item = usize>,
{
    let mut total = 0;
    let mut set = HashSet::new();
    for ((start, start_str), (end, end_str)) in ranges {
        for x in gen(start_str, end_str, *start, *end) {
            if x >= *start && x <= *end && !(SET && set.contains(&x)) {
                total += x;
                if SET {
                    set.insert(x);
                }
            }
        }
    }
    total
}

#[must_use]
fn part1(ranges: &[(Num<'_>, Num<'_>)]) -> usize {
    solve::<_, _, false>(ranges, |start, _, s, e| {
        let chunk_size = start.len() / 2;
        let shift = u32::try_from(start.len() - chunk_size).expect("Numbers should be small");
        let (start, end) = (s / 10usize.pow(shift), e / 10usize.pow(shift));
        (start..=end).filter(|&x| x != 0).map(|x| {
            let shift = 10usize.pow(x.ilog10() + 1);
            x * shift + x
        })
    })
}

#[must_use]
fn part2(ranges: &[(Num<'_>, Num<'_>)]) -> usize {
    solve::<_, _, true>(ranges, |start, end, s, e| {
        (2..=end.len())
            .flat_map(|reps| {
                let chunk_size = start.len() / reps;
                let shift =
                    u32::try_from(start.len() - chunk_size).expect("Numbers should be small");
                let (start, end) = (s / 10usize.pow(shift), e / 10usize.pow(shift));
                (start..=end).filter(move |&x| x != 0).map(move |x| {
                    let shift = 10usize.pow(x.ilog10() + 1);
                    (0..reps).fold(0, |acc, _| acc * shift + x)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    })
}
