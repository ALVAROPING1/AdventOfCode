#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetVec(pub i8, pub i8);

pub const DIR8: [OffsetVec; 8] = [
    OffsetVec(-1, -1),
    OffsetVec(0, -1),
    OffsetVec(1, -1),
    OffsetVec(-1, 0),
    OffsetVec(1, 0),
    OffsetVec(-1, 1),
    OffsetVec(0, 1),
    OffsetVec(1, 1),
];

impl Vec2D {
    #[must_use]
    pub const fn as_tuple(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    #[must_use]
    pub const fn idx(&self, cols: usize) -> usize {
        self.1 * cols + self.0
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

impl Sub<Self> for Vec2D {
    type Output = (isize, isize);
    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        (
            (self.0 as isize) - (rhs.0 as isize),
            (self.1 as isize) - (rhs.1 as isize),
        )
    }
}

impl From<(usize, usize)> for Vec2D {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl std::fmt::Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn input(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(path)?)
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
                let input = $crate::input($crate::input_path!($current))?;
                println!(concat!("Running ", stringify!($current)));
                Ok(super::$current::run(&input).map_err(|_| concat!("Error running day ", stringify!($current)))?)
            }
        }
    }
}

#[macro_export]
macro_rules! input_path {
    ($day:ident) => {
        concat!("./inputs/", stringify!($day), ".txt")
    };
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

#[macro_export]
macro_rules! bench {
    ( ($lib:ident) $( $pre:ident ,)* ) => {
        $crate::bench!();

        pub fn benchmark_crate(c: &mut Criterion) {
            $( $crate::bench!($lib, $pre, c); )*
        }
    };
    ( ($lib:ident) $( $pre:ident ,)*  > $current:ident, $( $mid:ident ,)*  > $current2:ident, $( $post:ident ,)* ) => {
        compile_error!("Multiple days can't be prefixed with `> `")
    };
    ( ($lib:ident) $( $pre:ident ,)* > $current:ident, $( $post:ident ,)* ) => {
        $crate::bench!();

        $( pub use $lib::$pre; )*
        $( pub use $lib::$post; )*

        pub fn benchmark_crate(c: &mut Criterion) {
            $crate::bench!($lib, $current, c);
        }
    };
    ($lib:ident, $day:ident, $c:ident) => {
        let input = $crate::input($crate::input_path!($day)).unwrap();
        $c.bench_function(stringify!($day), |b| b.iter(|| $lib::$day::run(&input)));
    };
    () => {
        use criterion::{criterion_group, criterion_main, Criterion};

        criterion_group! {
            name = benches;
            config = Criterion::default().measurement_time(std::time::Duration::from_secs(10));
            targets = benchmark_crate
        }
        criterion_main!(benches);
    }
}
