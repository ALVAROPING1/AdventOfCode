use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;
use utils_rust::NKBits;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let problems = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&problems))
        .part2(part2(&problems)))
}

#[derive(Debug)]
struct Problem {
    pub goal: u16,
    pub buttons: Vec<u16>,
    pub energy: Vec<u16>,
}

#[must_use]
fn parse_input(input: &str) -> Vec<Problem> {
    fn parse(x: &str) -> u16 {
        x.parse().expect("All values should be integers")
    }
    fn num_list(x: &str) -> impl Iterator<Item = u16> + '_ {
        x[1..x.len() - 1].split(',').map(parse)
    }
    input
        .split_terminator('\n')
        .map(|line| {
            let (lights, rest) = line
                .split_once(' ')
                .expect("There should be at least 2 spaces ");
            let (buttons, energy) = rest
                .rsplit_once(' ')
                .expect("There should be at least 2 spaces ");
            let goal = lights[1..lights.len() - 1]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|x| x.0)
                .fold(0, |acc, i| acc | (1 << i));
            let buttons = buttons
                .split(' ')
                .map(|pos| num_list(pos).fold(0, |acc, i| acc | (1 << i)))
                .collect();
            let energy = num_list(energy).collect();
            Problem {
                goal,
                buttons,
                energy,
            }
        })
        .collect()
}

/// Generate all combinations of `size` button presses with standard itertools'
/// [`Itertools::combinations()`] method
#[allow(dead_code)]
fn itertools_combinations(
    buttons: &[u16],
    size: usize,
) -> impl Iterator<Item = impl Iterator<Item = u16> + '_> + '_ {
    buttons
        .iter()
        .copied()
        .combinations(size)
        .map(IntoIterator::into_iter)
}

/// Generate all combinations of `size` button presses with bit hacks, doing no allocations
fn bit_hack_combinations(
    buttons: &[u16],
    size: usize,
) -> impl Iterator<Item = impl Iterator<Item = u16> + '_> + '_ {
    // Represent each combination with an `N` bit field, where the i-th bit determines if the i-th
    // button is pressed
    NKBits::new(buttons.len(), size).map(move |mut combination| {
        std::iter::from_fn(move || {
            if combination == 0 {
                return None;
            }
            // Get the first pressed button, remove it from the current combination, and yield it
            let i = combination.trailing_zeros();
            combination ^= 1 << i;
            Some(buttons[i as usize])
        })
    })
}

#[must_use]
fn part1(problems: &[Problem]) -> usize {
    fn solve(p: &Problem) -> usize {
        // Iterative depth-first search
        // For each amount of buttons to press, generate all combinations of that amount of unique
        // button presses and check if any of them is a solution
        for size in 1..p.buttons.len() {
            for combination in bit_hack_combinations(&p.buttons, size) {
                // Pressing a button is equivalent to XORing its bit field of connected lights with
                // the current state (initially 0)
                let state = combination
                    .reduce(|acc, x| acc ^ x)
                    .expect("There should be at least 1 element");
                if state == p.goal {
                    return size;
                }
            }
        }
        unreachable!("There should always be a solution");
    }

    problems.iter().map(solve).sum()
}

#[must_use]
fn part2(problems: &[Problem]) -> u64 {
    fn solve(p: &Problem) -> u64 {
        use z3::ast::{Bool, Int};
        // This is an Integer Linear Programming problem (ILP), so give the constraints and
        // optimization goal to an ILP solver

        let presses: Vec<_> = (0..p.buttons.len())
            .map(|_| Int::fresh_const("press"))
            .collect();
        let optimizer = z3::Optimize::new();

        optimizer.assert(&Bool::and(&presses.iter().map(|p| p.ge(0)).collect_vec()));
        for (i, &energy) in p.energy.iter().enumerate() {
            optimizer.assert(
                &Int::add(
                    &p.buttons
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &b)| (b & (1 << i) != 0).then_some(&presses[j]))
                        .collect_vec(),
                )
                .eq(u32::from(energy)),
            );
        }
        optimizer.minimize(&Int::add(&presses));

        assert_eq!(optimizer.check(&[]), z3::SatResult::Sat);
        let model = optimizer
            .get_model()
            .expect("We already checked that there is a solution");
        presses
            .iter()
            .map(|press| {
                model
                    .eval(press, false)
                    .expect("The variable evaluation shouldn't fail")
                    .as_u64()
                    .expect("The result should be positive and fit in a `u64`")
            })
            .sum()
    }

    problems.iter().map(solve).sum()
}
