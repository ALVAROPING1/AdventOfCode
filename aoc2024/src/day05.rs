use std::{cmp::Ordering, error::Error};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (rules, mut updates) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&rules, &mut updates))
        .part2(part2(&rules, &mut updates)))
}

fn parse_input(input: &str) -> ([[Option<bool>; 100]; 100], Vec<Vec<u8>>) {
    let parse_num = |x: &str| -> usize { x.parse().expect("There should always be a number") };
    let (rules_str, updates_str) = input
        .split_once("\n\n")
        .expect("There should be a rules section and an updates section");
    let mut rules = [[None; 100]; 100];
    for rule in rules_str.split_terminator('\n') {
        let (first, second) = (parse_num(&rule[..2]), parse_num(&rule[3..5]));
        rules[second][first] = Some(true);
        rules[first][second] = Some(false);
    }
    let updates = updates_str
        .split_terminator('\n')
        .map(|update| utils_rust::parse::value_list_comma(update).collect())
        .collect();
    (rules, updates)
}

fn solve(
    rules: &[[Option<bool>; 100]],
    updates: &mut [Vec<u8>],
    filter_correct: bool,
    map: impl Fn(&mut Vec<u8>) -> u16,
) -> u16 {
    updates
        .iter_mut()
        .filter(|update| {
            for (i, &x) in update.iter().enumerate() {
                for &y in &update[i + 1..] {
                    if rules[usize::from(x)][usize::from(y)] == Some(true) {
                        return !filter_correct;
                    }
                }
            }
            filter_correct
        })
        .map(map)
        .sum()
}

#[must_use]
fn part1(rules: &[[Option<bool>; 100]], updates: &mut [Vec<u8>]) -> u16 {
    solve(rules, updates, true, |update| {
        u16::from(update[update.len() / 2])
    })
}

#[must_use]
fn part2(rules: &[[Option<bool>; 100]], updates: &mut [Vec<u8>]) -> u16 {
    solve(rules, updates, false, |update| {
        update.sort_by(|&a, &b| match rules[usize::from(a)][usize::from(b)] {
            None => Ordering::Equal,
            Some(true) => Ordering::Greater,
            Some(false) => Ordering::Less,
        });
        u16::from(update[update.len() / 2])
    })
}
