use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::String2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut map, start) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&map, start))
        .part2(part2(&mut map, start)))
}

fn parse_input(input: &str) -> (String2D, Vec2D) {
    let map = String2D::new(input);
    let index = map
        .as_str()
        .find('^')
        .expect("There should be a single `^` character");
    let (x, y) = map.pos(index);
    (map, Vec2D(x, y))
}

static STEPS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

const fn out_of_bounds(map: &String2D, pos: Vec2D) -> bool {
    pos.0 >= map.cols() || pos.1 >= map.rows()
}

#[must_use]
fn part1(map: &String2D, mut pos: Vec2D) -> u16 {
    let mut step = 0;
    let mut visited = vec![true; map.cols() * map.rows()];
    let mut total = 0;
    loop {
        let idx = pos.1 * map.cols() + pos.0;
        total += u16::from(visited[idx]);
        visited[idx] = false;
        let mut new_pos = pos + STEPS[step];
        if out_of_bounds(map, new_pos) {
            break;
        }
        if map.char(&new_pos.as_tuple()) == '#' {
            step = (step + 1) % STEPS.len();
            new_pos = pos + STEPS[step];
            if out_of_bounds(map, new_pos) {
                break;
            }
        }
        pos = new_pos;
    }
    total
}

fn check_in_loop(map: &String2D, mut pos: Vec2D, mut step: usize, mut visited: Vec<u8>) -> bool {
    loop {
        let idx = pos.1 * map.cols() + pos.0;
        let bit_mask = 1 << step;
        visited[idx] |= bit_mask;
        let new_pos = pos + STEPS[step];
        if out_of_bounds(map, new_pos) {
            break;
        }
        if map.char(&new_pos.as_tuple()) == '#' {
            step = (step + 1) % STEPS.len();
        } else {
            pos = new_pos;
        }
        let idx = pos.1 * map.cols() + pos.0;
        let bit_mask = 1 << step;
        if (visited[idx] & bit_mask) != 0 {
            return true;
        }
    }
    false
}

#[must_use]
fn part2(map: &mut String2D, mut pos: Vec2D) -> u16 {
    let mut step = 0;
    let mut visited = vec![0; map.cols() * map.rows()];
    let mut total = 0;
    let cols = map.cols();
    let get_idx = |pos: Vec2D| pos.1 * cols + pos.0;
    let next = |step: usize| (step + 1) % STEPS.len();
    loop {
        let mut new_pos = pos + STEPS[step];
        if out_of_bounds(map, new_pos) {
            break;
        }
        if map.char(&new_pos.as_tuple()) == '#' {
            step = next(step);
            new_pos = pos + STEPS[step];
            if out_of_bounds(map, new_pos) {
                break;
            }
        }
        let idx = get_idx(new_pos);
        if visited[idx] == 0 {
            map.replace(&new_pos.as_tuple(), b'#');
            if check_in_loop(map, pos, next(step), visited.clone()) {
                total += 1;
            }
            map.replace(&new_pos.as_tuple(), b'.');
        }
        visited[get_idx(pos)] |= 1 << step;
        pos = new_pos;
    }
    total
}
