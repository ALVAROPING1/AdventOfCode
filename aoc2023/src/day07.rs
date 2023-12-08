use itertools::Itertools;
use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    process_input(input)
}

fn part2(input: &str) -> usize {
    process_input2(input)
}

fn process_input(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let hand = calculate_hand_value(iter.next().unwrap());
            let value: usize = iter.next().unwrap().parse().unwrap();
            (hand, value)
        })
        .sorted_unstable_by_key(|x| x.0)
        .enumerate()
        .map(|res| (res.0 + 1) * res.1 .1)
        .sum()
}

fn calculate_hand_value(input: &str) -> u32 {
    static VALUES: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let mut value: u32 = 0;
    let mut amount = [0; 13];
    for x in input.chars() {
        let x = VALUES.iter().position(|&val| val == x).unwrap();
        value = value * 13 + x as u32;
        amount[x] += 1;
    }
    amount.sort_unstable();
    let hand_type = if amount[12] == 5 {
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
    };
    hand_type * 13_u32.pow(5) + value
}

fn process_input2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let hand = calculate_hand_value2(iter.next().unwrap());
            let value: usize = iter.next().unwrap().parse().unwrap();
            (hand, value)
        })
        .sorted_unstable_by_key(|x| x.0)
        .enumerate()
        .map(|res| (res.0 + 1) * res.1 .1)
        .sum()
}

fn calculate_hand_value2(input: &str) -> u32 {
    static VALUES: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let mut value: u32 = 0;
    let mut amount = [0; 13];
    for x in input.chars() {
        let x = VALUES.iter().position(|&val| val == x).unwrap();
        value = value * 13 + x as u32;
        amount[x] += 1;
    }
    amount[1..].sort_unstable();
    let hand_type = if amount[12] + amount[0] == 5 {
        6
    } else if amount[12] + amount[0] == 4 {
        5
    } else if amount[12] + amount[0] == 3 && amount[11] + amount[0] - (3 - amount[12]) == 2 {
        4
    } else if amount[12] + amount[0] == 3 {
        3
    } else if amount[12] + amount[0] == 2 && amount[11] + amount[0] - (2 - amount[12]) == 2 {
        2
    } else {
        u32::from(amount[12] + amount[0] == 2)
    };
    hand_type * 13_u32.pow(5) + value
}
