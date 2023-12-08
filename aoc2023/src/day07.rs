use itertools::Itertools;
use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    process_input(input, |hand| {
        hand_value(hand, &PART1_VALUES, |mut amount| {
            amount.sort_unstable();
            if amount[12] == 5 {
                6
            } else if amount[12] == 4 {
                5
            } else if amount[12] == 3 && amount[11] == 2 {
                4
            } else if amount[12] == 3 {
                3
            } else if amount[12] == 2 && amount[11] == 2 {
                2
            } else {
                u32::from(amount[12] == 2)
            }
        })
    })
}

fn part2(input: &str) -> usize {
    process_input(input, |hand| {
        hand_value(hand, &PART2_VALUES, |mut amount| {
            amount[1..].sort_unstable();
            if amount[12] + amount[0] == 5 {
                6
            } else if amount[12] + amount[0] == 4 {
                5
            } else if amount[12] + amount[0] == 3 && amount[11] + amount[0] - (3 - amount[12]) == 2
            {
                4
            } else if amount[12] + amount[0] == 3 {
                3
            } else if amount[12] + amount[0] == 2 && amount[11] + amount[0] - (2 - amount[12]) == 2
            {
                2
            } else {
                u32::from(amount[12] + amount[0] == 2)
            }
        })
    })
}

static PART1_VALUES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

static PART2_VALUES: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn process_input(input: &str, hand_value: impl Fn(&str) -> u32) -> usize {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let hand = hand_value(iter.next().expect("There should be a hand"));
            let value: usize = iter
                .next()
                .expect("The hand should have a bid")
                .parse()
                .expect("The bid should be a number");
            (hand, value)
        })
        .sorted_unstable_by_key(|x| x.0)
        .enumerate()
        .map(|res| (res.0 + 1) * res.1 .1)
        .sum()
}

fn hand_value(input: &str, values: &[char; 13], hand_type: impl Fn([u8; 13]) -> u32) -> u32 {
    let mut value: u32 = 0;
    let mut amount = [0; 13];
    for x in input.chars() {
        let x = values
            .iter()
            .position(|&val| val == x)
            .expect("All the card types should have a value");
        value = value * 13 + u32::try_from(x).expect("The number should be small enough");
        amount[x] += 1;
    }
    hand_type(amount) * 13_u32.pow(5) + value
}
