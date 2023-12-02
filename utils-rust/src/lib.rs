mod solution;
pub use solution::Solution;

pub mod parse;

// Adapted from <https://github.com/MaxOhn/AdventOfCode/blob/main/2022/src/lib.rs>

#[macro_export]
macro_rules! days {
    ( $( $pre:ident ,)* ) => {
        compile_error!("One day must be prefixed with `> `")
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

            match aoc2023::current::run() {
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
