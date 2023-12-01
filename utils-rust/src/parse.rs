#[must_use]
pub fn from_ascii_digit(input: u8) -> u32 {
    u32::from(input) - '0' as u32
}
