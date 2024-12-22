use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

struct Secret(u64);
impl Iterator for Secret {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        const MODULE: u64 = 16_777_216;
        self.0 = (self.0 ^ (self.0 << 6)) % MODULE;
        self.0 = (self.0 ^ (self.0 >> 5)) % MODULE;
        self.0 = (self.0 ^ (self.0 << 11)) % MODULE;
        Some(self.0)
    }
}

struct DeltaPrice(u8, Secret);

impl DeltaPrice {
    const fn new(n: u64) -> Self {
        Self((n % 10) as u8, Secret(n))
    }
}

impl Iterator for DeltaPrice {
    type Item = (i8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        let next_secret = self.1.next().expect("The iterator is infinite");
        let prev_price = self.0;
        self.0 = (next_secret % 10) as u8;
        #[allow(clippy::cast_possible_wrap)]
        Some((self.0 as i8 - prev_price as i8, self.0))
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .lines()
        .map(|line| line.parse().expect("All lines should have a valid number"))
}

#[must_use]
fn part1(input: &str) -> u64 {
    parse_input(input)
        .map(|state: u64| Secret(state).nth(1999).expect("The iterator is infinite"))
        .sum()
}

#[must_use]
fn part2(input: &str) -> u64 {
    *parse_input(input)
        .fold(HashMap::new(), |mut totals, seed| {
            let delta_price = DeltaPrice::new(seed);
            let sequences = delta_price
                .take(2000)
                .tuple_windows()
                .map(|((a, _), (b, _), (c, _), (d, price))| ((a, b, c, d), price))
                .unique_by(|x| x.0);
            for (seq, price) in sequences {
                *totals.entry(seq).or_default() += u64::from(price);
            }
            totals
        })
        .values()
        .max()
        .expect("There should be at least 1 sequence")
}
