use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)))
}

fn part1(input: &str) -> i32 {
    process_input(input)
}

// fn part2(input: &str) -> i32 {
//     process_input(input, |line| {
//         process_history(
//             line,
//             |history, i| if i % 2 == 0 { -1 } else { 1 } * history[0],
//         )
//     })
// }

fn process_input(input: &str) -> i32 {
    static ADJ: [(i8, i8, [char; 3]); 4] = [
        (-1, 0, ['F', 'L', '-']),
        (1, 0, ['7', 'J', '-']),
        (0, -1, ['F', '7', '|']),
        (0, 1, ['L', 'J', '|']),
    ];
    let map = Str2D::new(input);
    let start = map.pos(
        input
            .find('S')
            .expect("There should be a starting position"),
    );
    let mut steps = 1;
    let mut prev = [start; 2];
    let mut positions: [(usize, usize); 2] =
        utils_rust::collect_array(ADJ.into_iter().filter_map(|(x, y, chars)| {
            let pos = (
                start.0.checked_add_signed(x.into())?,
                start.1.checked_add_signed(y.into())?,
            );
            chars.iter().find(|&&x| x == map.char(&pos))?;
            Some(pos)
        }));
    while positions[0] != positions[1] {
        for (prev, current) in prev.iter_mut().zip(positions.iter_mut()) {
            (*prev, *current) = (*current, get_next(&map, current, prev));
        }
        steps += 1;
    }
    steps
}

fn get_next(map: &Str2D, pos: &(usize, usize), prev_pos: &(usize, usize)) -> (usize, usize) {
    let possible: [(i8, i8); 2] = match map.char(pos) {
        '|' => [(0, -1), (0, 1)],
        '-' => [(-1, 0), (1, 0)],
        'L' => [(0, -1), (1, 0)],
        'J' => [(0, -1), (-1, 0)],
        '7' => [(-1, 0), (0, 1)],
        'F' => [(1, 0), (0, 1)],
        _ => unreachable!(),
    };
    possible
        .into_iter()
        .filter_map(|(x, y)| {
            Some((
                pos.0.checked_add_signed(x as isize)?,
                pos.1.checked_add_signed(y as isize)?,
            ))
        })
        .find(|pos| pos != prev_pos)
        .expect("A new position should be found")
}

struct Str2D<'a>(&'a str, usize);

impl<'a> Str2D<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(
            input,
            input.find('\n').expect("There should be at least a 1 line") + 1,
        )
    }

    pub const fn pos(&self, index: usize) -> (usize, usize) {
        let y = index / self.1;
        (index - y * self.1, y)
    }

    pub const fn index(&self, pos: &(usize, usize)) -> usize {
        pos.0 + self.1 * pos.1
    }

    pub const fn char(&self, pos: &(usize, usize)) -> char {
        self.char_idx(self.index(pos))
    }

    pub const fn char_idx(&self, index: usize) -> char {
        self.0.as_bytes()[index] as char
    }
}
