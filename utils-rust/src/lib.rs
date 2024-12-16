use std::ops::{Add, AddAssign, Sub};

mod solution;
pub use solution::Solution;

pub mod numbers;
pub mod parse;

pub fn collect_array<T: Default + Copy, const N: usize>(iter: impl Iterator<Item = T>) -> [T; N] {
    let mut out = [T::default(); N];
    for (i, val) in iter.take(N).enumerate() {
        out[i] = val;
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2D(pub usize, pub usize);

impl Vec2D {
    #[must_use]
    pub const fn as_tuple(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Add<(isize, isize)> for Vec2D {
    type Output = Self;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Self(
            ((self.0 as isize) + rhs.0) as usize,
            ((self.1 as isize) + rhs.1) as usize,
        )
    }
}

impl AddAssign<(isize, isize)> for Vec2D {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        *self = *self + rhs;
    }
}

impl Sub<(isize, isize)> for Vec2D {
    type Output = Self;
    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Self(
            ((self.0 as isize) - rhs.0) as usize,
            ((self.1 as isize) - rhs.1) as usize,
        )
    }
}

// Adapted from <https://github.com/MaxOhn/AdventOfCode/blob/main/2022/src/lib.rs>

#[macro_export]
macro_rules! days {
    ( $( $pre:ident ,)* ) => {
        compile_error!("One day must be prefixed with `> `")
    };
    ( $( $pre:ident ,)*  > $current:ident, $( $mid:ident ,)*  > $current2:ident, $( $post:ident ,)* ) => {
        compile_error!("Multiple days can't be prefixed with `> `")
    };
    ( $( $pre:ident ,)* > $current:ident, $( $post:ident ,)* ) => {
        mod prelude {
            pub use utils_rust::Solution;
        }

        $( pub mod $pre; )*
        pub mod $current;
        $( pub mod $post; )*

        pub mod current {
            use std::error::Error;
            use super::prelude::*;

            pub fn run() -> Result<Solution, Box<dyn Error>> {
                let path = concat!("./inputs/", stringify!($current), ".txt");
                let input = std::fs::read_to_string(path)?;

                Ok(super::$current::run(&input).map_err(|_| concat!("Error running day ", stringify!($current)))?)
            }
        }
    }
}

#[macro_export]
macro_rules! main {
    () => {
        fn main() {
            let start = std::time::Instant::now();

            match current::run() {
                Ok(solution) => {
                    let elapsed = start.elapsed();
                    print!("{solution}");
                    println!("Elapsed: {elapsed:?}");
                }
                Err(err) => eprintln!("{err}"),
            }
        }
    };
}
