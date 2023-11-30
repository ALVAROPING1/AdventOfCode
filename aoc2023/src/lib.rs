// Adapted from <https://github.com/MaxOhn/AdventOfCode/blob/main/2022/src/lib.rs>

modules! {
    > day01,
}

mod prelude {
    pub use utils_rust::Solution;
}

#[macro_export]
macro_rules! modules {
    ( $( $pre:ident ,)* ) => {
        compile_error!("One day must be prefixed with `> `")
    };
    ( $( $pre:ident ,)* > $current:ident, $( $post:ident ,)* ) => {
        $( pub mod $pre; )*
        pub mod $current;
        $( pub mod $post; )*

        pub mod current {
            use std::error::Error;
            use super::prelude::*;

            pub fn run() -> Result<Solution, Box<dyn Error>> {
                let path = concat!("./inputs/", stringify!($current), ".txt");
                let input = std::fs::read_to_string(path)?;

                Ok(super::day01::run(&input).map_err(|_| concat!("Error running day ", stringify!($current)))?)
            }
        }
    }
}
