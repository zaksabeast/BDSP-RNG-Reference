use std::num::ParseIntError;

pub fn parse_hex(s: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(s, 16)
}
