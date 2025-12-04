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
fn solve<const REMOVE: bool>(map: &mut String2D) -> usize {
    let add_offset = |a: usize, b, c| {
        a.checked_add_signed(b as isize)
            .and_then(|x| (x < c).then_some(x))
    };
    let mut total = 0;
    let rows = map.rows();
    let cols = map.cols();

    let mut queue = Vec::new();
    for y in 0..rows {
        for x in 0..cols {
            queue.push((x, y));
            while let Some((x, y)) = queue.pop() {
                if map.char(&(x, y)) != '@' {
                    continue;
                }
                let adj = utils_rust::DIR8.iter().filter_map(|offset| {
                    add_offset(x, offset.0, cols).zip(add_offset(y, offset.1, rows))
                });
                let adj = adj.filter(|pos| map.char(pos) == '@');
                let count = adj.clone().count();

                if count < 4 {
                    total += 1;
                    if REMOVE {
                        queue.extend(adj);
                        map.replace(&(x, y), b'.');
                    }
                }
            }
        }
    }
    total
}
