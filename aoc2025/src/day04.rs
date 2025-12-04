use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::String2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let mut map = String2D::new(input);
    Ok(Solution::default()
        .part1(solve::<false>(&mut map))
        .part2(solve::<true>(&mut map)))
}

#[must_use]
fn solve<const REMOVE: bool>(mut map: &mut String2D) -> usize {
    let add = |a: usize, b, c| {
        a.checked_add_signed(b as isize)
            .and_then(|x| (x < c).then_some(x))
    };
    let mut total = 0;
    for round in 0.. {
        let mut round_total = 0;
        for y in 0..map.rows() {
            for x in 0..map.cols() {
                if map.char(&(x, y)) != '@' {
                    continue;
                }
                let count = utils_rust::DIR8
                    .iter()
                    .filter_map(|offset| {
                        let i = add(y, offset.1, map.rows());
                        let j = add(x, offset.0, map.cols());
                        j.zip(i)
                    })
                    .filter(|pos| map.char(pos) == '@')
                    .count();
                if count < 4 {
                    round_total += 1;
                    if REMOVE {
                        map.replace(&(x, y), b'.');
                    }
                }
            }
        }
        total += round_total;
        if round_total == 0 || !REMOVE {
            break;
        }
    }
    total
}
