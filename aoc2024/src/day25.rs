use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::Str2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (locks, keys) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&locks, &keys))
        .part2("Nothing to solve".to_string()))
}

type Schematic = [u8; 5];

fn parse_schematic(input: &str) -> (Schematic, bool) {
    let schematic = Str2D::new(input);
    let mut heights = [0; 5];
    let c = schematic.char(&(0, 0));
    for (x, h) in heights.iter_mut().enumerate() {
        let mut y = 1u8;
        while y < 6 {
            if schematic.char(&(x, y as usize)) != c {
                break;
            }
            y += 1;
        }
        *h = y;
    }
    (heights, c == '#')
}

fn parse_input(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in input.split("\n\n") {
        let (schematic, is_lock) = parse_schematic(schematic);
        if is_lock {
            locks.push(schematic);
        } else {
            keys.push(schematic);
        }
    }
    (locks, keys)
}

#[must_use]
fn part1(locks: &[Schematic], keys: &[Schematic]) -> u32 {
    let mut total = 0;
    for lock in locks {
        for key in keys {
            total += u32::from(std::iter::zip(lock, key).all(|(lock_h, key_h)| lock_h <= key_h));
        }
    }
    total
}
