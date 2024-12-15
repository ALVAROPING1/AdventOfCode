use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::String2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (mut map1, mut map2, steps, start) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&mut map1, steps, start))
        .part2(part2(&mut map2, steps, Vec2D(start.0 * 2, start.1))))
}

fn parse_input(input: &str) -> (String2D, String2D, &str, Vec2D) {
    let (map, steps) = input
        .split_once("\n\n")
        .expect("There should be 2 sections of data");
    let mut map1 = String2D::new(map);
    let start = map1
        .find('@')
        .expect("There should be a single starting position");
    map1.replace(&start, b'.');
    let mut map2 = String::with_capacity(input.len() * 2);
    for c in map.as_bytes() {
        if *c == b'\n' {
            map2.push('\n');
        } else {
            let mut push = |c1, c2| {
                map2.push(c1);
                map2.push(c2);
            };
            match *c {
                b'.' | b'@' => push('.', '.'),
                b'#' => push('#', '#'),
                b'O' => push('[', ']'),
                _ => unreachable!(),
            }
        }
    }
    (
        map1,
        String2D::from_string(map2),
        steps,
        Vec2D(start.0, start.1),
    )
}

fn score(map: &String2D, obj: char) -> usize {
    let mut total = 0;
    for y in 0..map.rows() {
        for x in 0..map.cols() {
            if map.char(&(x, y)) == obj {
                total += 100 * y + x;
            }
        }
    }
    total
}

fn get_steps(x: &str) -> impl Iterator<Item = char> + '_ {
    x.chars().filter(|c| *c != '\n')
}

fn get_offset(c: char) -> (isize, isize) {
    match c {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => unreachable!(),
    }
}

#[must_use]
fn part1(map: &mut String2D, steps: &str, mut pos: Vec2D) -> usize {
    for step in get_steps(steps) {
        let offset = get_offset(step);
        let next_pos = pos + offset;
        let mut final_pos = next_pos;
        let mut c = map.char(&final_pos.as_tuple());
        while c == 'O' {
            final_pos += offset;
            c = map.char(&final_pos.as_tuple());
        }
        if c == '.' {
            if next_pos != final_pos {
                map.replace(&next_pos.as_tuple(), b'.');
                map.replace(&final_pos.as_tuple(), b'O');
            }
            pos = next_pos;
        }
    }
    score(map, 'O')
}

#[must_use]
fn part2(map: &mut String2D, steps: &str, mut pos: Vec2D) -> usize {
    fn check_vertical(map: &String2D, pos: Vec2D, offset: isize) -> bool {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        match map.char(&pos.as_tuple()) {
            '.' => true,
            '#' => false,
            c => {
                let y = (pos.1 as isize + offset) as usize;
                let pos1 = Vec2D(pos.0, y);
                let pos2 = (pos.0 as isize + if c == '[' { 1 } else { -1 }) as usize;
                let pos2 = Vec2D(pos2, y);
                check_vertical(map, pos1, offset) && check_vertical(map, pos2, offset)
            }
        }
    }

    fn move_vertical(map: &mut String2D, pos: Vec2D, offset: isize) {
        let swap = |map: &mut String2D, prev_pos: Vec2D, pos: Vec2D| {
            let c = map.char(&prev_pos.as_tuple());
            map.replace(&pos.as_tuple(), c as u8);
            map.replace(&prev_pos.as_tuple(), b'.');
        };
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        match map.char(&pos.as_tuple()) {
            '.' => {}
            c => {
                let y = (pos.1 as isize + offset) as usize;
                let x = (pos.0 as isize + if c == '[' { 1 } else { -1 }) as usize;
                let pos1 = Vec2D(pos.0, y);
                let pos2 = Vec2D(x, y);
                let pos3 = Vec2D(x, pos.1);
                move_vertical(map, pos1, offset);
                move_vertical(map, pos2, offset);
                swap(map, pos, pos1);
                swap(map, pos3, pos2);
            }
        }
    }

    fn move_horizontal(map: &mut String2D, pos: Vec2D, offset: (isize, isize)) -> Vec2D {
        let next_pos = pos + offset;
        let mut final_pos = next_pos;
        let mut c = map.char(&final_pos.as_tuple());
        while c == '[' || c == ']' {
            final_pos += offset;
            c = map.char(&final_pos.as_tuple());
        }
        if c == '.' {
            while final_pos != next_pos {
                let prev = final_pos - offset;
                let c = map.char(&prev.as_tuple());
                map.replace(&final_pos.as_tuple(), c as u8);
                final_pos = prev;
            }
            map.replace(&next_pos.as_tuple(), b'.');
            next_pos
        } else {
            pos
        }
    }

    for step in get_steps(steps) {
        let offset = get_offset(step);
        if offset.1 == 0 {
            pos = move_horizontal(map, pos, offset);
        } else {
            let next_pos = pos + offset;
            if check_vertical(map, next_pos, offset.1) {
                move_vertical(map, next_pos, offset.1);
                pos = next_pos;
            }
        }
    }
    score(map, '[')
}
