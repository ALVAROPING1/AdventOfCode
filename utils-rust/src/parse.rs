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
