use std::error::Error;

use crate::prelude::*;
use utils_rust::parse::Str2D;
use utils_rust::Vec2D;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (map, (distance, path)) = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&map, &distance, &path))
        .part2(part2(&map, &distance, &path)))
}

static STEPS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_input(input: &str) -> (Str2D, (Vec<u32>, Vec<Vec2D>)) {
    let map = Str2D::new(input);
    let start: Vec2D = map
        .find('S')
        .expect("There should be a start position")
        .into();
    let end: Vec2D = map
        .find('E')
        .expect("There should be an end position")
        .into();
    let path = calculate_path(&map, start, end);
    (map, path)
}

fn calculate_path(map: &Str2D, mut pos: Vec2D, end: Vec2D) -> (Vec<u32>, Vec<Vec2D>) {
    let mut distance = vec![0; map.cols() * map.rows()];
    let mut path = Vec::new();
    let mut parent = pos;
    let mut current = 0;
    while pos != end {
        path.push(pos);
        distance[pos.idx(map.cols())] = current;
        current += 1;
        for step in STEPS {
            let next_pos = pos + step;
            if map.char(&next_pos.as_tuple()) != '#' && next_pos != parent {
                parent = pos;
                pos = next_pos;
                break;
            }
        }
    }
    distance[pos.idx(map.cols())] = current;
    path.push(pos);
    // for y in 0..map.rows() {
    //     for x in 0..map.cols() {
    //         let pos = Vec2D(x, y);
    //         if let Some(d) = path[pos.idx(map.cols())] {
    //             print!("{d:02},");
    //         } else {
    //             print!("##,");
    //         }
    //     }
    //     println!();
    // }
    (distance, path)
}

#[must_use]
fn part1(map: &Str2D, distance: &[u32], path: &[Vec2D]) -> u32 {
    let idx = |pos: Vec2D| pos.idx(map.cols());
    let mut amount = 0;
    for pos in &path[..path.len() - 1] {
        let d1 = distance[idx(*pos)];
        for step in STEPS {
            let next_pos = *pos + step;
            if map.char(&next_pos.as_tuple()) == '#' {
                for step in STEPS {
                    let final_pos = next_pos + step;
                    if final_pos.0 >= map.cols() || final_pos.1 >= map.rows() {
                        continue;
                    }
                    if map.char(&final_pos.as_tuple()) != '#' {
                        let d2 = distance[idx(final_pos)];
                        #[allow(clippy::cast_possible_wrap)]
                        let saved = d2 as i32 - d1 as i32 - 2;
                        amount += u32::from(saved >= 100);
                    }
                }
            }
        }
    }
    amount
}

#[must_use]
fn part2(map: &Str2D, distance: &[u32], path: &[Vec2D]) -> u32 {
    let idx = |pos: Vec2D| pos.idx(map.cols());
    let mut amount = 0;
    for pos in &path[..path.len() - 1] {
        let d1 = distance[idx(*pos)];
        for diff_y in -20..=20isize {
            for diff_x in -20..=20isize {
                let diff = diff_x.abs() + diff_y.abs();
                if diff > 20 {
                    continue;
                }
                let final_pos = *pos + (diff_x, diff_y);
                if final_pos.0 >= map.cols() || final_pos.1 >= map.rows() {
                    continue;
                }
                let d2 = distance[idx(final_pos)];
                #[allow(clippy::cast_possible_wrap)]
                let saved = d2 as isize - d1 as isize - diff;
                amount += u32::from(saved >= 100);
            }
        }
    }
    amount
}
