use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let input = Str2D::new(input);
    Ok(Solution::default()
        .part1(part1(&input))
        .part2(part2(&input)))
}

#[must_use]
fn part1(input: &Str2D) -> u32 {
    static SEARCH: &[u8] = b"XMAS";

    fn check(input: &Str2D, search: &[u8], offset: impl Fn(usize) -> (usize, usize)) -> u32 {
        u32::from((0..search.len()).all(|i| input.char(&offset(i)) == search[i] as char))
            + u32::from(
                (0..search.len())
                    .all(|i| input.char(&offset(i)) == search[search.len() - i - 1] as char),
            )
    }

    let mut total = 0;
    for y in 0..input.rows() {
        for x in 0..input.cols() {
            if x <= input.cols() - SEARCH.len() {
                total += check(input, SEARCH, |i| (x + i, y));
            }
            if y <= input.rows() - SEARCH.len() {
                total += check(input, SEARCH, |i| (x, y + i));
            }
            if x <= input.cols() - SEARCH.len() && y <= input.rows() - SEARCH.len() {
                total += check(input, SEARCH, |i| (x + i, y + i));
            }
            if x >= SEARCH.len() - 1 && y <= input.rows() - SEARCH.len() {
                total += check(input, SEARCH, |i| (x - i, y + i));
            }
        }
    }
    total
}

#[must_use]
fn part2(input: &Str2D) -> u32 {
    let mut total = 0;
    let check_single = |a, b, pos_a, pos_b| input.char(&pos_a) == a && input.char(&pos_b) == b;
    let check_diagonal =
        |pos_a, pos_b| check_single('M', 'S', pos_a, pos_b) || check_single('S', 'M', pos_a, pos_b);
    for y in 1..input.rows() - 1 {
        for x in 1..input.cols() - 1 {
            total += u32::from(
                input.char(&(x, y)) == 'A'
                    && check_diagonal((x - 1, y - 1), (x + 1, y + 1))
                    && check_diagonal((x + 1, y - 1), (x - 1, y + 1)),
            );
        }
    }
    total
}

struct Str2D<'a> {
    buffer: &'a str,
    cols: usize,
    rows: usize,
}

impl<'a> Str2D<'a> {
    pub fn new(input: &'a str) -> Self {
        let cols = input.find('\n').expect("There should be at least a 1 line") + 1;
        let rows = input.len() / cols;
        Self {
            buffer: input,
            cols,
            rows,
        }
    }

    pub const fn index(&self, pos: &(usize, usize)) -> usize {
        pos.0 + self.cols * pos.1
    }

    pub const fn char(&self, pos: &(usize, usize)) -> char {
        self.char_idx(self.index(pos))
    }

    const fn char_idx(&self, index: usize) -> char {
        self.buffer.as_bytes()[index] as char
    }

    pub const fn cols(&self) -> usize {
        self.cols - 1
    }
    pub const fn rows(&self) -> usize {
        self.rows
    }
}
