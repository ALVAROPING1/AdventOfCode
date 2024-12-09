use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input.as_bytes())))
    // .part2(part2(&input)))
}

// TODO: refactor
#[must_use]
fn part1(input: &[u8]) -> usize {
    let mut total = 0;
    let mut start = 1;
    let mut id_start = 1;
    let mut id_end = input.len() / 2 - 1;
    let mut end = input.len() - 1;
    let range_sum = |a, b| ((b - a) * (a + b - 1)) / 2;
    if end % 2 == 1 {
        end -= 1;
    }
    let get = |i| usize::from(input[i] - b'0');
    let mut space = get(start);
    let mut data = get(end);
    let mut pos = get(0);
    while start < end {
        if space >= data {
            total += id_end * range_sum(pos, pos + data);
            space -= data;
            pos += data;
            end -= 2;
            data = get(end);
            id_end -= 1;
            if space == 0 {
                start += 2;
                if start > end {
                    break;
                }
                space = get(start);
                let mid = get(start - 1);
                total += id_start * range_sum(pos, pos + mid);
                pos += mid;
                id_start += 1;
            }
        } else {
            total += id_end * range_sum(pos, pos + space);
            data -= space;
            pos += space;
            start += 2;
            if start > end {
                if data != 0 {
                    total += id_end * range_sum(pos, pos + data);
                }
                break;
            }
            space = get(start);
            let mid = get(start - 1);
            total += id_start * range_sum(pos, pos + mid);
            pos += mid;
            id_start += 1;
        }
    }
    total
}

#[must_use]
fn part2(input: &str) -> u32 {
    todo!()
}
