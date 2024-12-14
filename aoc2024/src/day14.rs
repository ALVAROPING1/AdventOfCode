use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let mut input = parse_input(input);
    Ok(Solution::default()
        .part1(part1(&input))
        .part2(part2(&mut input)))
}

static MAP_SIZE: (i32, i32) = (101, 103);

struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

fn parse_int(input: &str) -> i32 {
    input.parse().expect("This should only parse numbers")
}

fn parse_value(input: &str) -> (i32, i32) {
    let (x, y) = input[2..]
        .split_once(',')
        .expect("There should always be 2 values");
    (parse_int(x), parse_int(y))
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|robot| {
            let (p, v) = robot
                .split_once(' ')
                .expect("There should always be 2 values");
            let p = parse_value(p);
            let v = parse_value(v);
            Robot { p, v }
        })
        .collect()
}

const fn module(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[must_use]
fn part1(input: &[Robot]) -> i64 {
    let mut totals = [[0, 0], [0, 0]];
    for robot in input {
        let x = module(robot.p.0 + 100 * robot.v.0, MAP_SIZE.0);
        let y = module(robot.p.1 + 100 * robot.v.1, MAP_SIZE.1);
        if x == MAP_SIZE.0 / 2 || y == MAP_SIZE.1 / 2 {
            continue;
        }
        totals[usize::from(x < MAP_SIZE.0 / 2)][usize::from(y < MAP_SIZE.1 / 2)] += 1;
    }
    totals.iter().flatten().product()
}

fn print(tiles: &[bool]) {
    let mut idx = 0;
    for _ in 0..MAP_SIZE.1 {
        for _ in 0..MAP_SIZE.0 {
            print!("{}", if tiles[idx] { '*' } else { ' ' });
            idx += 1;
        }
        println!();
    }
}

#[must_use]
fn part2(input: &mut [Robot]) -> u32 {
    let size = (MAP_SIZE.0 * MAP_SIZE.1)
        .try_into()
        .expect("The value should fit");
    let mut img = vec![false; size];
    let mut seconds = 0;
    let group = 10;
    loop {
        for robot in &mut *input {
            robot.p.0 = module(robot.p.0 + robot.v.0, MAP_SIZE.0);
            robot.p.1 = module(robot.p.1 + robot.v.1, MAP_SIZE.1);
            let idx: usize = (robot.p.0 + robot.p.1 * MAP_SIZE.0)
                .try_into()
                .expect("The value should fit");
            img[idx] = true;
        }
        seconds += 1;
        if (group..img.len()).any(|i| img[i - group..i].iter().all(|x| *x)) {
            print(&img);
            break seconds;
        }
        img.fill(false);
    }
}
