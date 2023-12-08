use std::error::Error;
use utils_rust::parse;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u64 {
    process_input(input, parse::value_list::<u64>)
}

fn part2(input: &str) -> u64 {
    process_input(input, |input| {
        [input
            .bytes()
            .filter(|&x| x != b' ')
            .fold(0, |acc, x| acc * 10 + u64::from(parse::from_ascii_digit(x)))]
        .into_iter()
    })
}

#[allow(
    clippy::cast_sign_loss, // The numbers are always positive
    clippy::cast_possible_truncation, // The numbers are small enough
    clippy::cast_precision_loss // The numbers are small enough
)]
fn process_input<'a, I>(input: &'a str, parse_values: impl Fn(&'a str) -> I) -> u64
where
    I: Iterator<Item = u64> + 'a,
{
    let mut iter = input.lines();
    let time = parse_values(&iter.next().expect("There should be a time line")[13..]);
    let distance = parse_values(&iter.next().expect("There should be a distance line")[12..]);
    time.zip(distance)
        .map(|(t, d)| {
            // Solving the equation `x(t) = (T - t)t = A`
            // gives the end points of the range of solutions
            let sqrt = ((t * t - 4 * d) as f64).sqrt();
            if sqrt.is_nan() {
                return 0;
            }
            let (min, max) = (
                ((t as f64 - sqrt) / 2.0).ceil() as u64,
                ((t as f64 + sqrt) / 2.0).ceil() as u64,
            );
            max - min
        })
        .product()
}
