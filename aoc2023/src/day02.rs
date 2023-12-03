use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u32 {
    process_lines(input, |(game_id, cubes)| {
        static MAX: [u32; 3] = [12, 13, 14];
        cubes
            .split([';', ','])
            .map(parse_cube)
            .all(|(n, color)| n <= MAX[color])
            .then_some(game_id.parse().expect("The game ID should be an integer"))
    })
}

fn part2(input: &str) -> u32 {
    process_lines(input, |(_, cubes)| {
        let mut minimum = [0; 3];
        for (n, color) in cubes.split([';', ',']).map(parse_cube) {
            minimum[color] = u32::max(minimum[color], n);
        }
        Some(minimum.iter().product())
    })
}

fn process_lines(input: &str, fun: impl Fn((&str, &str)) -> Option<u32>) -> u32 {
    input.lines().map(parse_line).filter_map(fun).sum()
}

fn parse_line(input: &str) -> (&str, &str) {
    let mut line = input[5..].split(": ");
    let game_id = line.next().expect("There should be a game ID");
    let cubes = line
        .next()
        .expect("The game should have at least 1 set of cubes");
    (game_id, cubes)
}

fn parse_cube(input: &str) -> (u32, usize) {
    let mut cubes = input.trim_start().split(' ');
    let n = cubes
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
