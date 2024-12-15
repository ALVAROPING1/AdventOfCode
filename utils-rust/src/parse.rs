use std::{fmt::Debug, str::FromStr};

#[must_use]
pub fn from_ascii_digit(input: u8) -> u32 {
    u32::from(input) - '0' as u32
}

pub fn value_list<T: FromStr>(vals: &str) -> impl Iterator<Item = T> + '_
where
    <T as FromStr>::Err: Debug,
{
    vals.split_whitespace().map(|x| {
        x.parse().expect(concat!(
            "Should only try to parse values of type `",
            stringify!(T),
            "`"
        ))
    })
}

pub fn value_list_comma<T: FromStr>(vals: &str) -> impl Iterator<Item = T> + '_
where
    <T as FromStr>::Err: Debug,
{
    vals.split(',').map(|x| {
        x.parse().expect(concat!(
            "Should only try to parse values of type `",
            stringify!(T),
            "`"
        ))
    })
}

macro_rules! impl_str2d {
    ($($i:ident)?) => {
        #[must_use]
        pub const fn pos(&self, index: usize) -> (usize, usize) {
            let y = index / self.cols;
            (index - y * self.cols, y)
        }

        #[must_use]
        pub const fn index(&self, pos: &(usize, usize)) -> usize {
            pos.0 + self.cols * pos.1
        }

        #[must_use]
        pub $($i)? fn as_str(&self) -> &str {
            &self.buffer
        }

        #[must_use]
        $($i)? fn char_idx(&self, index: usize) -> char {
            self.buffer.as_bytes()[index] as char
        }

        #[must_use]
        pub $($i)? fn char(&self, pos: &(usize, usize)) -> char {
            self.char_idx(self.index(pos))
        }

        #[must_use]
        pub const fn cols(&self) -> usize {
            self.cols - 1
        }

        #[must_use]
        pub const fn rows(&self) -> usize {
            self.rows
        }

        #[must_use]
        pub fn find(&self, c: char) -> Option<(usize, usize)> {
            self.buffer.find(c).map(|idx| self.pos(idx))
        }

        pub fn print(&self) {
            println!("{}", self.buffer)
        }
    };
}

pub struct Str2D<'a> {
    buffer: &'a str,
    cols: usize,
    rows: usize,
}

impl<'a> Str2D<'a> {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(input: &'a str) -> Self {
        let cols = input.find('\n').expect("There should be at least a 1 line") + 1;
        let rows = input.len() / cols;
        Self {
            buffer: input,
            cols,
            rows,
        }
    }

    impl_str2d!(const);
}

pub struct String2D {
    buffer: String,
    cols: usize,
    rows: usize,
}

impl String2D {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(input: &str) -> Self {
        let input = input.to_owned();
        Self::from_string(input)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn from_string(input: String) -> Self {
        let cols = input.find('\n').expect("There should be at least a 1 line") + 1;
        let rows = input.len() / cols;
        Self {
            buffer: input,
            cols,
            rows,
        }
    }

    pub fn replace(&mut self, pos: &(usize, usize), c: u8) {
        let idx = self.index(pos);
        unsafe {
            self.buffer.as_bytes_mut()[idx] = c;
        }
    }

    impl_str2d!();
}
