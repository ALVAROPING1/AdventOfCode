use std::collections::HashSet;
use std::error::Error;

use crate::prelude::*;

type Num = (usize, usize);

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let ranges = process_input(input);
    Ok(Solution::default()
        .part1(solve::<false>(&ranges, |_| 2))
        .part2(solve::<true>(&ranges, |end| end)))
}

fn process_input(input: &str) -> Vec<(Num, Num)> {
    let parse = |x: &str| -> usize { x.parse().expect("All values should be integers") };
    input[..input.len() - 1]
        .split(',')
        .map(|x| {
            let (s, e) = x.split_once('-').expect("There should be a `-`");
            ((parse(s), s.len()), (parse(e), e.len()))
        })
        .collect()
}

#[must_use]
fn solve<const SET: bool>(ranges: &[(Num, Num)], reps: impl Fn(usize) -> usize) -> usize {
    let mut total = 0;
    let mut set = HashSet::new();
    for ((start, start_size), (end, end_size)) in ranges {
        let candidates = (2..=reps(*end_size)).flat_map(|reps| {
            let chunk_size = start_size / reps;
            let shift = u32::try_from(start_size - chunk_size).expect("Numbers should be small");
            let (start, end) = (start / 10usize.pow(shift), end / 10usize.pow(shift));
            (start..=end).filter(|x: &usize| *x != 0).map(move |x| {
                let shift = 10usize.pow(x.ilog10() + 1);
                (0..reps).fold(0, |acc, _| acc * shift + x)
            })
        });
        for x in candidates {
            if x >= *start && x <= *end && !(SET && set.contains(&x)) {
                total += x;
                if SET {
                    set.insert(x);
                }
            }
        }
    }
    total
}
