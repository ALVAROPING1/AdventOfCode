use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let distances = [[1; 11]; 11];
    let distances = calculate_distances(&KEYS_MOVEMENT, &MOVEMENTS_DIR, &MOVEMENT_ID, &distances);
    let distances = calculate_distances(&KEYS_MOVEMENT, &MOVEMENTS_DIR, &MOVEMENT_ID, &distances);
    Ok(Solution::default()
        .part1(part1(&distances, input))
        .part2(part2(distances, input)))
}

static MOVEMENT_ID: [usize; 128] = {
    let mut id = [0; 128];
    id[b'A' as usize] = 0;
    id[b'^' as usize] = 1;
    id[b'<' as usize] = 2;
    id[b'v' as usize] = 3;
    id[b'>' as usize] = 4;
    id
};
static NUM_ID: [usize; 128] = {
    let mut id = [0; 128];
    id[b'A' as usize] = 0;
    id[b'0' as usize] = 1;
    id[b'1' as usize] = 2;
    id[b'2' as usize] = 3;
    id[b'3' as usize] = 4;
    id[b'4' as usize] = 5;
    id[b'5' as usize] = 6;
    id[b'6' as usize] = 7;
    id[b'7' as usize] = 8;
    id[b'8' as usize] = 9;
    id[b'9' as usize] = 10;
    id
};

static KEYS_MOVEMENT: [u8; 5] = [b'A', b'^', b'<', b'v', b'>'];
static KEYS_NUMBERS: [u8; 11] = [
    b'A', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
];

#[rustfmt::skip]
static MOVEMENTS_DIR: [[&[u8]; 11]; 5] = [
    [b"",    b"<",  b"v<<", b"<v", b"v",  b"", b"", b"", b"", b"", b""],
    [b">",   b"",   b"v<",  b"v",  b"v>", b"", b"", b"", b"", b"", b""],
    [b">>^", b">^", b"",    b">",  b">>", b"", b"", b"", b"", b"", b""],
    [b"^>",  b"^",  b"<",   b"",   b">",  b"", b"", b"", b"", b"", b""],
    [b"^",   b"<^", b"<<",  b"<",  b"",   b"", b"", b"", b"", b"", b""],
];

#[rustfmt::skip]
static MOVEMENTS_NUM: [[&[u8]; 11]; 11] = [
    //       A         0        1        2       3        4        5       6       7         8       9
    /*A*/ [b"",      b"<",    b"^<<",  b"<^",  b"^",    b"^^<<", b"<^^", b"^^",  b"^^^<<", b"<^^^", b"^^^"],
    /*0*/ [b">",     b"",     b"^<",   b"^",   b"^>",   b"^^<",  b"^^",  b"^^>", b"^^^<",  b"^^^",  b"^^^>"],
    /*1*/ [b">>v",   b">v",   b"",     b">",   b">>",   b"^",    b"^>",  b"^>>", b"^^",    b"^^>",  b"^^>>"],
    /*2*/ [b"v>",    b"v",    b"<",    b"",    b">",    b"<^",   b"^",   b"^>",  b"<^^",   b"^^",   b"^^>"],
    /*3*/ [b"v",     b"<v",   b"<<",   b"<",   b"",     b"<<^",  b"<^",  b"^",   b"<<^^",  b"<^^",  b"^^"],
    /*4*/ [b">>vv",  b">vv",  b"v",    b"v>",  b"v>>",  b"",     b">",   b">>",  b"^",     b"^>",   b"^>>"],
    /*5*/ [b"vv>",   b"vv",   b"<v",   b"v",   b"v>",   b"<",    b"",    b">",   b"<^",    b"^",    b"^>"],
    /*6*/ [b"vv",    b"<vv",  b"<<v",  b"<v",  b"v",    b"<<",   b"<",   b"",    b"<<^",   b"<^",   b"^"],
    /*7*/ [b">>vvv", b">vvv", b"vv",   b"vv>", b"vv>>", b"v",    b"v>",  b"v>>", b"",      b">",    b">>"],
    /*8*/ [b"vvv>",  b"vvv",  b"<vv",  b"vv",  b"vv>",  b"<v",   b"v",   b"v>",  b"<",     b"",     b">"],
    /*9*/ [b"vvv",   b"<vvv", b"<<vv", b"<vv", b"vv",   b"<<v",  b"<v",  b"v",   b"<<",    b"<",    b""],
];

fn calculate_distances(
    keys: &[u8],
    movements: &[[&[u8]; 11]],
    key_id: &[usize],
    current: &[[u64; 11]],
) -> [[u64; 11]; 11] {
    let mut next = [[0u64; 11]; 11];
    for position in keys {
        let position = key_id[*position as usize];
        next[position][position] = 1;
        for target in keys {
            let target = key_id[*target as usize];
            if target == position {
                continue;
            }
            let mut cost = 0;
            let mut pos = MOVEMENT_ID[b'A' as usize];
            for movement in movements[position][target] {
                let movement = MOVEMENT_ID[*movement as usize];
                cost += current[pos][movement];
                pos = movement;
            }
            cost += current[pos][MOVEMENT_ID[b'A' as usize]];
            next[position][target] = cost;
        }
    }
    // for row in current {
    //     for v in row {
    //         print!("{v:02},");
    //     }
    //     println!();
    // }
    next
}

fn solve_input(distances: &[[u64; 11]], keys: &[u8]) -> u64 {
    let mut cost = 0;
    let mut pos = NUM_ID[b'A' as usize];
    for key in keys {
        let id = NUM_ID[*key as usize];
        cost += distances[pos][id];
        pos = id;
    }
    cost
}

fn solve(distances: &[[u64; 11]], codes: &str) -> u64 {
    let distances = calculate_distances(&KEYS_NUMBERS, &MOVEMENTS_NUM, &NUM_ID, distances);
    codes
        .lines()
        .map(|code| {
            let cost = solve_input(&distances, code.as_bytes());
            let num: u64 = code[..3]
                .parse()
                .expect("The first 3 characters should always form a number");
            cost * num
        })
        .sum()
}

#[must_use]
fn part1(distances: &[[u64; 11]], codes: &str) -> u64 {
    solve(distances, codes)
}

#[must_use]
fn part2(mut distances: [[u64; 11]; 11], codes: &str) -> u64 {
    for _ in 0..(25 - 2) {
        distances = calculate_distances(&KEYS_MOVEMENT, &MOVEMENTS_DIR, &MOVEMENT_ID, &distances);
    }
    solve(&distances, codes)
}
