use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u32 {
    process_lines(input, possible_game)
}

fn part2(input: &str) -> u32 {
    process_lines(input, |line| Some(game_value(line)))
}

fn process_lines(input: &str, fun: impl Fn(&str) -> Option<u32>) -> u32 {
    input
        .lines()
        .filter_map(fun)
        .reduce(|acc, x| acc + x)
        .expect("There should be a result")
}

fn parse_line(input: &str) -> (&str, impl Iterator<Item = &str>) {
    let mut line = input[5..].split(": ");
    let game_id = line.next().expect("There should be a game ID");
    let cubes = line
        .next()
        .expect("The game should have at least 1 set of cubes")
        .split([';', ',']);
    (game_id, cubes)
}

fn parse_cube(input: &str) -> (u32, usize) {
    let mut cubes = input.trim_start().split(' ');
    let n: u32 = cubes
        .next()
        .expect("There should be an amount of cubes")
        .parse()
        .expect("The amount of cubes should be an integer");
    let color = match cubes
        .next()
        .expect("There should be a cube color")
        .as_bytes()[0]
    {
        b'r' => 0,
        b'g' => 1,
        b'b' => 2,
        x => panic!("Unknown cube type: {x}"),
    };
    (n, color)
}

const MAX: [u32; 3] = [12, 13, 14];

fn possible_game(input: &str) -> Option<u32> {
    let (game_id, mut cubes) = parse_line(input);
    cubes
        .all(|cube| {
            let (n, color) = parse_cube(cube);
            n <= MAX[color]
        })
        .then(|| game_id.parse().expect("The game ID should be an integer"))
}

fn game_value(input: &str) -> u32 {
    let (_, cubes) = parse_line(input);
    let mut minimum = [0; 3];
    for cube in cubes {
        let (n, color) = parse_cube(cube);
        minimum[color] = u32::max(minimum[color], n);
    }
    minimum[0] * minimum[1] * minimum[2]
}
