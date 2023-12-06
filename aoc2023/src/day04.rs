use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> usize {
    calculate_winned(input)
        .map(|n| if n == 0 { n } else { 1 << (n - 1) })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut counts = vec![1; input.lines().count()];
    for (i, n) in calculate_winned(input).enumerate() {
        let count = counts[i];
        for card in &mut counts[(i + 1)..(i + 1 + n)] {
            *card += count;
        }
    }
    counts.iter().sum()
}

fn calculate_winned(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(parse_line).map(|(winning, have)| {
        have.filter_map(|x: u8| winning.iter().find(|&&w| w == x))
            .count()
    })
}

fn parse_numbers(nums: &str) -> impl Iterator<Item = u8> + '_ {
    nums.split_whitespace()
        .map(|x| x.parse().expect("Should only try to parse numbers"))
}

fn parse_line(input: &str) -> ([u8; 10], impl Iterator<Item = u8> + '_) {
    let winning = utils_rust::collect_array(parse_numbers(&input[10..40]));
    let have = parse_numbers(&input[42..]);
    (winning, have)
}
