use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;
use utils_rust::{numbers::iter_ones, NKBits};

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
    fn num_list(x: &str) -> impl Iterator<Item = u16> + '_ {
        utils_rust::parse::value_list_comma(&x[1..x.len() - 1])
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
    NKBits::new(buttons.len(), size)
        .map(move |combination| iter_ones(combination).map(|i| buttons[i]))
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
                if combination.fold(0, |acc, x| acc ^ x) == p.goal {
                    return size;
                }
            }
        }
        unreachable!("There should always be a solution");
    }

    problems.iter().map(solve).sum()
}

// Solve each part 2 problem using an ILP solver
#[cfg(feature = "external-solver")]
#[allow(dead_code)]
fn solve_z3(p: &Problem) -> u64 {
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

// Solve part 2 problems recursively
//
// NOTE: idea from <https://reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/>
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn solve_dfs(p: &Problem, energy: &[i32], cache: &mut HashMap<Vec<i32>, u64>, max: u64) -> u64 {
    if let Some(x) = cache.get(energy) {
        return *x;
    }
    // Base cases
    // If we overshoot any energy counter, there is no solution through this branch
    if energy.iter().any(|x| *x < 0) {
        return 100_000;
    }
    // If the minimum amount of presses possible is bigger than the best solution found, don't
    // bother with this branch
    if energy.iter().fold(0, |acc, &x| acc.max(x)) as u64 > max {
        return 100_000;
    }
    // If we reached a solution, we don't need any extra presses
    if energy.iter().all(|x| *x == 0) {
        return 0;
    }
    // Recursive case
    // Calculate the combination of button presses that would make the remaining energy for each
    // counter even, pressing each button at most once
    let goal = energy
        .iter()
        .enumerate()
        .filter_map(|(i, x)| ((x & 1) != 0).then_some(i as u16))
        .fold(0, |acc, x| acc | (1 << x));
    let combinations = (0u16..1 << p.buttons.len()).filter(|&combination| {
        let buttons = p.buttons.iter().enumerate();
        let res = buttons
            .filter_map(|(i, x)| ((combination & (1 << i)) != 0).then_some(x))
            .fold(0, |acc, x| acc ^ x);
        res == goal
    });

    let mut min = 100_000;
    let mut prev_energy = energy.to_owned();
    // Recursively try each combination of buttons
    for comb in combinations {
        // Remove the energy from the combination of buttons from the goal. This leaves all
        // counters with an even goal, which means remaining presses must come from pressing each
        // button an even amount of times, so we can solve the problem recursively for the first
        // half of button presses/target goals and duplicate them. The solution will be the minimum
        // amount of presses needed between each of the combinations for the current step

        for button in iter_ones(comb.into()).map(|i| p.buttons[i]) {
            for j in iter_ones(button.into()) {
                prev_energy[j] -= 1;
            }
        }
        for e in &mut prev_energy {
            *e /= 2;
        }
        let presses = u64::from(comb.count_ones());
        let max = (min.min(max) - presses) / 2;
        min = min.min(solve_dfs(p, &prev_energy, cache, max) * 2 + presses);
        prev_energy.copy_from_slice(energy);
    }
    cache.insert(prev_energy, min);
    min
}

#[must_use]
fn part2(problems: &[Problem]) -> u64 {
    let energy = |p: &Problem| p.energy.iter().map(|x| i32::from(*x)).collect_vec();
    problems
        .iter()
        .map(|p| solve_dfs(p, &energy(p), &mut HashMap::new(), 100_000))
        .sum()
}
