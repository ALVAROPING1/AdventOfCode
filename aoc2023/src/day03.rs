use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default().part1(part1(input)).part2(part2(input)))
}

fn part1(input: &str) -> u32 {
    let engine = Str2D::new(input);
    let (mut total, mut current) = (0, 0);
    let mut add = false;
    for (i, char) in engine.0.bytes().enumerate() {
        if !char.is_ascii_digit() {
            if add {
                total += current;
            }
            current = 0;
            add = false;
            continue;
        }
        current = current * 10 + utils_rust::parse::from_ascii_digit(char);
        add = add || engine.adj_symbol(i);
    }
    total
}

fn part2(input: &str) -> u32 {
    let engine = Str2D::new(input);
    engine
        .0
        .bytes()
        .enumerate()
        .filter_map(|(i, char)| {
            if char != b'*' {
                return None;
            }
            engine.get_gear(i)
        })
        .sum()
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

    pub fn char(&self, pos: &(usize, usize)) -> Option<char> {
        self.char_idx(self.index(pos))
    }

    pub fn char_idx(&self, index: usize) -> Option<char> {
        self.0.as_bytes().get(index).map(|&x| x as char)
    }

    #[rustfmt::skip]
    pub fn adj(&self, index: usize) -> impl Iterator<Item = (usize, usize)> {
        self.offset(index, [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ].into_iter())
    }

    pub fn offset(
        &self,
        index: usize,
        offsets: impl Iterator<Item = (i8, i8)>,
    ) -> impl Iterator<Item = (usize, usize)> {
        Self::offset_pos(self.pos(index), offsets)
    }

    pub fn offset_pos(
        index: (usize, usize),
        offsets: impl Iterator<Item = (i8, i8)>,
    ) -> impl Iterator<Item = (usize, usize)> {
        offsets.filter_map(move |(x, y)| {
            Some((
                index.0.checked_add_signed(x as isize)?,
                index.1.checked_add_signed(y as isize)?,
            ))
        })
    }

    pub fn adj_symbol(&self, index: usize) -> bool {
        self.adj(index).any(|pos| {
            let character = self.char(&pos);
            character.is_some_and(|c| !c.is_ascii_digit() && c != '\n' && c != '.')
        })
    }

    pub fn is_digit(&self, pos: &(usize, usize)) -> bool {
        self.char(pos).is_some_and(|c| c.is_ascii_digit())
    }

    fn get_number(&self, pos: &(usize, usize)) -> u32 {
        let mut i = self.index(pos);
        while i > 0
            && self
                .char_idx(i - 1)
                .expect("The index should always be in range")
                .is_ascii_digit()
        {
            i -= 1;
        }
        let mut current = 0;
        for char in &self.0.as_bytes()[i..] {
            if !char.is_ascii_digit() {
                break;
            }
            current = current * 10 + utils_rust::parse::from_ascii_digit(*char);
        }
        current
    }

    fn get_gear(&self, index: usize) -> Option<u32> {
        let mut res: (u8, [(usize, usize); 2]) = (0, [(0, 0); 2]);
        for pos in self.offset(index, [(-1, 0), (1, 0)].into_iter()) {
            if self.is_digit(&pos) {
                if res.0 == 2 {
                    return None;
                }
                res.1[res.0 as usize] = pos;
                res.0 += 1;
            }
        }
        for pos in self.offset(index, [(0, -1), (0, 1)].into_iter()) {
            if self.is_digit(&pos) {
                if res.0 == 2 {
                    return None;
                }
                res.1[res.0 as usize] = pos;
                res.0 += 1;
            } else {
                for pos in Self::offset_pos(pos, [(-1, 0), (1, 0)].into_iter()) {
                    if self.is_digit(&pos) {
                        if res.0 == 2 {
                            return None;
                        }
                        res.1[res.0 as usize] = pos;
                        res.0 += 1;
                    }
                }
            }
        }
        if res.0 != 2 {
            return None;
        }
        Some(self.get_number(&res.1[0]) * self.get_number(&res.1[1]))
    }
}
