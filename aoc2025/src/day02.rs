use std::collections::HashSet;
use std::error::Error;

use crate::prelude::*;

type Num<'a> = (usize, &'a str);

fn parse_int(x: &str) -> usize {
    x.parse().unwrap_or(0)
}

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let ranges = process_input(input);
    Ok(Solution::default()
        .part1(part1(&ranges))
        .part2(part2(&ranges)))
}

fn process_input(input: &str) -> Vec<(Num<'_>, Num<'_>)> {
    input[..input.len() - 1]
        .split(',')
        .map(|x| {
            let (s, e) = x.split_once('-').expect("There should be a `-`");
            ((parse_int(s), s), (parse_int(e), e))
        })
        .collect()
}

#[must_use]
fn solve<'a, F, G>(ranges: &[(Num<'a>, Num<'a>)], gen: F) -> usize
where
    F: Fn(&str, &str) -> G,
    G: Iterator<Item = usize>,
{
    let mut total = 0;
    let mut set = HashSet::new();
    for ((start, start_str), (end, end_str)) in ranges {
        for x in gen(start_str, end_str) {
            if x >= *start && x <= *end && !set.contains(&x) {
                total += x;
                set.insert(x);
            }
        }
    }
    total
}

#[must_use]
fn part1(ranges: &[(Num<'_>, Num<'_>)]) -> usize {
    solve::<_, _>(ranges, |start, end| {
        let (len02, len12) = (start.len() / 2, end.len().div_ceil(2));
        let (start, end) = (parse_int(&start[..len02]), parse_int(&end[..len12]));
        (start..=end).map(|x| parse_int(&format!("{x}{x}")))
    })
}

#[must_use]
fn part2(ranges: &[(Num<'_>, Num<'_>)]) -> usize {
    solve::<_, _>(ranges, |start, end| {
        (2..=end.len())
            .flat_map(|reps| {
                let (len0, len1) = (start.len() / reps, end.len().div_ceil(reps));
                let (start, end) = (parse_int(&start[..len0]), parse_int(&end[..len1]));
                (start..=end).map(move |x| parse_int(&x.to_string().repeat(reps)))
            })
            .collect::<Vec<_>>()
            .into_iter()
    })
}
