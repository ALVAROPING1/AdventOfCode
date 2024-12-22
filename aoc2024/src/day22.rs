use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

const fn next(mut state: u64) -> u64 {
    const MODULE: u64 = 16_777_216;
    state = ((state << 6) ^ state) % MODULE;
    state = ((state >> 5) ^ state) % MODULE;
    ((state << 11) ^ state) % MODULE
}

fn parse_input(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .lines()
        .map(|line| line.parse().expect("All lines should have a valid number"))
}

#[must_use]
fn part1(input: &str) -> u64 {
    parse_input(input)
        .map(|state: u64| (0..2000).fold(state, |state, _| next(state)))
        .sum()
}

#[must_use]
fn part2(input: &str) -> u64 {
    let buyers = parse_input(input)
        // Convert initial state to hashmap of sequences of 4 changes to the price at the end of
        // the sequence
        .map(|mut state: u64| {
            // Calculate initial price
            let init = i8::try_from(state % 10).expect("The value should always fit");
            // Create iterator of the sequence of prices
            std::iter::repeat_with(move || {
                // Update state
                state = next(state);
                // Calculate price from state
                i8::try_from(state % 10).expect("The value should always fit")
            })
            // Take first N prices
            .take(2000)
            // Transform prices to tuples (change from the previous, current price)
            .scan(init, |prev, price| {
                let diff = price - *prev;
                *prev = price;
                Some((diff, price))
            })
            // Get windows of 4 consecutive tuples of (change, price)
            .tuple_windows()
            // Transform iterator to key-value tuples, where the key is 4 consecutive changes and
            // the value is the last price
            .map(|((a, _), (b, _), (c, _), (d, price))| ((a, b, c, d), price))
            // Store the key-value pairs on a hashmap, keeping only the first pair for each key
            .fold(HashMap::new(), |mut acc, (seq, price)| {
                acc.entry(seq).or_insert(price);
                acc
            })
        })
        .collect_vec();
    buyers
        .iter()
        // For each unique sequence on any buyer
        .flat_map(|buyer| buyer.keys())
        .unique()
        // Calculate the total bananas for each sequence
        .map(|seq| {
            #[allow(clippy::cast_sign_loss)]
            buyers
                .iter()
                .map(|buyer| *buyer.get(seq).unwrap_or(&0) as u64)
                .sum()
        })
        // Get the maximum total bananas for any sequence
        .max()
        .expect("There should be at least 1 sequence")
}
