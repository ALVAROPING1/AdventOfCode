use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let input = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&input))
        .part2(part2(&input)))
}

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

fn parse(input: &str) -> i64 {
    input.parse().expect("This should only parse numbers")
}

fn parse_input(input: &str) -> Vec<Machine> {
    input[..input.len() - 1]
        .split_terminator("\n\n")
        .map(|machine| {
            let a = (parse(&machine[12..14]), parse(&machine[18..20]));
            let b = (parse(&machine[33..35]), parse(&machine[39..41]));
            let target = &machine[51..]
                .split_once(',')
                .expect("There should always be 2 values");
            let target = (parse(target.0), parse(&target.1[3..]));
            Machine { a, b, target }
        })
        .collect()
}

#[must_use]
fn solve(input: &[Machine], offset: i64) -> i64 {
    input
        .iter()
        .filter_map(|machine| {
            let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
            if det == 0 {
                return None;
            }
            let adj = ((machine.b.1, -machine.b.0), (-machine.a.1, machine.a.0));
            let target = (machine.target.0 + offset, machine.target.1 + offset);
            let a = adj.0 .0 * target.0 + adj.0 .1 * target.1;
            let b = adj.1 .0 * target.0 + adj.1 .1 * target.1;
            if a % det != 0 || b % det != 0 {
                return None;
            }
            let a = a / det;
            let b = b / det;
            Some(3 * a + b)
        })
        .sum()
}

#[must_use]
fn part1(input: &[Machine]) -> i64 {
    solve(input, 0)
}

#[must_use]
fn part2(input: &[Machine]) -> i64 {
    solve(input, 10_000_000_000_000)
}
