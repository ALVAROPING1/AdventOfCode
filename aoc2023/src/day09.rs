use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> i32 {
    process_input(input, |line| {
        process_history(line, |history, i| history[history.len() - i])
    })
}

fn part2(input: &str) -> i32 {
    process_input(input, |line| {
        process_history(
            line,
            |history, i| if i % 2 == 0 { -1 } else { 1 } * history[0],
        )
    })
}

fn process_input(input: &str, extrapolate: impl Fn(&str) -> i32) -> i32 {
    input.lines().map(extrapolate).sum()
}

fn process_history(input: &str, get_value: impl Fn(&[i32], usize) -> i32) -> i32 {
    let mut history: [i32; 21] = utils_rust::collect_array(utils_rust::parse::value_list(input));
    let mut res = 0;
    for i in 1..history.len() {
        let mut finished = true;
        // res += if i % 2 == 0 { -1 } else { 1 } * history[0];
        res += get_value(&history, i);
        for j in 0..history.len() - i {
            history[j] = history[j + 1] - history[j];
            finished = finished && history[j] == 0;
        }
        if finished {
            break;
        }
    }
    res
}
